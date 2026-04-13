#!/usr/bin/env node
const http = require('http');
const fs = require('fs');
const path = require('path');
const { URL } = require('url');

const ROOT = path.resolve(__dirname, '..');
const DOCS_DIR = path.join(ROOT, 'docs');
const HOST = process.env.CLIPTUBE_WEB_HOST || '127.0.0.1';
const PORT = Number(process.env.CLIPTUBE_WEB_PORT || '4174');
const USER_AGENT = 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36';

function extractVideoId(input) {
  input = String(input || '').trim();
  if (/^[A-Za-z0-9_-]{11}$/.test(input)) return input;
  try {
    const url = new URL(input);
    if (url.hostname === 'youtu.be') return url.pathname.slice(1);
    if (url.pathname === '/watch') return url.searchParams.get('v');
    if (/^\/(shorts|embed|live)\//.test(url.pathname)) return url.pathname.split('/')[2];
  } catch {}
  return null;
}

function languageMatches(trackLang, requested) {
  const a = String(trackLang || '').toLowerCase();
  const b = String(requested || '').toLowerCase();
  return a === b || a.split('-')[0] === b || b.split('-')[0] === a;
}

function selectTrack(tracks, preferred) {
  const manual = tracks.filter((t) => t.kind !== 'asr');
  for (const lang of preferred) {
    const found = manual.find((t) => languageMatches(t.languageCode, lang));
    if (found) return found;
  }
  for (const lang of preferred) {
    const found = tracks.find((t) => languageMatches(t.languageCode, lang));
    if (found) return found;
  }
  return manual[0] || tracks[0] || null;
}

function decodeEntities(text) {
  return text
    .replace(/&amp;/g, '&')
    .replace(/&lt;/g, '<')
    .replace(/&gt;/g, '>')
    .replace(/&quot;/g, '"')
    .replace(/&#39;/g, "'")
    .replace(/&#(\d+);/g, (_, n) => String.fromCharCode(Number(n)))
    .replace(/&#x([0-9a-f]+);/gi, (_, n) => String.fromCharCode(parseInt(n, 16)));
}

function parseTranscriptXml(xml) {
  const lines = [];
  for (const match of xml.matchAll(/<text\b[^>]*>([\s\S]*?)<\/text>/g)) {
    const cleaned = decodeEntities(match[1].replace(/<[^>]+>/g, '').trim());
    if (cleaned) lines.push(cleaned);
  }
  if (!lines.length) throw new Error('Transcript XML empty');
  return lines.join('\n');
}

function extractApiKey(html) {
  for (const pattern of [
    /"INNERTUBE_API_KEY"\s*:\s*"([A-Za-z0-9_-]+)"/,
    /ytcfg\.set\(\{[\s\S]*?"INNERTUBE_API_KEY"\s*:\s*"([A-Za-z0-9_-]+)"/,
  ]) {
    const match = html.match(pattern);
    if (match?.[1]) return match[1];
  }
  return null;
}

async function fetchText(url, headers = {}) {
  const res = await fetch(url, { headers: { 'user-agent': USER_AGENT, ...headers } });
  if (!res.ok) throw new Error(`HTTP ${res.status} @ ${url}`);
  return await res.text();
}

async function fetchJson(url, init = {}) {
  const res = await fetch(url, {
    ...init,
    headers: { 'user-agent': USER_AGENT, ...(init.headers || {}) },
  });
  if (!res.ok) throw new Error(`HTTP ${res.status} @ ${url}`);
  return await res.json();
}

async function fetchTranscript(input, preferredLanguages = []) {
  const videoId = extractVideoId(input);
  if (!videoId) throw new Error('Invalid YouTube URL or video ID');
  const watchUrl = `https://www.youtube.com/watch?v=${videoId}`;
  const html = await fetchText(watchUrl);
  const apiKey = extractApiKey(html);
  if (!apiKey) throw new Error('Missing INNERTUBE_API_KEY');

  const player = await fetchJson(`https://www.youtube.com/youtubei/v1/player?key=${apiKey}`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({
      context: { client: { clientName: 'ANDROID', clientVersion: '20.10.38' } },
      videoId,
    }),
  });

  const captions = player?.captions?.playerCaptionsTracklistRenderer?.captionTracks || [];
  if (!captions.length) {
    const reason = player?.playabilityStatus?.reason || 'No caption tracks found';
    throw new Error(reason);
  }

  const track = selectTrack(captions, preferredLanguages);
  if (!track) throw new Error('No caption track selected');

  const xml = await fetchText(String(track.baseUrl || '').replace('&fmt=srv3', ''));
  const fullText = parseTranscriptXml(xml);
  return {
    sourceUrl: watchUrl,
    videoId,
    languageLabel: track.name?.simpleText || track.languageCode || 'unknown',
    isGenerated: track.kind === 'asr',
    fullText,
  };
}

function sendJson(res, status, payload) {
  res.writeHead(status, {
    'content-type': 'application/json; charset=utf-8',
    'access-control-allow-origin': '*',
  });
  res.end(JSON.stringify(payload));
}

function sendFile(res, filePath) {
  const ext = path.extname(filePath).toLowerCase();
  const contentTypes = {
    '.html': 'text/html; charset=utf-8',
    '.css': 'text/css; charset=utf-8',
    '.js': 'application/javascript; charset=utf-8',
    '.svg': 'image/svg+xml',
    '.png': 'image/png',
    '.jpg': 'image/jpeg',
    '.jpeg': 'image/jpeg',
    '.webp': 'image/webp',
    '.json': 'application/json; charset=utf-8',
  };
  const type = contentTypes[ext] || 'application/octet-stream';
  fs.createReadStream(filePath)
    .on('error', () => {
      res.writeHead(500);
      res.end('File error');
    })
    .once('open', () => {
      res.writeHead(200, { 'content-type': type });
    })
    .pipe(res);
}

const server = http.createServer(async (req, res) => {
  try {
    const reqUrl = new URL(req.url, `http://${req.headers.host}`);
    if (reqUrl.pathname === '/api/health') {
      return sendJson(res, 200, { ok: true });
    }
    if (reqUrl.pathname === '/api/transcript') {
      const input = reqUrl.searchParams.get('input') || '';
      const languages = (reqUrl.searchParams.get('languages') || '')
        .split(',')
        .map((part) => part.trim())
        .filter(Boolean);
      try {
        const data = await fetchTranscript(input, languages);
        return sendJson(res, 200, data);
      } catch (error) {
        return sendJson(res, 400, { error: String(error.message || error) });
      }
    }

    let pathname = reqUrl.pathname;
    if (pathname === '/') pathname = '/index.html';
    if (pathname.endsWith('/')) pathname += 'index.html';
    let filePath = path.join(DOCS_DIR, pathname.replace(/^\/+/, ''));
    if (!filePath.startsWith(DOCS_DIR)) {
      res.writeHead(403);
      return res.end('Forbidden');
    }
    if (!fs.existsSync(filePath) && !path.extname(filePath)) {
      const asIndex = path.join(filePath, 'index.html');
      if (fs.existsSync(asIndex)) filePath = asIndex;
    }
    if (fs.existsSync(filePath) && fs.statSync(filePath).isFile()) {
      return sendFile(res, filePath);
    }
    res.writeHead(404);
    res.end('Not found');
  } catch (error) {
    sendJson(res, 500, { error: String(error.message || error) });
  }
});

server.listen(PORT, HOST, () => {
  console.log(`ClipTube web runtime server on http://${HOST}:${PORT}`);
});

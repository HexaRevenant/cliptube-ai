use std::collections::HashMap;
use std::process::Command;

use regex::Regex;
use reqwest::{Client, cookie::Jar};
use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;

use crate::transcript_helpers::{extract_api_key, extract_video_id, parse_transcript_xml};

const WATCH_URL: &str = "https://www.youtube.com/watch?v={video_id}";
const INNERTUBE_API_URL: &str = "https://www.youtube.com/youtubei/v1/player?key={api_key}";
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36";

#[derive(Clone, Debug)]
pub struct TranscriptSegment {
    pub start: f64,
    pub duration: f64,
    pub text: String,
}

impl TranscriptSegment {
    pub fn start_formatted(&self) -> String {
        let total_secs = self.start as u64;
        let hours = total_secs / 3600;
        let mins = (total_secs % 3600) / 60;
        let secs = total_secs % 60;
        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, mins, secs)
        } else {
            format!("{:02}:{:02}", mins, secs)
        }
    }
}

#[derive(Clone, Debug)]
pub struct TranscriptBundle {
    pub source_url: String,
    pub video_id: String,
    pub title: Option<String>,
    pub channel: Option<String>,
    pub language_label: String,
    pub is_generated: bool,
    pub full_text: String,
    pub segments: Vec<TranscriptSegment>,
}

#[derive(Clone)]
pub struct TranscriptService {
    client: Client,
}

impl TranscriptService {
    pub fn new() -> Self {
        let jar = Jar::default();
        let client = Client::builder()
            .cookie_provider(std::sync::Arc::new(jar))
            .user_agent(USER_AGENT)
            .build()
            .expect("no se pudo crear cliente HTTP");

        Self { client }
    }

    pub async fn fetch(
        &self,
        input: &str,
        requested_languages: &[String],
    ) -> Result<TranscriptBundle, TranscriptError> {
        let video_id = extract_video_id(input)?;
        let source_url = WATCH_URL.replace("{video_id}", &video_id);

        match self
            .fetch_from_youtube_internal(&video_id, &source_url, requested_languages)
            .await
        {
            Ok(bundle) => Ok(bundle),
            Err(primary_error) => match self
                .fetch_with_ytdlp_fallback(&video_id, &source_url, requested_languages)
                .await
            {
                Ok(bundle) => Ok(bundle),
                Err(_) => Err(primary_error),
            },
        }
    }

    async fn fetch_from_youtube_internal(
        &self,
        video_id: &str,
        source_url: &str,
        requested_languages: &[String],
    ) -> Result<TranscriptBundle, TranscriptError> {
        let html = self.fetch_video_html(video_id).await?;
        let api_key = extract_api_key(&html).ok_or(TranscriptError::ApiKeyNotFound)?;

        let player = self.fetch_player_response(video_id, &api_key).await?;
        let captions = player
            .captions
            .and_then(|captions| captions.player_captions_tracklist_renderer)
            .ok_or(TranscriptError::TranscriptsDisabled)?;

        let track = select_track(&captions.caption_tracks, requested_languages)
            .ok_or(TranscriptError::NoTranscriptAvailable)?;

        let xml = self.fetch_transcript_xml(&track.base_url).await?;
        let segments = parse_transcript_xml(&xml)?;
        let full_text = segments
            .iter()
            .map(|s| s.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        let (title, channel) = extract_video_metadata(&html);

        Ok(TranscriptBundle {
            source_url: source_url.to_string(),
            video_id: video_id.to_string(),
            title,
            channel,
            language_label: track.display_name(),
            is_generated: track.kind.as_deref() == Some("asr"),
            full_text,
            segments,
        })
    }

    async fn fetch_with_ytdlp_fallback(
        &self,
        video_id: &str,
        source_url: &str,
        requested_languages: &[String],
    ) -> Result<TranscriptBundle, TranscriptError> {
        let output = Command::new("yt-dlp")
            .args([
                "--dump-single-json",
                "--no-warnings",
                "--skip-download",
                source_url,
            ])
            .output()?;
        if !output.status.success() {
            return Err(TranscriptError::YtDlp(format!(
                "yt-dlp devolvió estado {}",
                output.status
            )));
        }
        let meta: Value = serde_json::from_slice(&output.stdout)?;
        let (track_url, language_code, is_generated) =
            select_ytdlp_caption_track(&meta, requested_languages)
                .ok_or(TranscriptError::NoTranscriptAvailable)?;

        let fetch_url = if track_url.contains("fmt=") {
            track_url
        } else if track_url.contains('?') {
            format!("{track_url}&fmt=vtt")
        } else {
            format!("{track_url}?fmt=vtt")
        };

        let vtt = self
            .client
            .get(fetch_url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        let segments = parse_vtt_segments(&vtt)?;
        let full_text = segments
            .iter()
            .map(|s| s.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        let title = meta
            .get("title")
            .and_then(|v| v.as_str())
            .map(ToOwned::to_owned);
        let channel = meta
            .get("channel")
            .and_then(|v| v.as_str())
            .map(ToOwned::to_owned);

        Ok(TranscriptBundle {
            source_url: source_url.to_string(),
            video_id: video_id.to_string(),
            title,
            channel,
            language_label: format!("{language_code} (yt-dlp)"),
            is_generated,
            full_text,
            segments,
        })
    }

    async fn fetch_video_html(&self, video_id: &str) -> Result<String, TranscriptError> {
        let url = WATCH_URL.replace("{video_id}", video_id);
        let mut html = self.fetch_html(&url).await?;

        if html.contains("https://consent.youtube.com/s") {
            let consent_value = Regex::new(r#"name=\"v\" value=\"(.*?)\""#)
                .ok()
                .and_then(|regex| regex.captures(&html))
                .and_then(|captures| captures.get(1))
                .map(|value| value.as_str().to_string())
                .ok_or(TranscriptError::ConsentCookieFailed)?;

            let cookie = format!("CONSENT=YES+{consent_value}");
            html = self
                .client
                .get(url)
                .header("Cookie", cookie)
                .send()
                .await?
                .error_for_status()?
                .text()
                .await?;
        }

        Ok(html)
    }

    async fn fetch_html(&self, url: &str) -> Result<String, TranscriptError> {
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.text().await?)
    }

    async fn fetch_player_response(
        &self,
        video_id: &str,
        api_key: &str,
    ) -> Result<PlayerResponse, TranscriptError> {
        let payload = serde_json::json!({
            "context": {
                "client": {
                    "clientName": "ANDROID",
                    "clientVersion": "20.10.38"
                }
            },
            "videoId": video_id
        });

        let url = INNERTUBE_API_URL.replace("{api_key}", api_key);
        let response = self
            .client
            .post(url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        let player: PlayerResponse = response.json().await?;

        if let Some(playability) = &player.playability_status {
            let status = playability.status.as_deref().unwrap_or("OK");
            if status != "OK" {
                return Err(TranscriptError::VideoUnplayable(
                    playability
                        .reason
                        .clone()
                        .unwrap_or_else(|| format!("estado {status}")),
                ));
            }
        }

        Ok(player)
    }

    async fn fetch_transcript_xml(&self, base_url: &str) -> Result<String, TranscriptError> {
        let clean_url = base_url.replace("&fmt=srv3", "");
        let response = self
            .client
            .get(clean_url)
            .send()
            .await?
            .error_for_status()?;
        Ok(response.text().await?)
    }
}

fn select_ytdlp_caption_track(
    meta: &Value,
    preferred_languages: &[String],
) -> Option<(String, String, bool)> {
    let manual = extract_ytdlp_tracks(meta.get("subtitles"), false);
    let generated = extract_ytdlp_tracks(meta.get("automatic_captions"), true);

    for requested in preferred_languages {
        if let Some(track) = manual
            .iter()
            .find(|(lang, _, _)| language_matches(lang, requested))
        {
            return Some(track.clone());
        }
        if let Some(track) = generated
            .iter()
            .find(|(lang, _, _)| language_matches(lang, requested))
        {
            return Some(track.clone());
        }
    }

    manual
        .into_iter()
        .next()
        .or_else(|| generated.into_iter().next())
}

fn extract_ytdlp_tracks(map_value: Option<&Value>, is_generated: bool) -> Vec<(String, String, bool)> {
    let mut tracks = Vec::new();
    let Some(map) = map_value.and_then(|v| v.as_object()) else {
        return tracks;
    };

    for (lang, entries) in map {
        let Some(arr) = entries.as_array() else {
            continue;
        };
        let mut picked_url: Option<String> = None;

        for entry in arr {
            let ext = entry.get("ext").and_then(|v| v.as_str()).unwrap_or_default();
            let url = entry.get("url").and_then(|v| v.as_str()).unwrap_or_default();
            if url.is_empty() {
                continue;
            }
            if ext.eq_ignore_ascii_case("vtt") {
                picked_url = Some(url.to_string());
                break;
            }
            if picked_url.is_none() {
                picked_url = Some(url.to_string());
            }
        }

        if let Some(url) = picked_url {
            tracks.push((lang.to_string(), url, is_generated));
        }
    }

    tracks
}

fn parse_vtt_segments(vtt: &str) -> Result<Vec<TranscriptSegment>, TranscriptError> {
    let tag_re = Regex::new(r"<[^>]+>").map_err(|e| TranscriptError::TranscriptParse(e.to_string()))?;
    let mut segments = Vec::new();
    let mut lines = vtt.lines().peekable();

    while let Some(line) = lines.next() {
        let t = line.trim();
        if !t.contains("-->") {
            continue;
        }
        let mut parts = t.split("-->");
        let start_raw = parts.next().map(str::trim).unwrap_or_default();
        let end_raw = parts
            .next()
            .map(str::trim)
            .and_then(|v| v.split_whitespace().next())
            .unwrap_or_default();

        let start = parse_vtt_timestamp(start_raw).ok_or_else(|| {
            TranscriptError::TranscriptParse(format!("Timestamp de inicio inválido: {start_raw}"))
        })?;
        let end = parse_vtt_timestamp(end_raw).ok_or_else(|| {
            TranscriptError::TranscriptParse(format!("Timestamp de fin inválido: {end_raw}"))
        })?;

        let mut text_lines = Vec::new();
        while let Some(next_line) = lines.peek() {
            if next_line.trim().is_empty() {
                lines.next();
                break;
            }
            let cleaned = tag_re.replace_all(next_line.trim(), "");
            let decoded = html_escape::decode_html_entities(cleaned.as_ref()).to_string();
            if !decoded.is_empty() {
                text_lines.push(decoded);
            }
            lines.next();
        }

        let text = text_lines.join(" ").trim().to_string();
        if text.is_empty() {
            continue;
        }

        let duration = (end - start).max(0.0);
        segments.push(TranscriptSegment {
            start,
            duration,
            text,
        });
    }

    if segments.is_empty() {
        return Err(TranscriptError::TranscriptParse(
            "VTT sin segmentos útiles".to_string(),
        ));
    }

    Ok(segments)
}

fn parse_vtt_timestamp(raw: &str) -> Option<f64> {
    let normalized = raw.replace(',', ".");
    let parts: Vec<&str> = normalized.split(':').collect();
    match parts.len() {
        3 => {
            let h = parts[0].parse::<f64>().ok()?;
            let m = parts[1].parse::<f64>().ok()?;
            let s = parts[2].parse::<f64>().ok()?;
            Some(h * 3600.0 + m * 60.0 + s)
        }
        2 => {
            let m = parts[0].parse::<f64>().ok()?;
            let s = parts[1].parse::<f64>().ok()?;
            Some(m * 60.0 + s)
        }
        _ => None,
    }
}

fn select_track<'a>(
    tracks: &'a [CaptionTrack],
    preferred_languages: &[String],
) -> Option<&'a CaptionTrack> {
    if tracks.is_empty() {
        return None;
    }

    let mut manual: HashMap<&str, &CaptionTrack> = HashMap::new();
    let mut generated: HashMap<&str, &CaptionTrack> = HashMap::new();

    for track in tracks {
        if track.kind.as_deref() == Some("asr") {
            generated.insert(track.language_code.as_str(), track);
        } else {
            manual.insert(track.language_code.as_str(), track);
        }
    }

    for requested in preferred_languages {
        if let Some(found) = manual
            .values()
            .find(|track| language_matches(&track.language_code, requested))
        {
            return Some(found);
        }
        if let Some(found) = generated
            .values()
            .find(|track| language_matches(&track.language_code, requested))
        {
            return Some(found);
        }
    }

    tracks
        .iter()
        .find(|track| track.kind.as_deref() != Some("asr"))
        .or_else(|| tracks.first())
}

fn language_matches(track_lang: &str, requested: &str) -> bool {
    let track_lang = track_lang.to_lowercase();
    let requested = requested.to_lowercase();

    track_lang == requested
        || track_lang
            .split('-')
            .next()
            .map(|part| part == requested)
            .unwrap_or(false)
        || requested
            .split('-')
            .next()
            .map(|part| part == track_lang)
            .unwrap_or(false)
}

#[derive(Debug, Deserialize)]
struct PlayerResponse {
    #[serde(rename = "playabilityStatus")]
    playability_status: Option<PlayabilityStatus>,
    captions: Option<Captions>,
}

#[derive(Debug, Deserialize)]
struct PlayabilityStatus {
    status: Option<String>,
    reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Captions {
    #[serde(rename = "playerCaptionsTracklistRenderer")]
    player_captions_tracklist_renderer: Option<CaptionTrackList>,
}

#[derive(Debug, Deserialize)]
struct CaptionTrackList {
    #[serde(default, rename = "captionTracks")]
    caption_tracks: Vec<CaptionTrack>,
}

#[derive(Debug, Deserialize)]
struct CaptionTrack {
    #[serde(rename = "baseUrl")]
    base_url: String,
    name: CaptionName,
    #[serde(rename = "languageCode")]
    language_code: String,
    kind: Option<String>,
}

impl CaptionTrack {
    fn display_name(&self) -> String {
        self.name.resolve_text()
    }
}

#[derive(Debug, Deserialize)]
struct CaptionName {
    #[serde(default, rename = "simpleText")]
    simple_text: Option<String>,
    #[serde(default)]
    runs: Vec<CaptionRun>,
}

impl CaptionName {
    fn resolve_text(&self) -> String {
        self.simple_text.clone().unwrap_or_else(|| {
            self.runs
                .iter()
                .map(|run| run.text.clone())
                .collect::<Vec<_>>()
                .join("")
        })
    }
}

#[derive(Debug, Deserialize)]
struct CaptionRun {
    text: String,
}

#[derive(Debug, Error)]
pub enum TranscriptError {
    #[error("La URL o el video ID no son válidos")]
    InvalidUrlOrVideoId,
    #[error("No pude extraer la API key interna de YouTube")]
    ApiKeyNotFound,
    #[error("Este video no tiene subtítulos disponibles")]
    NoTranscriptAvailable,
    #[error("YouTube indica que los subtítulos están deshabilitados")]
    TranscriptsDisabled,
    #[error("No pude crear la cookie de consentimiento de YouTube")]
    ConsentCookieFailed,
    #[error("El video no se puede reproducir: {0}")]
    VideoUnplayable(String),
    #[error("Error parseando la transcripción: {0}")]
    TranscriptParse(String),
    #[error("yt-dlp fallback error: {0}")]
    YtDlp(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Xml(#[from] roxmltree::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

fn extract_video_metadata(html: &str) -> (Option<String>, Option<String>) {
    let title = Regex::new(r#"<meta\s+property="og:title"\s+content="([^"]*)""#)
        .ok()
        .and_then(|re| re.captures(html))
        .and_then(|cap| cap.get(1))
        .map(|m| html_escape::decode_html_entities(m.as_str()).to_string());

    let channel = Regex::new(r#"<link\s+itemprop="name"\s+content="([^"]*)""#)
        .ok()
        .and_then(|re| re.captures(html))
        .and_then(|cap| cap.get(1))
        .map(|m| html_escape::decode_html_entities(m.as_str()).to_string());

    (title, channel)
}

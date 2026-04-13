use std::collections::HashMap;

use regex::Regex;
use reqwest::{Client, cookie::Jar};
use serde::Deserialize;
use thiserror::Error;

use crate::transcript_helpers::{extract_api_key, extract_video_id, parse_transcript_xml};

const WATCH_URL: &str = "https://www.youtube.com/watch?v={video_id}";
const INNERTUBE_API_URL: &str = "https://www.youtube.com/youtubei/v1/player?key={api_key}";
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36";

#[derive(Clone, Debug)]
pub struct TranscriptBundle {
    pub source_url: String,
    pub video_id: String,
    pub language_label: String,
    pub is_generated: bool,
    pub full_text: String,
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

        let html = self.fetch_video_html(&video_id).await?;
        let api_key = extract_api_key(&html).ok_or(TranscriptError::ApiKeyNotFound)?;

        let player = self.fetch_player_response(&video_id, &api_key).await?;
        let captions = player
            .captions
            .and_then(|captions| captions.player_captions_tracklist_renderer)
            .ok_or(TranscriptError::TranscriptsDisabled)?;

        let track = select_track(&captions.caption_tracks, requested_languages)
            .ok_or(TranscriptError::NoTranscriptAvailable)?;

        let xml = self.fetch_transcript_xml(&track.base_url).await?;
        let full_text = parse_transcript_xml(&xml)?;

        Ok(TranscriptBundle {
            source_url,
            video_id,
            language_label: track.display_name(),
            is_generated: track.kind.as_deref() == Some("asr"),
            full_text,
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
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Xml(#[from] roxmltree::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

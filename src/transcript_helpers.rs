use html_escape::decode_html_entities;
use regex::Regex;
use roxmltree::{Document, Node};
use url::Url;

use crate::transcript::{TranscriptError, TranscriptSegment};

pub(crate) fn extract_video_id(input: &str) -> Result<String, TranscriptError> {
    let input = input.trim();
    let id_regex = Regex::new(r"^[A-Za-z0-9_-]{11}$").expect("regex válida");

    if id_regex.is_match(input) {
        return Ok(input.to_string());
    }

    let url = Url::parse(input).map_err(|_| TranscriptError::InvalidUrlOrVideoId)?;

    let candidate = if matches!(url.domain(), Some("youtu.be")) {
        url.path_segments()
            .and_then(|mut segments| segments.next())
            .map(str::to_string)
    } else {
        match url.path() {
            "/watch" => url
                .query_pairs()
                .find(|(key, _)| key == "v")
                .map(|(_, value)| value.to_string()),
            path if path.starts_with("/shorts/")
                || path.starts_with("/embed/")
                || path.starts_with("/live/") =>
            {
                path.split('/').nth(2).map(str::to_string)
            }
            _ => None,
        }
    };

    let candidate = candidate.ok_or(TranscriptError::InvalidUrlOrVideoId)?;

    if id_regex.is_match(&candidate) {
        Ok(candidate)
    } else {
        Err(TranscriptError::InvalidUrlOrVideoId)
    }
}

pub(crate) fn extract_api_key(html: &str) -> Option<String> {
    for pattern in [r#""INNERTUBE_API_KEY":\s*"([A-Za-z0-9_-]+)""#] {
        if let Ok(regex) = Regex::new(pattern)
            && let Some(captures) = regex.captures(html)
            && let Some(value) = captures.get(1)
        {
            return Some(value.as_str().to_string());
        }
    }

    None
}

pub(crate) fn parse_transcript_xml(xml: &str) -> Result<Vec<TranscriptSegment>, TranscriptError> {
    let doc = Document::parse(xml)?;
    let strip_tags = Regex::new(r"<[^>]+>").expect("regex válida");

    let mut segments = Vec::new();

    for node in doc.descendants().filter(|node| node.has_tag_name("text")) {
        let text = collect_node_text(node);
        let decoded = decode_html_entities(&text).to_string();
        let cleaned = strip_tags.replace_all(&decoded, "").trim().to_string();
        if !cleaned.is_empty() {
            let start = node
                .attribute("start")
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0);
            let duration = node
                .attribute("dur")
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0);
            segments.push(TranscriptSegment {
                start,
                duration,
                text: cleaned,
            });
        }
    }

    if segments.is_empty() {
        return Err(TranscriptError::TranscriptParse(
            "La transcripción llegó vacía".into(),
        ));
    }

    Ok(segments)
}

fn collect_node_text(node: Node<'_, '_>) -> String {
    node.descendants()
        .filter_map(|child| child.text())
        .collect::<Vec<_>>()
        .join("")
}

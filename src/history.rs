use regex::Regex;
use std::fs;

use crate::db::HistoryEntry;

pub fn parse_watch_history(
    html_path: &str,
) -> Result<Vec<HistoryEntry>, Box<dyn std::error::Error>> {
    let html = fs::read_to_string(html_path)?;

    let block_re = Regex::new(
        r#"<div class="content-cell mdl-cell mdl-cell--6-col mdl-typography--body-1">(.*?)</div>"#,
    )?;
    let video_re =
        Regex::new(r#"<a href="https://www\.youtube\.com/watch\?v=([^"]+)"[^>]*>(.*?)</a>"#)?;
    let channel_re =
        Regex::new(r#"<a href="https://www\.youtube\.com/channel/([^"]+)"[^>]*>(.*?)</a>"#)?;
    let date_re = Regex::new(r#"</a><br>([^<]+)<br>\s*$"#)?;

    let mut entries = Vec::new();

    for cap in block_re.captures_iter(&html) {
        let block = &cap[1];

        let video_match = match video_re.captures(block) {
            Some(m) => m,
            None => continue,
        };
        let video_id = video_match[1].to_string();
        let title = html_escape::decode_html_entities(&video_match[2]).to_string();

        let channel_match = match channel_re.captures(block) {
            Some(m) => m,
            None => continue,
        };
        let channel = html_escape::decode_html_entities(&channel_match[2]).to_string();

        let source_url = format!("https://www.youtube.com/watch?v={}", video_id);

        let watched_at = date_re.captures(block).and_then(|cap| cap.get(1)).map(|m| {
            html_escape::decode_html_entities(m.as_str())
                .trim()
                .to_string()
        });

        entries.push(HistoryEntry {
            video_id,
            source_url,
            title,
            channel,
            watched_at,
        });
    }

    Ok(entries)
}

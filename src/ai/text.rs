use std::collections::{HashMap, HashSet};

pub(crate) fn normalized_lang(code: &str) -> &'static str {
    let lower = code.to_lowercase();
    let base = lower.split(['-', '_']).next().unwrap_or_default();
    match base {
        "en" => "en",
        "pt" => "pt",
        "fr" => "fr",
        "de" => "de",
        "ja" => "ja",
        "zh" => "zh",
        "ru" => "ru",
        "ar" => "ar",
        "hi" => "hi",
        _ => "es",
    }
}

pub(crate) fn clean_transcript(input: &str) -> String {
    let mut cleaned_lines = Vec::new();
    let mut seen = HashSet::new();

    for raw_line in input.lines() {
        let mut line = raw_line.replace('♪', " ");
        for noise in ["[Music]", "[Applause]", "[Laughter]"] {
            line = line.replace(noise, " ");
        }
        line = collapse_whitespace(&line);
        line = line
            .trim_matches(|c: char| c == '[' || c == ']')
            .trim()
            .to_string();
        line = dedupe_repeated_halves(&line);
        line = collapse_whitespace(&line);

        if line.is_empty() {
            continue;
        }

        let normalized = normalize_for_dedup(&line);
        if normalized.len() < 8 || is_noise_line(&normalized) {
            continue;
        }
        if !seen.insert(normalized) {
            continue;
        }

        cleaned_lines.push(line);
    }

    cleaned_lines.join("\n")
}

pub(crate) fn dedupe_repeated_halves(line: &str) -> String {
    let words = line.split_whitespace().collect::<Vec<_>>();
    if words.len() >= 6 && words.len() % 2 == 0 {
        let half = words.len() / 2;
        if words[..half] == words[half..] {
            return words[..half].join(" ");
        }
    }
    line.to_string()
}

pub(crate) fn normalize_for_dedup(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn collapse_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(crate) fn is_noise_line(line: &str) -> bool {
    let words = line.split_whitespace().collect::<Vec<_>>();
    words.len() < 3 || words.windows(2).all(|w| w[0] == w[1])
}

pub(crate) fn limit_chars(input: &str, max_chars: usize) -> String {
    input.chars().take(max_chars).collect()
}

pub(crate) fn split_into_chunks(input: &str, chunk_size: usize, max_chunks: usize) -> Vec<String> {
    if input.is_empty() {
        return Vec::new();
    }

    let mut chunks = Vec::new();
    let mut current = String::new();

    for line in input.lines() {
        let proposed_len = current.chars().count() + line.chars().count() + 1;
        if proposed_len > chunk_size && !current.is_empty() {
            chunks.push(current.trim().to_string());
            current.clear();
            if chunks.len() >= max_chunks {
                break;
            }
        }

        if !current.is_empty() {
            current.push('\n');
        }
        current.push_str(line);
    }

    if chunks.len() < max_chunks && !current.trim().is_empty() {
        chunks.push(current.trim().to_string());
    }

    chunks
}

pub(crate) fn select_relevant_sentences(text: &str, limit: usize) -> Vec<String> {
    let sentences = text
        .split_terminator(['.', '!', '?', '\n'])
        .map(str::trim)
        .filter(|s| s.len() > 35)
        .map(normalize_text)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    if sentences.is_empty() {
        return Vec::new();
    }

    let mut freq = HashMap::<String, usize>::new();
    for sentence in &sentences {
        for word in keywords(sentence) {
            *freq.entry(word).or_default() += 1;
        }
    }

    let mut scored = sentences
        .iter()
        .enumerate()
        .map(|(idx, sentence)| {
            let score = keywords(sentence)
                .into_iter()
                .map(|w| freq.get(&w).copied().unwrap_or(0))
                .sum::<usize>();
            (idx, score, sentence.clone())
        })
        .collect::<Vec<_>>();

    scored.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let mut top = scored.into_iter().take(limit).collect::<Vec<_>>();
    top.sort_by_key(|item| item.0);
    top.into_iter().map(|(_, _, s)| s).collect()
}

pub(crate) fn keywords(sentence: &str) -> Vec<String> {
    sentence
        .to_lowercase()
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric() && c != 'ñ'))
        .filter(|w| w.len() > 4)
        .map(|w| w.to_string())
        .collect()
}

pub(crate) fn normalize_text(input: &str) -> String {
    collapse_whitespace(input)
        .trim()
        .trim_matches('.')
        .to_string()
        + "."
}

pub(crate) fn dedup_items(items: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for item in items {
        let key = normalize_for_dedup(&item);
        if key.len() < 8 || !seen.insert(key) {
            continue;
        }
        out.push(item);
    }
    out
}

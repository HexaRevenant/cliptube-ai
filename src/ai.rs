mod prompts;
mod text;

use serde::Deserialize;
use thiserror::Error;

use self::{
    prompts::{build_chat_text, build_chunk_prompt, build_combine_prompt, build_final_prompt},
    text::{
        clean_transcript, dedup_items, limit_chars, normalize_text, select_relevant_sentences,
        split_into_chunks,
    },
};
use crate::transcript::TranscriptBundle;

#[derive(Clone, Copy, Debug)]
pub enum OutputStyle {
    Chat,
    Executive,
    Bullets,
}

impl OutputStyle {
    pub fn label(self, language_code: &str) -> &'static str {
        let lang = language_code.to_lowercase();
        match self {
            Self::Chat if lang.starts_with("pt") => "Chat pronto para colar",
            Self::Executive if lang.starts_with("pt") => "Resumo executivo",
            Self::Bullets if lang.starts_with("pt") => "Bullets diretos",
            Self::Chat if lang.starts_with("fr") => "Chat prêt à coller",
            Self::Executive if lang.starts_with("fr") => "Résumé exécutif",
            Self::Bullets if lang.starts_with("fr") => "Points directs",
            Self::Chat if lang.starts_with("de") => "Chat zum Einfügen",
            Self::Executive if lang.starts_with("de") => "Management-Zusammenfassung",
            Self::Bullets if lang.starts_with("de") => "Direkte Stichpunkte",
            Self::Chat if lang.starts_with("ja") => "貼り付け用チャット",
            Self::Executive if lang.starts_with("ja") => "エグゼクティブ要約",
            Self::Bullets if lang.starts_with("ja") => "箇条書き",
            Self::Chat if lang.starts_with("zh") => "可直接粘贴的聊天文本",
            Self::Executive if lang.starts_with("zh") => "执行摘要",
            Self::Bullets if lang.starts_with("zh") => "精简要点",
            Self::Chat if lang.starts_with("ru") => "Текст для чата",
            Self::Executive if lang.starts_with("ru") => "Краткое резюме",
            Self::Bullets if lang.starts_with("ru") => "Короткие пункты",
            Self::Chat if lang.starts_with("ar") => "نص جاهز للدردشة",
            Self::Executive if lang.starts_with("ar") => "ملخص تنفيذي",
            Self::Bullets if lang.starts_with("ar") => "نقاط مباشرة",
            Self::Chat if lang.starts_with("hi") => "चैट में चिपकाने के लिए पाठ",
            Self::Executive if lang.starts_with("hi") => "कार्यकारी सारांश",
            Self::Bullets if lang.starts_with("hi") => "सीधे बुलेट पॉइंट्स",
            Self::Chat if lang.starts_with("en") => "Ready-to-paste chat",
            Self::Executive if lang.starts_with("en") => "Executive summary",
            Self::Bullets if lang.starts_with("en") => "Direct bullets",
            Self::Chat => "Chat listo para pegar",
            Self::Executive => "Resumen ejecutivo",
            Self::Bullets => "Bullets directos",
        }
    }

    pub(crate) fn chat_instruction(self) -> &'static str {
        match self {
            Self::Chat => {
                "'chat_text' debe ser un mensaje llamativo, corto y listo para copiar y pegar en un chat. Debe usar iconos/emojis, un gancho estilo clickbait PERO fiel al contenido, incluir la URL, un mini resumen potente y puntos importantes. No inventes nada."
            }
            Self::Executive => {
                "'chat_text' debe ser un texto ejecutivo y sobrio, listo para compartir con un equipo o jefatura. Debe incluir URL, contexto breve, resumen ejecutivo y puntos clave."
            }
            Self::Bullets => {
                "'chat_text' debe ser ultra directo, listo para pegar en chat, casi todo en bullets cortos, incluyendo URL y conclusiones accionables."
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct AiSummary {
    pub summary: String,
    pub key_points: Vec<String>,
    pub chat_text: String,
    pub status: String,
}

#[derive(Clone)]
pub struct SummaryService {
    client: reqwest::Client,
    model: String,
    endpoint: String,
    max_chars: usize,
    chunk_size: usize,
    max_chunks: usize,
}

impl SummaryService {
    pub fn from_env() -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(180))
            .build()
            .expect("no se pudo crear cliente HTTP");

        let max_chars = std::env::var("MAX_TRANSCRIPT_CHARS_FOR_AI")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(90_000);
        let chunk_size = std::env::var("OLLAMA_CHUNK_SIZE")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(9_000);
        let max_chunks = std::env::var("OLLAMA_MAX_CHUNKS")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(6);

        let endpoint = std::env::var("OLLAMA_CHAT_URL").unwrap_or_else(|_| {
            let host = std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "127.0.0.1".into());
            let port = std::env::var("OLLAMA_PORT").unwrap_or_else(|_| "11434".into());
            let scheme = if host.starts_with("http://") || host.starts_with("https://") {
                ""
            } else {
                "http://"
            };
            format!("{scheme}{host}:{port}/api/chat")
        });

        Self {
            client,
            model: std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "gemma4:31b-cloud".into()),
            endpoint,
            max_chars,
            chunk_size,
            max_chunks,
        }
    }

    pub fn model_name(&self) -> &str {
        &self.model
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn with_model(&self, model: impl Into<String>) -> Self {
        let mut cloned = self.clone();
        cloned.model = model.into();
        cloned
    }

    pub fn with_endpoint(&self, endpoint: impl Into<String>) -> Self {
        let mut cloned = self.clone();
        cloned.endpoint = endpoint.into();
        cloned
    }

    pub async fn list_models(&self) -> Result<Vec<String>, SummaryError> {
        let tags_url = if self.endpoint.ends_with("/api/chat") {
            self.endpoint.replace("/api/chat", "/api/tags")
        } else {
            format!("{}/api/tags", self.endpoint.trim_end_matches('/'))
        };

        let response = self.client.get(tags_url).send().await?.error_for_status()?;

        let tags: OllamaTagsResponse = response.json().await?;
        let mut names = tags
            .models
            .into_iter()
            .map(|model| model.name)
            .filter(|name| !name.trim().is_empty())
            .collect::<Vec<_>>();
        names.sort();
        names.dedup();
        Ok(names)
    }

    pub async fn summarize(
        &self,
        transcript: &TranscriptBundle,
        output_style: OutputStyle,
        response_language: &str,
    ) -> Result<AiSummary, SummaryError> {
        let cleaned = clean_transcript(&transcript.full_text);
        let limited = limit_chars(&cleaned, self.max_chars);
        let chunks = split_into_chunks(&limited, self.chunk_size, self.max_chunks);

        match self
            .summarize_with_ollama(transcript, &chunks, output_style, response_language)
            .await
        {
            Ok(mut summary) => {
                summary.status = if chunks.len() > 1 {
                    format!(
                        "Resumen IA generado con Ollama local ({}) usando {} bloques del transcript limpio.",
                        self.model,
                        chunks.len()
                    )
                } else {
                    format!("Resumen IA generado con Ollama local ({}).", self.model)
                };
                Ok(summary)
            }
            Err(error) => Ok(self.local_fallback(
                transcript,
                output_style,
                Some(error.to_string()),
                response_language,
            )),
        }
    }

    pub async fn ask_about_video(
        &self,
        source_url: &str,
        transcript_text: &str,
        current_share_text: &str,
        user_prompt: &str,
        replace_share_text: bool,
        response_language: &str,
    ) -> Result<String, SummaryError> {
        let cleaned = clean_transcript(transcript_text);
        let context = limit_chars(&cleaned, 30_000);
        let prompt = if replace_share_text {
            format!(
                "Tienes contexto de un video de YouTube.\n\
                URL: {source_url}\n\n\
                Texto listo para pegar actual:\n{current_share_text}\n\n\
                Transcripción útil:\n{context}\n\n\
                Tarea:\n{user_prompt}\n\n\
                Devuelve SOLO el nuevo texto final mejorado, sin comillas, sin explicación extra. \
                Mantén los hechos fieles al contenido y conserva la URL final. \
                Responde en el idioma {response_language}."
            )
        } else {
            format!(
                "Responde usando SOLO el contexto del video de YouTube cuando sea relevante.\n\
                URL: {source_url}\n\n\
                Texto listo para pegar actual:\n{current_share_text}\n\n\
                Transcripción útil:\n{context}\n\n\
                Pregunta/instrucción del usuario:\n{user_prompt}\n\n\
                Responde en el idioma {response_language}, claro y directo. Si propones mejoras, no inventes datos."
            )
        };

        self.run_ollama_text_prompt(&prompt).await
    }

    async fn summarize_with_ollama(
        &self,
        transcript: &TranscriptBundle,
        chunks: &[String],
        output_style: OutputStyle,
        response_language: &str,
    ) -> Result<AiSummary, SummaryError> {
        if chunks.is_empty() {
            return Err(SummaryError::InvalidResponse(
                "No hay contenido utilizable para resumir".into(),
            ));
        }

        let final_json = if chunks.len() == 1 {
            let prompt = build_final_prompt(
                &transcript.source_url,
                &chunks[0],
                output_style,
                response_language,
            );
            self.run_ollama_json_prompt(&prompt).await?
        } else {
            let mut partials = Vec::new();
            for (idx, chunk) in chunks.iter().enumerate() {
                let prompt = build_chunk_prompt(
                    idx + 1,
                    chunks.len(),
                    chunk,
                    output_style,
                    response_language,
                );
                let part = self.run_ollama_json_prompt(&prompt).await?;
                partials.push(part);
            }

            let combined_prompt = build_combine_prompt(
                &transcript.source_url,
                &partials,
                output_style,
                response_language,
            );
            self.run_ollama_json_prompt(&combined_prompt).await?
        };

        let summary = normalize_text(&final_json.summary);
        let key_points = dedup_items(final_json.key_points)
            .into_iter()
            .map(|point| normalize_text(&point))
            .filter(|point| point.len() > 10)
            .take(8)
            .collect::<Vec<_>>();
        let chat_text = if final_json.chat_text.trim().is_empty() {
            build_chat_text(
                &transcript.source_url,
                &summary,
                &key_points,
                output_style,
                response_language,
            )
        } else {
            normalize_text(&final_json.chat_text)
        };

        if summary.is_empty() {
            return Err(SummaryError::InvalidResponse(
                "Ollama devolvió JSON sin summary".into(),
            ));
        }

        Ok(AiSummary {
            summary,
            key_points,
            chat_text,
            status: String::new(),
        })
    }

    async fn run_ollama_json_prompt(
        &self,
        prompt: &str,
    ) -> Result<OllamaSummaryJson, SummaryError> {
        let payload = serde_json::json!({
            "model": self.model,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "stream": false,
            "format": "json"
        });

        let res = self
            .client
            .post(&self.endpoint)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;
        let chat_res: OllamaChatResponse = res.json().await?;
        let content = chat_res.message.content.trim().to_string();

        if content.is_empty() {
            return Err(SummaryError::EmptyResponse);
        }

        parse_model_json(&content)
    }

    async fn run_ollama_text_prompt(&self, prompt: &str) -> Result<String, SummaryError> {
        let payload = serde_json::json!({
            "model": self.model,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "stream": false
        });

        let res = self
            .client
            .post(&self.endpoint)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;
        let chat_res: OllamaChatResponse = res.json().await?;
        let content = chat_res.message.content.trim().to_string();

        if content.is_empty() {
            return Err(SummaryError::EmptyResponse);
        }

        Ok(content)
    }

    fn local_fallback(
        &self,
        transcript: &TranscriptBundle,
        output_style: OutputStyle,
        error: Option<String>,
        response_language: &str,
    ) -> AiSummary {
        let cleaned = clean_transcript(&transcript.full_text);
        let limited = limit_chars(&cleaned, self.max_chars);
        let sentences = select_relevant_sentences(&limited, 5);

        let summary = if sentences.is_empty() {
            if response_language.starts_with("en") {
                "Automatic summary could not be generated.".to_string()
            } else {
                "No se pudo generar un resumen automático.".to_string()
            }
        } else {
            sentences.join(" ")
        };

        let key_points = if sentences.is_empty() {
            vec![]
        } else {
            sentences.into_iter().map(|s| s.to_string()).collect()
        };

        let chat_text = build_chat_text(
            &transcript.source_url,
            &summary,
            &key_points,
            output_style,
            response_language,
        );

        AiSummary {
            summary,
            key_points,
            chat_text,
            status: error
                .map(|e| format!("Error en IA: {}. Usando fallback extractivo.", e))
                .unwrap_or_else(|| {
                    if response_language.starts_with("en") {
                        "Using extractive fallback.".to_string()
                    } else {
                        "Usando fallback extractivo.".to_string()
                    }
                }),
        }
    }
}

fn parse_model_json(raw: &str) -> Result<OllamaSummaryJson, SummaryError> {
    if let Ok(parsed) = serde_json::from_str::<OllamaSummaryJson>(raw) {
        return Ok(parsed);
    }

    let without_fences = raw
        .replace("```json", "")
        .replace("```JSON", "")
        .replace("```", "")
        .trim()
        .to_string();

    if let Ok(parsed) = serde_json::from_str::<OllamaSummaryJson>(&without_fences) {
        return Ok(parsed);
    }

    if let Some(snippet) = extract_json_object(&without_fences)
        && let Ok(parsed) = serde_json::from_str::<OllamaSummaryJson>(&snippet)
    {
        return Ok(parsed);
    }

    Err(SummaryError::InvalidResponse(format!(
        "Error parseando JSON de Ollama: {}",
        raw.chars().take(180).collect::<String>()
    )))
}

fn extract_json_object(raw: &str) -> Option<String> {
    let start = raw.find('{')?;
    let end = raw.rfind('}')?;
    (end > start).then(|| raw[start..=end].to_string())
}

#[derive(Clone, Debug, Deserialize)]
struct OllamaSummaryJson {
    summary: String,
    #[serde(default)]
    key_points: Vec<String>,
    #[serde(default)]
    chat_text: String,
}

#[derive(Debug, Deserialize)]
struct OllamaChatResponse {
    message: OllamaMessage,
}

#[derive(Debug, Deserialize)]
struct OllamaMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct OllamaTagsResponse {
    #[serde(default)]
    models: Vec<OllamaModelTag>,
}

#[derive(Debug, Deserialize)]
struct OllamaModelTag {
    name: String,
}

#[derive(Debug, Error)]
pub enum SummaryError {
    #[error("La respuesta de la IA vino vacía")]
    EmptyResponse,
    #[error("La respuesta de la IA no tiene el formato esperado: {0}")]
    InvalidResponse(String),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

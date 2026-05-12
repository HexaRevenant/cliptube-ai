use chrono::{Duration, Utc};
use thiserror::Error;

use crate::db::{self, ShareLinkEntry};

pub struct ShareLinkService {
    default_ttl_hours: i64,
}

impl ShareLinkService {
    pub fn from_env() -> Self {
        let default_ttl_hours = std::env::var("CLIPTUBE_SHARE_LINK_TTL_HOURS")
            .ok()
            .and_then(|v| v.parse::<i64>().ok())
            .filter(|hours| *hours > 0)
            .unwrap_or(72);
        Self { default_ttl_hours }
    }

    pub fn create_link(
        &self,
        conn: &rusqlite::Connection,
        video_id: &str,
        share_text: &str,
    ) -> Result<ShareLinkEntry, ShareLinkError> {
        if video_id.trim().is_empty() {
            return Err(ShareLinkError::Validation(
                "video_id no puede estar vacío".to_string(),
            ));
        }
        if share_text.trim().is_empty() {
            return Err(ShareLinkError::Validation(
                "share_text no puede estar vacío".to_string(),
            ));
        }

        let token = generate_token();
        let created_at = Utc::now();
        let expires_at = created_at + Duration::hours(self.default_ttl_hours);
        let entry = ShareLinkEntry {
            token,
            video_id: video_id.to_string(),
            share_text: share_text.to_string(),
            expires_at: Some(expires_at.to_rfc3339()),
            revoked: false,
            created_at: created_at.to_rfc3339(),
        };

        db::save_share_link(conn, &entry)?;
        Ok(entry)
    }

    pub fn resolve_link(
        &self,
        conn: &rusqlite::Connection,
        token: &str,
    ) -> Result<ShareLinkEntry, ShareLinkError> {
        let entry = db::load_share_link_by_token(conn, token)?
            .ok_or_else(|| ShareLinkError::NotFound(token.to_string()))?;
        if entry.revoked {
            return Err(ShareLinkError::Revoked(token.to_string()));
        }
        if let Some(expires_at) = &entry.expires_at
            && let Ok(dt) = chrono::DateTime::parse_from_rfc3339(expires_at)
            && Utc::now() > dt.with_timezone(&Utc)
        {
            return Err(ShareLinkError::Expired(token.to_string()));
        }
        Ok(entry)
    }

    pub fn revoke_link(
        &self,
        conn: &rusqlite::Connection,
        token: &str,
    ) -> Result<bool, ShareLinkError> {
        Ok(db::revoke_share_link(conn, token)?)
    }
}

fn generate_token() -> String {
    let seed = format!(
        "{}-{}",
        Utc::now().timestamp_nanos_opt().unwrap_or_default(),
        std::process::id()
    );
    let digest = std::collections::hash_map::DefaultHasher::new();
    use std::hash::{Hash, Hasher};
    let mut hasher = digest;
    seed.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

#[derive(Debug, Error)]
pub enum ShareLinkError {
    #[error("{0}")]
    Validation(String),
    #[error("Link no encontrado: {0}")]
    NotFound(String),
    #[error("Link expirado: {0}")]
    Expired(String),
    #[error("Link revocado: {0}")]
    Revoked(String),
    #[error(transparent)]
    Db(#[from] rusqlite::Error),
}

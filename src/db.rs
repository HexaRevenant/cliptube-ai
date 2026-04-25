use rusqlite::{params, Connection};

use crate::transcript::TranscriptSegment;

#[derive(Clone, Debug)]
pub struct VideoEntry {
    #[allow(dead_code)]
    pub id: i64,
    pub video_id: String,
    pub source_url: String,
    pub title: Option<String>,
    pub channel: Option<String>,
    pub summary: String,
    pub key_points: String,
    pub chat_text: String,
    pub share_text: String,
    pub transcript_text: String,
    pub transcript_char_count: i64,
    pub ai_status: String,
    pub language_label: String,
    pub is_generated: bool,
    pub subtitle_kind: String,
    pub output_style: String,
    pub output_style_index: i64,
    pub ui_language: String,
    pub model_name: String,
    pub ollama_endpoint: String,
    pub video_meta: String,
    pub watched_at: Option<String>,
    pub watched_at_sortable: i64,
    pub created_at: String,
}

impl VideoEntry {
    pub fn display_title(&self) -> String {
        self.title
            .as_ref()
            .filter(|t| !t.trim().is_empty())
            .cloned()
            .unwrap_or_else(|| {
                if self.video_id.is_empty() {
                    "Video sin título".to_string()
                } else {
                    self.video_id.clone()
                }
            })
    }
}

#[derive(Clone, Debug)]
pub struct HistoryEntry {
    pub video_id: String,
    pub source_url: String,
    pub title: String,
    pub channel: String,
    pub watched_at: Option<String>,
}

fn column_exists(conn: &Connection, table: &str, column: &str) -> Result<bool, rusqlite::Error> {
    let sql = format!("PRAGMA table_info({})", table);
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], |row| {
        let name: String = row.get(1)?;
        Ok(name)
    })?;
    for name in rows {
        if name?.eq_ignore_ascii_case(column) {
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn parse_spanish_date(date_str: &str) -> i64 {
    let s = date_str.trim().to_lowercase();
    let month_map: std::collections::HashMap<&str, u32> = [
        ("ene", 1),
        ("feb", 2),
        ("mar", 3),
        ("abr", 4),
        ("may", 5),
        ("jun", 6),
        ("jul", 7),
        ("ago", 8),
        ("sep", 9),
        ("oct", 10),
        ("nov", 11),
        ("dic", 12),
    ]
    .into_iter()
    .collect();

    // Regex: "22 abr 2026, 11:17:24 a.m. GMT-04:00"
    let re = regex::Regex::new(
        r"(\d{1,2})\s+([a-z]{3,4})\s+(\d{4}),\s+(\d{1,2}):(\d{2}):(\d{2})\s+([ap])\.m\.\s+GMT([+-]\d{2}):(\d{2})",
    );
    if let Ok(re) = re {
        if let Some(cap) = re.captures(&s) {
            let day: u32 = cap[1].parse().unwrap_or(1);
            let month_str = &cap[2];
            let year: i32 = cap[3].parse().unwrap_or(1970);
            let hour12: u32 = cap[4].parse().unwrap_or(0);
            let minute: u32 = cap[5].parse().unwrap_or(0);
            let second: u32 = cap[6].parse().unwrap_or(0);
            let ampm = &cap[7];
            let tz_hour: i32 = cap[8].parse().unwrap_or(0);
            let tz_min: i32 = cap[9].parse().unwrap_or(0);

            let month = month_map.get(month_str).copied().unwrap_or(1);

            let hour24 = if ampm == "p" && hour12 != 12 {
                hour12 + 12
            } else if ampm == "a" && hour12 == 12 {
                0
            } else {
                hour12
            };

            if let Some(dt) = chrono::NaiveDate::from_ymd_opt(year, month, day)
                .and_then(|d| d.and_hms_opt(hour24, minute, second))
            {
                let tz_seconds = (tz_hour.abs() as i32) * 3600 + (tz_min.abs() as i32) * 60;
                let tz_offset = if tz_hour < 0 {
                    chrono::FixedOffset::west_opt(tz_seconds)
                } else {
                    chrono::FixedOffset::east_opt(tz_seconds)
                }
                .unwrap_or_else(|| chrono::FixedOffset::east_opt(0).unwrap());

                let dt_with_tz = chrono::DateTime::<chrono::FixedOffset>::from_naive_utc_and_offset(
                    dt, tz_offset,
                );
                return dt_with_tz.timestamp();
            }
        }
    }

    // Fallback: try chrono parse
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
        return dt.timestamp();
    }

    0
}

pub fn db_path() -> std::path::PathBuf {
    let data_dir = dirs::data_dir().unwrap_or_else(|| {
        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
    });
    let app_dir = data_dir.join("cliptube");
    let _ = std::fs::create_dir_all(&app_dir);
    app_dir.join("cliptube.db")
}

fn maybe_migrate_from_cwd(new_path: &std::path::Path) {
    let cwd_db = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("cliptube.db");
    if cwd_db.exists() && !new_path.exists() {
        eprintln!(
            "Migrating database from {} to {}",
            cwd_db.display(),
            new_path.display()
        );
        let _ = std::fs::copy(&cwd_db, new_path);
    }
}

pub fn open_db() -> Result<Connection, rusqlite::Error> {
    let path = db_path();
    maybe_migrate_from_cwd(&path);
    let conn = Connection::open(&path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS videos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            video_id TEXT NOT NULL UNIQUE,
            source_url TEXT NOT NULL,
            title TEXT,
            channel TEXT,
            summary TEXT,
            key_points TEXT,
            chat_text TEXT,
            share_text TEXT NOT NULL,
            transcript_text TEXT,
            transcript_char_count INTEGER,
            ai_status TEXT,
            language_label TEXT,
            is_generated INTEGER,
            subtitle_kind TEXT,
            output_style TEXT,
            output_style_index INTEGER,
            ui_language TEXT,
            model_name TEXT,
            ollama_endpoint TEXT,
            video_meta TEXT,
            watched_at TEXT,
            watched_at_sortable INTEGER DEFAULT 0,
            created_at TEXT NOT NULL
        )",
        [],
    )?;
    if !column_exists(&conn, "videos", "watched_at")? {
        let _ = conn.execute("ALTER TABLE videos ADD COLUMN watched_at TEXT", []);
    }
    if !column_exists(&conn, "videos", "watched_at_sortable")? {
        let _ = conn.execute(
            "ALTER TABLE videos ADD COLUMN watched_at_sortable INTEGER DEFAULT 0",
            [],
        );
        // Backfill existing rows
        let _ = conn.execute(
            "UPDATE videos SET watched_at_sortable = 0 WHERE watched_at_sortable IS NULL",
            [],
        );
    }

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transcript_segments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            video_id TEXT NOT NULL,
            start_seconds REAL NOT NULL,
            duration_seconds REAL NOT NULL,
            text TEXT NOT NULL,
            FOREIGN KEY (video_id) REFERENCES videos(video_id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_transcript_segments_video_id
         ON transcript_segments(video_id)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_transcript_segments_video_id_start
         ON transcript_segments(video_id, start_seconds)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_videos_watched_created
         ON videos(watched_at_sortable DESC, created_at DESC)",
        [],
    )?;

    // Optional full-text indexes (FTS5) for text search on videos and transcript segments.
    // If FTS5 is unavailable in the SQLite build, we keep the app functional.
    let _ = conn.execute_batch(
        "
        CREATE VIRTUAL TABLE IF NOT EXISTS videos_fts USING fts5(
            video_id UNINDEXED,
            title,
            channel,
            summary,
            key_points,
            chat_text,
            share_text,
            transcript_text,
            content='videos',
            content_rowid='id',
            tokenize='unicode61 remove_diacritics 2'
        );

        CREATE TRIGGER IF NOT EXISTS videos_ai AFTER INSERT ON videos BEGIN
            INSERT INTO videos_fts(rowid, video_id, title, channel, summary, key_points, chat_text, share_text, transcript_text)
            VALUES (new.id, new.video_id, new.title, new.channel, new.summary, new.key_points, new.chat_text, new.share_text, new.transcript_text);
        END;

        CREATE TRIGGER IF NOT EXISTS videos_ad AFTER DELETE ON videos BEGIN
            INSERT INTO videos_fts(videos_fts, rowid, video_id, title, channel, summary, key_points, chat_text, share_text, transcript_text)
            VALUES('delete', old.id, old.video_id, old.title, old.channel, old.summary, old.key_points, old.chat_text, old.share_text, old.transcript_text);
        END;

        CREATE TRIGGER IF NOT EXISTS videos_au AFTER UPDATE ON videos BEGIN
            INSERT INTO videos_fts(videos_fts, rowid, video_id, title, channel, summary, key_points, chat_text, share_text, transcript_text)
            VALUES('delete', old.id, old.video_id, old.title, old.channel, old.summary, old.key_points, old.chat_text, old.share_text, old.transcript_text);
            INSERT INTO videos_fts(rowid, video_id, title, channel, summary, key_points, chat_text, share_text, transcript_text)
            VALUES (new.id, new.video_id, new.title, new.channel, new.summary, new.key_points, new.chat_text, new.share_text, new.transcript_text);
        END;

        CREATE VIRTUAL TABLE IF NOT EXISTS transcript_segments_fts USING fts5(
            video_id UNINDEXED,
            text,
            content='transcript_segments',
            content_rowid='id',
            tokenize='unicode61 remove_diacritics 2'
        );

        CREATE TRIGGER IF NOT EXISTS transcript_segments_ai AFTER INSERT ON transcript_segments BEGIN
            INSERT INTO transcript_segments_fts(rowid, video_id, text)
            VALUES (new.id, new.video_id, new.text);
        END;

        CREATE TRIGGER IF NOT EXISTS transcript_segments_ad AFTER DELETE ON transcript_segments BEGIN
            INSERT INTO transcript_segments_fts(transcript_segments_fts, rowid, video_id, text)
            VALUES('delete', old.id, old.video_id, old.text);
        END;

        CREATE TRIGGER IF NOT EXISTS transcript_segments_au AFTER UPDATE ON transcript_segments BEGIN
            INSERT INTO transcript_segments_fts(transcript_segments_fts, rowid, video_id, text)
            VALUES('delete', old.id, old.video_id, old.text);
            INSERT INTO transcript_segments_fts(rowid, video_id, text)
            VALUES (new.id, new.video_id, new.text);
        END;
        ",
    );

    Ok(conn)
}

fn entry_from_row(row: &rusqlite::Row) -> Result<VideoEntry, rusqlite::Error> {
    Ok(VideoEntry {
        id: row.get(0)?,
        video_id: row.get(1)?,
        source_url: row.get(2)?,
        title: row.get(3)?,
        channel: row.get(4)?,
        summary: row.get(5)?,
        key_points: row.get(6)?,
        chat_text: row.get(7)?,
        share_text: row.get(8)?,
        transcript_text: row.get(9)?,
        transcript_char_count: row.get(10)?,
        ai_status: row.get(11)?,
        language_label: row.get(12)?,
        is_generated: row.get::<_, i32>(13)? != 0,
        subtitle_kind: row.get(14)?,
        output_style: row.get(15)?,
        output_style_index: row.get(16)?,
        ui_language: row.get(17)?,
        model_name: row.get(18)?,
        ollama_endpoint: row.get(19)?,
        video_meta: row.get(20)?,
        watched_at: row.get(21)?,
        watched_at_sortable: row.get(22).unwrap_or(0),
        created_at: row.get(23)?,
    })
}

pub fn save_video(conn: &Connection, entry: &VideoEntry) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO videos (
            video_id, source_url, title, channel, summary, key_points, chat_text, share_text,
            transcript_text, transcript_char_count, ai_status, language_label, is_generated,
            subtitle_kind, output_style, output_style_index, ui_language, model_name,
            ollama_endpoint, video_meta, watched_at, watched_at_sortable, created_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23)
        ON CONFLICT(video_id) DO UPDATE SET
            source_url=excluded.source_url,
            title=COALESCE(excluded.title, videos.title),
            channel=COALESCE(excluded.channel, videos.channel),
            summary=excluded.summary,
            key_points=excluded.key_points,
            chat_text=excluded.chat_text,
            share_text=excluded.share_text,
            transcript_text=excluded.transcript_text,
            transcript_char_count=excluded.transcript_char_count,
            ai_status=excluded.ai_status,
            language_label=excluded.language_label,
            is_generated=excluded.is_generated,
            subtitle_kind=excluded.subtitle_kind,
            output_style=excluded.output_style,
            output_style_index=excluded.output_style_index,
            ui_language=excluded.ui_language,
            model_name=excluded.model_name,
            ollama_endpoint=excluded.ollama_endpoint,
            video_meta=excluded.video_meta,
            watched_at=COALESCE(excluded.watched_at, videos.watched_at),
            watched_at_sortable=COALESCE(excluded.watched_at_sortable, videos.watched_at_sortable),
            created_at=excluded.created_at",
        params![
            entry.video_id,
            entry.source_url,
            entry.title,
            entry.channel,
            entry.summary,
            entry.key_points,
            entry.chat_text,
            entry.share_text,
            entry.transcript_text,
            entry.transcript_char_count,
            entry.ai_status,
            entry.language_label,
            entry.is_generated as i32,
            entry.subtitle_kind,
            entry.output_style,
            entry.output_style_index,
            entry.ui_language,
            entry.model_name,
            entry.ollama_endpoint,
            entry.video_meta,
            entry.watched_at,
            entry.watched_at_sortable,
            entry.created_at,
        ],
    )?;
    Ok(())
}

pub fn load_videos(conn: &Connection) -> Result<Vec<VideoEntry>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, video_id, source_url, title, channel, summary, key_points, chat_text, share_text,
         transcript_text, transcript_char_count, ai_status, language_label, is_generated, subtitle_kind,
         output_style, output_style_index, ui_language, model_name, ollama_endpoint, video_meta, watched_at, watched_at_sortable, created_at
         FROM videos ORDER BY watched_at_sortable DESC, created_at DESC"
    )?;
    let rows = stmt.query_map([], entry_from_row)?;
    rows.collect()
}

#[allow(dead_code)]
pub fn load_video_by_id(
    conn: &Connection,
    video_id: &str,
) -> Result<Option<VideoEntry>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, video_id, source_url, title, channel, summary, key_points, chat_text, share_text,
         transcript_text, transcript_char_count, ai_status, language_label, is_generated, subtitle_kind,
         output_style, output_style_index, ui_language, model_name, ollama_endpoint, video_meta, watched_at, watched_at_sortable, created_at
         FROM videos WHERE video_id = ?1"
    )?;
    let mut rows = stmt.query_map([video_id], entry_from_row)?;
    Ok(rows.next().transpose()?)
}

pub fn delete_video(conn: &Connection, video_id: &str) -> Result<(), rusqlite::Error> {
    conn.execute("DELETE FROM videos WHERE video_id = ?1", [video_id])?;
    Ok(())
}

pub fn save_segments(
    conn: &Connection,
    video_id: &str,
    segments: &[TranscriptSegment],
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "DELETE FROM transcript_segments WHERE video_id = ?1",
        [video_id],
    )?;
    for segment in segments {
        conn.execute(
            "INSERT INTO transcript_segments (video_id, start_seconds, duration_seconds, text)
             VALUES (?1, ?2, ?3, ?4)",
            params![video_id, segment.start, segment.duration, segment.text,],
        )?;
    }
    Ok(())
}

pub fn load_segments(
    conn: &Connection,
    video_id: &str,
) -> Result<Vec<TranscriptSegment>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT start_seconds, duration_seconds, text
         FROM transcript_segments
         WHERE video_id = ?1
         ORDER BY start_seconds ASC",
    )?;
    let rows = stmt.query_map([video_id], |row| {
        Ok(TranscriptSegment {
            start: row.get(0)?,
            duration: row.get(1)?,
            text: row.get(2)?,
        })
    })?;
    rows.collect()
}

pub fn video_exists(conn: &Connection, video_id: &str) -> Result<bool, rusqlite::Error> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM videos WHERE video_id = ?1",
        [video_id],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

pub fn has_segments(conn: &Connection, video_id: &str) -> Result<bool, rusqlite::Error> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM transcript_segments WHERE video_id = ?1",
        [video_id],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

pub fn load_videos_without_segments(conn: &Connection) -> Result<Vec<VideoEntry>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, video_id, source_url, title, channel, summary, key_points, chat_text, share_text,
         transcript_text, transcript_char_count, ai_status, language_label, is_generated, subtitle_kind,
         output_style, output_style_index, ui_language, model_name, ollama_endpoint, video_meta, watched_at, watched_at_sortable, created_at
         FROM videos
         WHERE video_id NOT IN (SELECT video_id FROM transcript_segments)
         ORDER BY created_at DESC"
    )?;
    let rows = stmt.query_map([], entry_from_row)?;
    rows.collect()
}

/// Update only history-derived fields (title, channel, watched_at) without
/// touching analysis fields. Returns true if any row was updated.
pub fn update_history_fields(
    conn: &Connection,
    video_id: &str,
    title: Option<&str>,
    channel: Option<&str>,
    watched_at: Option<&str>,
) -> Result<bool, rusqlite::Error> {
    let sortable = watched_at.map(parse_spanish_date).unwrap_or(0);
    let changed = conn.execute(
        "UPDATE videos SET
            title = COALESCE(?1, title),
            channel = COALESCE(?2, channel),
            watched_at = COALESCE(?3, watched_at),
            watched_at_sortable = CASE WHEN ?3 IS NOT NULL THEN ?5 ELSE watched_at_sortable END
         WHERE video_id = ?4",
        params![title, channel, watched_at, video_id, sortable],
    )?;
    Ok(changed > 0)
}

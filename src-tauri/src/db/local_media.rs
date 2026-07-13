use regex::Regex;
use sqlx::types::chrono;

use crate::db::types::{EditableMediaFields, LocalFilepath, LocalMedia, ScanResult};
use crate::db::Database;
use crate::errors::AppError;
use std::path::Path;
use std::sync::Arc;
use tokio::fs;

#[derive(Clone)]
pub struct LocalMediaManager {
    db: Arc<Database>,
}

enum FileStatus {
    Added,
    Skipped,
}

static RESOLUTION_PATTERNS: &[(&[&str], &str)] = &[
    (&["2160p", "4k"], "2160p"),
    (&["1080p"], "1080p"),
    (&["720p"], "720p"),
    (&["480p"], "480p"),
];

static VIDEO_CODEC_PATTERNS: &[(&[&str], &str)] = &[
    (&["av1"], "AV1"),
    (&["hevc", "x265", "h.265", "h265"], "HEVC"),
    (&["avc", "x264", "h.264", "h264"], "H.264"),
    (&["vp9"], "VP9"),
    (&["vp8"], "VP8"),
];

static AUDIO_CODEC_PATTERNS: &[(&[&str], &str)] = &[
    (&["truehd", "true-hd"], "TrueHD"),
    (&["dts-hd ma", "dts-hd master"], "DTS-HD MA"),
    (&["dts-hd", "dts.hd"], "DTS-HD"),
    (&["dts-x", "dtsx"], "DTS:X"),
    (&["dts"], "DTS"),
    (&["atmos"], "Atmos"),
    (&["eac3", "e-ac-3", "ddp", "dd+"], "E-AC-3"),
    (&["ac3", "dd5.1"], "AC-3"),
    (&["aac"], "AAC"),
];

static AUDIO_CHANNEL_PATTERNS: &[&str] = &["7.1", "5.1", "2.0"];

static VIDEO_FILTER_PATTERNS: &[(&[&str], &str)] = &[
    (&["hdr10+"], "HDR10+"),
    (&["hdr10"], "HDR10"),
    (&["hdr"], "HDR"),
    (&["dolby vision", "dv"], "Dolby Vision"),
    (&["remux"], "Remux"),
    (&["bluray", "blu-ray"], "BluRay"),
    (&["web-dl", "webdl"], "WEB-DL"),
    (&["webrip"], "WEBRip"),
    (&["hdtv"], "HDTV"),
];

static TAG_PATTERNS: &[(&[&str], &str)] = &[
    (&["imax"], "IMAX"),
    (&["3d", "3-d"], "3D"),
    (&["extended"], "Extended"),
    (&["remastered"], "Remastered"),
    (&["proper"], "Proper"),
    (&["repack"], "Repack"),
    (&["directors cut", "director's cut"], "Director's Cut"),
    (&["uncut", "unrated"], "Uncut"),
    (&["criterion"], "Criterion"),
    (&["10bit"], "10-bit"),
    (&["8bit"], "8-bit"),
];

static LANGUAGE_PATTERNS: &[(&[&str], &str)] = &[
    (&[".en.", "english"], "en"),
    (&[".fr.", "french"], "fr"),
    (&[".es.", "spanish"], "es"),
    (&[".de.", "german"], "de"),
    (&[".it.", "italian"], "it"),
    (&[".ja.", "japanese"], "ja"),
    (&[".ko.", "korean"], "ko"),
    (&[".ru.", "russian"], "ru"),
    (&[".pt.", "portuguese"], "pt"),
    (&[".pl.", "polish"], "pl"),
    (&[".nl.", "dutch"], "nl"),
    (&[".tr.", "turkish"], "tr"),
    (&[".hi.", "hindi"], "hi"),
    (&[".ar.", "arabic"], "ar"),
];

fn match_first_pattern(lower: &str, patterns: &[(&[&str], &str)]) -> Option<String> {
    patterns
        .iter()
        .find(|(patterns, _)| patterns.iter().any(|p| lower.contains(p)))
        .map(|(_, result)| result.to_string())
}

fn match_all_patterns(lower: &str, patterns: &[(&[&str], &str)]) -> Vec<String> {
    patterns
        .iter()
        .filter(|(patterns, _)| patterns.iter().any(|p| lower.contains(p)))
        .map(|(_, result)| result.to_string())
        .collect()
}

fn match_ordered_patterns(lower: &str, ordered_patterns: &[&str]) -> Option<String> {
    ordered_patterns
        .iter()
        .find(|p| lower.contains(*p))
        .map(|s| s.to_string())
}

impl LocalMediaManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_local_media_by_imdb(
        &self,
        imdb_id: &str,
    ) -> Result<Option<(LocalMedia, Vec<LocalFilepath>)>, AppError> {
        let row = sqlx::query_as::<_, (i64, Option<String>)>(
            "SELECT id, imdb_id FROM local_media WHERE imdb_id = ?",
        )
        .bind(imdb_id)
        .fetch_optional(&self.db.pool)
        .await?;

        match row {
            Some((id, imdb_id)) => {
                let media = LocalMedia { id, imdb_id };
                let filepaths = self.get_filepaths(id).await?;
                Ok(Some((media, filepaths)))
            }
            None => Ok(None),
        }
    }

    pub async fn find_local_media(
        &self,
        title: &str,
        year: Option<i32>,
        imdb_id: &str,
        season: Option<i64>,
        episode: Option<i64>,
    ) -> Result<Option<(LocalMedia, Vec<LocalFilepath>)>, AppError> {
        if let Some((media, filepaths)) = self.get_local_media_by_imdb(imdb_id).await? {
            let filtered = if let (Some(s), Some(e)) = (season, episode) {
                let f: Vec<_> = filepaths
                    .into_iter()
                    .filter(|fp| fp.season == Some(s) && fp.episode == Some(e))
                    .collect();
                f
            } else {
                filepaths
            };
            return Ok(Some((media, filtered)));
        }
        let matched = self.match_unknown_media(title, year, imdb_id).await?;

        if matched > 0 {
            if let Some((media, filepaths)) = self.get_local_media_by_imdb(imdb_id).await? {
                let filtered = if let (Some(s), Some(e)) = (season, episode) {
                    filepaths
                        .into_iter()
                        .filter(|fp| fp.season == Some(s) && fp.episode == Some(e))
                        .collect()
                } else {
                    filepaths
                };
                return Ok(Some((media, filtered)));
            }
        }

        Ok(None)
    }

    async fn get_filepaths(&self, media_id: i64) -> Result<Vec<LocalFilepath>, AppError> {
        let rows = sqlx::query_as::<_, (
    i64,           
    i64,            
    String,         
    Option<i64>,    
    Option<i64>,    
    Option<f64>,    
    Option<String>, 
    Option<String>, 
    Option<String>, 
    Option<String>, 
    Option<String>, 
    Option<String>, 
    Option<String>, 
    Option<bool>,   
)>(
    "SELECT id, media_id, file_path, season, episode, size, resolution, video_codec, audio_codec, audio_channels, video_filters, tags, language, is_default FROM local_filepaths WHERE media_id = ?"
)
        .bind(media_id)
        .fetch_all(&self.db.pool)
        .await?;

        let files = rows
            .into_iter()
            .map(
                |(
                    id,
                    media_id,
                    file_path,
                    season,
                    episode,
                    size,
                    resolution,
                    video_codec,
                    audio_codec,
                    audio_channels,
                    video_filters,
                    tags,
                    language,
                    is_default,
                )| LocalFilepath {
                    id,
                    media_id,
                    file_path,
                    season,
                    episode,
                    size,
                    resolution,
                    video_codec,
                    audio_codec,
                    audio_channels,
                    video_filters,
                    tags,
                    language,
                    is_default,
                },
            )
            .collect();

        Ok(files)
    }

    pub async fn delete_by_directory(&self, directory: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM local_filepaths WHERE file_path LIKE ?")
            .bind(format!("{}%", directory))
            .execute(&self.db.pool)
            .await?
            .rows_affected();

        sqlx::query(
            "DELETE FROM local_media WHERE id NOT IN (SELECT DISTINCT media_id FROM local_filepaths)"
        )
        .execute(&self.db.pool)
        .await?
        .rows_affected();

        Ok(())
    }

    pub async fn match_unknown_media(
        &self,
        title: &str,
        year: Option<i32>,
        imdb_id: &str,
    ) -> Result<u64, AppError> {
        let cleaned_title = Self::clean_filename_for_matching(title);

        let target_media_id = if let Some((id, _)) = sqlx::query_as::<_, (i64, Option<String>)>(
            "SELECT id, imdb_id FROM local_media WHERE imdb_id = ?",
        )
        .bind(imdb_id)
        .fetch_optional(&self.db.pool)
        .await?
        {
            id
        } else {
            sqlx::query_as::<_, (i64,)>("INSERT INTO local_media (imdb_id) VALUES (?) RETURNING id")
                .bind(imdb_id)
                .fetch_one(&self.db.pool)
                .await?
                .0
        };

        let matching_media = if let Some(y) = year {
            sqlx::query_as::<_, (i64,)>(
                "SELECT DISTINCT lm.id 
             FROM local_media lm 
             JOIN local_filepaths lf ON lf.media_id = lm.id 
             WHERE lm.imdb_id IS NULL 
             AND LOWER(lf.file_path) LIKE ?
             AND lf.file_path LIKE ?",
            )
            .bind(format!("%{}%", cleaned_title))
            .bind(format!("%{}%", y))
            .fetch_all(&self.db.pool)
            .await?
        } else {
            sqlx::query_as::<_, (i64,)>(
                "SELECT DISTINCT lm.id 
             FROM local_media lm 
             JOIN local_filepaths lf ON lf.media_id = lm.id 
             WHERE lm.imdb_id IS NULL 
             AND LOWER(lf.file_path) LIKE ?",
            )
            .bind(format!("%{}%", cleaned_title))
            .fetch_all(&self.db.pool)
            .await?
        };

        let mut total_moved = 0u64;

        for (old_media_id,) in matching_media {
            if old_media_id != target_media_id {
                let moved =
                    sqlx::query("UPDATE local_filepaths SET media_id = ? WHERE media_id = ?")
                        .bind(target_media_id)
                        .bind(old_media_id)
                        .execute(&self.db.pool)
                        .await?
                        .rows_affected();

                total_moved += moved;

                sqlx::query("DELETE FROM local_media WHERE id = ?")
                    .bind(old_media_id)
                    .execute(&self.db.pool)
                    .await?;
            }
        }

        let remaining = if let Some(y) = year {
            sqlx::query(
                "UPDATE local_media 
             SET imdb_id = ? 
             WHERE id IN (
                 SELECT lm.id 
                 FROM local_media lm 
                 JOIN local_filepaths lf ON lf.media_id = lm.id 
                 WHERE lm.imdb_id IS NULL 
                 AND LOWER(lf.file_path) LIKE ?
                 AND lf.file_path LIKE ?
             )",
            )
            .bind(imdb_id)
            .bind(format!("%{}%", cleaned_title))
            .bind(format!("%{}%", y))
            .execute(&self.db.pool)
            .await?
            .rows_affected()
        } else {
            sqlx::query(
                "UPDATE local_media 
             SET imdb_id = ? 
             WHERE id IN (
                 SELECT lm.id 
                 FROM local_media lm 
                 JOIN local_filepaths lf ON lf.media_id = lm.id 
                 WHERE lm.imdb_id IS NULL 
                 AND LOWER(lf.file_path) LIKE ?
             )",
            )
            .bind(imdb_id)
            .bind(format!("%{}%", cleaned_title))
            .execute(&self.db.pool)
            .await?
            .rows_affected()
        };

        Ok(total_moved + remaining)
    }

    fn clean_filename_for_matching(filename: &str) -> String {
        let cleaned = filename.to_lowercase();

        let patterns = [
            (Regex::new(r"[sS]\d{2}[eE]\d{2}").unwrap(), ""),
            (Regex::new(r"\d{3,4}p").unwrap(), ""),
            (Regex::new(r"\d{4}").unwrap(), ""),
            (
                Regex::new(r"bluray|web-?dl|hdtv|dvdrip|brrip|x264|x265|hevc|aac|ac3").unwrap(),
                "",
            ),
            (Regex::new(r"[\.\-_\s]+").unwrap(), " "),
        ];

        let mut result = cleaned;
        for (pattern, replacement) in &patterns {
            result = pattern.replace_all(&result, *replacement).to_string();
        }

        result.trim().to_string()
    }

    pub async fn scan_folder(&self, directory: &str) -> Result<ScanResult, AppError> {
        let path = Path::new(directory);
               
        let mut result = ScanResult::default();
        self.scan_recursive(path, &mut result)
            .await?;

        self.cleanup_all().await?;

        let now = chrono::Utc::now().to_rfc3339();
        let column = if directory.to_lowercase().contains("tv") {
            "last_tv_scan"
        } else {
            "last_movie_scan"
        };

        sqlx::query(&format!(
            "UPDATE global_settings SET {} = ? WHERE id = 1",
            column
        ))
        .bind(&now)
        .execute(&self.db.pool)
        .await?;

        Ok(result)
    }

    async fn scan_recursive(&self, dir: &Path, result: &mut ScanResult) -> Result<(), AppError> {
        let mut entries = fs::read_dir(dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with('.') || name == "extras" || name == "sample" {
                        continue;
                    }
                }
                Box::pin(self.scan_recursive(&path, result)).await?;
            } else if Self::is_video_file(&path) {
                match self.process_file(&path).await {
                    Ok(status) => match status {
                        FileStatus::Added => result.added += 1,
                        FileStatus::Skipped => result.skipped += 1,
                    },
                    Err(_) => result.errors += 1,
                }
            }
        }

        Ok(())
    }

    fn is_video_file(path: &Path) -> bool {
        match path.extension().and_then(|e| e.to_str()) {
            Some(ext) => matches!(
                ext.to_lowercase().as_str(),
                "mkv" | "mp4" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "mpg" | "mpeg"
            ),
            None => false,
        }
    }

    async fn process_file(&self, path: &Path) -> Result<FileStatus, AppError> {
        let file_path = path.to_string_lossy().to_string();

        let existing =
            sqlx::query_as::<_, (i64,)>("SELECT id FROM local_filepaths WHERE file_path = ?")
                .bind(&file_path)
                .fetch_optional(&self.db.pool)
                .await?;

        if existing.is_some() {
            return Ok(FileStatus::Skipped);
        }

        let size_bytes = fs::metadata(path).await?.len() as f64;
        let size_gb = size_bytes / (1024.0 * 1024.0 * 1024.0);

        let lower = file_path.to_lowercase();

        let imdb_id = Self::extract_imdb_id(&file_path);
        let (season, episode) = Self::extract_season_episode(&file_path);
        let resolution = Self::extract_resolution(&lower);
        let video_codec = Self::extract_video_codec(&lower);
        let audio_codec = Self::extract_audio_codec(&lower);
        let audio_channels = Self::extract_audio_channels(&lower);
        let video_filters =
            serde_json::to_string(&Self::extract_video_filters(&lower)).unwrap_or_default();
        let tags = serde_json::to_string(&Self::extract_tags(&lower)).unwrap_or_default();
        let language = serde_json::to_string(&Self::extract_language(&lower)).unwrap_or_default();

        let media_id = if let Some(ref imdb) = imdb_id {
            let existing =
                sqlx::query_as::<_, (i64,)>("SELECT id FROM local_media WHERE imdb_id = ?")
                    .bind(imdb)
                    .fetch_optional(&self.db.pool)
                    .await?;

            if let Some((id,)) = existing {
                id
            } else {
                sqlx::query_as::<_, (i64,)>(
                    "INSERT INTO local_media (imdb_id) VALUES (?) RETURNING id",
                )
                .bind(imdb)
                .fetch_one(&self.db.pool)
                .await?
                .0
            }
        } else {
            sqlx::query_as::<_, (i64,)>(
                "INSERT INTO local_media (imdb_id) VALUES (NULL) RETURNING id",
            )
            .fetch_one(&self.db.pool)
            .await?
            .0
        };

        sqlx::query(
        "INSERT INTO local_filepaths (media_id, file_path, season, episode, size, resolution, video_codec, audio_codec, audio_channels, video_filters, tags, language) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(media_id)
    .bind(&file_path)
    .bind(season)
    .bind(episode)
    .bind(size_gb)
    .bind(&resolution)
    .bind(&video_codec)
    .bind(&audio_codec)
    .bind(&audio_channels)
    .bind(&video_filters)
    .bind(&tags)
    .bind(&language)
    .execute(&self.db.pool)
    .await?;

        Ok(FileStatus::Added)
    }

    fn extract_imdb_id(file_path: &str) -> Option<String> {
        let re = Regex::new(r"\{tt\d{7,8}\}").unwrap();
        re.find(file_path).map(|m| {
            let matched = m.as_str();
            matched[1..matched.len() - 1].to_string()
        })
    }

    fn extract_season_episode(file_path: &str) -> (Option<i64>, Option<i64>) {
        let patterns = [
            Regex::new(r"[sS](\d{1,2})[eE](\d{1,2})").unwrap(),
            Regex::new(r"(\d{1,2})x(\d{1,2})").unwrap(),
        ];

        for re in &patterns {
            if let Some(caps) = re.captures(file_path) {
                let season = caps.get(1).and_then(|s| s.as_str().parse().ok());
                let episode = caps.get(2).and_then(|s| s.as_str().parse().ok());
                return (season, episode);
            }
        }

        (None, None)
    }

    fn extract_resolution(lower: &str) -> String {
        match_first_pattern(lower, RESOLUTION_PATTERNS)
            .unwrap_or_else(|| "Unknown".to_string())
    }

    fn extract_video_codec(lower: &str) -> Option<String> {
        match_first_pattern(lower, VIDEO_CODEC_PATTERNS)
    }

    fn extract_audio_codec(lower: &str) -> Option<String> {
        match_first_pattern(lower, AUDIO_CODEC_PATTERNS)
    }

    fn extract_audio_channels(lower: &str) -> Option<String> {
        match_ordered_patterns(lower, AUDIO_CHANNEL_PATTERNS)
    }

    fn extract_video_filters(lower: &str) -> Vec<String> {
        let mut filters = match_all_patterns(lower, VIDEO_FILTER_PATTERNS);
        filters.sort();
        filters.dedup();
        filters
    }

    fn extract_tags(lower: &str) -> Vec<String> {
        let mut tags = match_all_patterns(lower, TAG_PATTERNS);
        tags.sort();
        tags.dedup();
        tags
    }

    fn extract_language(lower: &str) -> Vec<String> {
        let mut languages = match_all_patterns(lower, LANGUAGE_PATTERNS);
        languages.sort();
        languages.dedup();
        languages
    }

    pub async fn cleanup_all(&self) -> Result<(u64, u64), AppError> {
        let filepaths =
            sqlx::query_as::<_, (i64, String)>("SELECT id, file_path FROM local_filepaths")
                .fetch_all(&self.db.pool)
                .await?;

        let mut files_removed = 0;
        for (id, file_path) in filepaths {
            if !Path::new(&file_path).exists() {
                sqlx::query("DELETE FROM local_filepaths WHERE id = ?")
                    .bind(id)
                    .execute(&self.db.pool)
                    .await?;
                files_removed += 1;
            }
        }

        let media_removed = sqlx::query(
        "DELETE FROM local_media WHERE id NOT IN (SELECT DISTINCT media_id FROM local_filepaths)"
    )
    .execute(&self.db.pool)
    .await?
    .rows_affected();

        Ok((files_removed, media_removed))
    }

    pub async fn get_editable_local_media(&self) -> Result<Vec<EditableMediaFields>, AppError> {
        let rows = sqlx::query_as::<_, (i64, Option<String>, i64, String, Option<i64>, Option<i64>, Option<bool>)>(
            "SELECT lm.id, lm.imdb_id, lf.id, lf.file_path, lf.season, lf.episode, lf.is_default 
            FROM local_media lm 
            JOIN local_filepaths lf ON lf.media_id = lm.id 
            ORDER BY lm.id, lf.season, lf.episode"
        )
        .fetch_all(&self.db.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(media_id, imdb_id, filepath_id, file_path, season, episode, is_default)| EditableMediaFields {
                media_id,
                imdb_id,
                filepath_id,
                file_path,
                season,
                episode,
                is_default,
            })
            .collect())
    }

    pub async fn update_local_media(
        &self,
        media: Vec<EditableMediaFields>,
    ) -> Result<(), AppError> {
        for item in media {
            sqlx::query(
                "UPDATE local_filepaths SET season = ?, episode = ?, is_default = ? WHERE id = ?"
            )
            .bind(item.season)
            .bind(item.episode)
            .bind(item.is_default)
            .bind(item.filepath_id)
            .execute(&self.db.pool)
            .await?;

            if let Some(imdb) = &item.imdb_id {
                let (media_id,) = sqlx::query_as::<_, (i64,)>(
                    "SELECT media_id FROM local_filepaths WHERE id = ?"
                )
                .bind(item.filepath_id)
                .fetch_one(&self.db.pool)
                .await?;

                let existing = sqlx::query_as::<_, (i64,)>(
                    "SELECT id FROM local_media WHERE imdb_id = ?"
                )
                .bind(imdb)
                .fetch_optional(&self.db.pool)
                .await?;

                if let Some((existing_id,)) = existing {
                    if existing_id != media_id {
                        sqlx::query("UPDATE local_filepaths SET media_id = ? WHERE id = ?")
                            .bind(existing_id)
                            .bind(item.filepath_id)
                            .execute(&self.db.pool)
                            .await?;

                        sqlx::query("DELETE FROM local_media WHERE id = ?")
                            .bind(media_id)
                            .execute(&self.db.pool)
                            .await?;
                    }
                } else {
                    sqlx::query("UPDATE local_media SET imdb_id = ? WHERE id = ?")
                        .bind(imdb)
                        .bind(media_id)
                        .execute(&self.db.pool)
                        .await?;
                }
            }
        }

        Ok(())
    }



}

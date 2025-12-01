use std::path::{Path, PathBuf};
use reqwest::Url;
use chrono::{DateTime, Utc};

use crate::error::{AppError, AppResult};

pub struct VideoMetadata<'a> {
    pub title: &'a str,
    pub id: &'a str,
    pub date: Option<DateTime<Utc>>,
    pub duration: Option<&'a str>,
}

pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' || c == ' ' => c,
            _ => '_',
        })
        .collect::<String>()
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
        .trim()
        .to_string()
}

pub fn expand_path(path: &str) -> PathBuf {
    if !path.starts_with('~') {
        return PathBuf::from(path);
    }

    if let Some(home_dir) = dirs::home_dir() {
        if path == "~" {
            return home_dir;
        }

        return home_dir.join(&path[2..]);
    }

    PathBuf::from(path)
}

pub fn format_bytes(bytes: u64) -> String {
    const UNIT: f64 = 1024.0;
    if bytes < UNIT as u64 {
        return format!("{} B", bytes);
    }

    let parsed_bytes = bytes as f64;
    let exponent = (parsed_bytes.ln() / UNIT.ln()) as i32;
    let prefix = "KMGTPE".chars().nth((exponent - 1) as usize).unwrap_or('?');
    
    let value = parsed_bytes / UNIT.powi(exponent);

    format!("{:.2} {}B", value, prefix)
}

pub fn format_duration(total_seconds: u64) -> String {
    let hours = total_seconds / 3600;
    let rest = total_seconds % 3600;
    let minutes = rest / 60;
    let seconds = rest % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn parse_duration(seconds: &str) -> AppResult<u64> {
    let parts: Vec<&str> = seconds.split(':').collect();

    let parse_part = |s: &str| -> AppResult<u64> {
        s.parse().map_err(|_| AppError::InvalidTimeFormat(input.to_string()))
    }

    match parts.len() {
        3 => {
            let hours = parse_part(parts[0])?;
            let minutes = parse_part(parts[1])?;
            let seconds = parse_part(parts[2])?;
            Ok(hours * 3600 + minutes * 60 + seconds)
        }
        2 => {
            let minutes = parse_part(parts[0])? ;
            let seconds = parse_part(parts[1])?;
            Ok(minutes * 60 + seconds)
        }
        1 => parse_part(parts[0])?,
        _ => Err(AppError::InvalidTimeFormat(seconds.to_string())),
    }
}

pub fn extract_video_id(url: &str) -> Option<String> {
    let parsed_url = Url::parse(url).ok()?;
    let domain = parsed_url.domain()?;

    if domain.ends_with("youtube.com") {
        if let Some(pair) = parsed_url.query_pairs().find(|(key, _)| key == "v") {
            return Some(pair.1.to_string());
        }

        if parsed_url.path().starts_with("/embed/") {
            return parsed_url.path_segments()?.nth(1).map(|s| s.to_string());
        }
    } else if domain.ends_with("youtu.be") {
        return parsed_url.path_segments()?.next().map(|s| s.to_string());
    }
    None
}

pub fn extract_playlist_id(url_str: &str) -> Option<String> {
    let parsed_url = Url::parse(url_str).ok()?;
    parsed_url.query_pairs()
        .find(|(key, _)| key == "list")
        .map(|(_, value)| value.to_string())
}

pub fn apply_template(template: &str, meta: &VideoMetadata) -> String {
    let date_str = meta.date
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| Utc::now().format("%Y-%m-%d").to_string());
    
    let safe_title = sanitize_filename(meta.title);

    let filename = template
        .replace("{title}", &safe_title)
        .replace("{id}", meta.id)
        .replace("{date}", &date_str)
        .replace("{duration}", meta.duration.unwrap_or(""));

    filename
}
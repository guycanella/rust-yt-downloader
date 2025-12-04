//! Utility functions for file handling, formatting, and YouTube URL parsing.
//!
//! This module provides common utilities used throughout the application:
//! - Filename sanitization for safe filesystem operations
//! - Path expansion with tilde (~) support
//! - Human-readable byte and duration formatting
//! - YouTube video/playlist ID extraction from URLs
//! - Template-based filename generation

use std::path::PathBuf;
use reqwest::Url;
use chrono::{DateTime, Utc};

use crate::error::{AppError, AppResult};

/// Metadata for a YouTube video used in template-based filename generation.
///
/// This struct holds information about a video that can be used to generate
/// custom filenames using placeholders like `{title}`, `{id}`, `{date}`, and `{duration}`.
///
/// # Examples
///
/// ```
/// use chrono::Utc;
/// use rust_yt_downloader::utils::VideoMetadata;
///
/// let metadata = VideoMetadata {
///     title: "My Awesome Video",
///     id: "dQw4w9WgXcQ",
///     date: Some(Utc::now()),
///     duration: Some("03:45"),
/// };
/// ```
pub struct VideoMetadata<'a> {
    /// The video's title
    pub title: &'a str,
    /// The unique YouTube video ID
    pub id: &'a str,
    /// The upload date (optional)
    pub date: Option<DateTime<Utc>>,
    /// The video duration as a formatted string (optional)
    pub duration: Option<&'a str>,
}

/// Sanitizes a filename by replacing invalid characters with underscores.
///
/// Removes or replaces characters that are invalid in filenames across different operating systems,
/// including: `/ \ : * ? " < > |`. Also collapses consecutive underscores into a single underscore.
///
/// # Arguments
///
/// * `filename` - The filename to sanitize
///
/// # Returns
///
/// A sanitized filename safe for use on all major filesystems.
///
/// # Examples
///
/// ```
/// use rust_yt_downloader::utils::sanitize_filename;
///
/// let filename = sanitize_filename("My Video: Part 1?");
/// assert_eq!(filename, "My Video_ Part 1");
///
/// let filename = sanitize_filename("path/to/file.mp4");
/// assert_eq!(filename, "path_to_file.mp4");
/// ```
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

/// Expands a path with tilde (~) to the user's home directory.
///
/// Converts paths starting with `~` to their absolute equivalents using the user's home directory.
/// If the path doesn't start with `~` or the home directory cannot be determined, returns the path unchanged.
///
/// # Arguments
///
/// * `path` - The path to expand
///
/// # Returns
///
/// A `PathBuf` with the tilde expanded to the home directory, or the original path.
///
/// # Examples
///
/// ```
/// use rust_yt_downloader::utils::expand_path;
///
/// // Expands to /home/user/Downloads (on Unix)
/// let path = expand_path("~/Downloads");
///
/// // Returns the path unchanged
/// let path = expand_path("/absolute/path");
/// ```
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

/// Formats a byte count into a human-readable string with appropriate unit suffix.
///
/// Converts byte values into KB, MB, GB, TB, PB, or EB as appropriate, using binary units (1024 bytes = 1 KB).
///
/// # Arguments
///
/// * `bytes` - The number of bytes to format
///
/// # Returns
///
/// A formatted string with two decimal places and the appropriate unit (B, KB, MB, GB, TB, PB, EB).
///
/// # Examples
///
/// ```
/// use rust_yt_downloader::utils::format_bytes;
///
/// assert_eq!(format_bytes(500), "500 B");
/// assert_eq!(format_bytes(1024), "1.00 KB");
/// assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
/// assert_eq!(format_bytes(1536), "1.50 KB");
/// ```
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

/// Formats a duration in seconds to HH:MM:SS format.
///
/// # Arguments
///
/// * `total_seconds` - The total number of seconds
///
/// # Returns
///
/// A string in `HH:MM:SS` format with zero-padding.
///
/// # Examples
///
/// ```
/// use rust_yt_downloader::utils::format_duration;
///
/// assert_eq!(format_duration(45), "00:00:45");
/// assert_eq!(format_duration(3661), "01:01:01");
/// assert_eq!(format_duration(7384), "02:03:04");
/// ```
pub fn format_duration(total_seconds: u64) -> String {
    let hours = total_seconds / 3600;
    let rest = total_seconds % 3600;
    let minutes = rest / 60;
    let seconds = rest % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

/// Parses a duration string to total seconds.
///
/// Accepts durations in three formats:
/// - Seconds only: `"45"` → 45 seconds
/// - Minutes and seconds: `"02:30"` → 150 seconds
/// - Hours, minutes, and seconds: `"01:30:45"` → 5445 seconds
///
/// # Arguments
///
/// * `seconds` - A duration string in one of the supported formats
///
/// # Returns
///
/// The total duration in seconds, or an error if the format is invalid.
///
/// # Errors
///
/// Returns `AppError::InvalidTimeFormat` if the string cannot be parsed or contains non-numeric values.
///
/// # Examples
///
/// ```
/// use rust_yt_downloader::utils::parse_duration;
///
/// assert_eq!(parse_duration("45").unwrap(), 45);
/// assert_eq!(parse_duration("02:30").unwrap(), 150);
/// assert_eq!(parse_duration("01:30:45").unwrap(), 5445);
/// ```
pub fn parse_duration(seconds: &str) -> AppResult<u64> {
    let parts: Vec<&str> = seconds.split(':').collect();

    let parse_part = |s: &str| -> AppResult<u64> {
        s.parse().map_err(|_| AppError::InvalidTimeFormat(seconds.to_string()))
    };

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
        1 => {
            let seconds = parse_part(parts[0])?;
            Ok(seconds)
        }
        _ => Err(AppError::InvalidTimeFormat(seconds.to_string())),
    }
}

/// Extracts the video ID from a YouTube URL.
///
/// Supports multiple YouTube URL formats:
/// - Standard: `https://www.youtube.com/watch?v=VIDEO_ID`
/// - Short: `https://youtu.be/VIDEO_ID`
/// - Embed: `https://www.youtube.com/embed/VIDEO_ID`
///
/// # Arguments
///
/// * `url` - A YouTube video URL
///
/// # Returns
///
/// The video ID if the URL is valid, or `None` if it cannot be parsed.
///
/// # Examples
///
/// ```
/// use rust_yt_downloader::utils::extract_video_id;
///
/// let id = extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
/// assert_eq!(id, Some("dQw4w9WgXcQ".to_string()));
///
/// let id = extract_video_id("https://youtu.be/dQw4w9WgXcQ");
/// assert_eq!(id, Some("dQw4w9WgXcQ".to_string()));
/// ```
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

/// Extracts the playlist ID from a YouTube URL.
///
/// Parses the `list` query parameter from YouTube URLs to extract playlist IDs.
///
/// # Arguments
///
/// * `url_str` - A YouTube URL that may contain a playlist
///
/// # Returns
///
/// The playlist ID if present, or `None` if the URL doesn't contain a playlist parameter.
///
/// # Examples
///
/// ```
/// use rust_yt_downloader::utils::extract_playlist_id;
///
/// let id = extract_playlist_id("https://www.youtube.com/playlist?list=PLrAXtmErZgOe");
/// assert_eq!(id, Some("PLrAXtmErZgOe".to_string()));
///
/// let id = extract_playlist_id("https://www.youtube.com/watch?v=abc&list=PLrAXtmErZgOe");
/// assert_eq!(id, Some("PLrAXtmErZgOe".to_string()));
/// ```
pub fn extract_playlist_id(url_str: &str) -> Option<String> {
    let parsed_url = Url::parse(url_str).ok()?;
    parsed_url.query_pairs()
        .find(|(key, _)| key == "list")
        .map(|(_, value)| value.to_string())
}

/// Applies a filename template using video metadata.
///
/// Replaces placeholders in a template string with actual video metadata:
/// - `{title}` - Video title (sanitized for filesystem safety)
/// - `{id}` - YouTube video ID
/// - `{date}` - Upload date in YYYY-MM-DD format (or current date if unavailable)
/// - `{duration}` - Video duration (empty string if unavailable)
///
/// # Arguments
///
/// * `template` - A template string with placeholders
/// * `meta` - Video metadata to substitute into the template
///
/// # Returns
///
/// The filename with all placeholders replaced.
///
/// # Examples
///
/// ```
/// use rust_yt_downloader::utils::{apply_template, VideoMetadata};
/// use chrono::Utc;
///
/// let metadata = VideoMetadata {
///     title: "My Video",
///     id: "abc123",
///     date: Some(Utc::now()),
///     duration: Some("10:30"),
/// };
///
/// let filename = apply_template("{title}-{id}", &metadata);
/// assert_eq!(filename, "My Video-abc123");
/// ```
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

// ==================================================
//          UNITARY TESTS
// ==================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    // ============== sanitize_filename Tests ==============

    #[test]
    fn test_sanitize_filename_simple() {
        let result = sanitize_filename("my video");
        assert_eq!(result, "my video");
    }

    #[test]
    fn test_sanitize_filename_with_extension() {
        let result = sanitize_filename("my video.mp4");
        assert_eq!(result, "my video.mp4");
    }

    #[test]
    fn test_sanitize_filename_removes_slashes() {
        let result = sanitize_filename("my/video/name");
        assert_eq!(result, "my_video_name");
    }

    #[test]
    fn test_sanitize_filename_removes_backslashes() {
        let result = sanitize_filename("my\\video\\name");
        assert_eq!(result, "my_video_name");
    }

    #[test]
    fn test_sanitize_filename_removes_colons() {
        let result = sanitize_filename("video: the sequel");
        assert_eq!(result, "video_ the sequel");
    }

    #[test]
    fn test_sanitize_filename_removes_asterisks() {
        let result = sanitize_filename("video*name");
        assert_eq!(result, "video_name");
    }

    #[test]
    fn test_sanitize_filename_removes_question_marks() {
        let result = sanitize_filename("what is this?");
        assert_eq!(result, "what is this");
    }

    #[test]
    fn test_sanitize_filename_removes_quotes() {
        let result = sanitize_filename("video \"title\"");
        assert_eq!(result, "video _title");
    }

    #[test]
    fn test_sanitize_filename_removes_angle_brackets() {
        let result = sanitize_filename("video <title>");
        assert_eq!(result, "video _title");
    }

    #[test]
    fn test_sanitize_filename_removes_pipe() {
        let result = sanitize_filename("video|name");
        assert_eq!(result, "video_name");
    }

    #[test]
    fn test_sanitize_filename_keeps_hyphens() {
        let result = sanitize_filename("my-video-name");
        assert_eq!(result, "my-video-name");
    }

    #[test]
    fn test_sanitize_filename_keeps_underscores() {
        let result = sanitize_filename("my_video_name");
        assert_eq!(result, "my_video_name");
    }

    #[test]
    fn test_sanitize_filename_keeps_dots() {
        let result = sanitize_filename("video.2024.final.mp4");
        assert_eq!(result, "video.2024.final.mp4");
    }

    #[test]
    fn test_sanitize_filename_removes_multiple_invalid_chars() {
        let result = sanitize_filename("video:/\\*?name");
        assert_eq!(result, "video_name");
    }

    #[test]
    fn test_sanitize_filename_collapses_multiple_underscores() {
        let result = sanitize_filename("video///name");
        assert_eq!(result, "video_name");
    }

    #[test]
    fn test_sanitize_filename_with_unicode() {
        let result = sanitize_filename("vídeo música");
        assert_eq!(result, "vídeo música");
    }

    #[test]
    fn test_sanitize_filename_empty_string() {
        let result = sanitize_filename("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_sanitize_filename_only_invalid_chars() {
        let result = sanitize_filename("///\\\\:::");
        assert_eq!(result, "");
    }

    // ============== expand_path Tests ==============

    #[test]
    fn test_expand_path_no_tilde() {
        let result = expand_path("/absolute/path");
        assert_eq!(result, PathBuf::from("/absolute/path"));
    }

    #[test]
    fn test_expand_path_relative() {
        let result = expand_path("relative/path");
        assert_eq!(result, PathBuf::from("relative/path"));
    }

    #[test]
    fn test_expand_path_current_dir() {
        let result = expand_path(".");
        assert_eq!(result, PathBuf::from("."));
    }

    #[test]
    fn test_expand_path_tilde_only() {
        let result = expand_path("~");
        if let Some(home) = dirs::home_dir() {
            assert_eq!(result, home);
        }
    }

    #[test]
    fn test_expand_path_tilde_with_subdir() {
        let result = expand_path("~/Downloads");
        if let Some(home) = dirs::home_dir() {
            assert_eq!(result, home.join("Downloads"));
        }
    }

    #[test]
    fn test_expand_path_tilde_with_nested_subdirs() {
        let result = expand_path("~/Documents/videos/youtube");
        if let Some(home) = dirs::home_dir() {
            assert_eq!(result, home.join("Documents/videos/youtube"));
        }
    }

    #[test]
    fn test_expand_path_tilde_in_middle_not_expanded() {
        let result = expand_path("/path/~/something");
        assert_eq!(result, PathBuf::from("/path/~/something"));
    }

    // ============== format_bytes Tests ==============

    #[test]
    fn test_format_bytes_zero() {
        let result = format_bytes(0);
        assert_eq!(result, "0 B");
    }

    #[test]
    fn test_format_bytes_bytes() {
        let result = format_bytes(500);
        assert_eq!(result, "500 B");
    }

    #[test]
    fn test_format_bytes_one_kb() {
        let result = format_bytes(1024);
        assert_eq!(result, "1.00 KB");
    }

    #[test]
    fn test_format_bytes_kilobytes() {
        let result = format_bytes(1536);
        assert_eq!(result, "1.50 KB");
    }

    #[test]
    fn test_format_bytes_one_mb() {
        let result = format_bytes(1024 * 1024);
        assert_eq!(result, "1.00 MB");
    }

    #[test]
    fn test_format_bytes_megabytes() {
        let result = format_bytes(1024 * 1024 * 5);
        assert_eq!(result, "5.00 MB");
    }

    #[test]
    fn test_format_bytes_one_gb() {
        let result = format_bytes(1024 * 1024 * 1024);
        assert_eq!(result, "1.00 GB");
    }

    #[test]
    fn test_format_bytes_gigabytes() {
        let result = format_bytes(1024 * 1024 * 1024 * 2);
        assert_eq!(result, "2.00 GB");
    }

    #[test]
    fn test_format_bytes_one_tb() {
        let result = format_bytes(1024_u64 * 1024 * 1024 * 1024);
        assert_eq!(result, "1.00 TB");
    }

    #[test]
    fn test_format_bytes_fractional() {
        let result = format_bytes(1024 + 512);
        assert_eq!(result, "1.50 KB");
    }

    // ============== format_duration Tests ==============

    #[test]
    fn test_format_duration_zero() {
        let result = format_duration(0);
        assert_eq!(result, "00:00:00");
    }

    #[test]
    fn test_format_duration_seconds_only() {
        let result = format_duration(45);
        assert_eq!(result, "00:00:45");
    }

    #[test]
    fn test_format_duration_one_minute() {
        let result = format_duration(60);
        assert_eq!(result, "00:01:00");
    }

    #[test]
    fn test_format_duration_minutes_and_seconds() {
        let result = format_duration(125);
        assert_eq!(result, "00:02:05");
    }

    #[test]
    fn test_format_duration_one_hour() {
        let result = format_duration(3600);
        assert_eq!(result, "01:00:00");
    }

    #[test]
    fn test_format_duration_hours_minutes_seconds() {
        let result = format_duration(3661);
        assert_eq!(result, "01:01:01");
    }

    #[test]
    fn test_format_duration_large_value() {
        let result = format_duration(86400); // 24 hours
        assert_eq!(result, "24:00:00");
    }

    #[test]
    fn test_format_duration_complex() {
        let result = format_duration(7384); // 2:03:04
        assert_eq!(result, "02:03:04");
    }

    // ============== parse_duration Tests ==============

    #[test]
    fn test_parse_duration_seconds_only() {
        let result = parse_duration("45").unwrap();
        assert_eq!(result, 45);
    }

    #[test]
    fn test_parse_duration_minutes_seconds() {
        let result = parse_duration("02:30").unwrap();
        assert_eq!(result, 150);
    }

    #[test]
    fn test_parse_duration_hours_minutes_seconds() {
        let result = parse_duration("01:30:45").unwrap();
        assert_eq!(result, 5445);
    }

    #[test]
    fn test_parse_duration_zero() {
        let result = parse_duration("0").unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_parse_duration_zero_padded() {
        let result = parse_duration("00:00:00").unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_parse_duration_large_hours() {
        let result = parse_duration("100:00:00").unwrap();
        assert_eq!(result, 360000);
    }

    #[test]
    fn test_parse_duration_invalid_format() {
        let result = parse_duration("not:a:number:here");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_duration_invalid_characters() {
        let result = parse_duration("ab:cd:ef");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_duration_empty_string() {
        let result = parse_duration("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_duration_partial_invalid() {
        let result = parse_duration("10:ab");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_duration_roundtrip() {
        let original = 7384_u64;
        let formatted = format_duration(original);
        let parsed = parse_duration(&formatted).unwrap();
        assert_eq!(original, parsed);
    }

    // ============== extract_video_id Tests ==============

    #[test]
    fn test_extract_video_id_standard_url() {
        let result = extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
        assert_eq!(result, Some("dQw4w9WgXcQ".to_string()));
    }

    #[test]
    fn test_extract_video_id_without_www() {
        let result = extract_video_id("https://youtube.com/watch?v=dQw4w9WgXcQ");
        assert_eq!(result, Some("dQw4w9WgXcQ".to_string()));
    }

    #[test]
    fn test_extract_video_id_short_url() {
        let result = extract_video_id("https://youtu.be/dQw4w9WgXcQ");
        assert_eq!(result, Some("dQw4w9WgXcQ".to_string()));
    }

    #[test]
    fn test_extract_video_id_embed_url() {
        let result = extract_video_id("https://www.youtube.com/embed/dQw4w9WgXcQ");
        assert_eq!(result, Some("dQw4w9WgXcQ".to_string()));
    }

    #[test]
    fn test_extract_video_id_with_extra_params() {
        let result =
            extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ&list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf");
        assert_eq!(result, Some("dQw4w9WgXcQ".to_string()));
    }

    #[test]
    fn test_extract_video_id_with_timestamp() {
        let result = extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ&t=120");
        assert_eq!(result, Some("dQw4w9WgXcQ".to_string()));
    }

    #[test]
    fn test_extract_video_id_short_url_with_params() {
        let result = extract_video_id("https://youtu.be/dQw4w9WgXcQ?t=120");
        assert_eq!(result, Some("dQw4w9WgXcQ".to_string()));
    }

    #[test]
    fn test_extract_video_id_invalid_url() {
        let result = extract_video_id("not a url");
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_video_id_wrong_domain() {
        let result = extract_video_id("https://vimeo.com/123456789");
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_video_id_no_video_param() {
        let result = extract_video_id("https://www.youtube.com/");
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_video_id_empty_string() {
        let result = extract_video_id("");
        assert!(result.is_none());
    }

    // ============== extract_playlist_id Tests ==============

    #[test]
    fn test_extract_playlist_id_standard_url() {
        let result =
            extract_playlist_id("https://www.youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf");
        assert_eq!(result, Some("PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf".to_string()));
    }

    #[test]
    fn test_extract_playlist_id_from_video_url() {
        let result = extract_playlist_id(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ&list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf",
        );
        assert_eq!(result, Some("PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf".to_string()));
    }

    #[test]
    fn test_extract_playlist_id_no_list_param() {
        let result = extract_playlist_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_playlist_id_invalid_url() {
        let result = extract_playlist_id("not a url");
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_playlist_id_empty_string() {
        let result = extract_playlist_id("");
        assert!(result.is_none());
    }

    // ============== apply_template Tests ==============

    #[test]
    fn test_apply_template_title_only() {
        let meta = VideoMetadata {
            title: "My Video",
            id: "abc123",
            date: None,
            duration: None,
        };

        let result = apply_template("{title}", &meta);
        assert_eq!(result, "My Video");
    }

    #[test]
    fn test_apply_template_id_only() {
        let meta = VideoMetadata {
            title: "My Video",
            id: "abc123",
            date: None,
            duration: None,
        };

        let result = apply_template("{id}", &meta);
        assert_eq!(result, "abc123");
    }

    #[test]
    fn test_apply_template_title_and_id() {
        let meta = VideoMetadata {
            title: "My Video",
            id: "abc123",
            date: None,
            duration: None,
        };

        let result = apply_template("{title}-{id}", &meta);
        assert_eq!(result, "My Video-abc123");
    }

    #[test]
    fn test_apply_template_with_date() {
        let date = Utc.with_ymd_and_hms(2024, 6, 15, 0, 0, 0).unwrap();
        let meta = VideoMetadata {
            title: "My Video",
            id: "abc123",
            date: Some(date),
            duration: None,
        };

        let result = apply_template("{title}-{date}", &meta);
        assert_eq!(result, "My Video-2024-06-15");
    }

    #[test]
    fn test_apply_template_with_duration() {
        let meta = VideoMetadata {
            title: "My Video",
            id: "abc123",
            date: None,
            duration: Some("10:30"),
        };

        let result = apply_template("{title}-{duration}", &meta);
        assert_eq!(result, "My Video-10:30");
    }

    #[test]
    fn test_apply_template_full() {
        let date = Utc.with_ymd_and_hms(2024, 6, 15, 0, 0, 0).unwrap();
        let meta = VideoMetadata {
            title: "My Video",
            id: "abc123",
            date: Some(date),
            duration: Some("10:30"),
        };

        let result = apply_template("{title}-{id}-{date}-{duration}", &meta);
        assert_eq!(result, "My Video-abc123-2024-06-15-10:30");
    }

    #[test]
    fn test_apply_template_sanitizes_title() {
        let meta = VideoMetadata {
            title: "My: Video? Name",
            id: "abc123",
            date: None,
            duration: None,
        };
    
        let result = apply_template("{title}", &meta);
        assert_eq!(result, "My_ Video_ Name");
    }

    #[test]
    fn test_apply_template_no_placeholders() {
        let meta = VideoMetadata {
            title: "My Video",
            id: "abc123",
            date: None,
            duration: None,
        };

        let result = apply_template("static_name", &meta);
        assert_eq!(result, "static_name");
    }

    #[test]
    fn test_apply_template_missing_duration() {
        let meta = VideoMetadata {
            title: "My Video",
            id: "abc123",
            date: None,
            duration: None,
        };

        let result = apply_template("{title}-{duration}", &meta);
        assert_eq!(result, "My Video-");
    }

    #[test]
    fn test_apply_template_uses_current_date_when_none() {
        let meta = VideoMetadata {
            title: "My Video",
            id: "abc123",
            date: None,
            duration: None,
        };

        let result = apply_template("{date}", &meta);
        let today = Utc::now().format("%Y-%m-%d").to_string();
        assert_eq!(result, today);
    }

    // ============== VideoMetadata Tests ==============

    #[test]
    fn test_video_metadata_creation() {
        let meta = VideoMetadata {
            title: "Test Title",
            id: "test123",
            date: None,
            duration: Some("05:30"),
        };

        assert_eq!(meta.title, "Test Title");
        assert_eq!(meta.id, "test123");
        assert!(meta.date.is_none());
        assert_eq!(meta.duration, Some("05:30"));
    }

    #[test]
    fn test_video_metadata_with_all_fields() {
        let date = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let meta = VideoMetadata {
            title: "Full Video",
            id: "full123",
            date: Some(date),
            duration: Some("01:30:00"),
        };

        assert_eq!(meta.title, "Full Video");
        assert_eq!(meta.id, "full123");
        assert!(meta.date.is_some());
        assert_eq!(meta.duration, Some("01:30:00"));
    }
}
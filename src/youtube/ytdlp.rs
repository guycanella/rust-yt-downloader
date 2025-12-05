//! Integration with yt-dlp command-line tool.
//!
//! This module provides a client that wraps the yt-dlp command-line tool for
//! downloading YouTube videos and extracting metadata. yt-dlp is a feature-rich
//! YouTube downloader that must be installed separately.
//!
//! # Installation
//!
//! yt-dlp must be installed on the system:
//! - **macOS**: `brew install yt-dlp`
//! - **Linux**: `sudo apt install yt-dlp` or download from GitHub
//! - **Windows**: `choco install yt-dlp` or download from GitHub
//!
//! # Features
//!
//! - Extract video metadata with full details
//! - Download videos with quality selection
//! - Download audio with format conversion
//! - Process playlists with full video lists
//! - Automatic quality string to yt-dlp format conversion

use serde::Deserialize;
use std::process::Command;

use crate::error::{AppError, AppResult};
use crate::youtube::metadata::{PlaylistInfo, StreamInfo, VideoInfo};

/// Client for interacting with the yt-dlp command-line tool.
///
/// Provides a Rust interface to yt-dlp functionality including video downloads,
/// metadata extraction, and playlist processing. All methods check for yt-dlp
/// availability before execution.
///
/// # Examples
///
/// ```no_run
/// # use rust_yt_downloader::youtube::YtDlpClient;
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Check if yt-dlp is installed
/// if !YtDlpClient::is_available() {
///     eprintln!("yt-dlp is not installed");
///     return Ok(());
/// }
///
/// let client = YtDlpClient::new();
///
/// // Get video information
/// let info = client.get_video_info("https://www.youtube.com/watch?v=dQw4w9WgXcQ")?;
/// println!("Title: {}", info.title);
///
/// // Download video
/// client.download(
///     "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
///     "video.mp4",
///     None
/// )?;
/// # Ok(())
/// # }
/// ```
pub struct YtDlpClient;

/// Internal structure for deserializing yt-dlp JSON output for videos.
///
/// Maps to the JSON schema returned by `yt-dlp --dump-json`.
#[derive(Debug, Deserialize)]
struct YtDlpOutput {
    id: String,
    title: String,
    description: Option<String>,
    duration: Option<f64>,
    thumbnail: Option<String>,
    channel: Option<String>,
    upload_date: Option<String>,
    view_count: Option<u64>,
    formats: Option<Vec<YtDlpFormat>>,
}

/// Internal structure for deserializing format/stream information from yt-dlp.
///
/// Represents a single available format (stream) with its technical details.
#[derive(Debug, Deserialize)]
struct YtDlpFormat {
    format_id: String,
    url: Option<String>,
    ext: Option<String>,
    resolution: Option<String>,
    height: Option<u32>,
    width: Option<u32>,
    vcodec: Option<String>,
    acodec: Option<String>,
    filesize: Option<u64>,
    tbr: Option<f64>,
    fps: Option<f64>,
}

/// Internal structure for deserializing yt-dlp playlist JSON output.
///
/// Maps to the JSON schema returned by `yt-dlp --dump-json --flat-playlist`.
#[derive(Debug, Deserialize)]
struct YtDlpPlaylist {
    id: String,
    title: String,
    description: Option<String>,
    channel: Option<String>,
    playlist_count: Option<u64>,
    entries: Option<Vec<YtDlpPlaylistEntry>>,
}

/// Internal structure for deserializing individual playlist entries.
#[derive(Debug, Deserialize)]
struct YtDlpPlaylistEntry {
    id: String,
    title: Option<String>,
}

impl YtDlpClient {
    /// Creates a new yt-dlp client instance.
    pub fn new() -> Self {
        Self
    }

    /// Checks if yt-dlp is installed and available on the system.
    ///
    /// Attempts to run `yt-dlp --version` and returns `true` if successful.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_yt_downloader::youtube::YtDlpClient;
    ///
    /// if YtDlpClient::is_available() {
    ///     println!("yt-dlp is installed");
    /// } else {
    ///     println!("yt-dlp is not available");
    /// }
    /// ```
    pub fn is_available() -> bool {
        Command::new("yt-dlp")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Returns the installed yt-dlp version.
    ///
    /// # Errors
    ///
    /// Returns an error if yt-dlp is not installed or the version cannot be retrieved.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::youtube::YtDlpClient;
    ///
    /// match YtDlpClient::version() {
    ///     Ok(version) => println!("yt-dlp version: {}", version),
    ///     Err(_) => println!("yt-dlp not found"),
    /// }
    /// ```
    pub fn version() -> AppResult<String> {
        let output = Command::new("yt-dlp")
            .arg("--version")
            .output()
            .map_err(|_| AppError::Other("yt-dlp not found".to_string()))?;

        if !output.status.success() {
            return Err(AppError::Other("Failed to get yt-dlp version".to_string()));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Checks that yt-dlp is available and returns an error if not.
    ///
    /// This is a convenience method called by other methods that require yt-dlp.
    ///
    /// # Errors
    ///
    /// Returns an error with installation instructions if yt-dlp is not found.
    pub fn require() -> AppResult<()> {
        if !Self::is_available() {
            return Err(AppError::Other(
                "yt-dlp is required but not installed. Install with: brew install yt-dlp"
                    .to_string(),
            ));
        }
        Ok(())
    }

    /// Extracts video metadata using yt-dlp.
    ///
    /// Runs `yt-dlp --dump-json` to retrieve complete video information including
    /// all available formats/streams.
    ///
    /// # Arguments
    ///
    /// * `url` - A valid YouTube video URL
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - yt-dlp is not installed
    /// - The video URL is invalid
    /// - The video is unavailable or private
    /// - JSON parsing fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rust_yt_downloader::youtube::YtDlpClient;
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = YtDlpClient::new();
    /// let info = client.get_video_info("https://www.youtube.com/watch?v=dQw4w9WgXcQ")?;
    /// println!("Title: {}", info.title);
    /// println!("Streams: {}", info.streams.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_video_info(&self, url: &str) -> AppResult<VideoInfo> {
        Self::require()?;

        let output = Command::new("yt-dlp")
            .args(["--dump-json", "--no-warnings", "--no-playlist", url])
            .output()
            .map_err(|e| AppError::ExtractionFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::ExtractionFailed(stderr.to_string()));
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        let yt_info: YtDlpOutput = serde_json::from_str(&json_str)
            .map_err(|e| AppError::ExtractionFailed(format!("Failed to parse JSON: {}", e)))?;

        Ok(self.convert_to_video_info(yt_info))
    }

    /// Extracts playlist metadata using yt-dlp.
    ///
    /// Runs `yt-dlp --dump-json --flat-playlist` to retrieve playlist information
    /// and the list of video IDs without downloading.
    ///
    /// # Arguments
    ///
    /// * `url` - A valid YouTube playlist URL
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - yt-dlp is not installed
    /// - The playlist URL is invalid
    /// - The playlist is unavailable or private
    /// - No videos are found in the playlist
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rust_yt_downloader::youtube::YtDlpClient;
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = YtDlpClient::new();
    /// let playlist = client.get_playlist_info(
    ///     "https://www.youtube.com/playlist?list=PLtest"
    /// )?;
    /// println!("Playlist: {}", playlist.title);
    /// println!("Videos: {}", playlist.video_count);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_playlist_info(&self, url: &str) -> AppResult<PlaylistInfo> {
        Self::require()?;

        let output = Command::new("yt-dlp")
            .args(["--dump-json", "--flat-playlist", "--no-warnings", url])
            .output()
            .map_err(|e| AppError::ExtractionFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::ExtractionFailed(stderr.to_string()));
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = json_str.lines().collect();

        if lines.is_empty() {
            return Err(AppError::PlaylistNotFound {
                playlist_id: url.to_string(),
            });
        }

        if let Ok(playlist) = serde_json::from_str::<YtDlpPlaylist>(&json_str) {
            return Ok(self.convert_to_playlist_info(playlist));
        }

        let mut video_ids = Vec::new();
        let mut title = "Unknown Playlist".to_string();

        for line in lines {
            if let Ok(entry) = serde_json::from_str::<YtDlpPlaylistEntry>(line) {
                video_ids.push(entry.id);
            }
        }

        Ok(PlaylistInfo {
            id: "unknown".to_string(),
            title,
            description: None,
            channel: None,
            video_count: video_ids.len() as u64,
            video_ids,
        })
    }

    /// Downloads a video using yt-dlp.
    ///
    /// Downloads a video to the specified output path with optional format selection.
    ///
    /// # Arguments
    ///
    /// * `url` - YouTube video URL to download
    /// * `output_path` - File path for the downloaded video
    /// * `format` - Optional yt-dlp format string (e.g., "bestvideo+bestaudio")
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - yt-dlp is not installed
    /// - The download fails
    /// - The output path is invalid
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rust_yt_downloader::youtube::YtDlpClient;
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = YtDlpClient::new();
    ///
    /// // Download with default format
    /// client.download(
    ///     "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    ///     "video.mp4",
    ///     None
    /// )?;
    ///
    /// // Download with specific format
    /// client.download(
    ///     "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    ///     "video.mp4",
    ///     Some("bestvideo[height<=720]+bestaudio")
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn download(&self, url: &str, output_path: &str, format: Option<&str>) -> AppResult<()> {
        Self::require()?;

        let mut args = vec![
            "--no-warnings".to_string(),
            "-o".to_string(),
            output_path.to_string(),
        ];

        if let Some(fmt) = format {
            args.push("-f".to_string());
            args.push(fmt.to_string());
        }

        args.push(url.to_string());

        let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

        let output = Command::new("yt-dlp")
            .args(&args_ref)
            .output()
            .map_err(|e| AppError::ExtractionFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::ExtractionFailed(stderr.to_string()));
        }

        Ok(())
    }

    /// Downloads audio from a video and converts it to the specified format.
    ///
    /// Uses yt-dlp's `-x` flag to extract audio and `--audio-format` to convert
    /// to the desired format.
    ///
    /// # Arguments
    ///
    /// * `url` - YouTube video URL to download audio from
    /// * `output_path` - File path for the downloaded audio file
    /// * `format` - Audio format (mp3, m4a, flac, wav, opus)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - yt-dlp is not installed
    /// - The download or conversion fails
    /// - The format is not supported
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rust_yt_downloader::youtube::YtDlpClient;
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = YtDlpClient::new();
    ///
    /// client.download_audio(
    ///     "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    ///     "audio.mp3",
    ///     "mp3"
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn download_audio(&self, url: &str, output_path: &str, format: &str) -> AppResult<()> {
        Self::require()?;

        let output = Command::new("yt-dlp")
            .args([
                "--no-warnings",
                "-x",
                "--audio-format",
                format,
                "-o",
                output_path,
                url,
            ])
            .output()
            .map_err(|e| AppError::ExtractionFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::ExtractionFailed(stderr.to_string()));
        }

        Ok(())
    }

    /// Downloads a video with a specific quality preference.
    ///
    /// Converts quality strings (like "1080p", "best", "4k") to appropriate
    /// yt-dlp format strings automatically.
    ///
    /// # Arguments
    ///
    /// * `url` - YouTube video URL to download
    /// * `output_path` - File path for the downloaded video
    /// * `quality` - Quality string (best, worst, 4k, 2160p, 1440p, 1080p, 720p, etc.)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rust_yt_downloader::youtube::YtDlpClient;
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = YtDlpClient::new();
    ///
    /// // Download best quality
    /// client.download_with_quality(
    ///     "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    ///     "video.mp4",
    ///     "best"
    /// )?;
    ///
    /// // Download 720p
    /// client.download_with_quality(
    ///     "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    ///     "video.mp4",
    ///     "720p"
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn download_with_quality(
        &self,
        url: &str,
        output_path: &str,
        quality: &str,
    ) -> AppResult<()> {
        Self::require()?;

        let format_str = self.quality_to_format(quality);

        let output = Command::new("yt-dlp")
            .args(["--no-warnings", "-f", &format_str, "-o", output_path, url])
            .output()
            .map_err(|e| AppError::ExtractionFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::ExtractionFailed(stderr.to_string()));
        }

        Ok(())
    }

    /// Converts yt-dlp JSON output to VideoInfo structure.
    ///
    /// Transforms the yt-dlp output format into our internal VideoInfo representation,
    /// converting all streams and handling optional fields.
    fn convert_to_video_info(&self, yt: YtDlpOutput) -> VideoInfo {
        let streams = yt
            .formats
            .unwrap_or_default()
            .into_iter()
            .filter_map(|f| self.convert_to_stream_info(f))
            .collect();

        VideoInfo {
            id: yt.id,
            title: yt.title,
            description: yt.description,
            duration: yt.duration.map(|d| d as u64).unwrap_or(0),
            thumbnail_url: yt.thumbnail,
            channel: yt.channel,
            publish_date: yt.upload_date,
            view_count: yt.view_count,
            streams,
        }
    }

    /// Converts a yt-dlp format entry to StreamInfo.
    ///
    /// Handles codec detection, quality labeling, and audio-only stream identification.
    /// Returns None if the format lacks essential information (like a URL).
    fn convert_to_stream_info(&self, f: YtDlpFormat) -> Option<StreamInfo> {
        let url = f.url?;

        let is_audio_only =
            f.vcodec.as_deref() == Some("none") || (f.height.is_none() && f.acodec.is_some());

        let quality = if is_audio_only {
            "audio".to_string()
        } else {
            f.height
                .map(|h| format!("{}p", h))
                .or(f.resolution.clone())
                .unwrap_or_else(|| "unknown".to_string())
        };

        Some(StreamInfo {
            url,
            quality,
            format: f.ext.unwrap_or_else(|| "unknown".to_string()),
            video_codec: f.vcodec.filter(|v| v != "none"),
            audio_codec: f.acodec.filter(|a| a != "none"),
            is_audio_only,
            file_size: f.filesize,
            bitrate: f.tbr.map(|b| b as u64),
            fps: f.fps.map(|f| f as u32),
        })
    }

    /// Converts yt-dlp playlist JSON to PlaylistInfo structure.
    ///
    /// Extracts playlist metadata and video IDs from the yt-dlp output.
    fn convert_to_playlist_info(&self, pl: YtDlpPlaylist) -> PlaylistInfo {
        let video_ids = pl
            .entries
            .unwrap_or_default()
            .into_iter()
            .map(|e| e.id)
            .collect::<Vec<_>>();

        PlaylistInfo {
            id: pl.id,
            title: pl.title,
            description: pl.description,
            channel: pl.channel,
            video_count: pl.playlist_count.unwrap_or(video_ids.len() as u64),
            video_ids,
        }
    }

    /// Converts a quality string to a yt-dlp format selector.
    ///
    /// Maps human-readable quality strings (like "1080p", "best") to yt-dlp's
    /// format selection syntax. Ensures both video and audio are downloaded
    /// and merged when possible.
    ///
    /// # Format Mappings
    ///
    /// - `"best"` → `"bestvideo+bestaudio/best"`
    /// - `"worst"` → `"worstvideo+worstaudio/worst"`
    /// - `"1080p"` → `"bestvideo[height<=1080]+bestaudio/best[height<=1080]"`
    /// - And so on for other resolutions (4k, 1440p, 720p, 480p, 360p, 240p, 144p)
    fn quality_to_format(&self, quality: &str) -> String {
        match quality.to_lowercase().as_str() {
            "best" => "bestvideo+bestaudio/best".to_string(),
            "worst" => "worstvideo+worstaudio/worst".to_string(),
            "4k" | "2160p" => "bestvideo[height<=2160]+bestaudio/best[height<=2160]".to_string(),
            "1440p" => "bestvideo[height<=1440]+bestaudio/best[height<=1440]".to_string(),
            "1080p" => "bestvideo[height<=1080]+bestaudio/best[height<=1080]".to_string(),
            "720p" => "bestvideo[height<=720]+bestaudio/best[height<=720]".to_string(),
            "480p" => "bestvideo[height<=480]+bestaudio/best[height<=480]".to_string(),
            "360p" => "bestvideo[height<=360]+bestaudio/best[height<=360]".to_string(),
            "240p" => "bestvideo[height<=240]+bestaudio/best[height<=240]".to_string(),
            "144p" => "bestvideo[height<=144]+bestaudio/best[height<=144]".to_string(),
            _ => "bestvideo+bestaudio/best".to_string(),
        }
    }
}

impl Default for YtDlpClient {
    fn default() -> Self {
        Self::new()
    }
}

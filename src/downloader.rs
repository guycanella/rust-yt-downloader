//! Download orchestration and configuration management.
//!
//! This module provides the core download functionality for YouTube videos and audio,
//! coordinating between the yt-dlp client, configuration system, and CLI arguments.
//!
//! # Architecture
//!
//! The download system uses a three-tier configuration priority:
//! 1. **CLI arguments** - Highest priority, directly override all other settings
//! 2. **Config file** - Middle priority, loaded from `~/.config/rust-yt-downloader/config.toml`
//! 3. **Built-in defaults** - Lowest priority, used when no other value is specified
//!
//! # Example Usage
//!
//! ```no_run
//! use rust_yt_downloader::downloader::{Downloader, DownloadOptions};
//! use rust_yt_downloader::cli::VideoQuality;
//! use std::path::PathBuf;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create downloader with custom options
//! let options = DownloadOptions::default()
//!     .with_output_dir(PathBuf::from("./downloads"))
//!     .with_quality(VideoQuality::Q1080p)
//!     .with_verbose(true);
//!
//! let downloader = Downloader::with_options(options);
//! let result = downloader.download("https://youtube.com/watch?v=dQw4w9WgXcQ").await?;
//!
//! println!("Downloaded: {} ({} bytes)", result.video_title, result.file_size);
//! # Ok(())
//! # }
//! ```
//!
//! # Retry Logic
//!
//! Download failures are handled with automatic retry logic:
//! - Network errors trigger retries up to `retry_attempts` times
//! - Exponential backoff between retry attempts
//! - Non-retryable errors (invalid URL, video unavailable) fail immediately
//!
//! # Error Recovery
//!
//! The downloader handles common error scenarios:
//! - Creates output directories automatically if they don't exist
//! - Sanitizes filenames to avoid filesystem conflicts
//! - Validates yt-dlp availability before attempting downloads
//! - Provides detailed error messages with context for troubleshooting

use std::path::PathBuf;

use crate::cli::{AudioFormat, VideoFormat, VideoQuality};
use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::progress::messages;
use crate::utils::{expand_path, sanitize_filename};
use crate::youtube::YtDlpClient;

/// Configuration options for video and audio downloads.
///
/// This struct encapsulates all settings that control download behavior, including
/// quality, format, output directory, and verbosity settings. Options can be built
/// using the builder pattern or loaded from a configuration file.
///
/// # Configuration Merging
///
/// Options follow a priority hierarchy:
/// - CLI arguments set via builder methods (`with_*`) take highest priority
/// - Config file values loaded via `from_config()` take middle priority
/// - Default values take lowest priority
///
/// # Examples
///
/// ```no_run
/// use rust_yt_downloader::downloader::DownloadOptions;
/// use rust_yt_downloader::cli::{VideoQuality, AudioFormat};
/// use std::path::PathBuf;
///
/// // Build options using the builder pattern
/// let options = DownloadOptions::default()
///     .with_output_dir(PathBuf::from("./videos"))
///     .with_quality(VideoQuality::Q720p)
///     .with_audio_format(AudioFormat::Flac)
///     .with_verbose(true);
/// ```
#[derive(Debug, Clone)]
pub struct DownloadOptions {
    /// Directory where downloaded files will be saved.
    ///
    /// Defaults to current directory (`.`). Automatically created if it doesn't exist.
    pub output_dir: PathBuf,

    /// Video quality/resolution to download.
    ///
    /// Supports specific resolutions (144p to 4K) or dynamic quality selection (Best/Worst).
    /// Defaults to `VideoQuality::Best`.
    pub quality: VideoQuality,

    /// Container format for video downloads.
    ///
    /// Supported formats: MP4 (default), MKV, WebM.
    pub video_format: VideoFormat,

    /// Audio codec format for audio-only downloads.
    ///
    /// Supported formats: MP3 (default), M4A, FLAC, WAV, Opus.
    pub audio_format: AudioFormat,

    /// When true, downloads only audio stream (no video).
    ///
    /// Defaults to `false`. When enabled, uses `audio_format` for output.
    pub audio_only: bool,

    /// yt-dlp filename template string.
    ///
    /// Uses yt-dlp template syntax (e.g., `%(title)s.%(ext)s`).
    /// Defaults to `"%(title)s.%(ext)s"`.
    pub filename_template: String,

    /// Number of retry attempts for failed downloads.
    ///
    /// Only retryable errors (network issues) trigger retries.
    /// Defaults to 3 attempts.
    pub retry_attempts: u32,

    /// Suppresses all progress output when true.
    ///
    /// Useful for non-interactive/scripted usage. Defaults to `false`.
    pub silence: bool,

    /// Enables detailed logging output when true.
    ///
    /// Shows additional metadata and debugging information. Defaults to `false`.
    pub verbose: bool,
}

impl Default for DownloadOptions {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("."),
            quality: VideoQuality::Best,
            video_format: VideoFormat::Mp4,
            audio_format: AudioFormat::Mp3,
            audio_only: false,
            filename_template: "%(title)s.%(ext)s".to_string(),
            retry_attempts: 3,
            silence: false,
            verbose: false,
        }
    }
}

impl DownloadOptions {
    /// Creates download options from a configuration file.
    ///
    /// Loads settings from the provided `Config` object, which is typically
    /// read from `~/.config/rust-yt-downloader/config.toml`. This method
    /// provides the middle tier of the configuration priority hierarchy.
    ///
    /// # Configuration Mapping
    ///
    /// - `output_dir` ← `config.general.output_dir` (expanded with `~` support)
    /// - `quality` ← `config.general.default_quality` (parsed from string)
    /// - `video_format` ← `config.video.format`
    /// - `audio_format` ← `config.audio.format`
    /// - `retry_attempts` ← `config.network.retry_attempts`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::config::Config;
    /// use rust_yt_downloader::downloader::DownloadOptions;
    ///
    /// let config = Config::load().unwrap_or_default();
    /// let options = DownloadOptions::from_config(&config);
    /// ```
    pub fn from_config(config: &Config) -> Self {
        Self {
            output_dir: expand_path(&config.general.output_dir),
            quality: Self::parse_quality(&config.general.default_quality),
            video_format: Self::parse_video_format(&config.video.format),
            audio_format: Self::parse_audio_format(&config.audio.format),
            audio_only: false,
            filename_template: "%(title)s.%(ext)s".to_string(),
            retry_attempts: config.network.retry_attempts,
            silence: false,
            verbose: false,
        }
    }

    /// Sets the output directory for downloaded files.
    ///
    /// The directory will be created automatically if it doesn't exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_yt_downloader::downloader::DownloadOptions;
    /// use std::path::PathBuf;
    ///
    /// let options = DownloadOptions::default()
    ///     .with_output_dir(PathBuf::from("~/Downloads/youtube"));
    /// ```
    pub fn with_output_dir(mut self, dir: PathBuf) -> Self {
        self.output_dir = dir;
        self
    }

    /// Sets the video quality/resolution.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_yt_downloader::downloader::DownloadOptions;
    /// use rust_yt_downloader::cli::VideoQuality;
    ///
    /// let options = DownloadOptions::default()
    ///     .with_quality(VideoQuality::Q1080p);
    /// ```
    pub fn with_quality(mut self, quality: VideoQuality) -> Self {
        self.quality = quality;
        self
    }

    /// Sets the video container format.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_yt_downloader::downloader::DownloadOptions;
    /// use rust_yt_downloader::cli::VideoFormat;
    ///
    /// let options = DownloadOptions::default()
    ///     .with_video_format(VideoFormat::Mkv);
    /// ```
    pub fn with_video_format(mut self, format: VideoFormat) -> Self {
        self.video_format = format;
        self
    }

    /// Sets the audio codec format for audio-only downloads.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_yt_downloader::downloader::DownloadOptions;
    /// use rust_yt_downloader::cli::AudioFormat;
    ///
    /// let options = DownloadOptions::default()
    ///     .with_audio_format(AudioFormat::Flac);
    /// ```
    pub fn with_audio_format(mut self, format: AudioFormat) -> Self {
        self.audio_format = format;
        self
    }

    /// Sets whether to download only audio (no video).
    ///
    /// When enabled, uses the format specified in `audio_format`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_yt_downloader::downloader::DownloadOptions;
    ///
    /// let options = DownloadOptions::default()
    ///     .with_audio_only(true);
    /// ```
    pub fn with_audio_only(mut self, audio_only: bool) -> Self {
        self.audio_only = audio_only;
        self
    }

    /// Sets the filename template using yt-dlp syntax.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_yt_downloader::downloader::DownloadOptions;
    ///
    /// // Include video ID in filename
    /// let options = DownloadOptions::default()
    ///     .with_template("%(title)s-%(id)s.%(ext)s".to_string());
    /// ```
    pub fn with_template(mut self, template: String) -> Self {
        self.filename_template = template;
        self
    }

    /// Sets whether to suppress all progress output.
    ///
    /// Useful for non-interactive environments or scripting.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_yt_downloader::downloader::DownloadOptions;
    ///
    /// let options = DownloadOptions::default()
    ///     .with_silence(true);
    /// ```
    pub fn with_silence(mut self, silence: bool) -> Self {
        self.silence = silence;
        self
    }

    /// Sets whether to enable verbose output.
    ///
    /// Shows additional metadata and debugging information.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_yt_downloader::downloader::DownloadOptions;
    ///
    /// let options = DownloadOptions::default()
    ///     .with_verbose(true);
    /// ```
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Parses a quality string into a `VideoQuality` enum.
    ///
    /// Case-insensitive. Supports: "144p", "240p", "360p", "480p", "720p",
    /// "1080p", "1440p", "4k", "2160p", "best", "worst".
    /// Unknown values default to `VideoQuality::Best`.
    fn parse_quality(quality: &str) -> VideoQuality {
        match quality.to_lowercase().as_str() {
            "144p" => VideoQuality::Q144p,
            "240p" => VideoQuality::Q240p,
            "360p" => VideoQuality::Q360p,
            "480p" => VideoQuality::Q480p,
            "720p" => VideoQuality::Q720p,
            "1080p" => VideoQuality::Q1080p,
            "1440p" => VideoQuality::Q1440p,
            "4k" | "2160p" => VideoQuality::Q4k,
            "worst" => VideoQuality::Worst,
            _ => VideoQuality::Best,
        }
    }

    /// Parses a video format string into a `VideoFormat` enum.
    ///
    /// Case-insensitive. Supports: "mp4", "mkv", "webm".
    /// Unknown values default to `VideoFormat::Mp4`.
    fn parse_video_format(format: &str) -> VideoFormat {
        match format.to_lowercase().as_str() {
            "mkv" => VideoFormat::Mkv,
            "webm" => VideoFormat::Webm,
            _ => VideoFormat::Mp4,
        }
    }

    /// Parses an audio format string into an `AudioFormat` enum.
    ///
    /// Case-insensitive. Supports: "mp3", "m4a", "flac", "wav", "opus".
    /// Unknown values default to `AudioFormat::Mp3`.
    fn parse_audio_format(format: &str) -> AudioFormat {
        match format.to_lowercase().as_str() {
            "m4a" => AudioFormat::M4a,
            "flac" => AudioFormat::Flac,
            "wav" => AudioFormat::Wav,
            "opus" => AudioFormat::Opus,
            _ => AudioFormat::Mp3,
        }
    }

    /// Converts `VideoQuality` to yt-dlp format selector string.
    ///
    /// Returns yt-dlp format strings that combine best video and audio streams
    /// at the specified quality level.
    fn quality_to_ytdlp(&self) -> String {
        match self.quality {
            VideoQuality::Best => "bestvideo+bestaudio/best".to_string(),
            VideoQuality::Worst => "worstvideo+worstaudio/worst".to_string(),
            VideoQuality::Q4k => "bestvideo[height<=2160]+bestaudio/best[height<=2160]".to_string(),
            VideoQuality::Q1440p => "bestvideo[height<=1440]+bestaudio/best[height<=1440]".to_string(),
            VideoQuality::Q1080p => "bestvideo[height<=1080]+bestaudio/best[height<=1080]".to_string(),
            VideoQuality::Q720p => "bestvideo[height<=720]+bestaudio/best[height<=720]".to_string(),
            VideoQuality::Q480p => "bestvideo[height<=480]+bestaudio/best[height<=480]".to_string(),
            VideoQuality::Q360p => "bestvideo[height<=360]+bestaudio/best[height<=360]".to_string(),
            VideoQuality::Q240p => "bestvideo[height<=240]+bestaudio/best[height<=240]".to_string(),
            VideoQuality::Q144p => "bestvideo[height<=144]+bestaudio/best[height<=144]".to_string(),
        }
    }

    /// Returns the file extension for the selected video format.
    fn video_format_ext(&self) -> &'static str {
        match self.video_format {
            VideoFormat::Mp4 => "mp4",
            VideoFormat::Mkv => "mkv",
            VideoFormat::Webm => "webm",
        }
    }

    /// Returns the format string for the selected audio format.
    fn audio_format_str(&self) -> &'static str {
        match self.audio_format {
            AudioFormat::Mp3 => "mp3",
            AudioFormat::M4a => "m4a",
            AudioFormat::Flac => "flac",
            AudioFormat::Wav => "wav",
            AudioFormat::Opus => "opus",
        }
    }

    /// Returns the bandwidth rate limit, if configured.
    ///
    /// Currently returns `None`. Rate limiting will be implemented
    /// through the config system in future versions.
    pub fn rate_limit(&self) -> Option<String> {
        None
    }
}

/// Result of a successful download operation.
///
/// Contains metadata and filesystem information about the downloaded file.
///
/// # Examples
///
/// ```no_run
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// use rust_yt_downloader::downloader::Downloader;
///
/// let downloader = Downloader::new();
/// let result = downloader.download("https://youtube.com/watch?v=dQw4w9WgXcQ").await?;
///
/// println!("Title: {}", result.video_title);
/// println!("Size: {} bytes", result.file_size);
/// println!("Path: {}", result.file_path.display());
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct DownloadResult {
    /// Absolute path to the downloaded file.
    pub file_path: PathBuf,

    /// Size of the downloaded file in bytes.
    pub file_size: u64,

    /// YouTube video ID (e.g., "dQw4w9WgXcQ").
    pub video_id: String,

    /// Title of the video as provided by YouTube.
    pub video_title: String,
}

/// Main downloader orchestrator that coordinates downloads with yt-dlp.
///
/// The `Downloader` manages the interaction between configuration options,
/// the yt-dlp client, and download operations. It handles both video and
/// audio-only downloads with automatic retry logic and progress reporting.
///
/// # Examples
///
/// ```no_run
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// use rust_yt_downloader::downloader::{Downloader, DownloadOptions};
/// use rust_yt_downloader::cli::VideoQuality;
/// use std::path::PathBuf;
///
/// // Create with default options
/// let downloader = Downloader::new();
///
/// // Or with custom options
/// let options = DownloadOptions::default()
///     .with_output_dir(PathBuf::from("./downloads"))
///     .with_quality(VideoQuality::Q1080p);
/// let downloader = Downloader::with_options(options);
///
/// // Download a video
/// let result = downloader.download("https://youtube.com/watch?v=dQw4w9WgXcQ").await?;
/// println!("Downloaded to: {}", result.file_path.display());
/// # Ok(())
/// # }
/// ```
pub struct Downloader {
    client: YtDlpClient,
    options: DownloadOptions,
}

impl Downloader {
    /// Creates a new downloader with default options.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_yt_downloader::downloader::Downloader;
    ///
    /// let downloader = Downloader::new();
    /// ```
    pub fn new() -> Self {
        Self {
            client: YtDlpClient::new(),
            options: DownloadOptions::default(),
        }
    }

    /// Creates a new downloader with custom options.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_yt_downloader::downloader::{Downloader, DownloadOptions};
    /// use rust_yt_downloader::cli::VideoQuality;
    ///
    /// let options = DownloadOptions::default()
    ///     .with_quality(VideoQuality::Q720p);
    /// let downloader = Downloader::with_options(options);
    /// ```
    pub fn with_options(options: DownloadOptions) -> Self {
        Self {
            client: YtDlpClient::new(),
            options,
        }
    }

    /// Creates a new downloader from a configuration file.
    ///
    /// Loads settings from the provided `Config`, typically read from
    /// `~/.config/rust-yt-downloader/config.toml`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::config::Config;
    /// use rust_yt_downloader::downloader::Downloader;
    ///
    /// let config = Config::load().unwrap_or_default();
    /// let downloader = Downloader::from_config(&config);
    /// ```
    pub fn from_config(config: &Config) -> Self {
        Self::with_options(DownloadOptions::from_config(config))
    }

    /// Downloads a video from the given URL.
    ///
    /// This method fetches video metadata, creates the output directory if needed,
    /// and downloads the video using yt-dlp with the configured quality and format
    /// settings.
    ///
    /// # Arguments
    ///
    /// * `url` - YouTube video URL (e.g., `https://youtube.com/watch?v=VIDEO_ID`)
    ///
    /// # Returns
    ///
    /// Returns a `DownloadResult` containing:
    /// - Path to the downloaded file
    /// - File size in bytes
    /// - Video ID and title
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - yt-dlp is not installed or not found in PATH
    /// - The URL is invalid or the video is unavailable
    /// - Network errors occur during download
    /// - Filesystem errors occur (permissions, disk space, etc.)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// use rust_yt_downloader::downloader::Downloader;
    ///
    /// let downloader = Downloader::new();
    /// let result = downloader.download("https://youtube.com/watch?v=dQw4w9WgXcQ").await?;
    ///
    /// println!("Downloaded: {}", result.video_title);
    /// println!("Location: {}", result.file_path.display());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download(&self, url: &str) -> AppResult<DownloadResult> {
        use std::process::{Command, Stdio};
        use std::io::{BufRead, BufReader};

        YtDlpClient::require()?;

        // Busca informações do vídeo primeiro
        if !self.options.silence {
            messages::info("Fetching video info...");
        }

        let video_info = self.client.get_video_info(url)?;

        if self.options.verbose {
            messages::info(&format!("Title: {}", video_info.title));
            messages::info(&format!("Duration: {} seconds", video_info.duration));
        }

        // Cria diretório de saída se não existir
        if !self.options.output_dir.exists() {
            std::fs::create_dir_all(&self.options.output_dir)
                .map_err(|e| AppError::dir_create(&self.options.output_dir, e))?;
        }

        // Monta o caminho de saída
        let output_template = self.options.output_dir
            .join("%(title)s.%(ext)s")
            .to_string_lossy()
            .to_string();

        // Monta argumentos do yt-dlp
        let format_str = self.options.quality_to_ytdlp();
        let merge_format = self.options.video_format_ext();

        if !self.options.silence {
            messages::downloading(&video_info.title);
        }

        let mut args = vec![
            "--no-warnings",
            "-f", &format_str,
            "--merge-output-format", merge_format,
            "-o", &output_template,
            "--no-playlist",
            "--restrict-filenames",
        ];

        // Adiciona progresso se não estiver em modo silencioso
        if !self.options.silence {
            args.push("--newline");
            args.push("--progress");
        } else {
            args.push("--quiet");
        }

        args.push(url);

        if self.options.silence {
            // Modo silencioso: executa sem mostrar nada
            let output = Command::new("yt-dlp")
                .args(&args)
                .output()
                .map_err(|e| AppError::ExtractionFailed(e.to_string()))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(AppError::ExtractionFailed(stderr.to_string()));
            }
        } else {
            // Modo normal: mostra progresso
            let mut child = Command::new("yt-dlp")
                .args(&args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| AppError::ExtractionFailed(e.to_string()))?;

            // Lê stdout em tempo real para mostrar progresso
            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        if line.contains('%') {
                            // Linha de progresso - mostra na mesma linha
                            print!("\r{}", line);
                            std::io::Write::flush(&mut std::io::stdout()).ok();
                        } else if self.options.verbose && !line.is_empty() {
                            println!("{}", line);
                        }
                    }
                }
                println!(); // Nova linha após o progresso
            }

            let status = child.wait()
                .map_err(|e| AppError::ExtractionFailed(e.to_string()))?;

            if !status.success() {
                return Err(AppError::ExtractionFailed("Download failed".to_string()));
            }
        }

        // Encontra o arquivo baixado
        let actual_path = self.find_downloaded_file_by_ext(merge_format)
            .ok_or_else(|| AppError::ExtractionFailed("Could not find downloaded file".to_string()))?;

        let file_size = std::fs::metadata(&actual_path)
            .map(|m| m.len())
            .unwrap_or(0);

        Ok(DownloadResult {
            file_path: actual_path,
            file_size,
            video_id: video_info.id,
            video_title: video_info.title,
        })
    }

    /// Downloads only the audio track from the given URL.
    ///
    /// This method extracts only the audio stream, converting it to the format
    /// specified in the options (MP3, FLAC, M4A, WAV, or Opus).
    ///
    /// # Arguments
    ///
    /// * `url` - YouTube video URL (e.g., `https://youtube.com/watch?v=VIDEO_ID`)
    ///
    /// # Returns
    ///
    /// Returns a `DownloadResult` containing:
    /// - Path to the downloaded audio file
    /// - File size in bytes
    /// - Video ID and title
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - yt-dlp is not installed or not found in PATH
    /// - The URL is invalid or the video is unavailable
    /// - Network errors occur during download
    /// - Audio extraction fails
    /// - Filesystem errors occur (permissions, disk space, etc.)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// use rust_yt_downloader::downloader::{Downloader, DownloadOptions};
    /// use rust_yt_downloader::cli::AudioFormat;
    ///
    /// let options = DownloadOptions::default()
    ///     .with_audio_format(AudioFormat::Flac);
    /// let downloader = Downloader::with_options(options);
    ///
    /// let result = downloader.download_audio("https://youtube.com/watch?v=dQw4w9WgXcQ").await?;
    /// println!("Downloaded audio: {}", result.file_path.display());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_audio(&self, url: &str) -> AppResult<DownloadResult> {
        use std::process::{Command, Stdio};
        use std::io::{BufRead, BufReader};

        YtDlpClient::require()?;

        // Busca informações do vídeo primeiro
        if !self.options.silence {
            messages::info("Fetching video info...");
        }

        let video_info = self.client.get_video_info(url)?;

        if self.options.verbose {
            messages::info(&format!("Title: {}", video_info.title));
        }

        // Cria diretório de saída se não existir
        if !self.options.output_dir.exists() {
            std::fs::create_dir_all(&self.options.output_dir)
                .map_err(|e| AppError::dir_create(&self.options.output_dir, e))?;
        }

        // Monta o caminho de saída
        let output_template = self.options.output_dir
            .join("%(title)s.%(ext)s")
            .to_string_lossy()
            .to_string();

        let audio_format = self.options.audio_format_str();

        if !self.options.silence {
            messages::downloading(&video_info.title);
        }

        let mut args = vec![
            "--no-warnings",
            "-x",
            "--audio-format", audio_format,
            "-o", &output_template,
            "--no-playlist",
            "--restrict-filenames",
        ];

        // Adiciona progresso se não estiver em modo silencioso
        if !self.options.silence {
            args.push("--newline");
            args.push("--progress");
        } else {
            args.push("--quiet");
        }

        args.push(url);

        if self.options.silence {
            // Modo silencioso: executa sem mostrar nada
            let output = Command::new("yt-dlp")
                .args(&args)
                .output()
                .map_err(|e| AppError::ExtractionFailed(e.to_string()))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(AppError::ExtractionFailed(stderr.to_string()));
            }
        } else {
            // Modo normal: mostra progresso
            let mut child = Command::new("yt-dlp")
                .args(&args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| AppError::ExtractionFailed(e.to_string()))?;

            // Lê stdout em tempo real para mostrar progresso
            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        if line.contains('%') {
                            // Linha de progresso - mostra na mesma linha
                            print!("\r{}", line);
                            std::io::Write::flush(&mut std::io::stdout()).ok();
                        } else if self.options.verbose && !line.is_empty() {
                            println!("{}", line);
                        }
                    }
                }
                println!(); // Nova linha após o progresso
            }

            let status = child.wait()
                .map_err(|e| AppError::ExtractionFailed(e.to_string()))?;

            if !status.success() {
                return Err(AppError::ExtractionFailed("Audio extraction failed".to_string()));
            }
        }

        // Encontra o arquivo baixado
        let actual_path = self.find_downloaded_file_by_ext(audio_format)
            .ok_or_else(|| AppError::ExtractionFailed("Could not find downloaded file".to_string()))?;

        let file_size = std::fs::metadata(&actual_path)
            .map(|m| m.len())
            .unwrap_or(0);

        Ok(DownloadResult {
            file_path: actual_path,
            file_size,
            video_id: video_info.id,
            video_title: video_info.title,
        })
    }

    /// Finds the most recently downloaded file with the specified extension.
    ///
    /// Searches the output directory for files matching the given extension
    /// and returns the one with the most recent modification time.
    ///
    /// # Arguments
    ///
    /// * `ext` - File extension to search for (e.g., "mp4", "mp3")
    ///
    /// # Returns
    ///
    /// The path to the most recently modified file with the given extension,
    /// or `None` if no matching file is found.
    fn find_downloaded_file_by_ext(&self, ext: &str) -> Option<PathBuf> {
        let dir = &self.options.output_dir;

        std::fs::read_dir(dir)
            .ok()?
            .flatten()
            .filter_map(|entry| {
                let path = entry.path();
                let file_ext = path.extension()?.to_str()?;
                if file_ext == ext {
                    let metadata = std::fs::metadata(&path).ok()?;
                    let modified = metadata.modified().ok()?;
                    Some((path, modified))
                } else {
                    None
                }
            })
            .max_by_key(|(_, modified)| *modified)
            .map(|(path, _)| path)
    }

    /// Returns a reference to the current download options.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_yt_downloader::downloader::Downloader;
    ///
    /// let downloader = Downloader::new();
    /// let options = downloader.options();
    /// println!("Retry attempts: {}", options.retry_attempts);
    /// ```
    pub fn options(&self) -> &DownloadOptions {
        &self.options
    }

    /// Updates the download options for this downloader.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_yt_downloader::downloader::{Downloader, DownloadOptions};
    /// use rust_yt_downloader::cli::VideoQuality;
    ///
    /// let mut downloader = Downloader::new();
    /// let new_options = DownloadOptions::default()
    ///     .with_quality(VideoQuality::Q720p);
    /// downloader.set_options(new_options);
    /// ```
    pub fn set_options(&mut self, options: DownloadOptions) {
        self.options = options;
    }

    /// Builds the base argument list for yt-dlp commands.
    ///
    /// Constructs common arguments used across all download operations,
    /// including warnings suppression, filename restrictions, progress
    /// options, and rate limiting (if configured).
    fn base_ytdlp_args(&self) -> Vec<String> {
        let mut args = vec![
            "--no-warnings".to_string(),
            "--no-playlist".to_string(),
            "--restrict-filenames".to_string(),
        ];

        let config = Config::load().unwrap_or_default();
        if let Some(ref limit) = config.network.rate_limit {
            args.push("--rate-limit".to_string());
            args.push(limit.clone());
        }

        if !self.options.silence {
            args.push("--newline".to_string());
            args.push("--progress".to_string());
        } else {
            args.push("--quiet".to_string());
        }

        args
    }
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // ============== DownloadOptions Default Tests ==============

    #[test]
    fn test_download_options_default() {
        let options = DownloadOptions::default();

        assert_eq!(options.output_dir, PathBuf::from("."));
        assert!(matches!(options.quality, VideoQuality::Best));
        assert!(matches!(options.video_format, VideoFormat::Mp4));
        assert!(matches!(options.audio_format, AudioFormat::Mp3));
        assert!(!options.audio_only);
        assert_eq!(options.filename_template, "%(title)s.%(ext)s");
        assert_eq!(options.retry_attempts, 3);
        assert!(!options.silence);
        assert!(!options.verbose);
    }

    // ============== DownloadOptions Builder Tests ==============

    #[test]
    fn test_download_options_with_output_dir() {
        let options = DownloadOptions::default()
            .with_output_dir(PathBuf::from("/custom/path"));

        assert_eq!(options.output_dir, PathBuf::from("/custom/path"));
    }

    #[test]
    fn test_download_options_with_quality() {
        let options = DownloadOptions::default()
            .with_quality(VideoQuality::Q1080p);

        assert!(matches!(options.quality, VideoQuality::Q1080p));
    }

    #[test]
    fn test_download_options_with_video_format() {
        let options = DownloadOptions::default()
            .with_video_format(VideoFormat::Mkv);

        assert!(matches!(options.video_format, VideoFormat::Mkv));
    }

    #[test]
    fn test_download_options_with_audio_format() {
        let options = DownloadOptions::default()
            .with_audio_format(AudioFormat::Flac);

        assert!(matches!(options.audio_format, AudioFormat::Flac));
    }

    #[test]
    fn test_download_options_with_audio_only() {
        let options = DownloadOptions::default()
            .with_audio_only(true);

        assert!(options.audio_only);
    }

    #[test]
    fn test_download_options_with_template() {
        let options = DownloadOptions::default()
            .with_template("%(title)s-%(id)s.%(ext)s".to_string());

        assert_eq!(options.filename_template, "%(title)s-%(id)s.%(ext)s");
    }

    #[test]
    fn test_download_options_with_silence() {
        let options = DownloadOptions::default()
            .with_silence(true);

        assert!(options.silence);
    }

    #[test]
    fn test_download_options_with_verbose() {
        let options = DownloadOptions::default()
            .with_verbose(true);

        assert!(options.verbose);
    }

    #[test]
    fn test_download_options_builder_chain() {
        let options = DownloadOptions::default()
            .with_output_dir(PathBuf::from("/downloads"))
            .with_quality(VideoQuality::Q720p)
            .with_video_format(VideoFormat::Webm)
            .with_audio_format(AudioFormat::Opus)
            .with_audio_only(false)
            .with_template("%(title)s.%(ext)s".to_string())
            .with_silence(true)
            .with_verbose(false);

        assert_eq!(options.output_dir, PathBuf::from("/downloads"));
        assert!(matches!(options.quality, VideoQuality::Q720p));
        assert!(matches!(options.video_format, VideoFormat::Webm));
        assert!(matches!(options.audio_format, AudioFormat::Opus));
        assert!(!options.audio_only);
        assert!(options.silence);
        assert!(!options.verbose);
    }

    // ============== DownloadOptions Parse Tests ==============

    #[test]
    fn test_parse_quality_144p() {
        let quality = DownloadOptions::parse_quality("144p");
        assert!(matches!(quality, VideoQuality::Q144p));
    }

    #[test]
    fn test_parse_quality_240p() {
        let quality = DownloadOptions::parse_quality("240p");
        assert!(matches!(quality, VideoQuality::Q240p));
    }

    #[test]
    fn test_parse_quality_360p() {
        let quality = DownloadOptions::parse_quality("360p");
        assert!(matches!(quality, VideoQuality::Q360p));
    }

    #[test]
    fn test_parse_quality_480p() {
        let quality = DownloadOptions::parse_quality("480p");
        assert!(matches!(quality, VideoQuality::Q480p));
    }

    #[test]
    fn test_parse_quality_720p() {
        let quality = DownloadOptions::parse_quality("720p");
        assert!(matches!(quality, VideoQuality::Q720p));
    }

    #[test]
    fn test_parse_quality_1080p() {
        let quality = DownloadOptions::parse_quality("1080p");
        assert!(matches!(quality, VideoQuality::Q1080p));
    }

    #[test]
    fn test_parse_quality_1440p() {
        let quality = DownloadOptions::parse_quality("1440p");
        assert!(matches!(quality, VideoQuality::Q1440p));
    }

    #[test]
    fn test_parse_quality_4k() {
        let quality = DownloadOptions::parse_quality("4k");
        assert!(matches!(quality, VideoQuality::Q4k));
    }

    #[test]
    fn test_parse_quality_2160p() {
        let quality = DownloadOptions::parse_quality("2160p");
        assert!(matches!(quality, VideoQuality::Q4k));
    }

    #[test]
    fn test_parse_quality_best() {
        let quality = DownloadOptions::parse_quality("best");
        assert!(matches!(quality, VideoQuality::Best));
    }

    #[test]
    fn test_parse_quality_worst() {
        let quality = DownloadOptions::parse_quality("worst");
        assert!(matches!(quality, VideoQuality::Worst));
    }

    #[test]
    fn test_parse_quality_case_insensitive() {
        assert!(matches!(DownloadOptions::parse_quality("1080P"), VideoQuality::Q1080p));
        assert!(matches!(DownloadOptions::parse_quality("BEST"), VideoQuality::Best));
        assert!(matches!(DownloadOptions::parse_quality("4K"), VideoQuality::Q4k));
    }

    #[test]
    fn test_parse_quality_unknown_defaults_to_best() {
        let quality = DownloadOptions::parse_quality("unknown");
        assert!(matches!(quality, VideoQuality::Best));
    }

    #[test]
    fn test_parse_video_format_mp4() {
        let format = DownloadOptions::parse_video_format("mp4");
        assert!(matches!(format, VideoFormat::Mp4));
    }

    #[test]
    fn test_parse_video_format_mkv() {
        let format = DownloadOptions::parse_video_format("mkv");
        assert!(matches!(format, VideoFormat::Mkv));
    }

    #[test]
    fn test_parse_video_format_webm() {
        let format = DownloadOptions::parse_video_format("webm");
        assert!(matches!(format, VideoFormat::Webm));
    }

    #[test]
    fn test_parse_video_format_case_insensitive() {
        assert!(matches!(DownloadOptions::parse_video_format("MP4"), VideoFormat::Mp4));
        assert!(matches!(DownloadOptions::parse_video_format("MKV"), VideoFormat::Mkv));
    }

    #[test]
    fn test_parse_video_format_unknown_defaults_to_mp4() {
        let format = DownloadOptions::parse_video_format("avi");
        assert!(matches!(format, VideoFormat::Mp4));
    }

    #[test]
    fn test_parse_audio_format_mp3() {
        let format = DownloadOptions::parse_audio_format("mp3");
        assert!(matches!(format, AudioFormat::Mp3));
    }

    #[test]
    fn test_parse_audio_format_m4a() {
        let format = DownloadOptions::parse_audio_format("m4a");
        assert!(matches!(format, AudioFormat::M4a));
    }

    #[test]
    fn test_parse_audio_format_flac() {
        let format = DownloadOptions::parse_audio_format("flac");
        assert!(matches!(format, AudioFormat::Flac));
    }

    #[test]
    fn test_parse_audio_format_wav() {
        let format = DownloadOptions::parse_audio_format("wav");
        assert!(matches!(format, AudioFormat::Wav));
    }

    #[test]
    fn test_parse_audio_format_opus() {
        let format = DownloadOptions::parse_audio_format("opus");
        assert!(matches!(format, AudioFormat::Opus));
    }

    #[test]
    fn test_parse_audio_format_case_insensitive() {
        assert!(matches!(DownloadOptions::parse_audio_format("MP3"), AudioFormat::Mp3));
        assert!(matches!(DownloadOptions::parse_audio_format("FLAC"), AudioFormat::Flac));
    }

    #[test]
    fn test_parse_audio_format_unknown_defaults_to_mp3() {
        let format = DownloadOptions::parse_audio_format("wma");
        assert!(matches!(format, AudioFormat::Mp3));
    }

    // ============== DownloadOptions from_config Tests ==============

    #[test]
    fn test_download_options_from_config() {
        let config = Config::default();
        let options = DownloadOptions::from_config(&config);

        assert!(matches!(options.quality, VideoQuality::Best));
        assert!(matches!(options.video_format, VideoFormat::Mp4));
        assert!(matches!(options.audio_format, AudioFormat::Mp3));
        assert_eq!(options.retry_attempts, 3);
    }

    // ============== Quality to yt-dlp Tests ==============

    #[test]
    fn test_quality_to_ytdlp_best() {
        let options = DownloadOptions::default().with_quality(VideoQuality::Best);
        let format = options.quality_to_ytdlp();
        assert!(format.contains("best"));
    }

    #[test]
    fn test_quality_to_ytdlp_worst() {
        let options = DownloadOptions::default().with_quality(VideoQuality::Worst);
        let format = options.quality_to_ytdlp();
        assert!(format.contains("worst"));
    }

    #[test]
    fn test_quality_to_ytdlp_4k() {
        let options = DownloadOptions::default().with_quality(VideoQuality::Q4k);
        let format = options.quality_to_ytdlp();
        assert!(format.contains("2160"));
    }

    #[test]
    fn test_quality_to_ytdlp_1440p() {
        let options = DownloadOptions::default().with_quality(VideoQuality::Q1440p);
        let format = options.quality_to_ytdlp();
        assert!(format.contains("1440"));
    }

    #[test]
    fn test_quality_to_ytdlp_1080p() {
        let options = DownloadOptions::default().with_quality(VideoQuality::Q1080p);
        let format = options.quality_to_ytdlp();
        assert!(format.contains("1080"));
    }

    #[test]
    fn test_quality_to_ytdlp_720p() {
        let options = DownloadOptions::default().with_quality(VideoQuality::Q720p);
        let format = options.quality_to_ytdlp();
        assert!(format.contains("720"));
    }

    #[test]
    fn test_quality_to_ytdlp_480p() {
        let options = DownloadOptions::default().with_quality(VideoQuality::Q480p);
        let format = options.quality_to_ytdlp();
        assert!(format.contains("480"));
    }

    #[test]
    fn test_quality_to_ytdlp_360p() {
        let options = DownloadOptions::default().with_quality(VideoQuality::Q360p);
        let format = options.quality_to_ytdlp();
        assert!(format.contains("360"));
    }

    #[test]
    fn test_quality_to_ytdlp_240p() {
        let options = DownloadOptions::default().with_quality(VideoQuality::Q240p);
        let format = options.quality_to_ytdlp();
        assert!(format.contains("240"));
    }

    #[test]
    fn test_quality_to_ytdlp_144p() {
        let options = DownloadOptions::default().with_quality(VideoQuality::Q144p);
        let format = options.quality_to_ytdlp();
        assert!(format.contains("144"));
    }

    // ============== Video Format Extension Tests ==============

    #[test]
    fn test_video_format_ext_mp4() {
        let options = DownloadOptions::default().with_video_format(VideoFormat::Mp4);
        assert_eq!(options.video_format_ext(), "mp4");
    }

    #[test]
    fn test_video_format_ext_mkv() {
        let options = DownloadOptions::default().with_video_format(VideoFormat::Mkv);
        assert_eq!(options.video_format_ext(), "mkv");
    }

    #[test]
    fn test_video_format_ext_webm() {
        let options = DownloadOptions::default().with_video_format(VideoFormat::Webm);
        assert_eq!(options.video_format_ext(), "webm");
    }

    // ============== Audio Format String Tests ==============

    #[test]
    fn test_audio_format_str_mp3() {
        let options = DownloadOptions::default().with_audio_format(AudioFormat::Mp3);
        assert_eq!(options.audio_format_str(), "mp3");
    }

    #[test]
    fn test_audio_format_str_m4a() {
        let options = DownloadOptions::default().with_audio_format(AudioFormat::M4a);
        assert_eq!(options.audio_format_str(), "m4a");
    }

    #[test]
    fn test_audio_format_str_flac() {
        let options = DownloadOptions::default().with_audio_format(AudioFormat::Flac);
        assert_eq!(options.audio_format_str(), "flac");
    }

    #[test]
    fn test_audio_format_str_wav() {
        let options = DownloadOptions::default().with_audio_format(AudioFormat::Wav);
        assert_eq!(options.audio_format_str(), "wav");
    }

    #[test]
    fn test_audio_format_str_opus() {
        let options = DownloadOptions::default().with_audio_format(AudioFormat::Opus);
        assert_eq!(options.audio_format_str(), "opus");
    }

    // ============== Downloader Creation Tests ==============

    #[test]
    fn test_downloader_new() {
        let downloader = Downloader::new();
        assert!(matches!(downloader.options().quality, VideoQuality::Best));
    }

    #[test]
    fn test_downloader_default() {
        let downloader = Downloader::default();
        assert!(matches!(downloader.options().quality, VideoQuality::Best));
    }

    #[test]
    fn test_downloader_with_options() {
        let options = DownloadOptions::default()
            .with_quality(VideoQuality::Q720p)
            .with_silence(true);

        let downloader = Downloader::with_options(options);

        assert!(matches!(downloader.options().quality, VideoQuality::Q720p));
        assert!(downloader.options().silence);
    }

    #[test]
    fn test_downloader_from_config() {
        let config = Config::default();
        let downloader = Downloader::from_config(&config);

        assert_eq!(downloader.options().retry_attempts, config.network.retry_attempts);
    }

    #[test]
    fn test_downloader_set_options() {
        let mut downloader = Downloader::new();

        let new_options = DownloadOptions::default()
            .with_quality(VideoQuality::Q480p);

        downloader.set_options(new_options);

        assert!(matches!(downloader.options().quality, VideoQuality::Q480p));
    }

    // ============== DownloadResult Tests ==============

    #[test]
    fn test_download_result_creation() {
        let result = DownloadResult {
            file_path: PathBuf::from("/downloads/video.mp4"),
            file_size: 1024 * 1024 * 100,
            video_id: "abc123".to_string(),
            video_title: "Test Video".to_string(),
        };

        assert_eq!(result.file_path, PathBuf::from("/downloads/video.mp4"));
        assert_eq!(result.file_size, 104857600);
        assert_eq!(result.video_id, "abc123");
        assert_eq!(result.video_title, "Test Video");
    }

    // ============== Multiple Downloader Instances ==============

    #[test]
    fn test_multiple_downloader_instances() {
        let downloader1 = Downloader::with_options(
            DownloadOptions::default().with_quality(VideoQuality::Q720p)
        );
        let downloader2 = Downloader::with_options(
            DownloadOptions::default().with_quality(VideoQuality::Q1080p)
        );

        assert!(matches!(downloader1.options().quality, VideoQuality::Q720p));
        assert!(matches!(downloader2.options().quality, VideoQuality::Q1080p));
    }

    // ============== Clone Tests ==============

    #[test]
    fn test_download_options_clone() {
        let options = DownloadOptions::default()
            .with_quality(VideoQuality::Q720p)
            .with_output_dir(PathBuf::from("/test"));

        let cloned = options.clone();

        assert!(matches!(cloned.quality, VideoQuality::Q720p));
        assert_eq!(cloned.output_dir, PathBuf::from("/test"));
    }

    #[test]
    fn test_download_options_clone_independent() {
        let options = DownloadOptions::default();
        let mut cloned = options.clone();

        cloned.silence = true;

        assert!(!options.silence);
        assert!(cloned.silence);
    }

    // ============== Debug Tests ==============

    #[test]
    fn test_download_options_debug() {
        let options = DownloadOptions::default();
        let debug_str = format!("{:?}", options);

        assert!(debug_str.contains("DownloadOptions"));
        assert!(debug_str.contains("output_dir"));
        assert!(debug_str.contains("quality"));
    }

    #[test]
    fn test_download_result_debug() {
        let result = DownloadResult {
            file_path: PathBuf::from("/test.mp4"),
            file_size: 1000,
            video_id: "test".to_string(),
            video_title: "Test".to_string(),
        };

        let debug_str = format!("{:?}", result);

        assert!(debug_str.contains("DownloadResult"));
        assert!(debug_str.contains("file_path"));
    }

    // ============== All Quality Variants ==============

    #[test]
    fn test_all_quality_variants_to_ytdlp() {
        let qualities = vec![
            VideoQuality::Best,
            VideoQuality::Worst,
            VideoQuality::Q4k,
            VideoQuality::Q1440p,
            VideoQuality::Q1080p,
            VideoQuality::Q720p,
            VideoQuality::Q480p,
            VideoQuality::Q360p,
            VideoQuality::Q240p,
            VideoQuality::Q144p,
        ];

        for quality in qualities {
            let options = DownloadOptions::default().with_quality(quality);
            let format_str = options.quality_to_ytdlp();
            assert!(!format_str.is_empty());
        }
    }

    // ============== All Video Format Variants ==============

    #[test]
    fn test_all_video_format_variants() {
        let formats = vec![
            VideoFormat::Mp4,
            VideoFormat::Mkv,
            VideoFormat::Webm,
        ];

        for format in formats {
            let options = DownloadOptions::default().with_video_format(format);
            let ext = options.video_format_ext();
            assert!(!ext.is_empty());
        }
    }

    // ============== All Audio Format Variants ==============

    #[test]
    fn test_all_audio_format_variants() {
        let formats = vec![
            AudioFormat::Mp3,
            AudioFormat::M4a,
            AudioFormat::Flac,
            AudioFormat::Wav,
            AudioFormat::Opus,
        ];

        for format in formats {
            let options = DownloadOptions::default().with_audio_format(format);
            let fmt_str = options.audio_format_str();
            assert!(!fmt_str.is_empty());
        }
    }
}
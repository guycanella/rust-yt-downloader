# Modules Overview

This document provides detailed documentation for each module in the YouTube Downloader codebase, including their responsibilities, public APIs, key types, and inter-module dependencies.

## Core Infrastructure Modules

### cli.rs

**Purpose**: Command-line interface definition and argument parsing.

**Technology**: Uses `clap` v4.5 with derive macros for declarative CLI definitions.

#### Public API

```rust
// Main CLI entry point
pub struct Cli {
    pub command: Commands,
}

// Top-level commands
pub enum Commands {
    Download(DownloadArgs),
    Audio(AudioArgs),
    Playlist(PlaylistArgs),
    Info(InfoArgs),
    Config { command: ConfigCommands },
}

// Configuration subcommands
pub enum ConfigCommands {
    Show,
    Set { key: String, value: String },
    Get { key: String },
    Reset,
    Path,
}

// Video quality options
pub enum VideoQuality {
    Q144p, Q240p, Q360p, Q480p,
    Q720p, Q1080p, Q1440p, Q4k,
    Best,    // Default
    Worst,
}

// Audio format options
pub enum AudioFormat {
    Mp3,     // Default
    M4a,
    Flac,
    Wav,
    Opus,
}

// Video format options
pub enum VideoFormat {
    Mp4,     // Default
    Mkv,
    Webm,
}
```

#### Command Structures

**DownloadArgs**:
```rust
pub struct DownloadArgs {
    pub common: CommonArgs,      // URL, quality, output, etc.
    pub format: Option<VideoFormat>,
    pub no_thumbnail: bool,
    pub no_subtitles: bool,
}
```

**AudioArgs**:
```rust
pub struct AudioArgs {
    pub common: CommonArgs,
    pub format: Option<AudioFormat>,
    pub bitrate: Option<String>,   // e.g., "320k"
}
```

**PlaylistArgs**:
```rust
pub struct PlaylistArgs {
    pub urls: Vec<String>,
    pub common: CommonArgs,
    pub audio_only: bool,
    pub start: Option<usize>,      // Start index
    pub end: Option<usize>,        // End index
}
```

**CommonArgs** (shared across commands):
```rust
pub struct CommonArgs {
    pub url: String,
    pub quality: Option<VideoQuality>,
    pub output: Option<PathBuf>,
    pub verbose: bool,
}
```

#### Key Features

- **Type Safety**: All arguments are strongly typed
- **Validation**: Clap handles type validation automatically
- **Defaults**: Sensible defaults for all optional arguments
- **Help Generation**: Automatic `--help` text generation
- **Comprehensive Tests**: 1000+ lines of unit tests covering all commands and edge cases

#### Dependencies

- External: `clap` (derive macros)
- Internal: None (leaf module)

---

### config.rs

**Purpose**: Configuration file management with TOML serialization.

**Location**: `~/.config/rust-yt-downloader/config.toml` (Linux/macOS) or equivalent on Windows.

#### Public API

```rust
// Main configuration structure
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub audio: AudioConfig,
    pub video: VideoConfig,
    pub network: NetworkConfig,
}

impl Config {
    // Load config from file (or return defaults if not found)
    pub fn load() -> AppResult<Self>;

    // Save current config to file
    pub fn save(&self) -> AppResult<()>;

    // Get a config value by dot-notation key
    pub fn get(&self, key: &str) -> Option<String>;

    // Set a config value by dot-notation key
    pub fn set(&mut self, key: &str, value: &str) -> AppResult<()>;

    // Get the config file path
    pub fn config_path() -> AppResult<PathBuf>;

    // Reset to default values
    pub fn reset() -> Self;
}
```

#### Configuration Sections

**GeneralConfig**:
```rust
pub struct GeneralConfig {
    pub output_dir: String,              // Default: ~/Downloads/YouTube
    pub default_quality: String,         // Default: "best"
    pub max_parallel_downloads: u32,     // Default: 3
}
```

**AudioConfig**:
```rust
pub struct AudioConfig {
    pub format: String,                  // Default: "mp3"
    pub bitrate: String,                 // Default: "320k"
}
```

**VideoConfig**:
```rust
pub struct VideoConfig {
    pub format: String,                  // Default: "mp4"
    pub include_thumbnail: bool,         // Default: true
    pub include_subtitles: bool,         // Default: true
}
```

**NetworkConfig**:
```rust
pub struct NetworkConfig {
    pub rate_limit: Option<String>,      // Default: None (unlimited)
    pub retry_attempts: u32,             // Default: 3
    pub timeout: u64,                    // Default: 300 (seconds)
}
```

#### Key Features

- **Hierarchical Structure**: Organized into logical sections
- **Dot Notation Access**: `config.get("general.output_dir")`
- **Platform-Aware Defaults**: Uses `dirs` crate for platform-specific paths
- **Partial Files**: Missing sections use defaults
- **Type Validation**: Validates values when setting

#### Dependencies

- External: `serde`, `toml`, `dirs`
- Internal: `error` (for `AppError`, `AppResult`)

---

### error.rs

**Purpose**: Centralized error handling with categorized error types.

**Technology**: Uses `thiserror` for declarative error definitions.

#### Public API

```rust
// Result type alias used throughout the application
pub type AppResult<T> = std::result::Result<T, AppError>;

// Main error enum with categorized variants
#[derive(Error, Debug)]
pub enum AppError {
    // Network/HTTP Errors
    HttpRequest { message: String, status: u16 },
    Connection(String),
    Timeout { seconds: u64 },
    Network(#[from] reqwest::Error),

    // YouTube Errors
    InvalidUrl(String),
    VideoNotFound { video_id: String },
    VideoPrivate { video_id: String },
    AgeRestricted { video_id: String },
    RegionBlocked { video_id: String },
    PlaylistNotFound { playlist_id: String },
    ExtractionFailed(String),
    YouTube(#[from] rustube::Error),

    // Filesystem Errors
    FileRead { path: PathBuf, #[source] source: std::io::Error },
    FileWrite { path: PathBuf, #[source] source: std::io::Error },
    DirectoryCreate { path: PathBuf, #[source] source: std::io::Error },
    PathNotFound(PathBuf),
    PermissionDenied(PathBuf),
    Io(#[from] std::io::Error),

    // FFmpeg Errors
    FfmpegNotFound,
    FfmpegExecution { message: String, exit_code: Option<i32> },
    ConversionFailed { from_format: String, to_format: String, #[source] source: Box<AppError> },
    TrimmingFailed { start: String, end: String, #[source] source: Box<AppError> },

    // Configuration Errors
    ConfigParse { path: PathBuf, #[source] source: toml::de::Error },
    ConfigSerialize(#[from] toml::ser::Error),
    ConfigInvalid { field: String, message: String },
    ConfigNotFound(PathBuf),

    // Download Errors
    NoStreamsAvailable { video_id: String },
    QualityNotAvailable { requested: String, available: Vec<String> },
    FormatNotSupported(String),
    DownloadInterrupted(String),
    MaxRetriesExceeded { attempts: u32, message: String },

    // Validation Errors
    InvalidArgument { argument: String, message: String },
    InvalidTimeFormat(String),
    InvalidTemplate { template: String, message: String },

    // Generic Errors
    Cancelled,
    Other(String),
}

// Helper constructors
impl AppError {
    pub fn http(status: u16, message: impl Into<String>) -> Self;
    pub fn file_read(path: impl Into<PathBuf>, source: std::io::Error) -> Self;
    pub fn file_write(path: impl Into<PathBuf>, source: std::io::Error) -> Self;
    pub fn dir_create(path: impl Into<PathBuf>, source: std::io::Error) -> Self;
    pub fn ffmpeg(message: impl Into<String>, exit_code: Option<i32>) -> Self;
    pub fn invalid_arg(argument: impl Into<String>, message: impl Into<String>) -> Self;

    // Check if error can be retried
    pub fn is_retryable(&self) -> bool;
}
```

#### Error Categories

| Category | Description | Retryable |
|----------|-------------|-----------|
| Network/HTTP | Connection failures, timeouts, HTTP errors | Some (timeout, connection) |
| YouTube | Video/playlist not found, region blocks, API errors | No |
| Filesystem | File I/O errors, permissions, paths | No |
| FFmpeg | FFmpeg not found, conversion failures | No |
| Configuration | Config parsing, invalid values | No |
| Download | Stream issues, quality problems | Some (interrupted) |
| Validation | Invalid arguments, formats | No |

#### Key Features

- **Contextual Information**: Errors include file paths, URLs, status codes
- **Error Chaining**: Uses `#[source]` for error chains
- **Retryable Detection**: `is_retryable()` method for retry logic
- **User-Friendly Messages**: Display implementation provides clear messages
- **Helper Constructors**: Convenient constructors for common patterns

#### Dependencies

- External: `thiserror`
- Internal: None (leaf module)

---

## Business Logic Modules

### downloader.rs

**Purpose**: Orchestrate download operations and coordinate between components.

#### Public API

```rust
pub struct DownloadOptions {
    pub url: String,
    pub quality: String,
    pub output_dir: PathBuf,
    pub format: String,
    pub include_thumbnail: bool,
    pub include_subtitles: bool,
    pub retry_attempts: u32,
    pub timeout: u64,
}

impl DownloadOptions {
    // Create from configuration
    pub fn from_config(config: &Config) -> Self;

    // Merge CLI arguments (overrides config)
    pub fn merge_from_cli_args(self, args: &DownloadArgs) -> Self;
}

// Main download function
pub async fn download_video(options: DownloadOptions) -> AppResult<PathBuf>;
```

#### Responsibilities

1. **Configuration Merging**: Combine CLI args + config file + defaults
2. **Download Coordination**: Manage the download pipeline
3. **Retry Logic**: Implement automatic retries for transient failures
4. **Progress Tracking**: Update progress bars during downloads
5. **Filename Generation**: Apply templates and sanitization
6. **Error Recovery**: Handle and report download failures

#### Dependencies

- External: `tokio`, `reqwest`
- Internal: `config`, `error`, `youtube`, `media`, `progress`, `utils`

---

### youtube/ (Sub-module)

**Purpose**: YouTube API integration and metadata extraction.

#### Module Structure

```rust
// youtube/mod.rs - Public exports
pub use client::{YouTubeClient, validate_youtube_url};
pub use metadata::{VideoInfo, StreamInfo, QualityFilter};
pub use playlist::{PlaylistInfo, extract_playlist_id};
```

#### youtube/client.rs

```rust
pub struct YouTubeClient {
    // Internal rustube client
}

impl YouTubeClient {
    pub fn new() -> Self;

    pub async fn fetch_video_info(url: &str) -> AppResult<VideoInfo>;

    pub async fn get_stream_url(video_id: &str, quality: &str) -> AppResult<String>;
}

// Validate YouTube URL format
pub fn validate_youtube_url(url: &str) -> AppResult<()>;
```

#### youtube/metadata.rs

```rust
pub struct VideoInfo {
    pub id: String,
    pub title: String,
    pub duration: u64,
    pub author: String,
    pub description: String,
    pub thumbnail_url: String,
    pub available_qualities: Vec<String>,
}

pub struct StreamInfo {
    pub url: String,
    pub quality: String,
    pub format: String,
    pub file_size: Option<u64>,
}

pub struct QualityFilter {
    // Quality filtering logic
}

impl QualityFilter {
    pub fn filter_streams(streams: Vec<StreamInfo>, quality: &str) -> AppResult<StreamInfo>;
}
```

#### youtube/playlist.rs

```rust
pub struct PlaylistInfo {
    pub id: String,
    pub title: String,
    pub video_count: usize,
    pub video_ids: Vec<String>,
}

pub async fn fetch_playlist_info(url: &str) -> AppResult<PlaylistInfo>;

pub fn extract_playlist_id(url: &str) -> AppResult<String>;
```

#### Dependencies

- External: `rustube`, `reqwest`
- Internal: `error`

---

### media/ (Sub-module)

**Purpose**: Media processing, conversion, and FFmpeg integration.

#### Module Structure

```rust
// media/mod.rs - Public exports
pub use ffmpeg::FFmpeg;
pub use audio::{AudioExtractor, AudioOptions};
pub use converter::{VideoConverter, ConversionOptions};
```

#### media/ffmpeg.rs

```rust
pub struct FFmpeg {
    // FFmpeg binary path
}

impl FFmpeg {
    // Check if FFmpeg is available
    pub fn check_installed() -> AppResult<()>;

    // Execute FFmpeg command
    pub async fn execute(args: &[&str]) -> AppResult<()>;

    // Get FFmpeg version
    pub fn version() -> AppResult<String>;
}
```

#### media/audio.rs

```rust
pub struct AudioOptions {
    pub format: String,          // mp3, flac, m4a, wav, opus
    pub bitrate: String,         // e.g., "320k"
    pub input_path: PathBuf,
    pub output_path: PathBuf,
}

pub struct AudioExtractor;

impl AudioExtractor {
    pub async fn extract_audio(options: AudioOptions) -> AppResult<PathBuf>;
}
```

#### media/converter.rs

```rust
pub struct ConversionOptions {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub from_format: String,
    pub to_format: String,
}

pub struct VideoConverter;

impl VideoConverter {
    pub async fn convert_video(options: ConversionOptions) -> AppResult<PathBuf>;
}
```

#### Dependencies

- External: `tokio` (for async process execution)
- Internal: `error`

---

## Utility Modules

### progress.rs

**Purpose**: User feedback via progress bars and spinners.

#### Public API

```rust
pub struct ProgressStyles {
    pub download: ProgressStyle,
    pub default: ProgressStyle,
    pub spinner: ProgressStyle,
}

pub struct DownloadProgress {
    pub total_bytes: u64,
    pub downloaded_bytes: u64,
    pub progress_bar: ProgressBar,
}

impl DownloadProgress {
    pub fn new(total_bytes: u64) -> Self;

    pub fn update(&mut self, bytes: u64);

    pub fn finish(&self);
}

// User-facing messages
pub mod messages {
    pub fn downloading(title: &str) -> String;
    pub fn extracting_audio(format: &str) -> String;
    pub fn converting(from: &str, to: &str) -> String;
}
```

#### Dependencies

- External: `indicatif`, `colored`
- Internal: None

---

### utils.rs

**Purpose**: Common utility functions.

#### Public API

```rust
// Remove invalid filesystem characters
pub fn sanitize_filename(filename: &str) -> String;

// Expand ~ to home directory
pub fn expand_path(path: &str) -> PathBuf;

// Format bytes as human-readable (e.g., "1.5 MB")
pub fn format_bytes(bytes: u64) -> String;

// Video metadata for templating
pub struct VideoMetadata {
    pub title: String,
    pub id: String,
    pub author: String,
    pub date: String,
    pub quality: String,
}

// Apply filename template
// Template variables: {title}, {id}, {author}, {date}, {quality}
pub fn apply_template(template: &str, metadata: &VideoMetadata) -> String;
```

#### Template Examples

```rust
// Template: "{title}-{quality}"
// Result: "My_Video-1080p.mp4"

// Template: "{author}/{date}-{title}"
// Result: "Channel_Name/2024-01-15-My_Video.mp4"
```

#### Dependencies

- External: `dirs`
- Internal: `error`

---

### main.rs

**Purpose**: Application entry point.

#### Structure

```rust
#[tokio::main]
async fn main() -> AppResult<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Match on command and dispatch
    match cli.command {
        Commands::Download(args) => {
            // Handle download command
        }
        Commands::Audio(args) => {
            // Handle audio command
        }
        Commands::Playlist(args) => {
            // Handle playlist command
        }
        Commands::Info(args) => {
            // Handle info command
        }
        Commands::Config { command } => {
            // Handle config commands
        }
    }

    Ok(())
}
```

#### Dependencies

- External: `tokio`, `clap`
- Internal: All modules

---

## Module Dependency Graph

```
main.rs
  ├─ cli.rs
  ├─ config.rs ──► error.rs
  │              └─ utils.rs (expand_path)
  ├─ downloader.rs
  │     ├─ config.rs
  │     ├─ error.rs
  │     ├─ youtube/ ──► error.rs
  │     ├─ media/ ──► error.rs
  │     ├─ progress.rs
  │     └─ utils.rs
  ├─ progress.rs
  └─ error.rs

Legend:
  ──► = depends on
  └─  = optional dependency
```

## Key Takeaways

1. **Clear Layering**: Infrastructure → Business Logic → Utilities
2. **Minimal Dependencies**: Most modules are independent or depend only on `error`
3. **Sub-module Pattern**: `youtube/` and `media/` use sub-modules with clean public APIs
4. **Type Safety**: Strong typing throughout with custom types
5. **Error Handling**: All fallible operations return `AppResult<T>`
6. **Async-First**: I/O operations are async using Tokio

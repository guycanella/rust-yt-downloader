# YouTube Downloader - Configuration & Dependencies

## Project Overview

A professional, robust CLI tool for downloading YouTube videos with advanced features built in Rust.

---

## Core Features

### Essential Features
- [x] **Quality Selection**: Choose video quality (1080p, 720p, 480p, etc.)
- [x] **Audio-only Download**: Extract and download only audio
- [x] **Video Trimming**: Select start/end timestamps to download specific segments
- [x] **Feature Combination**: Combine quality selection, audio extraction, and trimming
- [x] **Playlist Download**: Download multiple videos from playlists
- [x] **Custom Audio Formats**: Support MP3, M4A, FLAC, etc.
- [x] **Subtitles Download**: Download auto-generated or official subtitles
- [x] **Progress Bar**: Display download progress with percentage and speed
- [x] **Auto Retry**: Automatic retry mechanism on failures

### Advanced Features
- [x] **Format Conversion**: Convert between formats (MP4 → MKV, WebM, etc.)
- [x] **File Naming Templates**: Customizable naming patterns (`{title}-{date}-{quality}.mp4`)
- [x] **Download Rate Limiting**: Control bandwidth usage
- [x] **Parallel Downloads**: Download multiple videos simultaneously
- [x] **Configuration File**: Save default preferences (quality, output folder, etc.)

### Premium Features
- [x] **Thumbnail Download**: Save video thumbnails alongside videos
- [x] **Metadata Embedding**: Include title, description, date in file metadata
- [x] **Download Summary**: Report statistics (files downloaded, failed, total size)
- [x] **Playlist Sync Mode**: Check playlist and download only new videos
- [x] **Private Playlist Support**: Authentication for watch later/favorites

---

## Technology Stack

### Core Dependencies

#### CLI & User Interface
```toml
clap = { version = "4.5", features = ["derive", "cargo"] }
# Command-line argument parsing with derive macros

indicatif = "0.17"
# Beautiful progress bars and spinners

colored = "2.1"
# Terminal color output
```

#### Async Runtime & Networking
```toml
tokio = { version = "1.40", features = ["full"] }
# Async runtime for parallel operations

reqwest = { version = "0.12", features = ["stream", "json"] }
# HTTP client for downloads

futures = "0.3"
# Future combinators and utilities
```

#### YouTube Integration
```toml
rustube = "0.7"
# YouTube API client and video fetching

youtube_dl = "0.9"
# Alternative: yt-dlp bindings (if needed)
```

#### Media Processing
```toml
ffmpeg-next = "7.0"
# FFmpeg bindings for video/audio processing

mp4amend = "0.1"
# MP4 metadata manipulation

id3 = "1.13"
# MP3 metadata (ID3 tags)
```

#### Serialization & Configuration
```toml
serde = { version = "1.0", features = ["derive"] }
# Serialization framework

serde_json = "1.0"
# JSON support

toml = "0.8"
# TOML configuration file support
```

#### Error Handling
```toml
anyhow = "1.0"
# Flexible error handling

thiserror = "1.0"
# Custom error types with derive macros
```

#### Utilities
```toml
chrono = "0.4"
# Date and time handling

regex = "1.10"
# Regular expressions

dirs = "5.0"
# Platform-specific directory paths
```

### Development Dependencies

```toml
[dev-dependencies]
tokio-test = "0.4"
# Testing utilities for async code

tempfile = "3.12"
# Temporary files for testing

mockito = "1.5"
# HTTP mocking for tests

criterion = "0.5"
# Benchmarking framework
```

---

## System Requirements

### External Dependencies

#### FFmpeg
FFmpeg must be installed on the system for video/audio processing features:

**Linux:**
```bash
sudo apt-get install ffmpeg libavcodec-dev libavformat-dev libavutil-dev libswscale-dev libavdevice-dev
```

**macOS:**
```bash
brew install ffmpeg
```

**Windows:**
- Download from https://ffmpeg.org/download.html
- Or use: `choco install ffmpeg`

### Minimum Rust Version
- **MSRV**: 1.75.0 or higher
- **Edition**: 2021

---

## Project Structure (Proposed)

```
youtube-downloader/
├── src/
│   ├── main.rs              # Entry point
│   ├── cli.rs               # CLI argument definitions
│   ├── config.rs            # Configuration management
│   ├── downloader.rs        # Download orchestration
│   ├── youtube/
│   │   ├── mod.rs
│   │   ├── client.rs        # YouTube API client
│   │   ├── playlist.rs      # Playlist handling
│   │   └── metadata.rs      # Video metadata
│   ├── media/
│   │   ├── mod.rs
│   │   ├── converter.rs     # Format conversion
│   │   ├── trimmer.rs       # Video trimming
│   │   └── metadata.rs      # Metadata embedding
│   ├── progress.rs          # Progress tracking
│   ├── error.rs             # Custom error types
│   └── utils.rs             # Utility functions
├── tests/
│   ├── integration/
│   └── unit/
├── benches/                 # Benchmarks
├── Cargo.toml
├── Cargo.lock
├── README.md
└── CONFIGURATION_DEPENDENCY.md
```

---

## Configuration File Format

Default location: `~/.config/youtube-downloader/config.toml`

```toml
[general]
output_dir = "~/Downloads/YouTube"
default_quality = "1080p"
max_parallel_downloads = 3

[audio]
format = "mp3"
bitrate = "320k"

[video]
format = "mp4"
include_thumbnail = true
include_subtitles = true

[network]
rate_limit = "5M"  # 5 MB/s
retry_attempts = 3
timeout = 300  # seconds

[naming]
template = "{title}-{date}-{quality}"
sanitize_filenames = true
```

---

## Build Instructions

### Development Build
```bash
cargo build
```

### Release Build (Optimized)
```bash
cargo build --release
```

### Run Tests
```bash
cargo test
```

### Run Benchmarks
```bash
cargo bench
```

### Generate Documentation
```bash
cargo doc --open
```

---

## Cross-Compilation Targets

### Linux (x86_64)
```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

### macOS (Intel)
```bash
cargo build --release --target x86_64-apple-darwin
```

### macOS (Apple Silicon)
```bash
cargo build --release --target aarch64-apple-darwin
```

### Windows
```bash
cargo build --release --target x86_64-pc-windows-gnu
```

---

## Notes

- All features should be thoroughly tested with unit and integration tests
- Error handling must be comprehensive and user-friendly
- Progress reporting should work correctly in CI/CD environments (non-TTY)
- Configuration file is optional; CLI arguments override config values
- FFmpeg integration should gracefully handle missing FFmpeg installation

---

## Future Considerations

- Web UI (optional separate project)
- Docker container support
- Plugin system for custom processors
- Integration with cloud storage (S3, GDrive)
- Batch processing from CSV/JSON file
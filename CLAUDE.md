# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A professional CLI tool for downloading YouTube videos and audio, built in Rust. The binary is named `ytdl` and supports video downloads, audio extraction, playlists, and configuration management.

## Development Commands

### Build & Run
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run the binary directly
cargo run -- <command> <args>

# Examples:
cargo run -- download https://youtube.com/watch?v=abc123
cargo run -- audio https://youtube.com/watch?v=abc123 -f mp3
cargo run -- playlist https://youtube.com/playlist?list=PL123
cargo run -- info https://youtube.com/watch?v=abc123
cargo run -- config show
```

### Testing
```bash
# Run all tests (unit + integration)
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test '*'

# Run specific integration test file
cargo test --test cli_tests
cargo test --test download_tests
cargo test --test audio_tests
cargo test --test playlist_tests
cargo test --test config_tests
cargo test --test info_tests

# Run tests for a specific module
cargo test --lib cli
cargo test --lib config
cargo test --lib error

# Run a specific test by name
cargo test test_download_minimal_args
cargo test test_config_default

# Run tests with output (shows println! statements)
cargo test -- --nocapture

# Run tests matching a pattern
cargo test download              # All tests with "download" in name
cargo test audio                 # All tests with "audio" in name
```

### Binary Installation
```bash
# Install locally (makes `ytdl` available in PATH)
cargo install --path .

# After installation, use directly:
ytdl download https://youtube.com/watch?v=abc123
```

## Architecture

### Module Structure

The codebase follows a modular architecture with seven core modules organized into a layered design:

#### Core Infrastructure Layer

1. **`cli.rs`** - Command-line interface and argument parsing
   - Uses `clap` with derive macros for declarative CLI definition
   - Defines 5 main commands: `download`, `audio`, `playlist`, `info`, `config`
   - Implements enums for quality levels (`VideoQuality`), formats (`AudioFormat`, `VideoFormat`)
   - Contains comprehensive unit tests (1000+ lines of test coverage)

2. **`config.rs`** - Configuration file management
   - Hierarchical config structure: `[general]`, `[audio]`, `[video]`, `[network]`
   - Uses TOML format, stored at `~/.config/rust-yt-downloader/config.toml`
   - Implements `get()` and `set()` methods with dot-notation keys (e.g., `"general.default_quality"`)
   - Default values use platform-specific paths via `dirs` crate
   - Extensively tested with unit tests for serialization, parsing, and validation

3. **`error.rs`** - Centralized error handling
   - Uses `thiserror` for custom error types
   - Categorized errors: Network, YouTube, Filesystem, FFmpeg, Configuration, Download, Validation
   - Helper constructors for common error patterns (e.g., `AppError::file_read()`, `AppError::http()`)
   - Implements `is_retryable()` method to identify transient failures
   - Comprehensive error messages with context (file paths, status codes, etc.)

#### Business Logic Layer

4. **`downloader.rs`** - Download orchestration and coordination
   - `DownloadOptions` struct with configuration merging (CLI args + config file)
   - Coordinates between YouTube client, media processing, and progress tracking
   - Handles filename templating via `apply_template()` from utils
   - Implements retry logic and error recovery

5. **`youtube/`** - YouTube API integration (sub-modules)
   - `ytdlp.rs` - Primary client using `yt-dlp` command-line tool (must be installed separately)
   - `client.rs` - Alternative YouTube client using `rustube` library, URL validation
   - `metadata.rs` - Video metadata extraction (`VideoInfo`, `StreamInfo`, quality filtering)
   - `playlist.rs` - Playlist handling and video ID extraction
   - Public exports: `YtDlpClient`, `YouTubeClient`, `VideoInfo`, `PlaylistInfo`, `QualityFilter`

6. **`media/`** - Media processing and conversion (sub-modules)
   - `ffmpeg.rs` - FFmpeg wrapper and command execution
   - `audio.rs` - Audio extraction with format/bitrate options (`AudioExtractor`, `AudioOptions`)
   - `converter.rs` - Video format conversion (`VideoConverter`, `ConversionOptions`)
   - Public exports: `FFmpeg`, `AudioExtractor`, `VideoConverter`

#### Utility Layer

7. **`progress.rs`** - User feedback and progress tracking
   - Uses `indicatif` for progress bars and spinners
   - `ProgressStyles` with predefined styles (download, default, spinner)
   - `DownloadProgress` struct for tracking download state
   - `messages` module for user-facing text

8. **`utils.rs`** - Common utilities
   - `sanitize_filename()` - Remove invalid filesystem characters
   - `expand_path()` - Handle `~` expansion for home directory
   - `format_bytes()` - Human-readable byte formatting
   - `apply_template()` - Filename template rendering with video metadata
   - `VideoMetadata` struct for template variables

9. **`main.rs`** - Application entry point
   - Parses CLI arguments with `clap`
   - Routes commands to appropriate handlers (download, audio, playlist, info, config)
   - Implements `handle_download()`, `handle_audio()`, `handle_playlist()`, `handle_info()`, `handle_config()`
   - Provides error handling and formatted output for all commands

### Data Flow Architecture

**Download Flow**:
1. `main.rs` → Parse CLI args with `clap`
2. `config.rs` → Load and merge configuration
3. `downloader.rs` → Create `DownloadOptions` from CLI + config
4. `youtube/ytdlp.rs` → Fetch video metadata and download via yt-dlp (primary method)
5. `youtube/metadata.rs` → Apply quality filtering and extract stream information
6. `progress.rs` → Display progress bars via `indicatif`
7. `media/audio.rs` or `media/converter.rs` → Post-processing with FFmpeg (if needed)
8. `utils.rs` → Generate final filename via template

**Configuration Priority** (highest to lowest):
1. CLI arguments (e.g., `-q 1080p`)
2. Config file (`~/.config/rust-yt-downloader/config.toml`)
3. Built-in defaults (defined in `Config::default()`)

**Error Propagation**:
- All fallible operations return `AppResult<T>` (alias for `Result<T, AppError>`)
- Errors bubble up through `?` operator to main handler
- Retryable errors (network timeouts) trigger retry logic in `downloader.rs`
- User-facing errors display via `AppError`'s `Display` implementation

### Type Patterns

**Command Pattern**: CLI uses nested enums for command dispatch
```rust
pub enum Commands {
    Download(DownloadArgs),
    Audio(AudioArgs),
    Playlist(PlaylistArgs),
    Info(InfoArgs),
    Config { command: ConfigCommands },
}
```

**Result Type Alias**: Consistent error handling throughout
```rust
pub type AppResult<T> = std::result::Result<T, AppError>;
```

**Builder Pattern**: Config struct uses serde defaults with custom default functions
```rust
#[serde(default = "GeneralConfig::default_output_dir")]
pub output_dir: String,
```

**Module Re-exports**: Sub-modules use `mod.rs` to expose clean public API
```rust
// youtube/mod.rs
pub use metadata::{VideoInfo, StreamInfo, QualityFilter};
pub use client::{YouTubeClient, validate_youtube_url};
```

### Configuration System

**Config Keys** (accessed via dot notation):
- `general.output_dir` - Download directory
- `general.default_quality` - Default video quality ("best", "1080p", etc.)
- `general.max_parallel_downloads` - Parallel download limit (default: 3)
- `audio.format` - Default audio format (mp3, flac, m4a, wav, opus)
- `audio.bitrate` - Audio bitrate (default: 320k)
- `video.format` - Default video format (mp4, mkv, webm)
- `video.include_thumbnail` - Download thumbnails (default: true)
- `video.include_subtitles` - Download subtitles (default: true)
- `network.rate_limit` - Bandwidth limit (optional)
- `network.retry_attempts` - Retry count (default: 3)
- `network.timeout` - Timeout in seconds (default: 300)

**Config Priority**: CLI arguments override config file values, which override built-in defaults.

## Key Dependencies

- **`clap`** (4.5) - CLI argument parsing with derive macros
- **`rustube`** (0.6) - YouTube API client (alternative to yt-dlp for specific use cases)
- **`tokio`** (1.40) - Async runtime (features = ["full"])
- **`reqwest`** (0.12) - HTTP client with streaming (features = ["stream", "json"])
- **`serde`** + **`serde_json`** + **`toml`** - Configuration and JSON serialization
- **`thiserror`** - Custom error types
- **`anyhow`** - Flexible error handling
- **`indicatif`** - Progress bars
- **`colored`** - Terminal colors
- **`dirs`** - Platform-specific directories
- **`chrono`** - Date and time handling
- **`regex`** - Regular expressions for URL parsing

**Dev Dependencies**:
- **`tokio-test`** - Testing utilities for async code
- **`tempfile`** - Temporary files for testing
- **`mockito`** - HTTP mocking

## Testing Strategy

**Unit Tests**: Each module has inline `#[cfg(test)] mod tests` with comprehensive coverage
- `cli.rs`: 1000+ lines of tests covering all commands, flags, edge cases, enum variants
- `config.rs`: ~500 lines of tests for serialization, parsing, get/set operations
- `error.rs`: ~300 lines of tests for error display, constructors, retryability

**Test Organization**:
```rust
// Tests are organized by functionality with comment headers
// ============== Download Command Tests ==============
// ============== Audio Command Tests ==============
// ============== Playlist Command Tests ==============
// ============== Config Command Tests ==============
// ============== VideoQuality Enum Tests ==============
```

**Running Specific Tests**: Use cargo's filtering
```bash
# Run all tests
cargo test

# Filter by test name substring
cargo test download              # All tests with "download" in name
cargo test test_audio_format     # Specific test

# Run tests for specific module
cargo test --lib cli             # Run cli.rs tests only
cargo test --lib config          # Run config.rs tests only

# Show test output (println! statements)
cargo test -- --nocapture

# Run a single test by exact name
cargo test test_download_minimal_args
```

**Async Testing**: Uses `tokio-test` for async code
```rust
#[tokio::test]
async fn test_youtube_download() {
    // Test async download flow
}
```

**Integration Tests**: Comprehensive end-to-end testing in `tests/` directory
- **Total**: 66 integration tests across 6 test files
- **`cli_tests.rs`** (18 tests) - CLI argument parsing, help flags, version, error handling
- **`download_tests.rs`** (9 tests) - Video download functionality, quality selection, format conversion
- **`audio_tests.rs`** (10 tests) - Audio extraction, format selection, bitrate options
- **`playlist_tests.rs`** (8 tests) - Playlist downloads, batch processing, error recovery
- **`config_tests.rs`** (13 tests) - Configuration management, get/set operations, validation
- **`info_tests.rs`** (8 tests) - Video information display, metadata extraction

**Integration Test Organization**:
```bash
tests/
├── common/           # Shared test utilities and helpers
│   └── mod.rs       # Helper functions: run_ytdl(), run_ytdl_stdout(), etc.
├── cli_tests.rs     # CLI interface and argument validation
├── download_tests.rs # Video download integration tests
├── audio_tests.rs   # Audio extraction integration tests
├── playlist_tests.rs # Playlist processing integration tests
├── config_tests.rs  # Configuration file integration tests
└── info_tests.rs    # Video info command integration tests
```

**Running Integration Tests**:
```bash
# Run all integration tests
cargo test --test '*'

# Run specific integration test file
cargo test --test cli_tests
cargo test --test download_tests

# Run specific integration test
cargo test --test cli_tests test_help_flag

# Run all tests (unit + integration)
cargo test
```

## External Requirements

### yt-dlp (Primary - Required)
The application primarily uses `yt-dlp` for downloading YouTube content:
- The `youtube/ytdlp.rs` module wraps yt-dlp command-line execution
- Handles video downloads, metadata extraction, and format selection
- Must be installed and available in system PATH

**Installation**:
- Linux: `sudo apt install yt-dlp` or download from https://github.com/yt-dlp/yt-dlp
- macOS: `brew install yt-dlp`
- Windows: `choco install yt-dlp` or download from GitHub releases

**Graceful Degradation**: The application checks for yt-dlp availability and returns appropriate errors if missing

### FFmpeg (Required for post-processing)
Required for audio extraction and video format conversion:
- The `media/ffmpeg.rs` module wraps FFmpeg command-line execution
- Audio extraction uses FFmpeg to convert to MP3, FLAC, M4A, WAV, or Opus
- Video conversion supports MP4, MKV, and WebM container formats
- FFmpeg must be installed and available in system PATH

**Installation**:
- Linux: `sudo apt-get install ffmpeg`
- macOS: `brew install ffmpeg`
- Windows: Download from ffmpeg.org or `choco install ffmpeg`

**Graceful Degradation**: The application checks for FFmpeg availability and returns `AppError::FfmpegNotFound` if missing

## Implementation Patterns

### Async Runtime
- All I/O operations (network, filesystem) are async using `tokio`
- HTTP requests use `reqwest` with streaming support for large downloads
- YouTube operations via `yt-dlp` are command-line based (synchronous subprocess execution)
- Alternative `rustube` client provides async YouTube API calls
- Progress bars work concurrently with downloads using `tokio::spawn`

### Error Handling Philosophy
1. **Use specific error variants**: Don't use generic errors when a specific variant exists
2. **Include context**: File paths, URLs, status codes in error messages
3. **Mark retryable errors**: Network timeouts can be retried, filesystem errors cannot
4. **Propagate with `?`**: Let errors bubble up, handle at orchestration layer

### Configuration Merging
```rust
// Priority: CLI > Config File > Defaults
let options = DownloadOptions::from_config(&config)
    .merge_from_cli_args(&args);
```

### Filename Templating
Templates support variables like `{title}`, `{id}`, `{date}`, `{quality}`:
```rust
let metadata = VideoMetadata { title: "My Video", id: "abc123", ... };
let filename = apply_template("{title}-{id}", &metadata);
// Result: "My_Video-abc123"
```

## Code Style

- Uses Rust 2021 edition (MSRV: 1.75.0)
- Follows standard Rust naming conventions
- Comprehensive inline documentation with section headers (e.g., `// ============== Audio Command Tests ==============`)
- Error messages are user-friendly with contextual information
- Defaults are sensible and platform-aware
- Tests are grouped with comment headers matching the functionality they test

## Implementation Status

See `CONFIGURATION_DEPENDENCY.md` for complete feature checklist. Key implemented features:
- ✅ Quality selection (144p to 4K)
- ✅ Audio-only download with multiple formats (MP3, FLAC, M4A, WAV, Opus)
- ✅ Playlist download support
- ✅ Subtitle and thumbnail download
- ✅ Progress bars and spinners
- ✅ Auto-retry mechanism
- ✅ Format conversion (MP4, MKV, WebM)
- ✅ File naming templates
- ✅ Download rate limiting
- ✅ Configuration file management
- ✅ Metadata embedding
- ✅ Comprehensive integration tests (66 tests across 6 test suites)

**Current State**: **Production-ready**. All core features implemented, fully tested, and documented.

**Test Coverage**:
- **Unit tests**: 1800+ lines across all modules
- **Integration tests**: 66 tests covering end-to-end workflows
- **Total test count**: 700+ tests (unit + integration)
- **Code coverage**: Comprehensive coverage of all public APIs and critical paths

**Documentation**:
- ✅ Complete Rust API documentation (`cargo doc`)
- ✅ Comprehensive user guide (mdbook with 18 chapters)
- ✅ Developer documentation with architecture diagrams
- ✅ CLI reference and configuration guide

**When Implementing New Features**:
1. Add CLI flags/commands to `cli.rs` with comprehensive tests
2. Add config options to `config.rs` with sensible defaults
3. Create specific error variants in `error.rs` (avoid generic errors)
4. Implement business logic in appropriate module (`youtube/`, `media/`, `downloader.rs`)
5. Add progress feedback via `progress.rs` for long-running operations
6. **Add both unit tests and integration tests** for new functionality
7. Update documentation (`cargo doc` comments and mdbook chapters)
8. Update this CLAUDE.md with architectural changes

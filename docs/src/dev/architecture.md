# Architecture

This document provides an overview of the system architecture for the YouTube Downloader CLI tool, including module organization, data flow, and design patterns.

## System Overview

The YouTube Downloader is a professional command-line tool built in Rust that follows a modular, layered architecture. The application is designed for extensibility, testability, and maintainability.

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         main.rs                             │
│                    Application Entry Point                  │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                         cli.rs                              │
│              Command-Line Interface Layer                   │
│    (Argument Parsing, Validation, Command Dispatch)        │
└──────────────────────┬──────────────────────────────────────┘
                       │
       ┌───────────────┼───────────────┐
       ▼               ▼               ▼
┌──────────┐    ┌──────────┐    ┌──────────┐
│ config.rs│    │ error.rs │    │progress.rs│
│          │    │          │    │          │
│Configuration│ │  Error   │    │ Progress │
│ Management│    │ Handling │    │ Tracking │
└──────────┘    └──────────┘    └──────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────────┐
│                      downloader.rs                          │
│              Download Orchestration Layer                   │
│    (Coordinates downloads, retries, file management)       │
└──────────────────────┬──────────────────────────────────────┘
                       │
       ┌───────────────┼───────────────┐
       ▼               ▼               ▼
┌──────────┐    ┌──────────┐    ┌──────────┐
│youtube/  │    │ media/   │    │ utils.rs │
│          │    │          │    │          │
│ YouTube  │    │  Media   │    │ Utilities│
│   API    │    │Processing│    │          │
└──────────┘    └──────────┘    └──────────┘
```

## Layered Architecture

The codebase is organized into three distinct layers:

### 1. Core Infrastructure Layer

Foundational modules that provide essential services to the entire application:

- **`cli.rs`** - Command-line interface and argument parsing
- **`config.rs`** - Configuration file management
- **`error.rs`** - Centralized error handling
- **`main.rs`** - Application entry point

### 2. Business Logic Layer

Modules that implement the core functionality:

- **`downloader.rs`** - Download orchestration and coordination
- **`youtube/`** - YouTube API integration (client, metadata, playlists)
- **`media/`** - Media processing and conversion (FFmpeg, audio, video)

### 3. Utility Layer

Support modules providing common functionality:

- **`progress.rs`** - Progress bars and user feedback
- **`utils.rs`** - Common utilities (filename sanitization, templating, formatting)

## Module Structure

### Core Modules

#### cli.rs
- **Purpose**: Define and parse command-line arguments
- **Technology**: Uses `clap` with derive macros
- **Structure**: Hierarchical command pattern with enums
- **Commands**: `download`, `audio`, `playlist`, `info`, `config`

#### config.rs
- **Purpose**: Manage application configuration
- **Technology**: TOML serialization via `serde`
- **Location**: `~/.config/rust-yt-downloader/config.toml`
- **Sections**: `[general]`, `[audio]`, `[video]`, `[network]`

#### error.rs
- **Purpose**: Centralized error handling
- **Technology**: `thiserror` for error types
- **Categories**: Network, YouTube, Filesystem, FFmpeg, Configuration, Download, Validation
- **Features**: Retryable error detection, helper constructors

### Business Logic Modules

#### downloader.rs
- **Purpose**: Orchestrate download operations
- **Responsibilities**:
  - Merge CLI arguments with configuration
  - Coordinate between YouTube client and media processors
  - Implement retry logic
  - Apply filename templates
  - Manage download progress

#### youtube/ (Sub-module)
- **`client.rs`**: YouTube API integration using `rustube`
- **`metadata.rs`**: Video metadata extraction and quality filtering
- **`playlist.rs`**: Playlist handling
- **Public API**: `YouTubeClient`, `VideoInfo`, `PlaylistInfo`, `QualityFilter`

#### media/ (Sub-module)
- **`ffmpeg.rs`**: FFmpeg command wrapper
- **`audio.rs`**: Audio extraction and format conversion
- **`converter.rs`**: Video format conversion
- **Public API**: `FFmpeg`, `AudioExtractor`, `VideoConverter`

### Utility Modules

#### progress.rs
- Progress bars using `indicatif`
- Spinner animations
- User-facing messages

#### utils.rs
- `sanitize_filename()` - Clean filenames
- `expand_path()` - Handle `~` expansion
- `format_bytes()` - Human-readable sizes
- `apply_template()` - Filename templating

## Data Flow

### Download Flow

```
1. User Input
   │
   ▼
2. CLI Argument Parsing (cli.rs)
   │
   ▼
3. Configuration Loading (config.rs)
   │ Merge CLI args + config file
   ▼
4. Download Options Creation (downloader.rs)
   │
   ▼
5. YouTube Metadata Fetch (youtube/client.rs)
   │
   ▼
6. Quality Filtering (youtube/metadata.rs)
   │
   ▼
7. Stream Download with Progress (downloader.rs + progress.rs)
   │
   ▼
8. Media Processing (media/audio.rs or media/converter.rs)
   │
   ▼
9. Filename Generation (utils.rs)
   │
   ▼
10. File Output
```

### Configuration Priority System

The application uses a three-tier configuration priority system:

```
┌─────────────────────────────────┐
│   1. CLI Arguments              │  ◄── Highest Priority
│   (e.g., -q 1080p)              │
└─────────────────────────────────┘
          ▼ (overrides)
┌─────────────────────────────────┐
│   2. Configuration File         │
│   (~/.config/.../config.toml)   │
└─────────────────────────────────┘
          ▼ (overrides)
┌─────────────────────────────────┐
│   3. Built-in Defaults          │  ◄── Lowest Priority
│   (Config::default())           │
└─────────────────────────────────┘
```

**Implementation**:
```rust
let options = DownloadOptions::from_config(&config)
    .merge_from_cli_args(&args);
```

### Error Propagation Flow

```
Operation Error
    │
    ▼
AppError variant
    │
    ▼
Is retryable? ───Yes──► Retry Logic (downloader.rs)
    │                            │
    No                           │
    │                            ▼
    │                   Max retries exceeded?
    │                            │
    │                           Yes
    │                            │
    ▼◄───────────────────────────┘
Display Error
(AppError::Display)
    │
    ▼
Exit to User
```

All operations return `AppResult<T>` (alias for `Result<T, AppError>`), allowing errors to bubble up using the `?` operator.

## Design Patterns

### Command Pattern

The CLI uses the Command pattern with nested enums:

```rust
pub enum Commands {
    Download(DownloadArgs),
    Audio(AudioArgs),
    Playlist(PlaylistArgs),
    Info(InfoArgs),
    Config { command: ConfigCommands },
}
```

**Benefits**:
- Type-safe command dispatch
- Exhaustive pattern matching
- Clear command structure

### Builder Pattern

Configuration uses the Builder pattern via `serde` defaults:

```rust
#[derive(Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "GeneralConfig::default_output_dir")]
    pub output_dir: String,
}
```

### Repository Pattern

The `youtube/` module uses a repository-like pattern:

```rust
// Public facade
pub struct YouTubeClient;

// Internal implementation details hidden in sub-modules
mod client;
mod metadata;
mod playlist;
```

### Result Type Alias Pattern

Consistent error handling throughout:

```rust
pub type AppResult<T> = std::result::Result<T, AppError>;

fn download() -> AppResult<()> {
    // ...
}
```

## Concurrency Model

The application uses **Tokio** for async/await concurrency:

- **Async Runtime**: `tokio` with full features
- **HTTP Client**: `reqwest` with streaming support
- **Parallel Downloads**: Controlled by `max_parallel_downloads` config
- **Progress Tracking**: Runs concurrently via `tokio::spawn`

```rust
// Concurrent download with progress tracking
let download_task = tokio::spawn(async move {
    download_stream().await
});

let progress_task = tokio::spawn(async move {
    update_progress_bar().await
});

tokio::try_join!(download_task, progress_task)?;
```

## External Dependencies

### Core Dependencies
- **clap** (4.5) - CLI parsing
- **rustube** (0.6) - YouTube API
- **tokio** (1.40) - Async runtime
- **reqwest** (0.12) - HTTP client
- **serde** + **toml** - Configuration
- **thiserror** - Error types
- **indicatif** - Progress bars
- **colored** - Terminal colors

### External Tools
- **FFmpeg** - Required for audio/video processing
  - Must be in system PATH
  - Checked at runtime with graceful error if missing

## Key Design Principles

1. **Modularity**: Clear separation of concerns across modules
2. **Type Safety**: Strong typing with enums and structs
3. **Error Handling**: Comprehensive, context-rich errors
4. **Testability**: Extensive unit tests in each module
5. **User Experience**: Progress feedback and clear error messages
6. **Configuration**: Flexible defaults with user overrides
7. **Async-First**: All I/O is asynchronous
8. **Platform Awareness**: Uses platform-specific paths and conventions

## ASCII Art: Complete Data Flow

```
┌────────────┐
│   User     │
│  ytdl cmd  │
└─────┬──────┘
      │
      ▼
┌──────────────────┐
│  main.rs         │
│  Parse CLI       │
└─────┬────────────┘
      │
      ▼
┌──────────────────┐         ┌──────────────┐
│  cli.rs          │────────►│  error.rs    │
│  Validate Args   │         │  AppError    │
└─────┬────────────┘         └──────────────┘
      │
      ▼
┌──────────────────┐         ┌──────────────┐
│  config.rs       │────────►│  utils.rs    │
│  Load Config     │         │  expand_path │
└─────┬────────────┘         └──────────────┘
      │
      ▼
┌──────────────────┐         ┌──────────────┐
│  downloader.rs   │────────►│ progress.rs  │
│  Orchestrate     │         │ Show Progress│
└─────┬────────────┘         └──────────────┘
      │
      ├─────────────┬────────────┐
      ▼             ▼            ▼
┌──────────┐  ┌──────────┐ ┌──────────┐
│youtube/  │  │ media/   │ │ utils.rs │
│ client   │  │ ffmpeg   │ │ template │
└────┬─────┘  └────┬─────┘ └────┬─────┘
     │             │            │
     └─────────────┴────────────┘
                   │
                   ▼
            ┌─────────────┐
            │ Video File  │
            └─────────────┘
```

This architecture enables clean separation of concerns, testability, and maintainability while providing a robust foundation for future features.

# Error Handling

This document describes the error handling strategy used in the YouTube Downloader, including the design of `AppError`, error categories, propagation patterns, and best practices.

## Overview

The YouTube Downloader uses a comprehensive, categorized error handling system built on the `thiserror` crate. All errors are represented by the `AppError` enum, which provides:

- **Context-rich error messages** with file paths, URLs, and status codes
- **Error categorization** for different failure types
- **Retryable error detection** for automatic retry logic
- **Error chaining** to preserve underlying causes
- **User-friendly display** for end-user consumption

## AppError Design

### Type Alias

Throughout the codebase, all fallible operations return `AppResult<T>`:

```rust
pub type AppResult<T> = std::result::Result<T, AppError>;
```

This provides:
- Consistent error handling interface
- Easy error propagation with `?` operator
- Clear function signatures

### Error Enum Structure

```rust
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
    // ... more variants

    // Filesystem Errors
    FileRead { path: PathBuf, #[source] source: std::io::Error },
    FileWrite { path: PathBuf, #[source] source: std::io::Error },
    // ... more variants

    // ... other categories
}
```

## Error Categories

### 1. Network/HTTP Errors

**Purpose**: Handle network connectivity and HTTP protocol issues.

| Error Variant | Description | Retryable |
|---------------|-------------|-----------|
| `HttpRequest` | HTTP request failed with status code | No |
| `Connection` | Network connection failed | Yes |
| `Timeout` | Request exceeded timeout duration | Yes |
| `Network` | Generic reqwest error | Yes* |

*Depends on the specific reqwest error type.

**Example Usage**:
```rust
let response = client.get(url).send().await?;
if !response.status().is_success() {
    return Err(AppError::http(
        response.status().as_u16(),
        "Failed to fetch video metadata"
    ));
}
```

### 2. YouTube Errors

**Purpose**: Handle YouTube-specific failures.

| Error Variant | Description | Retryable |
|---------------|-------------|-----------|
| `InvalidUrl` | URL is not a valid YouTube URL | No |
| `VideoNotFound` | Video ID doesn't exist on YouTube | No |
| `VideoPrivate` | Video is marked as private | No |
| `AgeRestricted` | Video requires age verification | No |
| `RegionBlocked` | Video unavailable in user's region | No |
| `PlaylistNotFound` | Playlist ID doesn't exist | No |
| `ExtractionFailed` | Failed to extract video metadata | No |
| `YouTube` | Generic rustube error | No |

**Example Usage**:
```rust
fn validate_youtube_url(url: &str) -> AppResult<()> {
    if !url.contains("youtube.com") && !url.contains("youtu.be") {
        return Err(AppError::InvalidUrl(url.to_string()));
    }
    Ok(())
}
```

### 3. Filesystem Errors

**Purpose**: Handle file I/O and filesystem operations.

| Error Variant | Description | Retryable |
|---------------|-------------|-----------|
| `FileRead` | Failed to read file | No |
| `FileWrite` | Failed to write file | No |
| `DirectoryCreate` | Failed to create directory | No |
| `PathNotFound` | Path doesn't exist | No |
| `PermissionDenied` | Insufficient permissions | No |
| `Io` | Generic I/O error | No |

**Example Usage**:
```rust
use std::fs;

fn read_config(path: &PathBuf) -> AppResult<String> {
    fs::read_to_string(path)
        .map_err(|e| AppError::file_read(path, e))
}

fn write_video(path: &PathBuf, data: &[u8]) -> AppResult<()> {
    fs::write(path, data)
        .map_err(|e| AppError::file_write(path, e))
}
```

**Note**: These errors include the file path in the error message for better debugging.

### 4. FFmpeg Errors

**Purpose**: Handle FFmpeg-related failures.

| Error Variant | Description | Retryable |
|---------------|-------------|-----------|
| `FfmpegNotFound` | FFmpeg not installed or not in PATH | No |
| `FfmpegExecution` | FFmpeg command failed | No |
| `ConversionFailed` | Media format conversion failed | No |
| `TrimmingFailed` | Video trimming operation failed | No |

**Example Usage**:
```rust
use std::process::Command;

async fn check_ffmpeg() -> AppResult<()> {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map_err(|_| AppError::FfmpegNotFound)?;
    Ok(())
}

async fn convert_audio(input: &Path, output: &Path) -> AppResult<()> {
    let result = Command::new("ffmpeg")
        .args(["-i", input.to_str().unwrap()])
        .args(["-acodec", "libmp3lame"])
        .arg(output)
        .output()
        .await?;

    if !result.status.success() {
        return Err(AppError::ffmpeg(
            String::from_utf8_lossy(&result.stderr),
            result.status.code()
        ));
    }
    Ok(())
}
```

### 5. Configuration Errors

**Purpose**: Handle configuration file issues.

| Error Variant | Description | Retryable |
|---------------|-------------|-----------|
| `ConfigParse` | Failed to parse TOML config | No |
| `ConfigSerialize` | Failed to serialize config to TOML | No |
| `ConfigInvalid` | Config value is invalid | No |
| `ConfigNotFound` | Config file doesn't exist | No |

**Example Usage**:
```rust
use toml;

fn load_config(path: &PathBuf) -> AppResult<Config> {
    let contents = fs::read_to_string(path)
        .map_err(|e| AppError::file_read(path, e))?;

    toml::from_str(&contents)
        .map_err(|e| AppError::ConfigParse {
            path: path.clone(),
            source: e,
        })
}

fn validate_quality(quality: &str) -> AppResult<()> {
    let valid = ["best", "1080p", "720p", "480p", "360p", "worst"];
    if !valid.contains(&quality) {
        return Err(AppError::ConfigInvalid {
            field: "default_quality".to_string(),
            message: format!("Invalid quality: {}", quality),
        });
    }
    Ok(())
}
```

### 6. Download Errors

**Purpose**: Handle download-specific failures.

| Error Variant | Description | Retryable |
|---------------|-------------|-----------|
| `NoStreamsAvailable` | No video streams found | No |
| `QualityNotAvailable` | Requested quality not available | No |
| `FormatNotSupported` | Requested format not supported | No |
| `DownloadInterrupted` | Download was interrupted | Yes |
| `MaxRetriesExceeded` | All retry attempts failed | No |

**Example Usage**:
```rust
async fn download_stream(url: &str, quality: &str) -> AppResult<StreamInfo> {
    let streams = fetch_available_streams(url).await?;

    if streams.is_empty() {
        return Err(AppError::NoStreamsAvailable {
            video_id: extract_video_id(url)?,
        });
    }

    streams.into_iter()
        .find(|s| s.quality == quality)
        .ok_or_else(|| AppError::QualityNotAvailable {
            requested: quality.to_string(),
            available: streams.iter().map(|s| s.quality.clone()).collect(),
        })
}
```

### 7. Validation Errors

**Purpose**: Handle input validation failures.

| Error Variant | Description | Retryable |
|---------------|-------------|-----------|
| `InvalidArgument` | Invalid CLI argument value | No |
| `InvalidTimeFormat` | Time format string is invalid | No |
| `InvalidTemplate` | Filename template is invalid | No |

**Example Usage**:
```rust
fn parse_bitrate(bitrate: &str) -> AppResult<u32> {
    bitrate.trim_end_matches('k')
        .parse()
        .map_err(|_| AppError::invalid_arg(
            "bitrate",
            "must be a number followed by 'k' (e.g., 320k)"
        ))
}

fn validate_template(template: &str) -> AppResult<()> {
    let valid_vars = ["{title}", "{id}", "{author}", "{date}", "{quality}"];

    // Check for unknown variables
    for cap in TEMPLATE_REGEX.captures_iter(template) {
        let var = &cap[0];
        if !valid_vars.contains(&var) {
            return Err(AppError::InvalidTemplate {
                template: template.to_string(),
                message: format!("Unknown variable: {}", var),
            });
        }
    }
    Ok(())
}
```

### 8. Generic Errors

**Purpose**: Handle miscellaneous errors.

| Error Variant | Description | Retryable |
|---------------|-------------|-----------|
| `Cancelled` | User cancelled the operation | No |
| `Other` | Generic error with custom message | No |

## Helper Constructors

For convenience, `AppError` provides helper constructors for common error patterns:

```rust
impl AppError {
    /// Create an HTTP error
    pub fn http(status: u16, message: impl Into<String>) -> Self;

    /// Create a file read error
    pub fn file_read(path: impl Into<PathBuf>, source: std::io::Error) -> Self;

    /// Create a file write error
    pub fn file_write(path: impl Into<PathBuf>, source: std::io::Error) -> Self;

    /// Create a directory creation error
    pub fn dir_create(path: impl Into<PathBuf>, source: std::io::Error) -> Self;

    /// Create an FFmpeg execution error
    pub fn ffmpeg(message: impl Into<String>, exit_code: Option<i32>) -> Self;

    /// Create an invalid argument error
    pub fn invalid_arg(argument: impl Into<String>, message: impl Into<String>) -> Self;

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool;
}
```

**Usage Examples**:
```rust
// Instead of:
AppError::HttpRequest {
    status: 404,
    message: "Not Found".to_string(),
}

// Use:
AppError::http(404, "Not Found")

// Instead of:
AppError::FileRead {
    path: path.into(),
    source: io_error,
}

// Use:
AppError::file_read(path, io_error)
```

## Retryable vs Non-Retryable Errors

### Retryable Errors

Errors that represent **transient failures** that might succeed on retry:

```rust
pub fn is_retryable(&self) -> bool {
    matches!(
        self,
        Self::Timeout { .. }
            | Self::Connection(_)
            | Self::Network(_)
            | Self::DownloadInterrupted(_)
    )
}
```

**Examples**:
- Network timeout
- Connection refused
- Download interrupted
- Temporary network issues

### Non-Retryable Errors

Errors that represent **permanent failures** that won't succeed on retry:

- Invalid URLs
- Videos not found
- Permission denied
- FFmpeg not installed
- Invalid configuration
- User cancellation

### Retry Logic Implementation

```rust
async fn download_with_retry(url: &str, max_attempts: u32) -> AppResult<PathBuf> {
    let mut attempts = 0;

    loop {
        attempts += 1;

        match download_video(url).await {
            Ok(path) => return Ok(path),
            Err(e) if e.is_retryable() && attempts < max_attempts => {
                eprintln!("Download failed (attempt {}/{}): {}", attempts, max_attempts, e);
                tokio::time::sleep(Duration::from_secs(2_u64.pow(attempts))).await;
                continue;
            }
            Err(e) => {
                return Err(AppError::MaxRetriesExceeded {
                    attempts,
                    message: e.to_string(),
                });
            }
        }
    }
}
```

## Error Propagation Patterns

### Basic Propagation with `?`

```rust
fn read_and_parse_config(path: &PathBuf) -> AppResult<Config> {
    let contents = fs::read_to_string(path)
        .map_err(|e| AppError::file_read(path, e))?;

    let config: Config = toml::from_str(&contents)
        .map_err(|e| AppError::ConfigParse {
            path: path.clone(),
            source: e,
        })?;

    Ok(config)
}
```

### Converting External Errors

**Automatic conversion** with `#[from]`:
```rust
// Automatically converts reqwest::Error to AppError::Network
async fn fetch_data(url: &str) -> AppResult<String> {
    let response = reqwest::get(url).await?;  // ? converts reqwest::Error
    Ok(response.text().await?)
}
```

**Manual conversion** for better context:
```rust
async fn fetch_video_info(url: &str) -> AppResult<VideoInfo> {
    let response = reqwest::get(url).await
        .map_err(|_| AppError::Connection("Failed to connect to YouTube".to_string()))?;

    if response.status() == 404 {
        return Err(AppError::VideoNotFound {
            video_id: extract_id(url)?,
        });
    }

    // ... parse response
}
```

### Error Chaining

Preserve underlying error causes:

```rust
async fn convert_audio(input: &Path, output: &Path, format: &str) -> AppResult<PathBuf> {
    let result = run_ffmpeg(input, output, format).await
        .map_err(|e| AppError::ConversionFailed {
            from_format: "video".to_string(),
            to_format: format.to_string(),
            source: Box::new(e),
        })?;

    Ok(result)
}
```

## Best Practices

### 1. Use Specific Error Variants

❌ **Bad** - Using generic `Other` error:
```rust
if config.quality.is_empty() {
    return Err(AppError::Other("Quality cannot be empty".to_string()));
}
```

✅ **Good** - Using specific error variant:
```rust
if config.quality.is_empty() {
    return Err(AppError::ConfigInvalid {
        field: "quality".to_string(),
        message: "Quality cannot be empty".to_string(),
    });
}
```

### 2. Include Context in Error Messages

❌ **Bad** - Vague error message:
```rust
return Err(AppError::Other("File operation failed".to_string()));
```

✅ **Good** - Contextual error with path:
```rust
return Err(AppError::file_write(output_path, io_error));
```

### 3. Mark Retryable Errors Correctly

Only mark errors as retryable if they represent transient failures:

```rust
// Retryable: Network timeout
AppError::Timeout { seconds: 30 }

// NOT retryable: Invalid URL won't succeed on retry
AppError::InvalidUrl("invalid-url".to_string())
```

### 4. Propagate Errors with `?`

Let errors bubble up to the orchestration layer:

❌ **Bad** - Swallowing errors:
```rust
fn download() -> AppResult<()> {
    match fetch_video().await {
        Ok(video) => process(video),
        Err(_) => return Ok(()),  // Bad: silently ignoring error
    }
}
```

✅ **Good** - Propagating errors:
```rust
async fn download() -> AppResult<()> {
    let video = fetch_video().await?;  // Propagate error
    process(video).await?;
    Ok(())
}
```

### 5. Use Helper Constructors

Simplify error creation with helper methods:

❌ **Bad** - Verbose construction:
```rust
AppError::InvalidArgument {
    argument: "quality".to_string(),
    message: "Invalid quality value".to_string(),
}
```

✅ **Good** - Using helper:
```rust
AppError::invalid_arg("quality", "Invalid quality value")
```

## Error Display Examples

### Error with Context

```rust
let err = AppError::file_read("/path/to/config.toml", io::Error::...);
println!("{}", err);
// Output: "Failed to read file: /path/to/config.toml"
```

### Error with Structured Data

```rust
let err = AppError::QualityNotAvailable {
    requested: "4K".to_string(),
    available: vec!["1080p".to_string(), "720p".to_string()],
};
println!("{}", err);
// Output: "Quality not available: 4K (available: ["1080p", "720p"])"
```

### Error Chain

```rust
let err = AppError::ConversionFailed {
    from_format: "mkv".to_string(),
    to_format: "mp4".to_string(),
    source: Box::new(AppError::FfmpegNotFound),
};
println!("{}", err);
// Output: "Conversion failed: mkv -> mp4"

// Access underlying error
if let Some(source) = err.source() {
    println!("Caused by: {}", source);
    // Output: "Caused by: FFmpeg not found. Please install FFmpeg and ensure it's in your PATH"
}
```

## Summary

The error handling system in the YouTube Downloader provides:

1. ✅ **Categorized errors** for different failure types
2. ✅ **Context-rich messages** with paths, URLs, and codes
3. ✅ **Retryable error detection** for automatic recovery
4. ✅ **Error chaining** to preserve root causes
5. ✅ **User-friendly display** for end users
6. ✅ **Helper constructors** for common patterns
7. ✅ **Type safety** with `AppResult<T>` throughout

This comprehensive approach ensures robust error handling while maintaining code clarity and user experience.

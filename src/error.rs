use thiserror::Error;
use std::path::PathBuf;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    // ============== Network/HTTP Errors ==============
    #[error("HTTP request failed: {message} (status code: {status})")]
    HttpRequest {
        message: String,
        status: u16,
    },

    #[error("Connection failed: {0}")]
    Connection(String),

    #[error("Request timeout after {seconds} seconds")]
    Timeout { seconds: u64 },

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    // ============== YouTube Errors ==============
    #[error("Invalid YouTube URL: {0}")]
    InvalidUrl(String),

    #[error("Video not found: {video_id}")]
    VideoNotFound { video_id: String },

    #[error("Video is private: {video_id}")]
    VideoPrivate { video_id: String },

    #[error("Video is age-restricted: {video_id}")]
    AgeRestricted { video_id: String },

    #[error("Video is unavailable in your region: {video_id}")]
    RegionBlocked { video_id: String },

    #[error("Playlist not found: {playlist_id}")]
    PlaylistNotFound { playlist_id: String },

    #[error("Failed to extract video info: {0}")]
    ExtractionFailed(String),

    #[error("YouTube error: {0}")]
    YouTube(#[from] rustube::Error),

    // ============== Filesystem Errors ==============
    #[error("Failed to read file: {path}")]
    FileRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write file: {path}")]
    FileWrite {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to create directory: {path}")]
    DirectoryCreate {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Path not found: {0}")]
    PathNotFound(PathBuf),

    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    // ============== FFmpeg Errors ==============
    #[error("FFmpeg not found. Please install FFmpeg and ensure it's in your PATH")]
    FfmpegNotFound,

    #[error("FFmpeg command failed: {message}")]
    FfmpegExecution { message: String, exit_code: Option<i32> },

    #[error("Conversion failed: {from_format} -> {to_format}")]
    ConversionFailed {
        from_format: String,
        to_format: String,
        #[source]
        source: Box<AppError>,
    },

    #[error("Trimming failed for time range {start} -> {end}")]
    TrimmingFailed {
        start: String,
        end: String,
        #[source]
        source: Box<AppError>,
    },

    // ============== Configuration Errors ==============
    #[error("Failed to parse config file: {path}")]
    ConfigParse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    #[error("Failed to serialize config")]
    ConfigSerialize(#[from] toml::ser::Error),

    #[error("Invalid configuration value: {field} - {message}")]
    ConfigInvalid { field: String, message: String },

    #[error("Config file not found: {0}")]
    ConfigNotFound(PathBuf),

    // ============== Download Errors ==============
    #[error("No streams available for video: {video_id}")]
    NoStreamsAvailable { video_id: String },

    #[error("Quality not available: {requested} (available: {available:?})")]
    QualityNotAvailable {
        requested: String,
        available: Vec<String>,
    },

    #[error("Format not supported: {0}")]
    FormatNotSupported(String),

    #[error("Download interrupted: {0}")]
    DownloadInterrupted(String),

    #[error("Download failed after {attempts} attempts: {message}")]
    MaxRetriesExceeded { attempts: u32, message: String },

    // ============== Validation Errors ==============
    #[error("Invalid argument: {argument} - {message}")]
    InvalidArgument { argument: String, message: String },

    #[error("Invalid time format: {0} (expected HH:MM:SS or seconds)")]
    InvalidTimeFormat(String),

    #[error("Invalid template: {template} - {message}")]
    InvalidTemplate { template: String, message: String },

    // ============== Generic Errors ==============
    #[error("Operation cancelled by user")]
    Cancelled,

    #[error("{0}")]
    Other(String),
}

// ============== Helper Constructors ==============
impl AppError {
    /// Create HTTP request error with status code and message
    pub fn http(status: u16, message: impl Into<String>) -> Self {
        Self::HttpRequest {
            status,
            message: message.into(),
        }
    }

    /// Create file read error
    pub fn file_read(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::FileRead {
            path: path.into(),
            source,
        }
    }

    /// Create file write error
    pub fn file_write(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::FileWrite {
            path: path.into(),
            source,
        }
    }

    /// Create directory creation error
    pub fn dir_create(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::DirectoryCreate {
            path: path.into(),
            source,
        }
    }

    /// Create FFmpeg error
    pub fn ffmpeg(message: impl Into<String>, exit_code: Option<i32>) -> Self {
        Self::FfmpegExecution {
            message: message.into(),
            exit_code,
        }
    }

    /// Create invalid argument error
    pub fn invalid_arg(argument: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InvalidArgument {
            argument: argument.into(),
            message: message.into(),
        }
    }

    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::Timeout { .. }
                | Self::Connection(_)
                | Self::Network(_)
                | Self::DownloadInterrupted(_)
        )
    }
}

// ==================================================
//          UNITARY TESTS
// ==================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::io::{self, ErrorKind};

    // ============== Display/Format Tests ==============

    #[test]
    fn test_http_error_display() {
        let error = AppError::http(404, "Not Found");
        assert_eq!(
            error.to_string(),
            "HTTP request failed: Not Found (status code: 404)"
        );
    }

    #[test]
    fn test_http_error_display_server_error() {
        let error = AppError::http(500, "Internal Server Error");
        assert_eq!(
            error.to_string(),
            "HTTP request failed: Internal Server Error (status code: 500)"
        );
    }

    #[test]
    fn test_connection_error_display() {
        let error = AppError::Connection("Connection refused".to_string());
        assert_eq!(error.to_string(), "Connection failed: Connection refused");
    }

    #[test]
    fn test_timeout_error_display() {
        let error = AppError::Timeout { seconds: 30 };
        assert_eq!(error.to_string(), "Request timeout after 30 seconds");
    }

    #[test]
    fn test_invalid_url_display() {
        let error = AppError::InvalidUrl("not-a-valid-url".to_string());
        assert_eq!(error.to_string(), "Invalid YouTube URL: not-a-valid-url");
    }

    #[test]
    fn test_video_not_found_display() {
        let error = AppError::VideoNotFound {
            video_id: "abc123".to_string(),
        };
        assert_eq!(error.to_string(), "Video not found: abc123");
    }

    #[test]
    fn test_video_private_display() {
        let error = AppError::VideoPrivate {
            video_id: "xyz789".to_string(),
        };
        assert_eq!(error.to_string(), "Video is private: xyz789");
    }

    #[test]
    fn test_age_restricted_display() {
        let error = AppError::AgeRestricted {
            video_id: "mature123".to_string(),
        };
        assert_eq!(error.to_string(), "Video is age-restricted: mature123");
    }

    #[test]
    fn test_region_blocked_display() {
        let error = AppError::RegionBlocked {
            video_id: "blocked456".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Video is unavailable in your region: blocked456"
        );
    }

    #[test]
    fn test_playlist_not_found_display() {
        let error = AppError::PlaylistNotFound {
            playlist_id: "PL123".to_string(),
        };
        assert_eq!(error.to_string(), "Playlist not found: PL123");
    }

    #[test]
    fn test_ffmpeg_not_found_display() {
        let error = AppError::FfmpegNotFound;
        assert_eq!(
            error.to_string(),
            "FFmpeg not found. Please install FFmpeg and ensure it's in your PATH"
        );
    }

    #[test]
    fn test_ffmpeg_execution_display() {
        let error = AppError::ffmpeg("encoding failed", Some(1));
        assert_eq!(error.to_string(), "FFmpeg command failed: encoding failed");
    }

    #[test]
    fn test_quality_not_available_display() {
        let error = AppError::QualityNotAvailable {
            requested: "4K".to_string(),
            available: vec!["1080p".to_string(), "720p".to_string()],
        };
        assert_eq!(
            error.to_string(),
            "Quality not available: 4K (available: [\"1080p\", \"720p\"])"
        );
    }

    #[test]
    fn test_invalid_time_format_display() {
        let error = AppError::InvalidTimeFormat("25:61:00".to_string());
        assert_eq!(
            error.to_string(),
            "Invalid time format: 25:61:00 (expected HH:MM:SS or seconds)"
        );
    }

    #[test]
    fn test_max_retries_exceeded_display() {
        let error = AppError::MaxRetriesExceeded {
            attempts: 3,
            message: "server unavailable".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Download failed after 3 attempts: server unavailable"
        );
    }

    #[test]
    fn test_cancelled_display() {
        let error = AppError::Cancelled;
        assert_eq!(error.to_string(), "Operation cancelled by user");
    }

    #[test]
    fn test_other_error_display() {
        let error = AppError::Other("Something unexpected happened".to_string());
        assert_eq!(error.to_string(), "Something unexpected happened");
    }

    // ============== Helper Constructor Tests ==============

    #[test]
    fn test_http_helper_constructor() {
        let error = AppError::http(403, "Forbidden");
        match error {
            AppError::HttpRequest { status, message } => {
                assert_eq!(status, 403);
                assert_eq!(message, "Forbidden");
            }
            _ => panic!("Expected HttpRequest variant"),
        }
    }

    #[test]
    fn test_file_read_helper_constructor() {
        let io_error = io::Error::new(ErrorKind::NotFound, "file not found");
        let error = AppError::file_read("/path/to/file.txt", io_error);
        match error {
            AppError::FileRead { path, source } => {
                assert_eq!(path, PathBuf::from("/path/to/file.txt"));
                assert_eq!(source.kind(), ErrorKind::NotFound);
            }
            _ => panic!("Expected FileRead variant"),
        }
    }

    #[test]
    fn test_file_write_helper_constructor() {
        let io_error = io::Error::new(ErrorKind::PermissionDenied, "permission denied");
        let error = AppError::file_write("/protected/file.txt", io_error);
        match error {
            AppError::FileWrite { path, source } => {
                assert_eq!(path, PathBuf::from("/protected/file.txt"));
                assert_eq!(source.kind(), ErrorKind::PermissionDenied);
            }
            _ => panic!("Expected FileWrite variant"),
        }
    }

    #[test]
    fn test_dir_create_helper_constructor() {
        let io_error = io::Error::new(ErrorKind::AlreadyExists, "directory exists");
        let error = AppError::dir_create("/existing/dir", io_error);
        match error {
            AppError::DirectoryCreate { path, source } => {
                assert_eq!(path, PathBuf::from("/existing/dir"));
                assert_eq!(source.kind(), ErrorKind::AlreadyExists);
            }
            _ => panic!("Expected DirectoryCreate variant"),
        }
    }

    #[test]
    fn test_ffmpeg_helper_constructor_with_exit_code() {
        let error = AppError::ffmpeg("conversion failed", Some(1));
        match error {
            AppError::FfmpegExecution { message, exit_code } => {
                assert_eq!(message, "conversion failed");
                assert_eq!(exit_code, Some(1));
            }
            _ => panic!("Expected FfmpegExecution variant"),
        }
    }

    #[test]
    fn test_ffmpeg_helper_constructor_without_exit_code() {
        let error = AppError::ffmpeg("process killed", None);
        match error {
            AppError::FfmpegExecution { message, exit_code } => {
                assert_eq!(message, "process killed");
                assert_eq!(exit_code, None);
            }
            _ => panic!("Expected FfmpegExecution variant"),
        }
    }

    #[test]
    fn test_invalid_arg_helper_constructor() {
        let error = AppError::invalid_arg("--quality", "must be a valid resolution");
        match error {
            AppError::InvalidArgument { argument, message } => {
                assert_eq!(argument, "--quality");
                assert_eq!(message, "must be a valid resolution");
            }
            _ => panic!("Expected InvalidArgument variant"),
        }
    }

    // ============== is_retryable Tests ==============

    #[test]
    fn test_timeout_is_retryable() {
        let error = AppError::Timeout { seconds: 30 };
        assert!(error.is_retryable());
    }

    #[test]
    fn test_connection_is_retryable() {
        let error = AppError::Connection("refused".to_string());
        assert!(error.is_retryable());
    }

    #[test]
    fn test_download_interrupted_is_retryable() {
        let error = AppError::DownloadInterrupted("connection lost".to_string());
        assert!(error.is_retryable());
    }

    #[test]
    fn test_invalid_url_not_retryable() {
        let error = AppError::InvalidUrl("bad-url".to_string());
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_video_not_found_not_retryable() {
        let error = AppError::VideoNotFound {
            video_id: "abc".to_string(),
        };
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_ffmpeg_not_found_not_retryable() {
        let error = AppError::FfmpegNotFound;
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_cancelled_not_retryable() {
        let error = AppError::Cancelled;
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_config_invalid_not_retryable() {
        let error = AppError::ConfigInvalid {
            field: "quality".to_string(),
            message: "invalid value".to_string(),
        };
        assert!(!error.is_retryable());
    }

    // ============== #[from] Conversion Tests ==============

    #[test]
    fn test_io_error_converts_to_app_error() {
        let io_error = io::Error::new(ErrorKind::NotFound, "file not found");
        let app_error: AppError = io_error.into();
        assert!(matches!(app_error, AppError::Io(_)));
    }

    #[test]
    fn test_io_error_preserves_kind() {
        let io_error = io::Error::new(ErrorKind::PermissionDenied, "access denied");
        let app_error: AppError = io_error.into();
        if let AppError::Io(inner) = app_error {
            assert_eq!(inner.kind(), ErrorKind::PermissionDenied);
        } else {
            panic!("Expected Io variant");
        }
    }

    // ============== #[source] Chain Tests ==============

    #[test]
    fn test_file_read_has_source() {
        let io_error = io::Error::new(ErrorKind::NotFound, "not found");
        let error = AppError::file_read("/test/path", io_error);
        assert!(error.source().is_some());
    }

    #[test]
    fn test_file_write_has_source() {
        let io_error = io::Error::new(ErrorKind::PermissionDenied, "denied");
        let error = AppError::file_write("/test/path", io_error);
        assert!(error.source().is_some());
    }

    #[test]
    fn test_directory_create_has_source() {
        let io_error = io::Error::new(ErrorKind::AlreadyExists, "exists");
        let error = AppError::dir_create("/test/dir", io_error);
        assert!(error.source().is_some());
    }

    #[test]
    fn test_simple_errors_have_no_source() {
        let error = AppError::InvalidUrl("test".to_string());
        assert!(error.source().is_none());

        let error = AppError::Cancelled;
        assert!(error.source().is_none());

        let error = AppError::FfmpegNotFound;
        assert!(error.source().is_none());
    }

    // ============== PathBuf Display Tests ==============

    #[test]
    fn test_file_read_display_includes_path() {
        let io_error = io::Error::new(ErrorKind::NotFound, "not found");
        let error = AppError::file_read("/home/user/video.mp4", io_error);
        assert!(error.to_string().contains("/home/user/video.mp4"));
    }

    #[test]
    fn test_path_not_found_display() {
        let error = AppError::PathNotFound(PathBuf::from("/missing/path"));
        assert_eq!(error.to_string(), "Path not found: /missing/path");
    }

    #[test]
    fn test_permission_denied_display() {
        let error = AppError::PermissionDenied(PathBuf::from("/root/secret"));
        assert_eq!(error.to_string(), "Permission denied: /root/secret");
    }

    #[test]
    fn test_config_not_found_display() {
        let error = AppError::ConfigNotFound(PathBuf::from("~/.config/app/config.toml"));
        assert_eq!(
            error.to_string(),
            "Config file not found: ~/.config/app/config.toml"
        );
    }
}
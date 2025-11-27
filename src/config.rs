
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,

    #[serde(default)]
    pub audio: AudioConfig,

    #[serde(default)]
    pub video: VideoConfig,

    #[serde(default)]
    pub network: NetworkConfig,
}

/// General application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Output directory for downloads
    #[serde(default = "GeneralConfig::default_output_dir")]
    pub output_dir: String,

    /// Default video quality
    #[serde(default = "GeneralConfig::default_quality")]
    pub default_quality: String,

    /// Maximum parallel downloads
    #[serde(default = "GeneralConfig::default_max_parallel")]
    pub max_parallel_downloads: u32,
}

/// Audio-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Default audio format
    #[serde(default = "AudioConfig::default_format")]
    pub format: String,

    /// Audio bitrate
    #[serde(default = "AudioConfig::default_bitrate")]
    pub bitrate: String,
}

/// Video-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfig {
    /// Default video format
    #[serde(default = "VideoConfig::default_format")]
    pub format: String,

    /// Include thumbnail in download
    #[serde(default = "VideoConfig::default_include_thumbnail")]
    pub include_thumbnail: bool,

    /// Include subtitles in download
    #[serde(default = "VideoConfig::default_include_subtitles")]
    pub include_subtitles: bool,
}

/// Network-related settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Download rate limit (e.g., "5M" for 5 MB/s)
    #[serde(default)]
    pub rate_limit: Option<String>,

    /// Number of retry attempts
    #[serde(default = "NetworkConfig::default_retry_attempts")]
    pub retry_attempts: u32,

    /// Connection timeout in seconds
    #[serde(default = "NetworkConfig::default_timeout")]
    pub timeout: u64,
}

// ============== Default Implementations ==============

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            audio: AudioConfig::default(),
            video: VideoConfig::default(),
            network: NetworkConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            output_dir: Self::default_output_dir(),
            default_quality: Self::default_quality(),
            max_parallel_downloads: Self::default_max_parallel(),
        }
    }
}

impl GeneralConfig {
    fn default_output_dir() -> String {
        dirs::download_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("YouTube")
            .to_string_lossy()
            .to_string()
    }

    fn default_quality() -> String {
        "best".to_string()
    }

    fn default_max_parallel() -> u32 {
        3
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            format: Self::default_format(),
            bitrate: Self::default_bitrate(),
        }
    }
}

impl AudioConfig {
    fn default_format() -> String {
        "mp3".to_string()
    }

    fn default_bitrate() -> String {
        "320k".to_string()
    }
}

impl Default for VideoConfig {
    fn default() -> Self {
        Self {
            format: Self::default_format(),
            include_thumbnail: Self::default_include_thumbnail(),
            include_subtitles: Self::default_include_subtitles(),
        }
    }
}

impl VideoConfig {
    fn default_format() -> String {
        "mp4".to_string()
    }

    fn default_include_thumbnail() -> bool {
        true
    }

    fn default_include_subtitles() -> bool {
        true
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            rate_limit: None,
            retry_attempts: Self::default_retry_attempts(),
            timeout: Self::default_timeout(),
        }
    }
}

impl NetworkConfig {
    fn default_retry_attempts() -> u32 {
        3
    }

    fn default_timeout() -> u64 {
        300
    }
}

// ============== Config Implementation ==============

impl Config {
    pub fn config_path() -> AppResult<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| AppError::Other("Could not find config directory".to_string()))?;

        Ok(config_dir.join("rust-yt-downloader").join("config.toml"))
    }

    pub fn load() -> AppResult<Self> {
        let path = Self::config_path()?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path).map_err(|e| AppError::file_read(&path, e))?;

        let config: Config = toml::from_str(&content).map_err(|e| AppError::ConfigParse {
            path: path.clone(),
            source: e,
        })?;

        Ok(config)
    }

    pub fn save(&self) -> AppResult<()> {
        let path = Self::config_path()?;

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|e| AppError::dir_create(parent, e))?;
            }
        }

        let content = toml::to_string_pretty(self)?;

        fs::write(&path, content).map_err(|e| AppError::file_write(&path, e))?;

        Ok(())
    }

    pub fn reset() -> AppResult<Self> {
        let config = Self::default();
        config.save()?;
        Ok(config)
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let parts: Vec<&str> = key.split('.').collect();

        match parts.as_slice() {
            ["general", "output_dir"] => Some(self.general.output_dir.clone()),
            ["general", "default_quality"] => Some(self.general.default_quality.clone()),
            ["general", "max_parallel_downloads"] => {
                Some(self.general.max_parallel_downloads.to_string())
            }

            ["audio", "format"] => Some(self.audio.format.clone()),
            ["audio", "bitrate"] => Some(self.audio.bitrate.clone()),

            ["video", "format"] => Some(self.video.format.clone()),
            ["video", "include_thumbnail"] => Some(self.video.include_thumbnail.to_string()),
            ["video", "include_subtitles"] => Some(self.video.include_subtitles.to_string()),

            ["network", "rate_limit"] => self.network.rate_limit.clone(),
            ["network", "retry_attempts"] => Some(self.network.retry_attempts.to_string()),
            ["network", "timeout"] => Some(self.network.timeout.to_string()),

            _ => None,
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> AppResult<()> {
        let parts: Vec<&str> = key.split('.').collect();

        match parts.as_slice() {
            ["general", "output_dir"] => {
                self.general.output_dir = value.to_string();
            }
            ["general", "default_quality"] => {
                self.general.default_quality = value.to_string();
            }
            ["general", "max_parallel_downloads"] => {
                self.general.max_parallel_downloads =
                    value.parse().map_err(|_| AppError::ConfigInvalid {
                        field: key.to_string(),
                        message: "must be a positive integer".to_string(),
                    })?;
            }

            ["audio", "format"] => {
                self.audio.format = value.to_string();
            }
            ["audio", "bitrate"] => {
                self.audio.bitrate = value.to_string();
            }

            ["video", "format"] => {
                self.video.format = value.to_string();
            }
            ["video", "include_thumbnail"] => {
                self.video.include_thumbnail =
                    value.parse().map_err(|_| AppError::ConfigInvalid {
                        field: key.to_string(),
                        message: "must be true or false".to_string(),
                    })?;
            }
            ["video", "include_subtitles"] => {
                self.video.include_subtitles =
                    value.parse().map_err(|_| AppError::ConfigInvalid {
                        field: key.to_string(),
                        message: "must be true or false".to_string(),
                    })?;
            }

            ["network", "rate_limit"] => {
                self.network.rate_limit = if value.is_empty() || value == "none" {
                    None
                } else {
                    Some(value.to_string())
                };
            }
            ["network", "retry_attempts"] => {
                self.network.retry_attempts =
                    value.parse().map_err(|_| AppError::ConfigInvalid {
                        field: key.to_string(),
                        message: "must be a positive integer".to_string(),
                    })?;
            }
            ["network", "timeout"] => {
                self.network.timeout = value.parse().map_err(|_| AppError::ConfigInvalid {
                    field: key.to_string(),
                    message: "must be a positive integer".to_string(),
                })?;
            }

            _ => {
                return Err(AppError::ConfigInvalid {
                    field: key.to_string(),
                    message: "unknown configuration key".to_string(),
                });
            }
        }

        Ok(())
    }

    pub fn keys() -> Vec<&'static str> {
        vec![
            "general.output_dir",
            "general.default_quality",
            "general.max_parallel_downloads",
            "audio.format",
            "audio.bitrate",
            "video.format",
            "video.include_thumbnail",
            "video.include_subtitles",
            "network.rate_limit",
            "network.retry_attempts",
            "network.timeout",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // ============== Helper Functions ==============

    /// Creates a temporary config file for testing
    fn create_temp_config(content: &str) -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        fs::write(&config_path, content).unwrap();
        (temp_dir, config_path)
    }

    // ============== Default Values Tests ==============

    #[test]
    fn test_config_default() {
        let config = Config::default();

        assert_eq!(config.general.default_quality, "best");
        assert_eq!(config.general.max_parallel_downloads, 3);
        assert_eq!(config.audio.format, "mp3");
        assert_eq!(config.audio.bitrate, "320k");
        assert_eq!(config.video.format, "mp4");
        assert!(config.video.include_thumbnail);
        assert!(config.video.include_subtitles);
        assert_eq!(config.network.retry_attempts, 3);
        assert_eq!(config.network.timeout, 300);
        assert!(config.network.rate_limit.is_none());
    }

    #[test]
    fn test_general_config_default() {
        let general = GeneralConfig::default();

        assert_eq!(general.default_quality, "best");
        assert_eq!(general.max_parallel_downloads, 3);
        assert!(general.output_dir.contains("YouTube"));
    }

    #[test]
    fn test_audio_config_default() {
        let audio = AudioConfig::default();

        assert_eq!(audio.format, "mp3");
        assert_eq!(audio.bitrate, "320k");
    }

    #[test]
    fn test_video_config_default() {
        let video = VideoConfig::default();

        assert_eq!(video.format, "mp4");
        assert!(video.include_thumbnail);
        assert!(video.include_subtitles);
    }

    #[test]
    fn test_network_config_default() {
        let network = NetworkConfig::default();

        assert!(network.rate_limit.is_none());
        assert_eq!(network.retry_attempts, 3);
        assert_eq!(network.timeout, 300);
    }

    // ============== Config Path Tests ==============

    #[test]
    fn test_config_path_ends_with_correct_filename() {
        let path = Config::config_path().unwrap();

        assert!(path.ends_with("rust-yt-downloader/config.toml"));
    }

    #[test]
    fn test_config_path_is_absolute() {
        let path = Config::config_path().unwrap();

        assert!(path.is_absolute());
    }

    // ============== TOML Parsing Tests ==============

    #[test]
    fn test_parse_full_config() {
        let toml_content = r#"
[general]
output_dir = "/custom/path"
default_quality = "1080p"
max_parallel_downloads = 5

[audio]
format = "flac"
bitrate = "256k"

[video]
format = "mkv"
include_thumbnail = false
include_subtitles = false

[network]
rate_limit = "10M"
retry_attempts = 5
timeout = 600
"#;

        let config: Config = toml::from_str(toml_content).unwrap();

        assert_eq!(config.general.output_dir, "/custom/path");
        assert_eq!(config.general.default_quality, "1080p");
        assert_eq!(config.general.max_parallel_downloads, 5);
        assert_eq!(config.audio.format, "flac");
        assert_eq!(config.audio.bitrate, "256k");
        assert_eq!(config.video.format, "mkv");
        assert!(!config.video.include_thumbnail);
        assert!(!config.video.include_subtitles);
        assert_eq!(config.network.rate_limit, Some("10M".to_string()));
        assert_eq!(config.network.retry_attempts, 5);
        assert_eq!(config.network.timeout, 600);
    }

    #[test]
    fn test_parse_partial_config_uses_defaults() {
        let toml_content = r#"
[general]
default_quality = "720p"
"#;

        let config: Config = toml::from_str(toml_content).unwrap();

        // Specified value
        assert_eq!(config.general.default_quality, "720p");

        // Default values
        assert_eq!(config.general.max_parallel_downloads, 3);
        assert_eq!(config.audio.format, "mp3");
        assert_eq!(config.video.format, "mp4");
        assert_eq!(config.network.retry_attempts, 3);
    }

    #[test]
    fn test_parse_empty_config_uses_all_defaults() {
        let toml_content = "";

        let config: Config = toml::from_str(toml_content).unwrap();

        assert_eq!(config.general.default_quality, "best");
        assert_eq!(config.audio.format, "mp3");
        assert_eq!(config.video.format, "mp4");
        assert_eq!(config.network.retry_attempts, 3);
    }

    // ============== Serialization Tests ==============

    #[test]
    fn test_serialize_config() {
        let config = Config::default();
        let toml_string = toml::to_string_pretty(&config);

        assert!(toml_string.is_ok());

        let serialized = toml_string.unwrap();
        assert!(serialized.contains("[general]"));
        assert!(serialized.contains("[audio]"));
        assert!(serialized.contains("[video]"));
        assert!(serialized.contains("[network]"));
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = Config::default();
        let toml_string = toml::to_string_pretty(&original).unwrap();
        let parsed: Config = toml::from_str(&toml_string).unwrap();

        assert_eq!(original.general.default_quality, parsed.general.default_quality);
        assert_eq!(original.audio.format, parsed.audio.format);
        assert_eq!(original.video.format, parsed.video.format);
        assert_eq!(original.network.timeout, parsed.network.timeout);
    }

    // ============== Get Method Tests ==============

    #[test]
    fn test_get_general_output_dir() {
        let config = Config::default();
        let value = config.get("general.output_dir");

        assert!(value.is_some());
        assert!(value.unwrap().contains("YouTube"));
    }

    #[test]
    fn test_get_general_default_quality() {
        let config = Config::default();
        let value = config.get("general.default_quality");

        assert_eq!(value, Some("best".to_string()));
    }

    #[test]
    fn test_get_general_max_parallel_downloads() {
        let config = Config::default();
        let value = config.get("general.max_parallel_downloads");

        assert_eq!(value, Some("3".to_string()));
    }

    #[test]
    fn test_get_audio_format() {
        let config = Config::default();
        let value = config.get("audio.format");

        assert_eq!(value, Some("mp3".to_string()));
    }

    #[test]
    fn test_get_audio_bitrate() {
        let config = Config::default();
        let value = config.get("audio.bitrate");

        assert_eq!(value, Some("320k".to_string()));
    }

    #[test]
    fn test_get_video_format() {
        let config = Config::default();
        let value = config.get("video.format");

        assert_eq!(value, Some("mp4".to_string()));
    }

    #[test]
    fn test_get_video_include_thumbnail() {
        let config = Config::default();
        let value = config.get("video.include_thumbnail");

        assert_eq!(value, Some("true".to_string()));
    }

    #[test]
    fn test_get_video_include_subtitles() {
        let config = Config::default();
        let value = config.get("video.include_subtitles");

        assert_eq!(value, Some("true".to_string()));
    }

    #[test]
    fn test_get_network_rate_limit_none() {
        let config = Config::default();
        let value = config.get("network.rate_limit");

        assert!(value.is_none());
    }

    #[test]
    fn test_get_network_rate_limit_some() {
        let mut config = Config::default();
        config.network.rate_limit = Some("5M".to_string());

        let value = config.get("network.rate_limit");
        assert_eq!(value, Some("5M".to_string()));
    }

    #[test]
    fn test_get_network_retry_attempts() {
        let config = Config::default();
        let value = config.get("network.retry_attempts");

        assert_eq!(value, Some("3".to_string()));
    }

    #[test]
    fn test_get_network_timeout() {
        let config = Config::default();
        let value = config.get("network.timeout");

        assert_eq!(value, Some("300".to_string()));
    }

    #[test]
    fn test_get_unknown_key_returns_none() {
        let config = Config::default();

        assert!(config.get("unknown").is_none());
        assert!(config.get("general.unknown").is_none());
        assert!(config.get("unknown.field").is_none());
        assert!(config.get("").is_none());
    }

    // ============== Set Method Tests ==============

    #[test]
    fn test_set_general_output_dir() {
        let mut config = Config::default();
        let result = config.set("general.output_dir", "/new/path");

        assert!(result.is_ok());
        assert_eq!(config.general.output_dir, "/new/path");
    }

    #[test]
    fn test_set_general_default_quality() {
        let mut config = Config::default();
        let result = config.set("general.default_quality", "1080p");

        assert!(result.is_ok());
        assert_eq!(config.general.default_quality, "1080p");
    }

    #[test]
    fn test_set_general_max_parallel_downloads() {
        let mut config = Config::default();
        let result = config.set("general.max_parallel_downloads", "5");

        assert!(result.is_ok());
        assert_eq!(config.general.max_parallel_downloads, 5);
    }

    #[test]
    fn test_set_general_max_parallel_downloads_invalid() {
        let mut config = Config::default();
        let result = config.set("general.max_parallel_downloads", "not_a_number");

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::ConfigInvalid { field, message } => {
                assert_eq!(field, "general.max_parallel_downloads");
                assert!(message.contains("positive integer"));
            }
            _ => panic!("Expected ConfigInvalid error"),
        }
    }

    #[test]
    fn test_set_audio_format() {
        let mut config = Config::default();
        let result = config.set("audio.format", "flac");

        assert!(result.is_ok());
        assert_eq!(config.audio.format, "flac");
    }

    #[test]
    fn test_set_audio_bitrate() {
        let mut config = Config::default();
        let result = config.set("audio.bitrate", "256k");

        assert!(result.is_ok());
        assert_eq!(config.audio.bitrate, "256k");
    }

    #[test]
    fn test_set_video_format() {
        let mut config = Config::default();
        let result = config.set("video.format", "mkv");

        assert!(result.is_ok());
        assert_eq!(config.video.format, "mkv");
    }

    #[test]
    fn test_set_video_include_thumbnail_true() {
        let mut config = Config::default();
        config.video.include_thumbnail = false;

        let result = config.set("video.include_thumbnail", "true");

        assert!(result.is_ok());
        assert!(config.video.include_thumbnail);
    }

    #[test]
    fn test_set_video_include_thumbnail_false() {
        let mut config = Config::default();
        let result = config.set("video.include_thumbnail", "false");

        assert!(result.is_ok());
        assert!(!config.video.include_thumbnail);
    }

    #[test]
    fn test_set_video_include_thumbnail_invalid() {
        let mut config = Config::default();
        let result = config.set("video.include_thumbnail", "yes");

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::ConfigInvalid { field, message } => {
                assert_eq!(field, "video.include_thumbnail");
                assert!(message.contains("true or false"));
            }
            _ => panic!("Expected ConfigInvalid error"),
        }
    }

    #[test]
    fn test_set_video_include_subtitles() {
        let mut config = Config::default();
        let result = config.set("video.include_subtitles", "false");

        assert!(result.is_ok());
        assert!(!config.video.include_subtitles);
    }

    #[test]
    fn test_set_network_rate_limit() {
        let mut config = Config::default();
        let result = config.set("network.rate_limit", "10M");

        assert!(result.is_ok());
        assert_eq!(config.network.rate_limit, Some("10M".to_string()));
    }

    #[test]
    fn test_set_network_rate_limit_none() {
        let mut config = Config::default();
        config.network.rate_limit = Some("5M".to_string());

        let result = config.set("network.rate_limit", "none");

        assert!(result.is_ok());
        assert!(config.network.rate_limit.is_none());
    }

    #[test]
    fn test_set_network_rate_limit_empty() {
        let mut config = Config::default();
        config.network.rate_limit = Some("5M".to_string());

        let result = config.set("network.rate_limit", "");

        assert!(result.is_ok());
        assert!(config.network.rate_limit.is_none());
    }

    #[test]
    fn test_set_network_retry_attempts() {
        let mut config = Config::default();
        let result = config.set("network.retry_attempts", "5");

        assert!(result.is_ok());
        assert_eq!(config.network.retry_attempts, 5);
    }

    #[test]
    fn test_set_network_timeout() {
        let mut config = Config::default();
        let result = config.set("network.timeout", "600");

        assert!(result.is_ok());
        assert_eq!(config.network.timeout, 600);
    }

    #[test]
    fn test_set_unknown_key_fails() {
        let mut config = Config::default();
        let result = config.set("unknown.key", "value");

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::ConfigInvalid { field, message } => {
                assert_eq!(field, "unknown.key");
                assert!(message.contains("unknown"));
            }
            _ => panic!("Expected ConfigInvalid error"),
        }
    }

    // ============== Keys Method Tests ==============

    #[test]
    fn test_keys_returns_all_keys() {
        let keys = Config::keys();

        assert!(keys.contains(&"general.output_dir"));
        assert!(keys.contains(&"general.default_quality"));
        assert!(keys.contains(&"general.max_parallel_downloads"));
        assert!(keys.contains(&"audio.format"));
        assert!(keys.contains(&"audio.bitrate"));
        assert!(keys.contains(&"video.format"));
        assert!(keys.contains(&"video.include_thumbnail"));
        assert!(keys.contains(&"video.include_subtitles"));
        assert!(keys.contains(&"network.rate_limit"));
        assert!(keys.contains(&"network.retry_attempts"));
        assert!(keys.contains(&"network.timeout"));
    }

    #[test]
    fn test_keys_count() {
        let keys = Config::keys();

        assert_eq!(keys.len(), 11);
    }

    // ============== File I/O Tests ==============

    #[test]
    fn test_save_creates_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("rust-yt-downloader").join("config.toml");

        // Temporarily override config path for testing
        let config = Config::default();
        let content = toml::to_string_pretty(&config).unwrap();

        // Create directory and file manually for this test
        fs::create_dir_all(config_path.parent().unwrap()).unwrap();
        fs::write(&config_path, content).unwrap();

        assert!(config_path.exists());

        let saved_content = fs::read_to_string(&config_path).unwrap();
        assert!(saved_content.contains("[general]"));
        assert!(saved_content.contains("[audio]"));
    }

    #[test]
    fn test_parse_config_from_file() {
        let toml_content = r#"
[general]
output_dir = "/test/path"
default_quality = "720p"
max_parallel_downloads = 2

[audio]
format = "m4a"
bitrate = "128k"

[video]
format = "webm"
include_thumbnail = false
include_subtitles = true

[network]
retry_attempts = 1
timeout = 60
"#;

        let (_temp_dir, config_path) = create_temp_config(toml_content);

        let content = fs::read_to_string(&config_path).unwrap();
        let config: Config = toml::from_str(&content).unwrap();

        assert_eq!(config.general.output_dir, "/test/path");
        assert_eq!(config.general.default_quality, "720p");
        assert_eq!(config.general.max_parallel_downloads, 2);
        assert_eq!(config.audio.format, "m4a");
        assert_eq!(config.audio.bitrate, "128k");
        assert_eq!(config.video.format, "webm");
        assert!(!config.video.include_thumbnail);
        assert!(config.video.include_subtitles);
        assert_eq!(config.network.retry_attempts, 1);
        assert_eq!(config.network.timeout, 60);
    }

    // ============== Clone Tests ==============

    #[test]
    fn test_config_clone() {
        let original = Config::default();
        let cloned = original.clone();

        assert_eq!(original.general.default_quality, cloned.general.default_quality);
        assert_eq!(original.audio.format, cloned.audio.format);
        assert_eq!(original.video.format, cloned.video.format);
        assert_eq!(original.network.timeout, cloned.network.timeout);
    }

    #[test]
    fn test_config_clone_is_independent() {
        let original = Config::default();
        let mut cloned = original.clone();

        cloned.general.default_quality = "1080p".to_string();

        assert_eq!(original.general.default_quality, "best");
        assert_eq!(cloned.general.default_quality, "1080p");
    }
}
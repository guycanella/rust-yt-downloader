//! Command-line interface definitions and argument parsing.
//!
//! This module defines the complete CLI structure for the YouTube downloader using `clap`.
//! It provides a declarative, type-safe interface for all commands, arguments, and options.
//!
//! # Architecture
//!
//! The CLI uses a hierarchical command structure:
//! - **Main commands**: `download`, `audio`, `playlist`, `info`, `config`
//! - **Subcommands**: Only `config` has subcommands (`show`, `set`, `get`, `reset`, `path`)
//! - **Common args**: Shared arguments are grouped in `CommonArgs` and flattened into commands
//!
//! # Examples
//!
//! ```rust,no_run
//! use clap::Parser;
//! use rust_yt_downloader::cli::Cli;
//!
//! let cli = Cli::parse();
//! match cli.command {
//!     Commands::Download(args) => {
//!         println!("Downloading: {}", args.common.url);
//!     }
//!     _ => {}
//! }
//! ```

use std::path::PathBuf;
use clap::{Parser, Subcommand, Args, ValueEnum};

/// Main CLI structure for the YouTube downloader application.
///
/// This is the entry point for command-line argument parsing. All commands
/// are defined as variants in the [`Commands`] enum.
#[derive(Parser)]
#[command(name = "ytdl")]
#[command(version, about = "A professional CLI tool for downloading YouTube videos")]
pub struct Cli {
    /// The subcommand to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands for the YouTube downloader.
///
/// Each variant represents a top-level command with its associated arguments.
/// Commands are executed based on the user's input from the command line.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Download a YouTube video with specified quality and format
    ///
    /// # Examples
    ///
    /// ```bash
    /// ytdl download https://youtube.com/watch?v=abc123
    /// ytdl download https://youtube.com/watch?v=abc123 -q 1080p -f mkv
    /// ```
    Download(DownloadArgs),

    /// Extract and download only audio from a YouTube video
    ///
    /// # Examples
    ///
    /// ```bash
    /// ytdl audio https://youtube.com/watch?v=abc123
    /// ytdl audio https://youtube.com/watch?v=abc123 -f flac
    /// ```
    Audio(AudioArgs),

    /// Download multiple videos from YouTube playlists
    ///
    /// # Examples
    ///
    /// ```bash
    /// ytdl playlist https://youtube.com/playlist?list=PL123
    /// ytdl playlist url1 url2 url3 --audio-only
    /// ```
    Playlist(PlaylistArgs),

    /// Display information about a YouTube video without downloading
    ///
    /// Shows metadata like title, duration, available formats, and qualities.
    Info(InfoArgs),

    /// Manage application configuration
    ///
    /// Allows viewing and modifying the configuration file located at
    /// `~/.config/rust-yt-downloader/config.toml`.
    Config{
        #[command(subcommand)]
        command: ConfigCommands,
    }
}

/// Configuration subcommands for managing application settings.
///
/// These commands interact with the TOML configuration file to persist
/// user preferences across sessions.
#[derive(Subcommand, Clone, Debug)]
pub enum ConfigCommands {
    /// Display all current configuration settings
    Show,

    /// Set a configuration value
    ///
    /// Uses dot notation for nested keys (e.g., `general.output_dir`).
    Set {
        /// Configuration key in dot notation (e.g., "general.default_quality")
        key: String,
        /// Value to set
        value: String
    },

    /// Get a specific configuration value
    Get {
        /// Configuration key in dot notation
        key: String
    },

    /// Reset configuration to default values
    Reset,

    /// Show the path to the configuration file
    Path
}

/// Video quality options for downloads.
///
/// Quality selection allows users to choose the resolution of downloaded videos.
/// The actual availability depends on what YouTube provides for each video.
///
/// # Default
///
/// The default quality is [`VideoQuality::Best`], which selects the highest
/// available quality.
#[derive(ValueEnum, Clone, Debug, Default)]
pub enum VideoQuality {
    /// 144p resolution (lowest quality, smallest file size)
    #[value(name = "144p")]
    Q144p,

    /// 240p resolution
    #[value(name = "240p")]
    Q240p,

    /// 360p resolution (standard definition)
    #[value(name = "360p")]
    Q360p,

    /// 480p resolution (standard definition)
    #[value(name = "480p")]
    Q480p,

    /// 720p resolution (HD)
    #[value(name = "720p")]
    Q720p,

    /// 1080p resolution (Full HD)
    #[value(name = "1080p")]
    Q1080p,

    /// 1440p resolution (2K)
    #[value(name = "1440p")]
    Q1440p,

    /// 4K resolution (2160p, highest quality)
    #[value(name = "4k")]
    Q4k,

    /// Automatically select the best available quality (default)
    #[default]
    Best,

    /// Automatically select the worst available quality (smallest file size)
    Worst
}

/// Audio format options for audio extraction.
///
/// When downloading audio-only or extracting audio from videos,
/// these formats are available. Conversion is performed using FFmpeg.
///
/// # Default
///
/// The default format is [`AudioFormat::Mp3`].
#[derive(ValueEnum, Clone, Debug, Default)]
pub enum AudioFormat {
    /// MP3 format (lossy compression, widely compatible)
    #[default]
    Mp3,

    /// M4A/AAC format (lossy compression, good quality)
    M4a,

    /// FLAC format (lossless compression, best quality)
    Flac,

    /// WAV format (uncompressed, largest file size)
    Wav,

    /// Opus format (modern lossy codec, good quality at low bitrates)
    Opus
}

/// Video container format options.
///
/// These are the container formats for video downloads. The actual
/// video and audio codecs inside may vary depending on YouTube's source.
///
/// # Default
///
/// The default format is [`VideoFormat::Mp4`].
#[derive(ValueEnum, Clone, Debug, Default)]
pub enum VideoFormat {
    /// MP4 container (widely compatible, recommended)
    #[default]
    Mp4,

    /// Matroska container (supports more codecs and features)
    Mkv,

    /// WebM container (open format, good for web)
    Webm
}

/// Common arguments shared across multiple commands.
///
/// These arguments are flattened into command structs to avoid repetition
/// and ensure consistent behavior across download, audio, and other commands.
#[derive(Args, Debug)]
pub struct CommonArgs {
    /// YouTube video URL to process
    ///
    /// Accepts standard YouTube URLs in various formats:
    /// - `https://youtube.com/watch?v=VIDEO_ID`
    /// - `https://youtu.be/VIDEO_ID`
    /// - `https://www.youtube.com/watch?v=VIDEO_ID`
    pub url: String,

    /// Output directory for downloaded files
    ///
    /// Defaults to current directory (`.`). Supports tilde expansion for home directory.
    #[arg(short = 'o', long, default_value = ".")]
    pub output: PathBuf,

    /// Suppress progress bars and non-error output
    ///
    /// Useful for scripting or when running in non-interactive environments.
    #[arg(short = 's', long, default_value_t = false)]
    pub silence: bool,

    /// Enable verbose logging output
    ///
    /// Shows detailed information about the download process, HTTP requests,
    /// and FFmpeg operations.
    #[arg(short = 'v', long, default_value_t = false)]
    pub verbose: bool,
}

/// Arguments for the `download` command.
///
/// Downloads a complete YouTube video with both video and audio streams.
/// Allows selection of quality and output format.
#[derive(Args, Debug)]
pub struct DownloadArgs {
    /// Common arguments (URL, output directory, verbosity flags)
    #[command(flatten)]
    pub common: CommonArgs,

    /// Video quality/resolution to download
    ///
    /// If the requested quality is not available, the closest match will be selected.
    #[arg(short = 'q', long, value_enum, default_value_t = VideoQuality::Best)]
    pub quality: VideoQuality,

    /// Video container format
    ///
    /// Determines the output file container. May require format conversion via FFmpeg.
    #[arg(short = 'f', long, value_enum, default_value_t = VideoFormat::Mp4)]
    pub format: VideoFormat,
}

/// Arguments for the `audio` command.
///
/// Extracts and downloads only the audio stream from a YouTube video.
/// The audio is converted to the specified format using FFmpeg.
#[derive(Args, Debug)]
pub struct AudioArgs {
    /// Common arguments (URL, output directory, verbosity flags)
    #[command(flatten)]
    pub common: CommonArgs,

    /// Audio format for the extracted audio
    ///
    /// All audio is extracted and converted using FFmpeg to ensure consistent quality.
    #[arg(short = 'f', long, value_enum, default_value_t = AudioFormat::Mp3)]
    pub format: AudioFormat,
}

/// Arguments for the `playlist` command.
///
/// Downloads multiple videos from one or more YouTube playlists.
/// Supports both video and audio-only modes.
#[derive(Args, Debug)]
pub struct PlaylistArgs {
    /// One or more playlist URLs to download
    ///
    /// Accepts standard YouTube playlist URLs:
    /// - `https://youtube.com/playlist?list=PLAYLIST_ID`
    #[arg(required = true, num_args = 1..)]
    pub urls: Vec<String>,

    /// Output directory for downloaded files
    ///
    /// All playlist videos will be saved to this directory.
    #[arg(short = 'o', long, default_value = ".")]
    pub output: PathBuf,

    /// Video quality/resolution for video downloads
    #[arg(short = 'q', long, value_enum, default_value_t = VideoQuality::Best)]
    pub quality: VideoQuality,

    /// Video container format for video downloads
    #[arg(short = 'f', long, value_enum, default_value_t = VideoFormat::Mp4)]
    pub format: VideoFormat,

    /// Download only audio from playlist videos
    ///
    /// When enabled, extracts audio instead of downloading full videos.
    #[arg(long, default_value_t = false)]
    pub audio_only: bool,

    /// Audio format when `--audio-only` is enabled
    #[arg(long, value_enum, default_value_t = AudioFormat::Mp3)]
    pub audio_format: AudioFormat,

    /// Suppress progress bars and non-error output
    #[arg(short = 's', long, default_value_t = false)]
    pub silence: bool,

    /// Enable verbose logging output
    #[arg(short = 'v', long, default_value_t = false)]
    pub verbose: bool,
}

/// Arguments for the `info` command.
///
/// Retrieves and displays metadata about a YouTube video without downloading it.
/// Useful for inspecting available formats, qualities, and video information.
#[derive(Args, Debug)]
pub struct InfoArgs {
    /// YouTube video URL to retrieve information about
    pub url: String,
}

/// Arguments for the `config` command (deprecated in favor of ConfigCommands).
///
/// This struct exists for backward compatibility but is not currently used.
/// Use [`ConfigCommands`] instead for configuration management.
#[derive(Args, Debug)]
pub struct ConfigArgs {
    /// Set a configuration value (deprecated)
    #[arg(short = 's', long)]
    pub set: Option<String>,

    /// Get a configuration value (deprecated)
    #[arg(short = 'g', long)]
    pub get: Option<String>,
}

// ==================================================
//          UNITARY TESTS
// ==================================================

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    // ============== Download Command Tests ==============

    #[test]
    fn test_download_minimal_args() {
        let cli = Cli::try_parse_from(["ytdl", "download", "https://youtube.com/watch?v=abc123"]);
        assert!(cli.is_ok());

        match cli.unwrap().command {
            Commands::Download(args) => {
                assert_eq!(args.common.url, "https://youtube.com/watch?v=abc123");
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_download_default_values() {
        let cli =
            Cli::try_parse_from(["ytdl", "download", "https://youtube.com/watch?v=abc123"]).unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(matches!(args.quality, VideoQuality::Best));
                assert!(matches!(args.format, VideoFormat::Mp4));
                assert_eq!(args.common.output, PathBuf::from("."));
                assert!(!args.common.silence);
                assert!(!args.common.verbose);
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_download_with_quality() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-q",
            "1080p",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(matches!(args.quality, VideoQuality::Q1080p));
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_download_with_quality_long_flag() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "--quality",
            "720p",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(matches!(args.quality, VideoQuality::Q720p));
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_download_with_format() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-f",
            "mkv",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(matches!(args.format, VideoFormat::Mkv));
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_download_with_output() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-o",
            "/home/user/videos",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert_eq!(args.common.output, PathBuf::from("/home/user/videos"));
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_download_with_all_args() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-q",
            "1080p",
            "-f",
            "mkv",
            "-o",
            "/downloads",
            "-s",
            "-v",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert_eq!(args.common.url, "https://youtube.com/watch?v=abc123");
                assert!(matches!(args.quality, VideoQuality::Q1080p));
                assert!(matches!(args.format, VideoFormat::Mkv));
                assert_eq!(args.common.output, PathBuf::from("/downloads"));
                assert!(args.common.silence);
                assert!(args.common.verbose);
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_download_missing_url_fails() {
        let result = Cli::try_parse_from(["ytdl", "download"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_download_invalid_quality_fails() {
        let result = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-q",
            "9999p",
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn test_download_invalid_format_fails() {
        let result = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-f",
            "avi",
        ]);
        assert!(result.is_err());
    }

    // ============== Audio Command Tests ==============

    #[test]
    fn test_audio_minimal_args() {
        let cli = Cli::try_parse_from(["ytdl", "audio", "https://youtube.com/watch?v=abc123"]);
        assert!(cli.is_ok());

        match cli.unwrap().command {
            Commands::Audio(args) => {
                assert_eq!(args.common.url, "https://youtube.com/watch?v=abc123");
            }
            _ => panic!("Expected Audio command"),
        }
    }

    #[test]
    fn test_audio_default_format() {
        let cli =
            Cli::try_parse_from(["ytdl", "audio", "https://youtube.com/watch?v=abc123"]).unwrap();

        match cli.command {
            Commands::Audio(args) => {
                assert!(matches!(args.format, AudioFormat::Mp3));
            }
            _ => panic!("Expected Audio command"),
        }
    }

    #[test]
    fn test_audio_with_format_mp3() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "audio",
            "https://youtube.com/watch?v=abc123",
            "-f",
            "mp3",
        ])
        .unwrap();

        match cli.command {
            Commands::Audio(args) => {
                assert!(matches!(args.format, AudioFormat::Mp3));
            }
            _ => panic!("Expected Audio command"),
        }
    }

    #[test]
    fn test_audio_with_format_flac() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "audio",
            "https://youtube.com/watch?v=abc123",
            "-f",
            "flac",
        ])
        .unwrap();

        match cli.command {
            Commands::Audio(args) => {
                assert!(matches!(args.format, AudioFormat::Flac));
            }
            _ => panic!("Expected Audio command"),
        }
    }

    #[test]
    fn test_audio_with_format_m4a() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "audio",
            "https://youtube.com/watch?v=abc123",
            "-f",
            "m4a",
        ])
        .unwrap();

        match cli.command {
            Commands::Audio(args) => {
                assert!(matches!(args.format, AudioFormat::M4a));
            }
            _ => panic!("Expected Audio command"),
        }
    }

    #[test]
    fn test_audio_with_format_wav() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "audio",
            "https://youtube.com/watch?v=abc123",
            "-f",
            "wav",
        ])
        .unwrap();

        match cli.command {
            Commands::Audio(args) => {
                assert!(matches!(args.format, AudioFormat::Wav));
            }
            _ => panic!("Expected Audio command"),
        }
    }

    #[test]
    fn test_audio_with_format_opus() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "audio",
            "https://youtube.com/watch?v=abc123",
            "-f",
            "opus",
        ])
        .unwrap();

        match cli.command {
            Commands::Audio(args) => {
                assert!(matches!(args.format, AudioFormat::Opus));
            }
            _ => panic!("Expected Audio command"),
        }
    }

    #[test]
    fn test_audio_missing_url_fails() {
        let result = Cli::try_parse_from(["ytdl", "audio"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_audio_invalid_format_fails() {
        let result = Cli::try_parse_from([
            "ytdl",
            "audio",
            "https://youtube.com/watch?v=abc123",
            "-f",
            "wma",
        ]);
        assert!(result.is_err());
    }

    // ============== Playlist Command Tests ==============

    #[test]
    fn test_playlist_single_url() {
        let cli =
            Cli::try_parse_from(["ytdl", "playlist", "https://youtube.com/playlist?list=PL123"]);
        assert!(cli.is_ok());

        match cli.unwrap().command {
            Commands::Playlist(args) => {
                assert_eq!(args.urls.len(), 1);
                assert_eq!(args.urls[0], "https://youtube.com/playlist?list=PL123");
            }
            _ => panic!("Expected Playlist command"),
        }
    }

    #[test]
    fn test_playlist_multiple_urls() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "playlist",
            "https://youtube.com/playlist?list=PL123",
            "https://youtube.com/playlist?list=PL456",
            "https://youtube.com/playlist?list=PL789",
        ])
        .unwrap();

        match cli.command {
            Commands::Playlist(args) => {
                assert_eq!(args.urls.len(), 3);
                assert_eq!(args.urls[0], "https://youtube.com/playlist?list=PL123");
                assert_eq!(args.urls[1], "https://youtube.com/playlist?list=PL456");
                assert_eq!(args.urls[2], "https://youtube.com/playlist?list=PL789");
            }
            _ => panic!("Expected Playlist command"),
        }
    }

    #[test]
    fn test_playlist_default_values() {
        let cli =
            Cli::try_parse_from(["ytdl", "playlist", "https://youtube.com/playlist?list=PL123"])
                .unwrap();

        match cli.command {
            Commands::Playlist(args) => {
                assert!(matches!(args.quality, VideoQuality::Best));
                assert!(matches!(args.format, VideoFormat::Mp4));
                assert!(!args.audio_only);
                assert!(matches!(args.audio_format, AudioFormat::Mp3));
                assert_eq!(args.output, PathBuf::from("."));
            }
            _ => panic!("Expected Playlist command"),
        }
    }

    #[test]
    fn test_playlist_with_audio_only() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "playlist",
            "https://youtube.com/playlist?list=PL123",
            "--audio-only",
        ])
        .unwrap();

        match cli.command {
            Commands::Playlist(args) => {
                assert!(args.audio_only);
            }
            _ => panic!("Expected Playlist command"),
        }
    }

    #[test]
    fn test_playlist_with_audio_only_and_format() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "playlist",
            "https://youtube.com/playlist?list=PL123",
            "--audio-only",
            "--audio-format",
            "flac",
        ])
        .unwrap();

        match cli.command {
            Commands::Playlist(args) => {
                assert!(args.audio_only);
                assert!(matches!(args.audio_format, AudioFormat::Flac));
            }
            _ => panic!("Expected Playlist command"),
        }
    }

    #[test]
    fn test_playlist_with_quality() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "playlist",
            "https://youtube.com/playlist?list=PL123",
            "-q",
            "480p",
        ])
        .unwrap();

        match cli.command {
            Commands::Playlist(args) => {
                assert!(matches!(args.quality, VideoQuality::Q480p));
            }
            _ => panic!("Expected Playlist command"),
        }
    }

    #[test]
    fn test_playlist_missing_url_fails() {
        let result = Cli::try_parse_from(["ytdl", "playlist"]);
        assert!(result.is_err());
    }

    // ============== Info Command Tests ==============

    #[test]
    fn test_info_command() {
        let cli = Cli::try_parse_from(["ytdl", "info", "https://youtube.com/watch?v=abc123"]);
        assert!(cli.is_ok());

        match cli.unwrap().command {
            Commands::Info(args) => {
                assert_eq!(args.url, "https://youtube.com/watch?v=abc123");
            }
            _ => panic!("Expected Info command"),
        }
    }

    #[test]
    fn test_info_missing_url_fails() {
        let result = Cli::try_parse_from(["ytdl", "info"]);
        assert!(result.is_err());
    }

    // ============== Config Command Tests ==============

    #[test]
    fn test_config_show() {
        let cli = Cli::try_parse_from(["ytdl", "config", "show"]);
        assert!(cli.is_ok());

        match cli.unwrap().command {
            Commands::Config { command } => {
                assert!(matches!(command, ConfigCommands::Show));
            }
            _ => panic!("Expected Config command"),
        }
    }

    #[test]
    fn test_config_path() {
        let cli = Cli::try_parse_from(["ytdl", "config", "path"]);
        assert!(cli.is_ok());

        match cli.unwrap().command {
            Commands::Config { command } => {
                assert!(matches!(command, ConfigCommands::Path));
            }
            _ => panic!("Expected Config command"),
        }
    }

    #[test]
    fn test_config_reset() {
        let cli = Cli::try_parse_from(["ytdl", "config", "reset"]);
        assert!(cli.is_ok());

        match cli.unwrap().command {
            Commands::Config { command } => {
                assert!(matches!(command, ConfigCommands::Reset));
            }
            _ => panic!("Expected Config command"),
        }
    }

    #[test]
    fn test_config_set() {
        let cli = Cli::try_parse_from(["ytdl", "config", "set", "default_quality", "1080p"]);
        assert!(cli.is_ok());

        match cli.unwrap().command {
            Commands::Config { command } => match command {
                ConfigCommands::Set { key, value } => {
                    assert_eq!(key, "default_quality");
                    assert_eq!(value, "1080p");
                }
                _ => panic!("Expected Set subcommand"),
            },
            _ => panic!("Expected Config command"),
        }
    }

    #[test]
    fn test_config_get() {
        let cli = Cli::try_parse_from(["ytdl", "config", "get", "output_dir"]);
        assert!(cli.is_ok());

        match cli.unwrap().command {
            Commands::Config { command } => match command {
                ConfigCommands::Get { key } => {
                    assert_eq!(key, "output_dir");
                }
                _ => panic!("Expected Get subcommand"),
            },
            _ => panic!("Expected Config command"),
        }
    }

    #[test]
    fn test_config_set_missing_value_fails() {
        let result = Cli::try_parse_from(["ytdl", "config", "set", "default_quality"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_get_missing_key_fails() {
        let result = Cli::try_parse_from(["ytdl", "config", "get"]);
        assert!(result.is_err());
    }

    // ============== VideoQuality Enum Tests ==============

    #[test]
    fn test_quality_144p() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-q",
            "144p",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(matches!(args.quality, VideoQuality::Q144p));
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_quality_240p() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-q",
            "240p",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(matches!(args.quality, VideoQuality::Q240p));
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_quality_360p() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-q",
            "360p",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(matches!(args.quality, VideoQuality::Q360p));
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_quality_480p() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-q",
            "480p",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(matches!(args.quality, VideoQuality::Q480p));
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_quality_720p() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-q",
            "720p",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(matches!(args.quality, VideoQuality::Q720p));
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_quality_1080p() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-q",
            "1080p",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(matches!(args.quality, VideoQuality::Q1080p));
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_quality_1440p() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-q",
            "1440p",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(matches!(args.quality, VideoQuality::Q1440p));
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_quality_4k() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-q",
            "4k",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(matches!(args.quality, VideoQuality::Q4k));
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_quality_best() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-q",
            "best",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(matches!(args.quality, VideoQuality::Best));
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_quality_worst() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-q",
            "worst",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(matches!(args.quality, VideoQuality::Worst));
            }
            _ => panic!("Expected Download command"),
        }
    }

    // ============== VideoFormat Enum Tests ==============

    #[test]
    fn test_video_format_mp4() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-f",
            "mp4",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(matches!(args.format, VideoFormat::Mp4));
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_video_format_mkv() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-f",
            "mkv",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(matches!(args.format, VideoFormat::Mkv));
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_video_format_webm() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-f",
            "webm",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(matches!(args.format, VideoFormat::Webm));
            }
            _ => panic!("Expected Download command"),
        }
    }

    // ============== Flags Tests ==============

    #[test]
    fn test_silence_flag_short() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-s",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(args.common.silence);
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_silence_flag_long() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "--silence",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(args.common.silence);
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_verbose_flag_short() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "-v",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(args.common.verbose);
            }
            _ => panic!("Expected Download command"),
        }
    }

    #[test]
    fn test_verbose_flag_long() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "--verbose",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert!(args.common.verbose);
            }
            _ => panic!("Expected Download command"),
        }
    }

    // ============== Edge Cases ==============

    #[test]
    fn test_unknown_command_fails() {
        let result = Cli::try_parse_from(["ytdl", "unknown"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_no_command_fails() {
        let result = Cli::try_parse_from(["ytdl"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_output_long_flag() {
        let cli = Cli::try_parse_from([
            "ytdl",
            "download",
            "https://youtube.com/watch?v=abc123",
            "--output",
            "/tmp/videos",
        ])
        .unwrap();

        match cli.command {
            Commands::Download(args) => {
                assert_eq!(args.common.output, PathBuf::from("/tmp/videos"));
            }
            _ => panic!("Expected Download command"),
        }
    }
}
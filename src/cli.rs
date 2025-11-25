use std::path::PathBuf;
use clap::{Parser, Subcommand, Args, ValueEnum};

#[derive(Parser)]
#[command(name = "ytdl")]
#[command(version, about = "A professional CLI tool for downloading YouTube videos")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Download(DownloadArgs),
    Audio(AudioArgs),
    Playlist(PlaylistArgs),
    Info(InfoArgs),
    Config{
        #[command(subcommand)]
        command: ConfigCommands,
    }
}

#[derive(Subcommand, Clone, Debug)]
pub enum ConfigCommands {
    Show,
    Set { key: String, value: String },
    Get { key: String },
    Reset,
    Path
}

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum VideoQuality {
    #[value(name = "144p")]
    Q144p,
    #[value(name = "240p")]
    Q240p,
    #[value(name = "360p")]
    Q360p,
    #[value(name = "480p")]
    Q480p,
    #[value(name = "720p")]
    Q720p,
    #[value(name = "1080p")]
    Q1080p,
    #[value(name = "1440p")]
    Q1440p,
    #[value(name = "4k")]
    Q4k,
    #[default]
    Best,
    Worst
}

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum AudioFormat {
    #[default]
    Mp3,
    M4a,
    Flac,
    Wav,
    Opus
}

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum VideoFormat {
    #[default]
    Mp4,
    Mkv,
    Webm
}

#[derive(Args, Debug)]
pub struct CommonArgs {
    pub url: String,

    #[arg(short = 'o', long, default_value = ".")]
    pub output: PathBuf,

    #[arg(short = 's', long, default_value_t = false)]
    pub silence: bool,

    #[arg(short = 'v', long, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Args, Debug)]
pub struct DownloadArgs {
    #[command(flatten)]
    pub common: CommonArgs,

    #[arg(short = 'q', long, value_enum, default_value_t = VideoQuality::Best)]
    pub quality: VideoQuality,

    #[arg(short = 'f', long, value_enum, default_value_t = VideoFormat::Mp4)]
    pub format: VideoFormat,
}

#[derive(Args, Debug)]
pub struct AudioArgs {
    #[command(flatten)]
    pub common: CommonArgs,

    #[arg(short = 'f', long, value_enum, default_value_t = AudioFormat::Mp3)]
    pub format: AudioFormat,
}

#[derive(Args, Debug)]
pub struct PlaylistArgs {
    #[arg(required = true, num_args = 1..)]
    pub urls: Vec<String>,

    #[arg(short = 'o', long, default_value = ".")]
    pub output: PathBuf,

    #[arg(short = 'q', long, value_enum, default_value_t = VideoQuality::Best)]
    pub quality: VideoQuality,

    #[arg(short = 'f', long, value_enum, default_value_t = VideoFormat::Mp4)]
    pub format: VideoFormat,

    #[arg(long, default_value_t = false)]
    pub audio_only: bool,

    #[arg(long, value_enum, default_value_t = AudioFormat::Mp3)]
    pub audio_format: AudioFormat,

    #[arg(short = 's', long, default_value_t = false)]
    pub silence: bool,

    #[arg(short = 'v', long, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Args, Debug)]
pub struct InfoArgs {
    pub url: String,
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[arg(short = 's', long)]
    pub set: Option<String>,

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
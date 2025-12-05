//! YouTube Downloader - A professional CLI tool for downloading YouTube videos.
//!
//! This application provides a feature-rich command-line interface for downloading
//! YouTube videos and audio with support for quality selection, format conversion,
//! playlists, and extensive configuration options.
//!
//! # Features
//!
//! - **Video Downloads**: Download YouTube videos in various qualities (144p to 4K)
//! - **Audio Extraction**: Extract audio in multiple formats (MP3, FLAC, M4A, WAV, Opus)
//! - **Playlist Support**: Download entire playlists or multiple videos
//! - **Configuration Management**: Persistent configuration file for default settings
//! - **Progress Tracking**: Real-time progress bars with download speed and ETA
//! - **Format Conversion**: Convert between video formats using FFmpeg
//! - **Video Information**: Display metadata without downloading
//!
//! # Usage
//!
//! ```bash
//! # Download a video
//! ytdl download https://youtube.com/watch?v=VIDEO_ID
//!
//! # Extract audio only
//! ytdl audio https://youtube.com/watch?v=VIDEO_ID -f mp3
//!
//! # Download a playlist
//! ytdl playlist https://youtube.com/playlist?list=PLAYLIST_ID
//!
//! # Show video information
//! ytdl info https://youtube.com/watch?v=VIDEO_ID
//!
//! # Manage configuration
//! ytdl config show
//! ytdl config set general.default_quality 1080p
//! ```
//!
//! # Architecture
//!
//! The application is organized into focused modules:
//!
//! - **`cli`**: Command-line argument parsing using `clap`
//! - **`config`**: Configuration file management (TOML format)
//! - **`downloader`**: Download orchestration and coordination
//! - **`error`**: Centralized error handling with retryable logic
//! - **`media`**: FFmpeg integration for audio/video processing
//! - **`progress`**: Progress bars and user feedback via `indicatif`
//! - **`utils`**: Common utilities (filename sanitization, path expansion, etc.)
//! - **`youtube`**: YouTube API integration using `yt-dlp`
//!
//! # External Dependencies
//!
//! - **FFmpeg**: Required for audio extraction and format conversion
//! - **yt-dlp**: Used internally for YouTube API access

mod cli;
mod config;
mod downloader;
mod error;
mod media;
mod progress;
mod utils;
mod youtube;

use std::path::PathBuf;

use clap::Parser;
use colored::Colorize;

use cli::{Cli, Commands, ConfigCommands};
use config::Config;
use downloader::{DownloadOptions, Downloader};
use error::{AppError, AppResult};
use progress::messages;
use youtube::YtDlpClient;

/// Application entry point.
///
/// Parses command-line arguments and delegates to the appropriate handler.
/// All errors are caught and displayed with colored output before exiting.
#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        messages::error(&format!("{}", e));
        std::process::exit(1);
    }
}

/// Main application logic coordinating command execution.
///
/// Parses CLI arguments and routes to the appropriate command handler.
///
/// # Errors
///
/// Returns an error if any command handler fails.
async fn run() -> AppResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Download(args) => {
            handle_download(
                &args.common.url,
                args.common.output,
                args.quality,
                args.format,
                args.common.silence,
                args.common.verbose,
            )
            .await?;
        }

        Commands::Audio(args) => {
            handle_audio(
                &args.common.url,
                args.common.output,
                args.format,
                args.common.silence,
                args.common.verbose,
            )
            .await?;
        }

        Commands::Playlist(args) => {
            handle_playlist(
                &args.urls,
                args.output,
                args.quality,
                args.format,
                args.audio_only,
                args.audio_format,
                args.silence,
                args.verbose,
            )
            .await?;
        }

        Commands::Info(args) => {
            handle_info(&args.url).await?;
        }

        Commands::Config { command } => {
            handle_config(command)?;
        }
    }

    Ok(())
}

/// Handles the `download` command for video downloads.
///
/// Downloads a complete YouTube video with both video and audio streams,
/// merging them into the specified format.
async fn handle_download(
    url: &str,
    output: PathBuf,
    quality: cli::VideoQuality,
    format: cli::VideoFormat,
    silence: bool,
    verbose: bool,
) -> AppResult<()> {
    let config = Config::load()?;

    let options = DownloadOptions::from_config(&config)
        .with_output_dir(output)
        .with_quality(quality)
        .with_video_format(format)
        .with_silence(silence)
        .with_verbose(verbose);

    let downloader = Downloader::with_options(options);
    let result = downloader.download(url).await?;

    if !silence {
        println!();
        messages::success(&format!("Downloaded: {}", result.file_path.display()));
        messages::info(&format!("Size: {}", utils::format_bytes(result.file_size)));
    }

    Ok(())
}

/// Handles the `audio` command for audio-only downloads.
///
/// Extracts and downloads only the audio stream, converting it to the
/// specified format using FFmpeg.
async fn handle_audio(
    url: &str,
    output: PathBuf,
    format: cli::AudioFormat,
    silence: bool,
    verbose: bool,
) -> AppResult<()> {
    let config = Config::load()?;

    let options = DownloadOptions::from_config(&config)
        .with_output_dir(output)
        .with_audio_only(true)
        .with_audio_format(format)
        .with_silence(silence)
        .with_verbose(verbose);

    let downloader = Downloader::with_options(options);
    let result = downloader.download_audio(url).await?;

    if !silence {
        println!();
        messages::success(&format!("Downloaded: {}", result.file_path.display()));
        messages::info(&format!("Size: {}", utils::format_bytes(result.file_size)));
    }

    Ok(())
}

/// Handles the `playlist` command for downloading multiple videos.
///
/// Downloads all videos from one or more playlists, with support for
/// both video and audio-only modes. Continues downloading even if some
/// videos fail, reporting a final summary.
async fn handle_playlist(
    urls: &[String],
    output: PathBuf,
    quality: cli::VideoQuality,
    format: cli::VideoFormat,
    audio_only: bool,
    audio_format: cli::AudioFormat,
    silence: bool,
    verbose: bool,
) -> AppResult<()> {
    let config = Config::load()?;

    let options = DownloadOptions::from_config(&config)
        .with_output_dir(output)
        .with_quality(quality)
        .with_video_format(format)
        .with_audio_only(audio_only)
        .with_audio_format(audio_format)
        .with_silence(silence)
        .with_verbose(verbose);

    let downloader = Downloader::with_options(options);

    let mut success_count = 0;
    let mut error_count = 0;

    for (index, url) in urls.iter().enumerate() {
        if !silence {
            messages::info(&format!(
                "Processing playlist {} of {}...",
                index + 1,
                urls.len()
            ));
        }

        let result = if audio_only {
            downloader.download_audio(url).await
        } else {
            downloader.download(url).await
        };

        match result {
            Ok(r) => {
                success_count += 1;
                if verbose {
                    messages::success(&format!("Downloaded: {}", r.file_path.display()));
                }
            }
            Err(e) => {
                error_count += 1;
                if !silence {
                    messages::error(&format!("Failed to download {}: {}", url, e));
                }
            }
        }
    }

    if !silence {
        println!();
        messages::info(&format!(
            "Playlist complete: {} succeeded, {} failed",
            success_count, error_count
        ));
    }

    if error_count > 0 && success_count == 0 {
        return Err(AppError::Other("All downloads failed".to_string()));
    }

    Ok(())
}

/// Handles the `info` command for displaying video metadata.
///
/// Fetches and displays detailed information about a YouTube video
/// including title, duration, channel, views, available qualities, and audio streams.
async fn handle_info(url: &str) -> AppResult<()> {
    let client = YtDlpClient::new();

    messages::info("Fetching video information...");
    println!();

    let video = client.get_video_info(url)?;

    println!("{}: {}", "Title".cyan().bold(), video.title);
    println!("{}: {}", "ID".cyan().bold(), video.id);
    println!(
        "{}: {}",
        "Duration".cyan().bold(),
        utils::format_duration(video.duration)
    );

    if let Some(channel) = &video.channel {
        println!("{}: {}", "Channel".cyan().bold(), channel);
    }

    if let Some(views) = video.view_count {
        println!("{}: {}", "Views".cyan().bold(), format_views(views));
    }

    if let Some(desc) = &video.description {
        let short_desc: String = desc.chars().take(200).collect();
        println!("{}: {}...", "Description".cyan().bold(), short_desc);
    }

    println!();
    println!("{}", "Available Qualities:".yellow().bold());

    let qualities = video.available_qualities();
    if qualities.is_empty() {
        println!("  No video streams available");
    } else {
        for quality in qualities {
            println!("  • {}", quality);
        }
    }

    println!();
    println!("{}", "Audio Streams:".yellow().bold());

    let audio_streams: Vec<_> = video.streams.iter().filter(|s| s.is_audio_only).collect();

    if audio_streams.is_empty() {
        println!("  No audio-only streams available");
    } else {
        for stream in audio_streams {
            let bitrate = stream
                .bitrate
                .map(|b| format!("{}kbps", b / 1000))
                .unwrap_or_else(|| "unknown".to_string());
            println!("  • {} ({})", stream.format, bitrate);
        }
    }

    Ok(())
}

/// Handles the `config` command and its subcommands.
///
/// Manages the application configuration file, supporting operations
/// to show, get, set, reset configuration values, and display the config file path.
fn handle_config(command: ConfigCommands) -> AppResult<()> {
    match command {
        ConfigCommands::Show => {
            let config = Config::load()?;

            println!("{}", "Current Configuration:".yellow().bold());
            println!();

            for key in Config::keys() {
                if let Some(value) = config.get(key) {
                    println!("  {}: {}", key.cyan(), value);
                } else {
                    println!("  {}: {}", key.cyan(), "(not set)".dimmed());
                }
            }
        }

        ConfigCommands::Get { key } => {
            let config = Config::load()?;

            match config.get(&key) {
                Some(value) => println!("{}", value),
                None => {
                    return Err(AppError::ConfigInvalid {
                        field: key,
                        message: "unknown configuration key".to_string(),
                    });
                }
            }
        }

        ConfigCommands::Set { key, value } => {
            let mut config = Config::load()?;
            config.set(&key, &value)?;
            config.save()?;

            messages::success(&format!("{} set to {}", key, value));
        }

        ConfigCommands::Reset => {
            Config::reset()?;
            messages::success("Configuration reset to defaults");
        }

        ConfigCommands::Path => {
            let path = Config::config_path()?;
            println!("{}", path.display());
        }
    }

    Ok(())
}

/// Formats a view count into a human-readable string with K/M/B suffixes.
///
/// # Examples
///
/// - 500 → "500"
/// - 1,500 → "1.5K"
/// - 1,500,000 → "1.5M"
/// - 2,500,000,000 → "2.5B"
fn format_views(views: u64) -> String {
    if views >= 1_000_000_000 {
        format!("{:.1}B", views as f64 / 1_000_000_000.0)
    } else if views >= 1_000_000 {
        format!("{:.1}M", views as f64 / 1_000_000.0)
    } else if views >= 1_000 {
        format!("{:.1}K", views as f64 / 1_000.0)
    } else {
        views.to_string()
    }
}

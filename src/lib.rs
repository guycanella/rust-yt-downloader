//! # rust-yt-downloader
//!
//! A professional CLI tool for downloading YouTube videos and audio.
//!
//! This library provides a complete solution for downloading YouTube content with features including:
//! - High-quality video downloads with quality selection
//! - Audio extraction in multiple formats (MP3, FLAC, M4A, WAV, Opus)
//! - Playlist downloads with parallel processing
//! - FFmpeg integration for format conversion
//! - Progress tracking and user feedback
//! - Comprehensive configuration management
//!
//! # Examples
//!
//! ```no_run
//! use rust_yt_downloader::youtube::YouTubeClient;
//! use rust_yt_downloader::downloader::Downloader;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = YouTubeClient::new();
//! let info = client.get_video_info("https://www.youtube.com/watch?v=dQw4w9WgXcQ").await?;
//!
//! println!("Title: {}", info.title);
//! println!("Duration: {} seconds", info.duration);
//! # Ok(())
//! # }
//! ```
//!
//! # Modules
//!
//! - [`cli`] - Command-line interface and argument parsing
//! - [`config`] - Configuration file management
//! - [`downloader`] - Core download functionality
//! - [`error`] - Error types and handling
//! - [`media`] - FFmpeg integration for media processing
//! - [`progress`] - Progress tracking and display
//! - [`utils`] - Utility functions and helpers
//! - [`youtube`] - YouTube API client and metadata extraction

pub mod cli;
pub mod config;
pub mod downloader;
pub mod error;
pub mod media;
pub mod progress;
pub mod utils;
pub mod youtube;

// Re-export commonly used types for convenience
pub use config::Config;
pub use downloader::Downloader;
pub use error::{AppError, AppResult};
pub use youtube::YouTubeClient;

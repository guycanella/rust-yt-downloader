//! YouTube integration module for video and playlist operations.
//!
//! This module provides a comprehensive interface for interacting with YouTube,
//! supporting both the rustube library and yt-dlp command-line tool. It handles
//! video metadata extraction, playlist processing, URL validation, and stream
//! information retrieval.
//!
//! # Modules
//!
//! - [`metadata`] - Data structures for video and playlist information
//! - [`client`] - YouTube client using the rustube library
//! - [`playlist`] - Playlist URL validation and video ID extraction
//! - [`ytdlp`] - Integration with yt-dlp command-line tool
//!
//! # Examples
//!
//! ```no_run
//! use rust_yt_downloader::youtube::{YouTubeClient, validate_youtube_url};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Validate a YouTube URL
//! validate_youtube_url("https://www.youtube.com/watch?v=dQw4w9WgXcQ")?;
//!
//! // Get video information
//! let client = YouTubeClient::new();
//! let info = client.get_video_info("https://www.youtube.com/watch?v=dQw4w9WgXcQ").await?;
//! println!("Title: {}", info.title);
//! # Ok(())
//! # }
//! ```

pub mod metadata;
pub mod client;
pub mod playlist;
pub mod ytdlp;

pub use metadata::{PlaylistInfo, QualityFilter, StreamInfo, VideoInfo};
pub use client::{YouTubeClient, validate_youtube_url};
pub use playlist::{extract_playlist_ids, filter_valid_playlist_urls, PlaylistClient};
pub use ytdlp::YtDlpClient;
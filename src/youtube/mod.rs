//! YouTube integration module for video and playlist operations.
//!
//! This module provides a comprehensive interface for interacting with YouTube
//! using the yt-dlp command-line tool. It handles video metadata extraction,
//! playlist processing, URL validation, and stream information retrieval.
//!
//! # Modules
//!
//! - [`metadata`] - Data structures for video and playlist information
//! - [`playlist`] - Playlist URL validation and video ID extraction
//! - [`ytdlp`] - Integration with yt-dlp command-line tool (primary client)
//!
//! # Examples
//!
//! ```no_run
//! use rust_yt_downloader::youtube::YtDlpClient;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Get video information using yt-dlp
//! let client = YtDlpClient::new();
//! let info = client.get_video_info("https://www.youtube.com/watch?v=dQw4w9WgXcQ")?;
//! println!("Title: {}", info.title);
//! # Ok(())
//! # }
//! ```

pub mod metadata;
pub mod playlist;
pub mod ytdlp;

pub use metadata::{PlaylistInfo, QualityFilter, StreamInfo, VideoInfo};
pub use playlist::{extract_playlist_ids, filter_valid_playlist_urls, PlaylistClient};
pub use ytdlp::YtDlpClient;

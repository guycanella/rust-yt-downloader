pub mod metadata;
pub mod client;
pub mod playlist;

pub use metadata::{PlaylistInfo, QualityFilter, StreamInfo, VideoInfo};
pub use client::{YouTubeClient, validate_youtube_url};
pub use playlist::{extract_playlist_ids, filter_valid_playlist_urls, PlaylistClient};
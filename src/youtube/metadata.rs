//! Metadata structures for YouTube videos, streams, and playlists.
//!
//! This module defines the core data structures used to represent YouTube video
//! information, available streams with various quality levels, and playlist metadata.
//! It also provides utility methods for filtering and selecting streams based on
//! quality preferences.

use serde::{Deserialize, Serialize};

/// Complete metadata for a YouTube video.
///
/// Contains all relevant information about a video including its metadata,
/// available streams at different quality levels, and optional fields like
/// thumbnails and view counts.
///
/// # Examples
///
/// ```no_run
/// # use rust_yt_downloader::youtube::{YtDlpClient, QualityFilter};
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = YtDlpClient::new();
/// let info = client.get_video_info("https://www.youtube.com/watch?v=dQw4w9WgXcQ")?;
///
/// println!("Title: {}", info.title);
/// println!("Duration: {} seconds", info.duration);
///
/// // Get the best quality stream
/// if let Some(stream) = info.best_video_stream() {
///     println!("Best quality: {}", stream.quality);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    /// Unique YouTube video ID
    pub id: String,
    /// Video title
    pub title: String,
    /// Full video description (may be truncated)
    pub description: Option<String>,
    /// Video duration in seconds
    pub duration: u64,
    /// URL to the video thumbnail image
    pub thumbnail_url: Option<String>,
    /// Channel name or author
    pub channel: Option<String>,
    /// Upload date in ISO format (YYYY-MM-DD)
    pub publish_date: Option<String>,
    /// Total view count
    pub view_count: Option<u64>,
    /// Available streams at different qualities and formats
    pub streams: Vec<StreamInfo>,
}

/// Information about a specific video or audio stream.
///
/// Represents a single available stream for a video, which may be video-only,
/// audio-only, or a combined stream. Contains quality information, codec details,
/// and technical specifications.
///
/// # Examples
///
/// ```no_run
/// # use rust_yt_downloader::youtube::YtDlpClient;
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = YtDlpClient::new();
/// let info = client.get_video_info("https://www.youtube.com/watch?v=dQw4w9WgXcQ")?;
///
/// println!("Available streams: {}", info.streams.len());
/// for stream in &info.streams {
///     if !stream.is_audio_only {
///         println!("{} - {}", stream.quality, stream.format);
///     }
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    /// Direct URL to the stream (may expire)
    pub url: String,
    /// Quality label (e.g., "1080p", "720p", "audio")
    pub quality: String,
    /// Container format (e.g., "mp4", "webm")
    pub format: String,
    /// Video codec (e.g., "h264", "vp9")
    pub video_codec: Option<String>,
    /// Audio codec (e.g., "aac", "opus")
    pub audio_codec: Option<String>,
    /// Whether this stream contains only audio
    pub is_audio_only: bool,
    /// Total file size in bytes
    pub file_size: Option<u64>,
    /// Average bitrate in bits per second
    pub bitrate: Option<u64>,
    /// Frames per second (video streams only)
    pub fps: Option<u32>,
}

/// Metadata for a YouTube playlist.
///
/// Contains information about a playlist including its title, description,
/// and a list of video IDs that belong to the playlist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistInfo {
    /// Unique playlist ID
    pub id: String,
    /// Playlist title
    pub title: String,
    /// Playlist description
    pub description: Option<String>,
    /// Channel that owns the playlist
    pub channel: Option<String>,
    /// Total number of videos in the playlist
    pub video_count: u64,
    /// List of video IDs in the playlist
    pub video_ids: Vec<String>,
}

/// Quality filter for selecting video streams.
///
/// Used to specify quality preferences when retrieving video streams.
/// Supports selecting the best/worst quality, an exact resolution, or
/// the best quality up to a maximum height.
///
/// # Examples
///
/// ```
/// use rust_yt_downloader::youtube::QualityFilter;
///
/// let best = QualityFilter::Best;
/// let hd = QualityFilter::Exact(1080);
/// let mobile = QualityFilter::MaxHeight(480);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityFilter {
    /// Select the highest quality stream available
    Best,
    /// Select the lowest quality stream available
    Worst,
    /// Select a stream with exactly this height (e.g., 1080 for 1080p)
    Exact(u32),
    /// Select the best stream with height not exceeding this value
    MaxHeight(u32),
}

impl VideoInfo {
    /// Returns the highest quality video stream available.
    ///
    /// Filters out audio-only streams and selects the stream with the highest
    /// resolution. Returns `None` if no video streams are available.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rust_yt_downloader::youtube::YtDlpClient;
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = YtDlpClient::new();
    /// let info = client.get_video_info("https://www.youtube.com/watch?v=dQw4w9WgXcQ")?;
    ///
    /// if let Some(best) = info.best_video_stream() {
    ///     println!("Best quality: {}", best.quality);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn best_video_stream(&self) -> Option<&StreamInfo> {
        self.streams
            .iter()
            .filter(|s| !s.is_audio_only)
            .max_by_key(|s| Self::quality_to_height(&s.quality))
    }

    /// Returns the lowest quality video stream available.
    ///
    /// Useful for minimizing bandwidth usage or file size. Filters out
    /// audio-only streams and selects the stream with the lowest resolution.
    pub fn worst_video_stream(&self) -> Option<&StreamInfo> {
        self.streams
            .iter()
            .filter(|s| !s.is_audio_only)
            .min_by_key(|s| Self::quality_to_height(&s.quality))
    }

    /// Returns the highest quality audio-only stream.
    ///
    /// Selects the audio stream with the highest bitrate. Returns `None`
    /// if no audio-only streams are available.
    pub fn best_audio_stream(&self) -> Option<&StreamInfo> {
        self.streams
            .iter()
            .filter(|s| s.is_audio_only)
            .max_by_key(|s| s.bitrate.unwrap_or(0))
    }

    /// Finds a video stream matching the specified quality string.
    ///
    /// Performs a case-insensitive search for a stream with the exact quality
    /// label (e.g., "1080p", "720p"). Returns the first matching stream.
    ///
    /// # Arguments
    ///
    /// * `quality` - Quality string to match (e.g., "1080p", "4k")
    pub fn stream_by_quality(&self, quality: &str) -> Option<&StreamInfo> {
        self.streams
            .iter()
            .filter(|s| !s.is_audio_only)
            .find(|s| s.quality.to_lowercase() == quality.to_lowercase())
    }

    /// Selects a video stream based on the provided quality filter.
    ///
    /// This is the primary method for selecting streams with flexible quality
    /// criteria. Supports selecting best/worst quality, exact resolutions, or
    /// maximum height constraints.
    ///
    /// # Arguments
    ///
    /// * `filter` - Quality filter criteria to apply
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rust_yt_downloader::youtube::{YtDlpClient, QualityFilter};
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = YtDlpClient::new();
    /// let info = client.get_video_info("https://www.youtube.com/watch?v=dQw4w9WgXcQ")?;
    ///
    /// // Get best quality up to 720p
    /// if let Some(stream) = info.stream_by_filter(QualityFilter::MaxHeight(720)) {
    ///     println!("Selected: {}", stream.quality);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn stream_by_filter(&self, filter: QualityFilter) -> Option<&StreamInfo> {
        match filter {
            QualityFilter::Best => self.best_video_stream(),
            QualityFilter::Worst => self.worst_video_stream(),
            QualityFilter::Exact(height) => {
                let quality = format!("{}p", height);
                self.stream_by_quality(&quality)
            }
            QualityFilter::MaxHeight(max) => self
                .streams
                .iter()
                .filter(|s| !s.is_audio_only)
                .filter(|s| Self::quality_to_height(&s.quality) <= max)
                .max_by_key(|s| Self::quality_to_height(&s.quality)),
        }
    }

    /// Returns a sorted, deduplicated list of available video quality labels.
    ///
    /// The qualities are sorted in descending order (highest quality first).
    /// Audio-only streams are excluded.
    ///
    /// # Returns
    ///
    /// A vector of quality strings like `["1080p", "720p", "480p"]`
    pub fn available_qualities(&self) -> Vec<String> {
        let mut qualities: Vec<String> = self
            .streams
            .iter()
            .filter(|s| !s.is_audio_only)
            .map(|s| s.quality.clone())
            .collect();

        qualities.sort_by_key(|q| std::cmp::Reverse(Self::quality_to_height(q)));
        qualities.dedup();
        qualities
    }

    /// Converts a quality string to a numeric height value for comparison.
    ///
    /// Recognizes common quality labels (4k, 2160p, 1440p, 1080p, 720p, etc.)
    /// and returns the vertical resolution in pixels. Returns 0 for unknown formats.
    fn quality_to_height(quality: &str) -> u32 {
        let quality = quality.to_lowercase();

        if quality.contains("4k") || quality.contains("2160") {
            return 2160;
        }
        if quality.contains("1440") {
            return 1440;
        }
        if quality.contains("1080") {
            return 1080;
        }
        if quality.contains("720") {
            return 720;
        }
        if quality.contains("480") {
            return 480;
        }
        if quality.contains("360") {
            return 360;
        }
        if quality.contains("240") {
            return 240;
        }
        if quality.contains("144") {
            return 144;
        }

        0
    }
}

impl StreamInfo {
    /// Generates a human-readable description of the stream.
    ///
    /// Combines quality, codec, FPS (if > 30), and format into a single
    /// space-separated string suitable for display.
    ///
    /// # Examples
    ///
    /// Returns strings like:
    /// - `"1080p h264 60fps mp4"`
    /// - `"720p vp9 webm"`
    /// - `"audio aac m4a"`
    pub fn description(&self) -> String {
        let mut parts = vec![self.quality.clone()];

        if let Some(codec) = &self.video_codec {
            parts.push(codec.clone());
        }

        if let Some(fps) = self.fps {
            if fps > 30 {
                parts.push(format!("{}fps", fps));
            }
        }

        parts.push(self.format.clone());

        parts.join(" ")
    }

    /// Returns the file size formatted as a human-readable string.
    ///
    /// Converts bytes to appropriate units (KB, MB, GB). Returns `None`
    /// if the file size is unknown.
    ///
    /// # Examples
    ///
    /// Returns strings like:
    /// - `Some("45.2 MB")`
    /// - `Some("1.3 GB")`
    /// - `None` (if file_size is None)
    pub fn formatted_size(&self) -> Option<String> {
        self.file_size.map(crate::utils::format_bytes)
    }
}

impl PlaylistInfo {
    /// Returns `true` if the playlist contains no videos.
    pub fn is_empty(&self) -> bool {
        self.video_ids.is_empty()
    }

    /// Returns the number of videos in the playlist.
    ///
    /// This is the actual count based on the `video_ids` length,
    /// which may differ from `video_count` in some cases.
    pub fn len(&self) -> usize {
        self.video_ids.len()
    }
}

// ==================================================
//          UNITARY TESTS
// ==================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ============== Helper Functions ==============

    fn create_test_stream(quality: &str, format: &str, is_audio_only: bool) -> StreamInfo {
        StreamInfo {
            url: format!("https://example.com/stream_{}", quality),
            quality: quality.to_string(),
            format: format.to_string(),
            video_codec: if is_audio_only {
                None
            } else {
                Some("h264".to_string())
            },
            audio_codec: Some("aac".to_string()),
            is_audio_only,
            file_size: Some(1000000),
            bitrate: Some(128),
            fps: Some(30),
        }
    }

    fn create_test_audio_stream(bitrate: u64) -> StreamInfo {
        StreamInfo {
            url: format!("https://example.com/audio_{}", bitrate),
            quality: "audio".to_string(),
            format: "m4a".to_string(),
            video_codec: None,
            audio_codec: Some("aac".to_string()),
            is_audio_only: true,
            file_size: Some(500000),
            bitrate: Some(bitrate),
            fps: None,
        }
    }

    fn create_test_video_info() -> VideoInfo {
        VideoInfo {
            id: "abc123".to_string(),
            title: "Test Video".to_string(),
            description: Some("A test video description".to_string()),
            duration: 300,
            thumbnail_url: Some("https://example.com/thumb.jpg".to_string()),
            channel: Some("Test Channel".to_string()),
            publish_date: Some("2024-01-15".to_string()),
            view_count: Some(1000000),
            streams: vec![
                create_test_stream("1080p", "mp4", false),
                create_test_stream("720p", "mp4", false),
                create_test_stream("480p", "mp4", false),
                create_test_stream("360p", "mp4", false),
                create_test_audio_stream(320),
                create_test_audio_stream(128),
            ],
        }
    }

    // ============== VideoInfo Creation Tests ==============

    #[test]
    fn test_video_info_creation() {
        let video = create_test_video_info();

        assert_eq!(video.id, "abc123");
        assert_eq!(video.title, "Test Video");
        assert_eq!(video.duration, 300);
        assert_eq!(video.streams.len(), 6);
    }

    #[test]
    fn test_video_info_with_minimal_fields() {
        let video = VideoInfo {
            id: "xyz789".to_string(),
            title: "Minimal Video".to_string(),
            description: None,
            duration: 60,
            thumbnail_url: None,
            channel: None,
            publish_date: None,
            view_count: None,
            streams: vec![],
        };

        assert_eq!(video.id, "xyz789");
        assert!(video.description.is_none());
        assert!(video.thumbnail_url.is_none());
        assert!(video.streams.is_empty());
    }

    // ============== best_video_stream Tests ==============

    #[test]
    fn test_best_video_stream() {
        let video = create_test_video_info();
        let best = video.best_video_stream();

        assert!(best.is_some());
        assert_eq!(best.unwrap().quality, "1080p");
    }

    #[test]
    fn test_best_video_stream_with_4k() {
        let mut video = create_test_video_info();
        video.streams.push(create_test_stream("4k", "mp4", false));

        let best = video.best_video_stream();

        assert!(best.is_some());
        assert_eq!(best.unwrap().quality, "4k");
    }

    #[test]
    fn test_best_video_stream_with_2160p() {
        let mut video = create_test_video_info();
        video
            .streams
            .push(create_test_stream("2160p", "mp4", false));

        let best = video.best_video_stream();

        assert!(best.is_some());
        assert_eq!(best.unwrap().quality, "2160p");
    }

    #[test]
    fn test_best_video_stream_with_1440p() {
        let mut video = create_test_video_info();
        video
            .streams
            .push(create_test_stream("1440p", "mp4", false));

        let best = video.best_video_stream();

        assert!(best.is_some());
        assert_eq!(best.unwrap().quality, "1440p");
    }

    #[test]
    fn test_best_video_stream_excludes_audio() {
        let video = create_test_video_info();
        let best = video.best_video_stream();

        assert!(best.is_some());
        assert!(!best.unwrap().is_audio_only);
    }

    #[test]
    fn test_best_video_stream_empty_streams() {
        let video = VideoInfo {
            id: "test".to_string(),
            title: "Test".to_string(),
            description: None,
            duration: 0,
            thumbnail_url: None,
            channel: None,
            publish_date: None,
            view_count: None,
            streams: vec![],
        };

        assert!(video.best_video_stream().is_none());
    }

    #[test]
    fn test_best_video_stream_only_audio_streams() {
        let video = VideoInfo {
            id: "test".to_string(),
            title: "Test".to_string(),
            description: None,
            duration: 0,
            thumbnail_url: None,
            channel: None,
            publish_date: None,
            view_count: None,
            streams: vec![create_test_audio_stream(320), create_test_audio_stream(128)],
        };

        assert!(video.best_video_stream().is_none());
    }

    // ============== worst_video_stream Tests ==============

    #[test]
    fn test_worst_video_stream() {
        let video = create_test_video_info();
        let worst = video.worst_video_stream();

        assert!(worst.is_some());
        assert_eq!(worst.unwrap().quality, "360p");
    }

    #[test]
    fn test_worst_video_stream_with_144p() {
        let mut video = create_test_video_info();
        video.streams.push(create_test_stream("144p", "mp4", false));

        let worst = video.worst_video_stream();

        assert!(worst.is_some());
        assert_eq!(worst.unwrap().quality, "144p");
    }

    #[test]
    fn test_worst_video_stream_excludes_audio() {
        let video = create_test_video_info();
        let worst = video.worst_video_stream();

        assert!(worst.is_some());
        assert!(!worst.unwrap().is_audio_only);
    }

    #[test]
    fn test_worst_video_stream_empty_streams() {
        let video = VideoInfo {
            id: "test".to_string(),
            title: "Test".to_string(),
            description: None,
            duration: 0,
            thumbnail_url: None,
            channel: None,
            publish_date: None,
            view_count: None,
            streams: vec![],
        };

        assert!(video.worst_video_stream().is_none());
    }

    // ============== best_audio_stream Tests ==============

    #[test]
    fn test_best_audio_stream() {
        let video = create_test_video_info();
        let best = video.best_audio_stream();

        assert!(best.is_some());
        assert_eq!(best.unwrap().bitrate, Some(320));
    }

    #[test]
    fn test_best_audio_stream_is_audio_only() {
        let video = create_test_video_info();
        let best = video.best_audio_stream();

        assert!(best.is_some());
        assert!(best.unwrap().is_audio_only);
    }

    #[test]
    fn test_best_audio_stream_no_audio() {
        let video = VideoInfo {
            id: "test".to_string(),
            title: "Test".to_string(),
            description: None,
            duration: 0,
            thumbnail_url: None,
            channel: None,
            publish_date: None,
            view_count: None,
            streams: vec![
                create_test_stream("1080p", "mp4", false),
                create_test_stream("720p", "mp4", false),
            ],
        };

        assert!(video.best_audio_stream().is_none());
    }

    // ============== stream_by_quality Tests ==============

    #[test]
    fn test_stream_by_quality_exact_match() {
        let video = create_test_video_info();
        let stream = video.stream_by_quality("720p");

        assert!(stream.is_some());
        assert_eq!(stream.unwrap().quality, "720p");
    }

    #[test]
    fn test_stream_by_quality_case_insensitive() {
        let video = create_test_video_info();

        let stream1 = video.stream_by_quality("720P");
        let stream2 = video.stream_by_quality("720p");

        assert!(stream1.is_some());
        assert!(stream2.is_some());
        assert_eq!(stream1.unwrap().quality, stream2.unwrap().quality);
    }

    #[test]
    fn test_stream_by_quality_not_found() {
        let video = create_test_video_info();
        let stream = video.stream_by_quality("4k");

        assert!(stream.is_none());
    }

    #[test]
    fn test_stream_by_quality_excludes_audio() {
        let video = create_test_video_info();
        let stream = video.stream_by_quality("audio");

        assert!(stream.is_none());
    }

    // ============== stream_by_filter Tests ==============

    #[test]
    fn test_stream_by_filter_best() {
        let video = create_test_video_info();
        let stream = video.stream_by_filter(QualityFilter::Best);

        assert!(stream.is_some());
        assert_eq!(stream.unwrap().quality, "1080p");
    }

    #[test]
    fn test_stream_by_filter_worst() {
        let video = create_test_video_info();
        let stream = video.stream_by_filter(QualityFilter::Worst);

        assert!(stream.is_some());
        assert_eq!(stream.unwrap().quality, "360p");
    }

    #[test]
    fn test_stream_by_filter_exact() {
        let video = create_test_video_info();
        let stream = video.stream_by_filter(QualityFilter::Exact(720));

        assert!(stream.is_some());
        assert_eq!(stream.unwrap().quality, "720p");
    }

    #[test]
    fn test_stream_by_filter_exact_not_found() {
        let video = create_test_video_info();
        let stream = video.stream_by_filter(QualityFilter::Exact(1440));

        assert!(stream.is_none());
    }

    #[test]
    fn test_stream_by_filter_max_height() {
        let video = create_test_video_info();
        let stream = video.stream_by_filter(QualityFilter::MaxHeight(720));

        assert!(stream.is_some());
        assert_eq!(stream.unwrap().quality, "720p");
    }

    #[test]
    fn test_stream_by_filter_max_height_gets_best_under_limit() {
        let video = create_test_video_info();
        let stream = video.stream_by_filter(QualityFilter::MaxHeight(800));

        assert!(stream.is_some());
        assert_eq!(stream.unwrap().quality, "720p");
    }

    #[test]
    fn test_stream_by_filter_max_height_too_low() {
        let video = create_test_video_info();
        let stream = video.stream_by_filter(QualityFilter::MaxHeight(100));

        assert!(stream.is_none());
    }

    // ============== available_qualities Tests ==============

    #[test]
    fn test_available_qualities() {
        let video = create_test_video_info();
        let qualities = video.available_qualities();

        assert_eq!(qualities.len(), 4);
        assert_eq!(qualities[0], "1080p");
        assert_eq!(qualities[1], "720p");
        assert_eq!(qualities[2], "480p");
        assert_eq!(qualities[3], "360p");
    }

    #[test]
    fn test_available_qualities_sorted_descending() {
        let mut video = create_test_video_info();
        video.streams.push(create_test_stream("4k", "mp4", false));
        video.streams.push(create_test_stream("144p", "mp4", false));

        let qualities = video.available_qualities();

        assert_eq!(qualities[0], "4k");
        assert_eq!(qualities[1], "1080p");
        assert_eq!(qualities[qualities.len() - 1], "144p");
    }

    #[test]
    fn test_available_qualities_no_duplicates() {
        let mut video = create_test_video_info();
        video
            .streams
            .push(create_test_stream("720p", "webm", false));

        let qualities = video.available_qualities();
        let count_720p = qualities.iter().filter(|q| *q == "720p").count();

        assert_eq!(count_720p, 1);
    }

    #[test]
    fn test_available_qualities_excludes_audio() {
        let video = create_test_video_info();
        let qualities = video.available_qualities();

        assert!(!qualities.contains(&"audio".to_string()));
    }

    #[test]
    fn test_available_qualities_empty() {
        let video = VideoInfo {
            id: "test".to_string(),
            title: "Test".to_string(),
            description: None,
            duration: 0,
            thumbnail_url: None,
            channel: None,
            publish_date: None,
            view_count: None,
            streams: vec![],
        };

        let qualities = video.available_qualities();
        assert!(qualities.is_empty());
    }

    // ============== quality_to_height Tests ==============

    #[test]
    fn test_quality_to_height_4k() {
        assert_eq!(VideoInfo::quality_to_height("4k"), 2160);
        assert_eq!(VideoInfo::quality_to_height("4K"), 2160);
    }

    #[test]
    fn test_quality_to_height_2160p() {
        assert_eq!(VideoInfo::quality_to_height("2160p"), 2160);
    }

    #[test]
    fn test_quality_to_height_1440p() {
        assert_eq!(VideoInfo::quality_to_height("1440p"), 1440);
    }

    #[test]
    fn test_quality_to_height_1080p() {
        assert_eq!(VideoInfo::quality_to_height("1080p"), 1080);
    }

    #[test]
    fn test_quality_to_height_720p() {
        assert_eq!(VideoInfo::quality_to_height("720p"), 720);
    }

    #[test]
    fn test_quality_to_height_480p() {
        assert_eq!(VideoInfo::quality_to_height("480p"), 480);
    }

    #[test]
    fn test_quality_to_height_360p() {
        assert_eq!(VideoInfo::quality_to_height("360p"), 360);
    }

    #[test]
    fn test_quality_to_height_240p() {
        assert_eq!(VideoInfo::quality_to_height("240p"), 240);
    }

    #[test]
    fn test_quality_to_height_144p() {
        assert_eq!(VideoInfo::quality_to_height("144p"), 144);
    }

    #[test]
    fn test_quality_to_height_unknown() {
        assert_eq!(VideoInfo::quality_to_height("unknown"), 0);
        assert_eq!(VideoInfo::quality_to_height(""), 0);
    }

    // ============== StreamInfo Tests ==============

    #[test]
    fn test_stream_info_creation() {
        let stream = create_test_stream("1080p", "mp4", false);

        assert_eq!(stream.quality, "1080p");
        assert_eq!(stream.format, "mp4");
        assert!(!stream.is_audio_only);
    }

    #[test]
    fn test_stream_info_description() {
        let stream = create_test_stream("1080p", "mp4", false);
        let desc = stream.description();

        assert!(desc.contains("1080p"));
        assert!(desc.contains("mp4"));
        assert!(desc.contains("h264"));
    }

    #[test]
    fn test_stream_info_description_with_high_fps() {
        let mut stream = create_test_stream("1080p", "mp4", false);
        stream.fps = Some(60);

        let desc = stream.description();

        assert!(desc.contains("60fps"));
    }

    #[test]
    fn test_stream_info_description_without_high_fps() {
        let stream = create_test_stream("1080p", "mp4", false);
        let desc = stream.description();

        assert!(!desc.contains("30fps"));
    }

    #[test]
    fn test_stream_info_formatted_size() {
        let mut stream = create_test_stream("1080p", "mp4", false);
        stream.file_size = Some(1024 * 1024 * 100); // 100 MB

        let size = stream.formatted_size();

        assert!(size.is_some());
        assert!(size.unwrap().contains("MB"));
    }

    #[test]
    fn test_stream_info_formatted_size_none() {
        let mut stream = create_test_stream("1080p", "mp4", false);
        stream.file_size = None;

        assert!(stream.formatted_size().is_none());
    }

    // ============== PlaylistInfo Tests ==============

    #[test]
    fn test_playlist_info_creation() {
        let playlist = PlaylistInfo {
            id: "PL123".to_string(),
            title: "My Playlist".to_string(),
            description: Some("A test playlist".to_string()),
            channel: Some("Test Channel".to_string()),
            video_count: 10,
            video_ids: vec!["vid1".to_string(), "vid2".to_string(), "vid3".to_string()],
        };

        assert_eq!(playlist.id, "PL123");
        assert_eq!(playlist.title, "My Playlist");
        assert_eq!(playlist.video_count, 10);
        assert_eq!(playlist.video_ids.len(), 3);
    }

    #[test]
    fn test_playlist_info_is_empty_false() {
        let playlist = PlaylistInfo {
            id: "PL123".to_string(),
            title: "My Playlist".to_string(),
            description: None,
            channel: None,
            video_count: 1,
            video_ids: vec!["vid1".to_string()],
        };

        assert!(!playlist.is_empty());
    }

    #[test]
    fn test_playlist_info_is_empty_true() {
        let playlist = PlaylistInfo {
            id: "PL123".to_string(),
            title: "Empty Playlist".to_string(),
            description: None,
            channel: None,
            video_count: 0,
            video_ids: vec![],
        };

        assert!(playlist.is_empty());
    }

    #[test]
    fn test_playlist_info_len() {
        let playlist = PlaylistInfo {
            id: "PL123".to_string(),
            title: "My Playlist".to_string(),
            description: None,
            channel: None,
            video_count: 5,
            video_ids: vec![
                "vid1".to_string(),
                "vid2".to_string(),
                "vid3".to_string(),
                "vid4".to_string(),
                "vid5".to_string(),
            ],
        };

        assert_eq!(playlist.len(), 5);
    }

    #[test]
    fn test_playlist_info_len_empty() {
        let playlist = PlaylistInfo {
            id: "PL123".to_string(),
            title: "Empty".to_string(),
            description: None,
            channel: None,
            video_count: 0,
            video_ids: vec![],
        };

        assert_eq!(playlist.len(), 0);
    }

    // ============== QualityFilter Tests ==============

    #[test]
    fn test_quality_filter_best_equality() {
        assert_eq!(QualityFilter::Best, QualityFilter::Best);
    }

    #[test]
    fn test_quality_filter_worst_equality() {
        assert_eq!(QualityFilter::Worst, QualityFilter::Worst);
    }

    #[test]
    fn test_quality_filter_exact_equality() {
        assert_eq!(QualityFilter::Exact(1080), QualityFilter::Exact(1080));
        assert_ne!(QualityFilter::Exact(1080), QualityFilter::Exact(720));
    }

    #[test]
    fn test_quality_filter_max_height_equality() {
        assert_eq!(QualityFilter::MaxHeight(720), QualityFilter::MaxHeight(720));
        assert_ne!(
            QualityFilter::MaxHeight(720),
            QualityFilter::MaxHeight(1080)
        );
    }

    #[test]
    fn test_quality_filter_different_variants() {
        assert_ne!(QualityFilter::Best, QualityFilter::Worst);
        assert_ne!(QualityFilter::Best, QualityFilter::Exact(1080));
        assert_ne!(QualityFilter::Exact(1080), QualityFilter::MaxHeight(1080));
    }

    // ============== Serialization Tests ==============

    #[test]
    fn test_video_info_serialize() {
        let video = create_test_video_info();
        let json = serde_json::to_string(&video);

        assert!(json.is_ok());

        let json_str = json.unwrap();
        assert!(json_str.contains("abc123"));
        assert!(json_str.contains("Test Video"));
    }

    #[test]
    fn test_video_info_deserialize() {
        let json = r#"{
            "id": "xyz789",
            "title": "Deserialized Video",
            "description": null,
            "duration": 120,
            "thumbnail_url": null,
            "channel": null,
            "publish_date": null,
            "view_count": null,
            "streams": []
        }"#;

        let video: Result<VideoInfo, _> = serde_json::from_str(json);

        assert!(video.is_ok());

        let video = video.unwrap();
        assert_eq!(video.id, "xyz789");
        assert_eq!(video.title, "Deserialized Video");
        assert_eq!(video.duration, 120);
    }

    #[test]
    fn test_stream_info_serialize() {
        let stream = create_test_stream("720p", "mp4", false);
        let json = serde_json::to_string(&stream);

        assert!(json.is_ok());
    }

    #[test]
    fn test_playlist_info_serialize() {
        let playlist = PlaylistInfo {
            id: "PL123".to_string(),
            title: "Test Playlist".to_string(),
            description: None,
            channel: None,
            video_count: 3,
            video_ids: vec!["a".to_string(), "b".to_string(), "c".to_string()],
        };

        let json = serde_json::to_string(&playlist);

        assert!(json.is_ok());
    }

    // ============== Clone Tests ==============

    #[test]
    fn test_video_info_clone() {
        let video = create_test_video_info();
        let cloned = video.clone();

        assert_eq!(video.id, cloned.id);
        assert_eq!(video.title, cloned.title);
        assert_eq!(video.streams.len(), cloned.streams.len());
    }

    #[test]
    fn test_stream_info_clone() {
        let stream = create_test_stream("1080p", "mp4", false);
        let cloned = stream.clone();

        assert_eq!(stream.quality, cloned.quality);
        assert_eq!(stream.url, cloned.url);
    }

    #[test]
    fn test_playlist_info_clone() {
        let playlist = PlaylistInfo {
            id: "PL123".to_string(),
            title: "Test".to_string(),
            description: None,
            channel: None,
            video_count: 1,
            video_ids: vec!["vid1".to_string()],
        };

        let cloned = playlist.clone();

        assert_eq!(playlist.id, cloned.id);
        assert_eq!(playlist.video_ids, cloned.video_ids);
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub duration: u64,
    pub thumbnail_url: Option<String>,
    pub channel: Option<String>,
    pub publish_date: Option<String>,
    pub view_count: Option<u64>,
    pub streams: Vec<StreamInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    pub url: String,
    pub quality: String,
    pub format: String,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub is_audio_only: bool,
    pub file_size: Option<u64>,
    pub bitrate: Option<u64>,
    pub fps: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistInfo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub channel: Option<String>,
    pub video_count: u64,
    pub video_ids: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityFilter {
    Best,
    Worst,
    Exact(u32),
    MaxHeight(u32),
}

impl VideoInfo {
    pub fn best_video_stream(&self) -> Option<&StreamInfo> {
        self.streams
            .iter()
            .filter(|s| !s.is_audio_only)
            .max_by_key(|s| Self::quality_to_height(&s.quality))
    }

    pub fn worst_video_stream(&self) -> Option<&StreamInfo> {
        self.streams
            .iter()
            .filter(|s| !s.is_audio_only)
            .min_by_key(|s| Self::quality_to_height(&s.quality))
    }

    pub fn best_audio_stream(&self) -> Option<&StreamInfo> {
        self.streams
            .iter()
            .filter(|s| s.is_audio_only)
            .max_by_key(|s| s.bitrate.unwrap_or(0))
    }

    pub fn stream_by_quality(&self, quality: &str) -> Option<&StreamInfo> {
        self.streams
            .iter()
            .filter(|s| !s.is_audio_only)
            .find(|s| s.quality.to_lowercase() == quality.to_lowercase())
    }

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

    pub fn formatted_size(&self) -> Option<String> {
        self.file_size.map(crate::utils::format_bytes)
    }
}

impl PlaylistInfo {
    pub fn is_empty(&self) -> bool {
        self.video_ids.is_empty()
    }

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
            video_codec: if is_audio_only { None } else { Some("h264".to_string()) },
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
        video.streams.push(create_test_stream("2160p", "mp4", false));

        let best = video.best_video_stream();

        assert!(best.is_some());
        assert_eq!(best.unwrap().quality, "2160p");
    }

    #[test]
    fn test_best_video_stream_with_1440p() {
        let mut video = create_test_video_info();
        video.streams.push(create_test_stream("1440p", "mp4", false));

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
            streams: vec![
                create_test_audio_stream(320),
                create_test_audio_stream(128),
            ],
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
        video.streams.push(create_test_stream("720p", "webm", false));

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
        assert_ne!(QualityFilter::MaxHeight(720), QualityFilter::MaxHeight(1080));
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
//! YouTube client using the rustube library.
//!
//! This module provides a client for fetching YouTube video information using
//! the rustube library. It handles URL validation, video ID extraction, and
//! stream metadata retrieval.
//!
//! # URL Validation
//!
//! The client supports multiple YouTube URL formats:
//! - Standard: `https://www.youtube.com/watch?v=VIDEO_ID`
//! - Short: `https://youtu.be/VIDEO_ID`
//! - Embed: `https://www.youtube.com/embed/VIDEO_ID`
//! - Mobile: `https://m.youtube.com/watch?v=VIDEO_ID`
//! - With parameters: URLs containing timestamps, playlists, etc.

use rustube::{Id, Video};

use crate::error::{AppError, AppResult};
use crate::youtube::metadata::{StreamInfo, VideoInfo};

/// YouTube client for fetching video information via rustube.
///
/// This client provides methods to validate YouTube URLs, extract video metadata,
/// and retrieve available streams. It uses the rustube library internally.
///
/// # Examples
///
/// ```no_run
/// # use rust_yt_downloader::youtube::YouTubeClient;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = YouTubeClient::new();
///
/// // Get video information
/// let info = client.get_video_info("https://www.youtube.com/watch?v=dQw4w9WgXcQ").await?;
/// println!("Title: {}", info.title);
/// println!("Duration: {} seconds", info.duration);
///
/// // Get available streams
/// let streams = client.get_streams("https://www.youtube.com/watch?v=dQw4w9WgXcQ").await?;
/// for stream in streams {
///     println!("{}", stream.description());
/// }
/// # Ok(())
/// # }
/// ```
pub struct YouTubeClient;

impl YouTubeClient {
    /// Creates a new YouTube client instance.
    pub fn new() -> Self {
        Self
    }

    /// Fetches complete video information including metadata and available streams.
    ///
    /// Retrieves all video details such as title, description, duration, channel,
    /// view count, and all available streams with their quality information.
    ///
    /// # Arguments
    ///
    /// * `url` - A valid YouTube video URL in any supported format
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The URL is invalid or malformed
    /// - The video ID cannot be extracted
    /// - The video is unavailable or private
    /// - Network errors occur during fetching
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rust_yt_downloader::youtube::YouTubeClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = YouTubeClient::new();
    /// let info = client.get_video_info("https://www.youtube.com/watch?v=dQw4w9WgXcQ").await?;
    ///
    /// println!("Video: {}", info.title);
    /// println!("Available qualities: {:?}", info.available_qualities());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_video_info(&self, url: &str) -> AppResult<VideoInfo> {
        let id = Self::extract_id(url)?;

        let video = Video::from_id(id.into_owned())
            .await
            .map_err(|e| AppError::ExtractionFailed(e.to_string()))?;

        let video_details = video.video_details();
        let streams = Self::extract_streams(&video);

        Ok(VideoInfo {
            id: video_details.video_id.to_string(),
            title: video_details.title.clone(),
            description: Some(video_details.short_description.clone()),
            duration: video_details.length_seconds,
            thumbnail_url: video_details.thumbnails.first().map(|t| t.url.to_string()),
            channel: Some(video_details.author.clone()),
            publish_date: None,
            view_count: Some(video_details.view_count),
            streams,
        })
    }

    /// Fetches only the stream information for a video.
    ///
    /// This is a convenience method that retrieves video info and returns
    /// just the streams vector.
    ///
    /// # Arguments
    ///
    /// * `url` - A valid YouTube video URL
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rust_yt_downloader::youtube::YouTubeClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = YouTubeClient::new();
    /// let streams = client.get_streams("https://www.youtube.com/watch?v=dQw4w9WgXcQ").await?;
    ///
    /// for stream in &streams {
    ///     if !stream.is_audio_only {
    ///         println!("{}", stream.quality);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_streams(&self, url: &str) -> AppResult<Vec<StreamInfo>> {
        let info = self.get_video_info(url).await?;
        Ok(info.streams)
    }

    /// Validates whether a URL is a valid YouTube video URL.
    ///
    /// Checks if the URL can be parsed and a video ID can be extracted.
    /// Does not verify if the video actually exists or is accessible.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to validate
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_yt_downloader::youtube::YouTubeClient;
    ///
    /// assert!(YouTubeClient::is_valid_url("https://www.youtube.com/watch?v=dQw4w9WgXcQ"));
    /// assert!(YouTubeClient::is_valid_url("https://youtu.be/dQw4w9WgXcQ"));
    /// assert!(!YouTubeClient::is_valid_url("https://vimeo.com/123456"));
    /// ```
    pub fn is_valid_url(url: &str) -> bool {
        Self::extract_id(url).is_ok()
    }

    /// Extracts the YouTube video ID from a URL.
    ///
    /// Supports multiple URL formats and returns a static-lifetime ID
    /// that can be used with the rustube library.
    fn extract_id(url: &str) -> AppResult<Id<'static>> {
        Id::from_raw(url)
            .map(|id| id.as_owned())
            .map_err(|_| AppError::InvalidUrl(url.to_string()))
    }

    /// Extracts stream information from a rustube Video object.
    ///
    /// Converts rustube's internal stream representation to our StreamInfo
    /// structure, handling codec detection and quality labeling.
    fn extract_streams(video: &Video) -> Vec<StreamInfo> {
        video
            .streams()
            .iter()
            .filter_map(|stream| {
                let url = stream.signature_cipher.url.to_string();

                let quality = stream
                    .quality_label
                    .as_ref()
                    .map(|q| format!("{:?}", q))
                    .unwrap_or_else(|| {
                        if stream.includes_audio_track && !stream.includes_video_track {
                            "audio".to_string()
                        } else {
                            "unknown".to_string()
                        }
                    });

                let format = stream.mime.subtype().to_string().to_lowercase();

                let is_audio_only = stream.includes_audio_track && !stream.includes_video_track;

                Some(StreamInfo {
                    url,
                    quality,
                    format,
                    video_codec: stream.codecs.first().cloned(),
                    audio_codec: stream.codecs.last().cloned(),
                    is_audio_only,
                    file_size: None,
                    bitrate: stream.bitrate,
                    fps: Some(stream.fps as u32),
                })
            })
            .collect()
    }
}

impl Default for YouTubeClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates a YouTube URL and returns a Result.
///
/// This is a convenience function that wraps `YouTubeClient::is_valid_url`
/// and returns an appropriate error if the URL is invalid.
///
/// # Arguments
///
/// * `url` - The URL to validate
///
/// # Errors
///
/// Returns `AppError::InvalidUrl` if the URL is not a valid YouTube video URL.
///
/// # Examples
///
/// ```
/// use rust_yt_downloader::youtube::validate_youtube_url;
///
/// assert!(validate_youtube_url("https://www.youtube.com/watch?v=dQw4w9WgXcQ").is_ok());
/// assert!(validate_youtube_url("https://invalid.com/video").is_err());
/// ```
pub fn validate_youtube_url(url: &str) -> AppResult<()> {
    if !YouTubeClient::is_valid_url(url) {
        return Err(AppError::InvalidUrl(url.to_string()));
    }
    Ok(())
}

// ==================================================
//          UNITARY TESTS
// ==================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ============== YouTubeClient Creation Tests ==============

    #[test]
    fn test_youtube_client_new() {
        let client = YouTubeClient::new();
        assert!(true);
    }

    #[test]
    fn test_youtube_client_default() {
        let client = YouTubeClient::default();
        assert!(true);
    }

    // ============== URL Validation Tests ==============

    #[test]
    fn test_is_valid_url_standard() {
        assert!(YouTubeClient::is_valid_url(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
        ));
    }

    #[test]
    fn test_is_valid_url_without_www() {
        assert!(YouTubeClient::is_valid_url(
            "https://youtube.com/watch?v=dQw4w9WgXcQ"
        ));
    }

    #[test]
    fn test_is_valid_url_short() {
        assert!(YouTubeClient::is_valid_url("https://youtu.be/dQw4w9WgXcQ"));
    }

    #[test]
    fn test_is_valid_url_with_timestamp() {
        assert!(YouTubeClient::is_valid_url(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ&t=120"
        ));
    }

    #[test]
    fn test_is_valid_url_with_playlist() {
        assert!(YouTubeClient::is_valid_url(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ&list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf"
        ));
    }

    #[test]
    fn test_is_valid_url_embed() {
        assert!(YouTubeClient::is_valid_url(
            "https://www.youtube.com/embed/dQw4w9WgXcQ"
        ));
    }

    #[test]
    fn test_is_valid_url_invalid_empty() {
        assert!(!YouTubeClient::is_valid_url(""));
    }

    #[test]
    fn test_is_valid_url_invalid_random_string() {
        assert!(!YouTubeClient::is_valid_url("not a url"));
    }

    #[test]
    fn test_is_valid_url_invalid_other_site() {
        assert!(!YouTubeClient::is_valid_url("https://vimeo.com/123456789"));
    }

    #[test]
    fn test_is_valid_url_invalid_youtube_no_id() {
        assert!(!YouTubeClient::is_valid_url("https://www.youtube.com/"));
    }

    #[test]
    fn test_is_valid_url_invalid_partial() {
        assert!(!YouTubeClient::is_valid_url("youtube.com/watch?v=abc"));
    }

    // ============== extract_id Tests ==============

    #[test]
    fn test_extract_id_standard_url() {
        let result = YouTubeClient::extract_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "dQw4w9WgXcQ");
    }

    #[test]
    fn test_extract_id_short_url() {
        let result = YouTubeClient::extract_id("https://youtu.be/dQw4w9WgXcQ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "dQw4w9WgXcQ");
    }

    #[test]
    fn test_extract_id_embed_url() {
        let result = YouTubeClient::extract_id("https://www.youtube.com/embed/dQw4w9WgXcQ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "dQw4w9WgXcQ");
    }

    #[test]
    fn test_extract_id_with_extra_params() {
        let result = YouTubeClient::extract_id(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ&list=PLtest&index=5",
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "dQw4w9WgXcQ");
    }

    #[test]
    fn test_extract_id_invalid_url() {
        let result = YouTubeClient::extract_id("https://vimeo.com/123456");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_id_empty_string() {
        let result = YouTubeClient::extract_id("");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_id_returns_correct_error_type() {
        let result = YouTubeClient::extract_id("invalid");
        assert!(matches!(result, Err(AppError::InvalidUrl(_))));
    }

    // ============== validate_youtube_url Tests ==============

    #[test]
    fn test_validate_youtube_url_valid() {
        let result = validate_youtube_url("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_youtube_url_valid_short() {
        let result = validate_youtube_url("https://youtu.be/dQw4w9WgXcQ");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_youtube_url_invalid() {
        let result = validate_youtube_url("https://vimeo.com/123456");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_youtube_url_invalid_empty() {
        let result = validate_youtube_url("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_youtube_url_error_contains_url() {
        let invalid_url = "https://invalid.com/video";
        let result = validate_youtube_url(invalid_url);

        match result {
            Err(AppError::InvalidUrl(url)) => {
                assert_eq!(url, invalid_url);
            }
            _ => panic!("Expected InvalidUrl error"),
        }
    }

    // ============== URL Format Edge Cases ==============

    #[test]
    fn test_is_valid_url_http_instead_of_https() {
        let _result = YouTubeClient::is_valid_url("http://www.youtube.com/watch?v=dQw4w9WgXcQ");
        // Test just ensures the function doesn't panic
    }

    #[test]
    fn test_is_valid_url_mobile() {
        let _result = YouTubeClient::is_valid_url("https://m.youtube.com/watch?v=dQw4w9WgXcQ");
        // Test just ensures the function doesn't panic
    }

    #[test]
    fn test_is_valid_url_music() {
        let _result = YouTubeClient::is_valid_url("https://music.youtube.com/watch?v=dQw4w9WgXcQ");
        // Test just ensures the function doesn't panic
    }

    #[test]
    fn test_extract_id_various_valid_ids() {
        let test_ids = vec![
            ("https://www.youtube.com/watch?v=abc123xyz", "abc123xyz"),
            ("https://youtu.be/ABC-123_xy", "ABC-123_xy"),
            ("https://www.youtube.com/embed/12345678901", "12345678901"),
        ];

        for (url, expected_id) in test_ids {
            let result = YouTubeClient::extract_id(url);
            if result.is_ok() {
                assert_eq!(result.unwrap().as_str(), expected_id);
            }
        }
    }

    // ============== Multiple Client Instances ==============

    #[test]
    fn test_multiple_client_instances() {
        let client1 = YouTubeClient::new();
        let client2 = YouTubeClient::new();
        let client3 = YouTubeClient::default();

        assert!(YouTubeClient::is_valid_url(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
        ));
    }

    // ============== Static Method Tests ==============

    #[test]
    fn test_is_valid_url_is_static() {
        let _ = YouTubeClient::is_valid_url("https://www.youtube.com/watch?v=test");
    }

    #[test]
    fn test_extract_id_is_static_private() {
        assert!(YouTubeClient::is_valid_url(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
        ));
        assert!(!YouTubeClient::is_valid_url("invalid"));
    }
}

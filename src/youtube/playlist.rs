//! YouTube playlist URL validation and video ID extraction.
//!
//! This module provides functionality for working with YouTube playlists,
//! including URL validation, playlist ID extraction, and utilities for
//! filtering and processing multiple playlist URLs.
//!
//! # Supported URL Formats
//!
//! - Standard: `https://www.youtube.com/playlist?list=PLxxxxx`
//! - With video: `https://www.youtube.com/watch?v=VIDEO_ID&list=PLxxxxx`
//! - With parameters: URLs containing index, shuffle, etc.

use crate::error::{AppError, AppResult};
use crate::utils::extract_playlist_id;
use crate::youtube::metadata::PlaylistInfo;

/// Client for working with YouTube playlists.
///
/// Provides methods to validate playlist URLs, extract playlist IDs,
/// and fetch playlist metadata. Currently uses basic playlist info;
/// full implementation with yt-dlp integration is planned.
///
/// # Examples
///
/// ```
/// use rust_yt_downloader::youtube::PlaylistClient;
///
/// let client = PlaylistClient::new();
///
/// // Validate a playlist URL
/// let is_valid = PlaylistClient::is_playlist_url(
///     "https://www.youtube.com/playlist?list=PLrAXtmErZgOe"
/// );
///
/// // Extract playlist ID
/// let id = PlaylistClient::get_playlist_id(
///     "https://www.youtube.com/playlist?list=PLrAXtmErZgOe"
/// ).unwrap();
/// assert_eq!(id, "PLrAXtmErZgOe");
/// ```
pub struct PlaylistClient;

impl PlaylistClient {
    /// Creates a new playlist client instance.
    pub fn new() -> Self {
        Self
    }

    /// Checks if a URL is a valid YouTube playlist URL.
    ///
    /// Validates the URL format and checks for the presence of a `list` parameter.
    /// Does not verify if the playlist actually exists.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to check
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_yt_downloader::youtube::PlaylistClient;
    ///
    /// assert!(PlaylistClient::is_playlist_url(
    ///     "https://www.youtube.com/playlist?list=PLtest"
    /// ));
    /// assert!(PlaylistClient::is_playlist_url(
    ///     "https://www.youtube.com/watch?v=abc&list=PLtest"
    /// ));
    /// assert!(!PlaylistClient::is_playlist_url(
    ///     "https://www.youtube.com/watch?v=abc"
    /// ));
    /// ```
    pub fn is_playlist_url(url: &str) -> bool {
        extract_playlist_id(url).is_some()
    }

    /// Extracts the playlist ID from a YouTube URL.
    ///
    /// Parses the URL and extracts the value of the `list` query parameter.
    ///
    /// # Arguments
    ///
    /// * `url` - A YouTube URL containing a playlist ID
    ///
    /// # Errors
    ///
    /// Returns `AppError::InvalidUrl` if the URL does not contain a valid playlist ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_yt_downloader::youtube::PlaylistClient;
    ///
    /// let id = PlaylistClient::get_playlist_id(
    ///     "https://www.youtube.com/playlist?list=PLtest123"
    /// ).unwrap();
    /// assert_eq!(id, "PLtest123");
    /// ```
    pub fn get_playlist_id(url: &str) -> AppResult<String> {
        extract_playlist_id(url).ok_or_else(|| AppError::InvalidUrl(url.to_string()))
    }

    /// Fetches playlist information from YouTube.
    ///
    /// Currently returns basic playlist structure with the extracted ID.
    /// Full implementation using yt-dlp for actual playlist metadata is planned.
    ///
    /// # Arguments
    ///
    /// * `url` - A valid YouTube playlist URL
    ///
    /// # Errors
    ///
    /// Returns an error if the URL is invalid or the playlist ID cannot be extracted.
    pub async fn get_playlist_info(&self, url: &str) -> AppResult<PlaylistInfo> {
        let playlist_id = Self::get_playlist_id(url)?;

        Ok(PlaylistInfo {
            id: playlist_id,
            title: "Unknown Playlist".to_string(),
            description: None,
            channel: None,
            video_count: 0,
            video_ids: vec![],
        })
    }

    /// Fetches the list of video IDs from a playlist.
    ///
    /// This is a convenience method that retrieves playlist info and returns
    /// just the video IDs vector.
    ///
    /// # Arguments
    ///
    /// * `url` - A valid YouTube playlist URL
    pub async fn get_video_ids(&self, url: &str) -> AppResult<Vec<String>> {
        let info = self.get_playlist_info(url).await?;
        Ok(info.video_ids)
    }

    /// Validates a playlist URL and returns a Result.
    ///
    /// Similar to `is_playlist_url` but returns a Result with a descriptive
    /// error message instead of a boolean.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to validate
    ///
    /// # Errors
    ///
    /// Returns `AppError::InvalidUrl` with a descriptive message if the URL
    /// is not a valid playlist URL.
    pub fn validate_playlist_url(url: &str) -> AppResult<()> {
        if !Self::is_playlist_url(url) {
            return Err(AppError::InvalidUrl(format!(
                "Not a valid playlist URL: {}",
                url
            )));
        }
        Ok(())
    }
}

impl Default for PlaylistClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Extracts playlist IDs from multiple URLs.
///
/// Processes a slice of URLs and attempts to extract the playlist ID from each.
/// Returns a vector of Results, preserving the order of the input URLs.
///
/// # Arguments
///
/// * `urls` - A slice of URL strings to process
///
/// # Returns
///
/// A vector where each element is either:
/// - `Ok(String)` containing the extracted playlist ID
/// - `Err(AppError)` if the URL is invalid or lacks a playlist ID
///
/// # Examples
///
/// ```
/// use rust_yt_downloader::youtube::extract_playlist_ids;
///
/// let urls = vec![
///     "https://www.youtube.com/playlist?list=PLfirst".to_string(),
///     "https://www.youtube.com/watch?v=abc".to_string(), // No playlist
///     "https://www.youtube.com/playlist?list=PLsecond".to_string(),
/// ];
///
/// let results = extract_playlist_ids(&urls);
/// assert!(results[0].is_ok());
/// assert!(results[1].is_err());
/// assert!(results[2].is_ok());
/// ```
pub fn extract_playlist_ids(urls: &[String]) -> Vec<AppResult<String>> {
    urls.iter()
        .map(|url| PlaylistClient::get_playlist_id(url))
        .collect()
}

/// Filters a list of URLs to include only valid playlist URLs.
///
/// Returns references to URLs that contain valid playlist IDs, maintaining
/// the original order.
///
/// # Arguments
///
/// * `urls` - A slice of URL strings to filter
///
/// # Returns
///
/// A vector of references to URLs that are valid playlist URLs
///
/// # Examples
///
/// ```
/// use rust_yt_downloader::youtube::filter_valid_playlist_urls;
///
/// let urls = vec![
///     "https://www.youtube.com/playlist?list=PLtest1".to_string(),
///     "https://www.youtube.com/watch?v=abc".to_string(),
///     "https://www.youtube.com/playlist?list=PLtest2".to_string(),
/// ];
///
/// let valid = filter_valid_playlist_urls(&urls);
/// assert_eq!(valid.len(), 2);
/// ```
pub fn filter_valid_playlist_urls(urls: &[String]) -> Vec<&String> {
    urls.iter()
        .filter(|url| PlaylistClient::is_playlist_url(url))
        .collect()
}

// ==================================================
//          UNITARY TESTS
// ==================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ============== PlaylistClient Creation Tests ==============

    #[test]
    fn test_playlist_client_new() {
        let client = PlaylistClient::new();
        assert!(true);
    }

    #[test]
    fn test_playlist_client_default() {
        let client = PlaylistClient::default();
        assert!(true);
    }

    // ============== is_playlist_url Tests ==============

    #[test]
    fn test_is_playlist_url_standard() {
        assert!(PlaylistClient::is_playlist_url(
            "https://www.youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf"
        ));
    }

    #[test]
    fn test_is_playlist_url_with_video() {
        assert!(PlaylistClient::is_playlist_url(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ&list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf"
        ));
    }

    #[test]
    fn test_is_playlist_url_without_www() {
        assert!(PlaylistClient::is_playlist_url(
            "https://youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf"
        ));
    }

    #[test]
    fn test_is_playlist_url_with_index() {
        assert!(PlaylistClient::is_playlist_url(
            "https://www.youtube.com/watch?v=abc123&list=PLtest123&index=5"
        ));
    }

    #[test]
    fn test_is_playlist_url_video_only() {
        assert!(!PlaylistClient::is_playlist_url(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
        ));
    }

    #[test]
    fn test_is_playlist_url_short_url() {
        assert!(!PlaylistClient::is_playlist_url(
            "https://youtu.be/dQw4w9WgXcQ"
        ));
    }

    #[test]
    fn test_is_playlist_url_empty() {
        assert!(!PlaylistClient::is_playlist_url(""));
    }

    #[test]
    fn test_is_playlist_url_invalid() {
        assert!(!PlaylistClient::is_playlist_url("not a url"));
    }

    #[test]
    fn test_is_playlist_url_other_site() {
        assert!(!PlaylistClient::is_playlist_url(
            "https://vimeo.com/playlist/123456"
        ));
    }

    #[test]
    fn test_is_playlist_url_youtube_homepage() {
        assert!(!PlaylistClient::is_playlist_url("https://www.youtube.com/"));
    }

    // ============== get_playlist_id Tests ==============

    #[test]
    fn test_get_playlist_id_standard() {
        let result =
            PlaylistClient::get_playlist_id("https://www.youtube.com/playlist?list=PLtest123456");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "PLtest123456");
    }

    #[test]
    fn test_get_playlist_id_from_video_url() {
        let result = PlaylistClient::get_playlist_id(
            "https://www.youtube.com/watch?v=abc123&list=PLmyPlaylist",
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "PLmyPlaylist");
    }

    #[test]
    fn test_get_playlist_id_with_extra_params() {
        let result = PlaylistClient::get_playlist_id(
            "https://www.youtube.com/watch?v=abc&list=PLtest&index=3&shuffle=1",
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "PLtest");
    }

    #[test]
    fn test_get_playlist_id_invalid_no_list() {
        let result =
            PlaylistClient::get_playlist_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ");

        assert!(result.is_err());
    }

    #[test]
    fn test_get_playlist_id_invalid_empty() {
        let result = PlaylistClient::get_playlist_id("");

        assert!(result.is_err());
    }

    #[test]
    fn test_get_playlist_id_returns_correct_error() {
        let invalid_url = "https://www.youtube.com/watch?v=test";
        let result = PlaylistClient::get_playlist_id(invalid_url);

        match result {
            Err(AppError::InvalidUrl(url)) => {
                assert_eq!(url, invalid_url);
            }
            _ => panic!("Expected InvalidUrl error"),
        }
    }

    // ============== validate_playlist_url Tests ==============

    #[test]
    fn test_validate_playlist_url_valid() {
        let result = PlaylistClient::validate_playlist_url(
            "https://www.youtube.com/playlist?list=PLtest123",
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_playlist_url_valid_with_video() {
        let result = PlaylistClient::validate_playlist_url(
            "https://www.youtube.com/watch?v=abc&list=PLtest",
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_playlist_url_invalid() {
        let result = PlaylistClient::validate_playlist_url(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_playlist_url_error_message() {
        let url = "https://www.youtube.com/watch?v=test";
        let result = PlaylistClient::validate_playlist_url(url);

        match result {
            Err(AppError::InvalidUrl(msg)) => {
                assert!(msg.contains("Not a valid playlist URL"));
                assert!(msg.contains(url));
            }
            _ => panic!("Expected InvalidUrl error"),
        }
    }

    // ============== extract_playlist_ids Tests ==============

    #[test]
    fn test_extract_playlist_ids_single() {
        let urls = vec!["https://www.youtube.com/playlist?list=PLtest123".to_string()];

        let results = extract_playlist_ids(&urls);

        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());
        assert_eq!(results[0].as_ref().unwrap(), "PLtest123");
    }

    #[test]
    fn test_extract_playlist_ids_multiple() {
        let urls = vec![
            "https://www.youtube.com/playlist?list=PLfirst".to_string(),
            "https://www.youtube.com/playlist?list=PLsecond".to_string(),
            "https://www.youtube.com/playlist?list=PLthird".to_string(),
        ];

        let results = extract_playlist_ids(&urls);

        assert_eq!(results.len(), 3);
        assert_eq!(results[0].as_ref().unwrap(), "PLfirst");
        assert_eq!(results[1].as_ref().unwrap(), "PLsecond");
        assert_eq!(results[2].as_ref().unwrap(), "PLthird");
    }

    #[test]
    fn test_extract_playlist_ids_mixed_valid_invalid() {
        let urls = vec![
            "https://www.youtube.com/playlist?list=PLvalid".to_string(),
            "https://www.youtube.com/watch?v=invalid".to_string(),
            "https://www.youtube.com/playlist?list=PLalsoValid".to_string(),
        ];

        let results = extract_playlist_ids(&urls);

        assert_eq!(results.len(), 3);
        assert!(results[0].is_ok());
        assert!(results[1].is_err());
        assert!(results[2].is_ok());
    }

    #[test]
    fn test_extract_playlist_ids_all_invalid() {
        let urls = vec![
            "https://www.youtube.com/watch?v=abc".to_string(),
            "https://youtu.be/def".to_string(),
            "invalid url".to_string(),
        ];

        let results = extract_playlist_ids(&urls);

        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_err()));
    }

    #[test]
    fn test_extract_playlist_ids_empty() {
        let urls: Vec<String> = vec![];

        let results = extract_playlist_ids(&urls);

        assert!(results.is_empty());
    }

    // ============== filter_valid_playlist_urls Tests ==============

    #[test]
    fn test_filter_valid_playlist_urls_all_valid() {
        let urls = vec![
            "https://www.youtube.com/playlist?list=PL1".to_string(),
            "https://www.youtube.com/playlist?list=PL2".to_string(),
            "https://www.youtube.com/playlist?list=PL3".to_string(),
        ];

        let valid = filter_valid_playlist_urls(&urls);

        assert_eq!(valid.len(), 3);
    }

    #[test]
    fn test_filter_valid_playlist_urls_all_invalid() {
        let urls = vec![
            "https://www.youtube.com/watch?v=abc".to_string(),
            "https://youtu.be/def".to_string(),
            "not a url".to_string(),
        ];

        let valid = filter_valid_playlist_urls(&urls);

        assert!(valid.is_empty());
    }

    #[test]
    fn test_filter_valid_playlist_urls_mixed() {
        let urls = vec![
            "https://www.youtube.com/playlist?list=PLvalid1".to_string(),
            "https://www.youtube.com/watch?v=videoOnly".to_string(),
            "https://www.youtube.com/watch?v=abc&list=PLvalid2".to_string(),
            "https://youtu.be/shortUrl".to_string(),
            "https://www.youtube.com/playlist?list=PLvalid3".to_string(),
        ];

        let valid = filter_valid_playlist_urls(&urls);

        assert_eq!(valid.len(), 3);
        assert!(valid[0].contains("PLvalid1"));
        assert!(valid[1].contains("PLvalid2"));
        assert!(valid[2].contains("PLvalid3"));
    }

    #[test]
    fn test_filter_valid_playlist_urls_empty() {
        let urls: Vec<String> = vec![];

        let valid = filter_valid_playlist_urls(&urls);

        assert!(valid.is_empty());
    }

    #[test]
    fn test_filter_valid_playlist_urls_preserves_original() {
        let urls = vec![
            "https://www.youtube.com/playlist?list=PLtest&extra=param".to_string(),
        ];

        let valid = filter_valid_playlist_urls(&urls);

        assert_eq!(valid.len(), 1);
        assert_eq!(*valid[0], urls[0]);
    }

    // ============== Async Method Tests (Basic) ==============

    #[tokio::test]
    async fn test_get_playlist_info_returns_struct() {
        let client = PlaylistClient::new();
        let result = client
            .get_playlist_info("https://www.youtube.com/playlist?list=PLtest123")
            .await;

        assert!(result.is_ok());

        let info = result.unwrap();
        assert_eq!(info.id, "PLtest123");
    }

    #[tokio::test]
    async fn test_get_playlist_info_invalid_url() {
        let client = PlaylistClient::new();
        let result = client
            .get_playlist_info("https://www.youtube.com/watch?v=noPlaylist")
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_video_ids_returns_vec() {
        let client = PlaylistClient::new();
        let result = client
            .get_video_ids("https://www.youtube.com/playlist?list=PLtest")
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_video_ids_invalid_url() {
        let client = PlaylistClient::new();
        let result = client
            .get_video_ids("https://www.youtube.com/watch?v=noPlaylist")
            .await;

        assert!(result.is_err());
    }

    // ============== Edge Cases ==============

    #[test]
    fn test_playlist_id_with_special_characters() {
        let result = PlaylistClient::get_playlist_id(
            "https://www.youtube.com/playlist?list=PL_abc-123_XYZ",
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "PL_abc-123_XYZ");
    }

    #[test]
    fn test_playlist_id_long() {
        let long_id = "PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf12345678901234567890";
        let url = format!("https://www.youtube.com/playlist?list={}", long_id);

        let result = PlaylistClient::get_playlist_id(&url);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), long_id);
    }

    #[test]
    fn test_is_playlist_url_case_sensitivity() {
        assert!(PlaylistClient::is_playlist_url(
            "https://www.youtube.com/playlist?list=PLtest"
        ));

        assert!(!PlaylistClient::is_playlist_url(
            "https://www.youtube.com/playlist?LIST=PLtest"
        ));
    }

    #[test]
    fn test_multiple_client_instances_independent() {
        let client1 = PlaylistClient::new();
        let client2 = PlaylistClient::new();

        assert!(PlaylistClient::is_playlist_url(
            "https://www.youtube.com/playlist?list=PLtest"
        ));
    }
}
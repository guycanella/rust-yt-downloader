use std::path::PathBuf;
use std::io::Write;
use futures::StreamExt;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::cli::{AudioFormat, VideoFormat, VideoQuality};
use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::progress::{messages, DownloadProgress};
use crate::utils::{apply_template, expand_path, sanitize_filename, VideoMetadata};
use crate::youtube::{QualityFilter, VideoInfo, YouTubeClient};

#[derive(Debug, Clone)]
pub struct DownloadOptions {
    pub output_dir: PathBuf,
    pub quality: VideoQuality,
    pub video_format: VideoFormat,
    pub audio_format: AudioFormat,
    pub audio_only: bool,
    pub filename_template: String,
    pub retry_attempts: u32,
    pub silence: bool,
    pub verbose: bool,
}

impl Default for DownloadOptions {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("."),
            quality: VideoQuality::Best,
            video_format: VideoFormat::Mp4,
            audio_format: AudioFormat::Mp3,
            audio_only: false,
            filename_template: "{title}".to_string(),
            retry_attempts: 3,
            silence: false,
            verbose: false,
        }
    }
}

impl DownloadOptions {
    pub fn from_config(config: &Config) -> Self {
        Self {
            output_dir: expand_path(&config.general.output_dir),
            quality: Self::parse_quality(&config.general.default_quality),
            video_format: Self::parse_video_format(&config.video.format),
            audio_format: Self::parse_audio_format(&config.audio.format),
            audio_only: false,
            filename_template: "{title}".to_string(),
            retry_attempts: config.network.retry_attempts,
            silence: false,
            verbose: false,
        }
    }

    pub fn with_output_dir(mut self, dir: PathBuf) -> Self {
        self.output_dir = dir;
        self
    }

    pub fn with_quality(mut self, quality: VideoQuality) -> Self {
        self.quality = quality;
        self
    }

    pub fn with_video_format(mut self, format: VideoFormat) -> Self {
        self.video_format = format;
        self
    }

    pub fn with_audio_format(mut self, format: AudioFormat) -> Self {
        self.audio_format = format;
        self
    }

    pub fn with_audio_only(mut self, audio_only: bool) -> Self {
        self.audio_only = audio_only;
        self
    }

    pub fn with_template(mut self, template: String) -> Self {
        self.filename_template = template;
        self
    }

    pub fn with_silence(mut self, silence: bool) -> Self {
        self.silence = silence;
        self
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    fn parse_quality(quality: &str) -> VideoQuality {
        match quality.to_lowercase().as_str() {
            "144p" => VideoQuality::Q144p,
            "240p" => VideoQuality::Q240p,
            "360p" => VideoQuality::Q360p,
            "480p" => VideoQuality::Q480p,
            "720p" => VideoQuality::Q720p,
            "1080p" => VideoQuality::Q1080p,
            "1440p" => VideoQuality::Q1440p,
            "4k" | "2160p" => VideoQuality::Q4k,
            "worst" => VideoQuality::Worst,
            _ => VideoQuality::Best,
        }
    }

    fn parse_video_format(format: &str) -> VideoFormat {
        match format.to_lowercase().as_str() {
            "mkv" => VideoFormat::Mkv,
            "webm" => VideoFormat::Webm,
            _ => VideoFormat::Mp4,
        }
    }

    fn parse_audio_format(format: &str) -> AudioFormat {
        match format.to_lowercase().as_str() {
            "m4a" => AudioFormat::M4a,
            "flac" => AudioFormat::Flac,
            "wav" => AudioFormat::Wav,
            "opus" => AudioFormat::Opus,
            _ => AudioFormat::Mp3,
        }
    }
}

#[derive(Debug)]
pub struct DownloadResult {
    pub file_path: PathBuf,
    pub file_size: u64,
    pub video_id: String,
    pub video_title: String,
}

pub struct Downloader {
    client: YouTubeClient,
    http_client: Client,
    options: DownloadOptions,
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            client: YouTubeClient::new(),
            http_client: Client::new(),
            options: DownloadOptions::default(),
        }
    }

    pub fn with_options(options: DownloadOptions) -> Self {
        Self {
            client: YouTubeClient::new(),
            http_client: Client::new(),
            options,
        }
    }

    pub fn from_config(config: &Config) -> Self {
        Self::with_options(DownloadOptions::from_config(config))
    }

    pub async fn download(&self, url: &str) -> AppResult<DownloadResult> {
        if !self.options.silence {
            messages::info(&format!("Fetching video info..."));
        }

        let video_info = self.client.get_video_info(url).await?;

        if self.options.verbose {
            messages::info(&format!("Title: {}", video_info.title));
            messages::info(&format!("Duration: {} seconds", video_info.duration));
        }

        let stream = self.select_stream(&video_info)?;

        if self.options.verbose {
            messages::info(&format!("Selected quality: {}", stream.quality));
            messages::info(&format!("Format: {}", stream.format));
        }

        let filename = self.generate_filename(&video_info);
        let file_path = self.options.output_dir.join(&filename);

        if let Some(parent) = file_path.parent() {
            if !parent.exists() {
                tokio::fs::create_dir_all(parent)
                    .await
                    .map_err(|e| AppError::dir_create(parent, e))?;
            }
        }

        let file_size = self.download_with_retry(&stream.url, &file_path).await?;

        if !self.options.silence {
            messages::success(&format!("Downloaded: {}", filename));
        }

        Ok(DownloadResult {
            file_path,
            file_size,
            video_id: video_info.id,
            video_title: video_info.title,
        })
    }

    pub async fn download_audio(&self, url: &str) -> AppResult<DownloadResult> {
        if !self.options.silence {
            messages::info("Fetching video info...");
        }

        let video_info = self.client.get_video_info(url).await?;

        if self.options.verbose {
            messages::info(&format!("Title: {}", video_info.title));
        }

        let stream = video_info
            .best_audio_stream()
            .ok_or_else(|| AppError::NoStreamsAvailable {
                video_id: video_info.id.clone(),
            })?;

        let filename = self.generate_audio_filename(&video_info);
        let file_path = self.options.output_dir.join(&filename);

        if let Some(parent) = file_path.parent() {
            if !parent.exists() {
                tokio::fs::create_dir_all(parent)
                    .await
                    .map_err(|e| AppError::dir_create(parent, e))?;
            }
        }

        let file_size = self.download_with_retry(&stream.url, &file_path).await?;

        if !self.options.silence {
            messages::success(&format!("Downloaded: {}", filename));
        }

        Ok(DownloadResult {
            file_path,
            file_size,
            video_id: video_info.id,
            video_title: video_info.title,
        })
    }

    fn select_stream<'a>(&self, video_info: &'a VideoInfo) -> AppResult<&'a crate::youtube::StreamInfo> {
        let filter = self.quality_to_filter();

        video_info
            .stream_by_filter(filter)
            .or_else(|| video_info.best_video_stream())
            .ok_or_else(|| {
                let available = video_info.available_qualities();
                AppError::QualityNotAvailable {
                    requested: format!("{:?}", self.options.quality),
                    available,
                }
            })
    }

    fn quality_to_filter(&self) -> QualityFilter {
        match self.options.quality {
            VideoQuality::Best => QualityFilter::Best,
            VideoQuality::Worst => QualityFilter::Worst,
            VideoQuality::Q144p => QualityFilter::Exact(144),
            VideoQuality::Q240p => QualityFilter::Exact(240),
            VideoQuality::Q360p => QualityFilter::Exact(360),
            VideoQuality::Q480p => QualityFilter::Exact(480),
            VideoQuality::Q720p => QualityFilter::Exact(720),
            VideoQuality::Q1080p => QualityFilter::Exact(1080),
            VideoQuality::Q1440p => QualityFilter::Exact(1440),
            VideoQuality::Q4k => QualityFilter::Exact(2160),
        }
    }

    fn generate_filename(&self, video_info: &VideoInfo) -> String {
        let meta = VideoMetadata {
            title: &video_info.title,
            id: &video_info.id,
            date: None,
            duration: Some(&crate::utils::format_duration(video_info.duration)),
        };

        let base_name = apply_template(&self.options.filename_template, &meta);
        let extension = self.get_video_extension();

        format!("{}.{}", base_name, extension)
    }

    fn generate_audio_filename(&self, video_info: &VideoInfo) -> String {
        let meta = VideoMetadata {
            title: &video_info.title,
            id: &video_info.id,
            date: None,
            duration: Some(&crate::utils::format_duration(video_info.duration)),
        };

        let base_name = apply_template(&self.options.filename_template, &meta);
        let extension = self.get_audio_extension();

        format!("{}.{}", base_name, extension)
    }

    fn get_video_extension(&self) -> &'static str {
        match self.options.video_format {
            VideoFormat::Mp4 => "mp4",
            VideoFormat::Mkv => "mkv",
            VideoFormat::Webm => "webm",
        }
    }

    fn get_audio_extension(&self) -> &'static str {
        match self.options.audio_format {
            AudioFormat::Mp3 => "mp3",
            AudioFormat::M4a => "m4a",
            AudioFormat::Flac => "flac",
            AudioFormat::Wav => "wav",
            AudioFormat::Opus => "opus",
        }
    }

    async fn download_with_retry(&self, url: &str, file_path: &PathBuf) -> AppResult<u64> {
        let mut last_error = None;

        for attempt in 1..=self.options.retry_attempts {
            match self.download_file(url, file_path).await {
                Ok(size) => return Ok(size),
                Err(e) => {
                    if e.is_retryable() && attempt < self.options.retry_attempts {
                        if !self.options.silence {
                            messages::warning(&format!(
                                "Download failed, retrying ({}/{})...",
                                attempt, self.options.retry_attempts
                            ));
                        }
                        last_error = Some(e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            AppError::MaxRetriesExceeded {
                attempts: self.options.retry_attempts,
                message: "Download failed".to_string(),
            }
        }))
    }

    async fn download_file(&self, url: &str, file_path: &PathBuf) -> AppResult<u64> {
        let response = self
            .http_client
            .get(url)
            .send()
            .await
            .map_err(|e| AppError::Network(e))?;

        if !response.status().is_success() {
            return Err(AppError::http(
                response.status().as_u16(),
                format!("Failed to download: {}", response.status()),
            ));
        }

        let total_size = response.content_length().unwrap_or(0);

        let progress = if !self.options.silence && total_size > 0 {
            Some(DownloadProgress::new(total_size))
        } else if !self.options.silence {
            Some(DownloadProgress::new_spinner("Downloading..."))
        } else {
            None
        };

        let mut file = File::create(file_path)
            .await
            .map_err(|e| AppError::file_write(file_path, e))?;

        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| AppError::Network(e))?;

            file.write_all(&chunk)
                .await
                .map_err(|e| AppError::file_write(file_path, e))?;

            downloaded += chunk.len() as u64;

            if let Some(ref pb) = progress {
                pb.set_position(downloaded);
            }
        }

        if let Some(pb) = progress {
            pb.finish();
        }

        Ok(downloaded)
    }

    pub fn options(&self) -> &DownloadOptions {
        &self.options
    }

    pub fn set_options(&mut self, options: DownloadOptions) {
        self.options = options;
    }
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}

// ==================================================
//          UNITARY TESTS
// ==================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // ============== DownloadOptions Default Tests ==============

    #[test]
    fn test_download_options_default() {
        let options = DownloadOptions::default();

        assert_eq!(options.output_dir, PathBuf::from("."));
        assert!(matches!(options.quality, VideoQuality::Best));
        assert!(matches!(options.video_format, VideoFormat::Mp4));
        assert!(matches!(options.audio_format, AudioFormat::Mp3));
        assert!(!options.audio_only);
        assert_eq!(options.filename_template, "{title}");
        assert_eq!(options.retry_attempts, 3);
        assert!(!options.silence);
        assert!(!options.verbose);
    }

    // ============== DownloadOptions Builder Tests ==============

    #[test]
    fn test_download_options_with_output_dir() {
        let options = DownloadOptions::default()
            .with_output_dir(PathBuf::from("/custom/path"));

        assert_eq!(options.output_dir, PathBuf::from("/custom/path"));
    }

    #[test]
    fn test_download_options_with_quality() {
        let options = DownloadOptions::default()
            .with_quality(VideoQuality::Q1080p);

        assert!(matches!(options.quality, VideoQuality::Q1080p));
    }

    #[test]
    fn test_download_options_with_video_format() {
        let options = DownloadOptions::default()
            .with_video_format(VideoFormat::Mkv);

        assert!(matches!(options.video_format, VideoFormat::Mkv));
    }

    #[test]
    fn test_download_options_with_audio_format() {
        let options = DownloadOptions::default()
            .with_audio_format(AudioFormat::Flac);

        assert!(matches!(options.audio_format, AudioFormat::Flac));
    }

    #[test]
    fn test_download_options_with_audio_only() {
        let options = DownloadOptions::default()
            .with_audio_only(true);

        assert!(options.audio_only);
    }

    #[test]
    fn test_download_options_with_template() {
        let options = DownloadOptions::default()
            .with_template("{title}-{id}".to_string());

        assert_eq!(options.filename_template, "{title}-{id}");
    }

    #[test]
    fn test_download_options_with_silence() {
        let options = DownloadOptions::default()
            .with_silence(true);

        assert!(options.silence);
    }

    #[test]
    fn test_download_options_with_verbose() {
        let options = DownloadOptions::default()
            .with_verbose(true);

        assert!(options.verbose);
    }

    #[test]
    fn test_download_options_builder_chain() {
        let options = DownloadOptions::default()
            .with_output_dir(PathBuf::from("/downloads"))
            .with_quality(VideoQuality::Q720p)
            .with_video_format(VideoFormat::Webm)
            .with_audio_format(AudioFormat::Opus)
            .with_audio_only(false)
            .with_template("{title}-{date}".to_string())
            .with_silence(true)
            .with_verbose(false);

        assert_eq!(options.output_dir, PathBuf::from("/downloads"));
        assert!(matches!(options.quality, VideoQuality::Q720p));
        assert!(matches!(options.video_format, VideoFormat::Webm));
        assert!(matches!(options.audio_format, AudioFormat::Opus));
        assert!(!options.audio_only);
        assert_eq!(options.filename_template, "{title}-{date}");
        assert!(options.silence);
        assert!(!options.verbose);
    }

    // ============== DownloadOptions Parse Tests ==============

    #[test]
    fn test_parse_quality_144p() {
        let quality = DownloadOptions::parse_quality("144p");
        assert!(matches!(quality, VideoQuality::Q144p));
    }

    #[test]
    fn test_parse_quality_240p() {
        let quality = DownloadOptions::parse_quality("240p");
        assert!(matches!(quality, VideoQuality::Q240p));
    }

    #[test]
    fn test_parse_quality_360p() {
        let quality = DownloadOptions::parse_quality("360p");
        assert!(matches!(quality, VideoQuality::Q360p));
    }

    #[test]
    fn test_parse_quality_480p() {
        let quality = DownloadOptions::parse_quality("480p");
        assert!(matches!(quality, VideoQuality::Q480p));
    }

    #[test]
    fn test_parse_quality_720p() {
        let quality = DownloadOptions::parse_quality("720p");
        assert!(matches!(quality, VideoQuality::Q720p));
    }

    #[test]
    fn test_parse_quality_1080p() {
        let quality = DownloadOptions::parse_quality("1080p");
        assert!(matches!(quality, VideoQuality::Q1080p));
    }

    #[test]
    fn test_parse_quality_1440p() {
        let quality = DownloadOptions::parse_quality("1440p");
        assert!(matches!(quality, VideoQuality::Q1440p));
    }

    #[test]
    fn test_parse_quality_4k() {
        let quality = DownloadOptions::parse_quality("4k");
        assert!(matches!(quality, VideoQuality::Q4k));
    }

    #[test]
    fn test_parse_quality_2160p() {
        let quality = DownloadOptions::parse_quality("2160p");
        assert!(matches!(quality, VideoQuality::Q4k));
    }

    #[test]
    fn test_parse_quality_best() {
        let quality = DownloadOptions::parse_quality("best");
        assert!(matches!(quality, VideoQuality::Best));
    }

    #[test]
    fn test_parse_quality_worst() {
        let quality = DownloadOptions::parse_quality("worst");
        assert!(matches!(quality, VideoQuality::Worst));
    }

    #[test]
    fn test_parse_quality_case_insensitive() {
        assert!(matches!(DownloadOptions::parse_quality("1080P"), VideoQuality::Q1080p));
        assert!(matches!(DownloadOptions::parse_quality("BEST"), VideoQuality::Best));
        assert!(matches!(DownloadOptions::parse_quality("4K"), VideoQuality::Q4k));
    }

    #[test]
    fn test_parse_quality_unknown_defaults_to_best() {
        let quality = DownloadOptions::parse_quality("unknown");
        assert!(matches!(quality, VideoQuality::Best));
    }

    #[test]
    fn test_parse_video_format_mp4() {
        let format = DownloadOptions::parse_video_format("mp4");
        assert!(matches!(format, VideoFormat::Mp4));
    }

    #[test]
    fn test_parse_video_format_mkv() {
        let format = DownloadOptions::parse_video_format("mkv");
        assert!(matches!(format, VideoFormat::Mkv));
    }

    #[test]
    fn test_parse_video_format_webm() {
        let format = DownloadOptions::parse_video_format("webm");
        assert!(matches!(format, VideoFormat::Webm));
    }

    #[test]
    fn test_parse_video_format_case_insensitive() {
        assert!(matches!(DownloadOptions::parse_video_format("MP4"), VideoFormat::Mp4));
        assert!(matches!(DownloadOptions::parse_video_format("MKV"), VideoFormat::Mkv));
    }

    #[test]
    fn test_parse_video_format_unknown_defaults_to_mp4() {
        let format = DownloadOptions::parse_video_format("avi");
        assert!(matches!(format, VideoFormat::Mp4));
    }

    #[test]
    fn test_parse_audio_format_mp3() {
        let format = DownloadOptions::parse_audio_format("mp3");
        assert!(matches!(format, AudioFormat::Mp3));
    }

    #[test]
    fn test_parse_audio_format_m4a() {
        let format = DownloadOptions::parse_audio_format("m4a");
        assert!(matches!(format, AudioFormat::M4a));
    }

    #[test]
    fn test_parse_audio_format_flac() {
        let format = DownloadOptions::parse_audio_format("flac");
        assert!(matches!(format, AudioFormat::Flac));
    }

    #[test]
    fn test_parse_audio_format_wav() {
        let format = DownloadOptions::parse_audio_format("wav");
        assert!(matches!(format, AudioFormat::Wav));
    }

    #[test]
    fn test_parse_audio_format_opus() {
        let format = DownloadOptions::parse_audio_format("opus");
        assert!(matches!(format, AudioFormat::Opus));
    }

    #[test]
    fn test_parse_audio_format_case_insensitive() {
        assert!(matches!(DownloadOptions::parse_audio_format("MP3"), AudioFormat::Mp3));
        assert!(matches!(DownloadOptions::parse_audio_format("FLAC"), AudioFormat::Flac));
    }

    #[test]
    fn test_parse_audio_format_unknown_defaults_to_mp3() {
        let format = DownloadOptions::parse_audio_format("wma");
        assert!(matches!(format, AudioFormat::Mp3));
    }

    // ============== DownloadOptions from_config Tests ==============

    #[test]
    fn test_download_options_from_config() {
        let config = Config::default();
        let options = DownloadOptions::from_config(&config);

        assert!(matches!(options.quality, VideoQuality::Best));
        assert!(matches!(options.video_format, VideoFormat::Mp4));
        assert!(matches!(options.audio_format, AudioFormat::Mp3));
        assert_eq!(options.retry_attempts, 3);
    }

    // ============== Downloader Creation Tests ==============

    #[test]
    fn test_downloader_new() {
        let downloader = Downloader::new();
        assert!(matches!(downloader.options().quality, VideoQuality::Best));
    }

    #[test]
    fn test_downloader_default() {
        let downloader = Downloader::default();
        assert!(matches!(downloader.options().quality, VideoQuality::Best));
    }

    #[test]
    fn test_downloader_with_options() {
        let options = DownloadOptions::default()
            .with_quality(VideoQuality::Q720p)
            .with_silence(true);

        let downloader = Downloader::with_options(options);

        assert!(matches!(downloader.options().quality, VideoQuality::Q720p));
        assert!(downloader.options().silence);
    }

    #[test]
    fn test_downloader_from_config() {
        let config = Config::default();
        let downloader = Downloader::from_config(&config);

        assert_eq!(downloader.options().retry_attempts, config.network.retry_attempts);
    }

    #[test]
    fn test_downloader_set_options() {
        let mut downloader = Downloader::new();

        let new_options = DownloadOptions::default()
            .with_quality(VideoQuality::Q480p);

        downloader.set_options(new_options);

        assert!(matches!(downloader.options().quality, VideoQuality::Q480p));
    }

    // ============== Downloader quality_to_filter Tests ==============

    #[test]
    fn test_quality_to_filter_best() {
        let downloader = Downloader::with_options(
            DownloadOptions::default().with_quality(VideoQuality::Best)
        );
        let filter = downloader.quality_to_filter();
        assert!(matches!(filter, QualityFilter::Best));
    }

    #[test]
    fn test_quality_to_filter_worst() {
        let downloader = Downloader::with_options(
            DownloadOptions::default().with_quality(VideoQuality::Worst)
        );
        let filter = downloader.quality_to_filter();
        assert!(matches!(filter, QualityFilter::Worst));
    }

    #[test]
    fn test_quality_to_filter_144p() {
        let downloader = Downloader::with_options(
            DownloadOptions::default().with_quality(VideoQuality::Q144p)
        );
        let filter = downloader.quality_to_filter();
        assert!(matches!(filter, QualityFilter::Exact(144)));
    }

    #[test]
    fn test_quality_to_filter_240p() {
        let downloader = Downloader::with_options(
            DownloadOptions::default().with_quality(VideoQuality::Q240p)
        );
        let filter = downloader.quality_to_filter();
        assert!(matches!(filter, QualityFilter::Exact(240)));
    }

    #[test]
    fn test_quality_to_filter_360p() {
        let downloader = Downloader::with_options(
            DownloadOptions::default().with_quality(VideoQuality::Q360p)
        );
        let filter = downloader.quality_to_filter();
        assert!(matches!(filter, QualityFilter::Exact(360)));
    }

    #[test]
    fn test_quality_to_filter_480p() {
        let downloader = Downloader::with_options(
            DownloadOptions::default().with_quality(VideoQuality::Q480p)
        );
        let filter = downloader.quality_to_filter();
        assert!(matches!(filter, QualityFilter::Exact(480)));
    }

    #[test]
    fn test_quality_to_filter_720p() {
        let downloader = Downloader::with_options(
            DownloadOptions::default().with_quality(VideoQuality::Q720p)
        );
        let filter = downloader.quality_to_filter();
        assert!(matches!(filter, QualityFilter::Exact(720)));
    }

    #[test]
    fn test_quality_to_filter_1080p() {
        let downloader = Downloader::with_options(
            DownloadOptions::default().with_quality(VideoQuality::Q1080p)
        );
        let filter = downloader.quality_to_filter();
        assert!(matches!(filter, QualityFilter::Exact(1080)));
    }

    #[test]
    fn test_quality_to_filter_1440p() {
        let downloader = Downloader::with_options(
            DownloadOptions::default().with_quality(VideoQuality::Q1440p)
        );
        let filter = downloader.quality_to_filter();
        assert!(matches!(filter, QualityFilter::Exact(1440)));
    }

    #[test]
    fn test_quality_to_filter_4k() {
        let downloader = Downloader::with_options(
            DownloadOptions::default().with_quality(VideoQuality::Q4k)
        );
        let filter = downloader.quality_to_filter();
        assert!(matches!(filter, QualityFilter::Exact(2160)));
    }

    // ============== Downloader Extension Tests ==============

    #[test]
    fn test_get_video_extension_mp4() {
        let downloader = Downloader::with_options(
            DownloadOptions::default().with_video_format(VideoFormat::Mp4)
        );
        assert_eq!(downloader.get_video_extension(), "mp4");
    }

    #[test]
    fn test_get_video_extension_mkv() {
        let downloader = Downloader::with_options(
            DownloadOptions::default().with_video_format(VideoFormat::Mkv)
        );
        assert_eq!(downloader.get_video_extension(), "mkv");
    }

    #[test]
    fn test_get_video_extension_webm() {
        let downloader = Downloader::with_options(
            DownloadOptions::default().with_video_format(VideoFormat::Webm)
        );
        assert_eq!(downloader.get_video_extension(), "webm");
    }

    #[test]
    fn test_get_audio_extension_mp3() {
        let downloader = Downloader::with_options(
            DownloadOptions::default().with_audio_format(AudioFormat::Mp3)
        );
        assert_eq!(downloader.get_audio_extension(), "mp3");
    }

    #[test]
    fn test_get_audio_extension_m4a() {
        let downloader = Downloader::with_options(
            DownloadOptions::default().with_audio_format(AudioFormat::M4a)
        );
        assert_eq!(downloader.get_audio_extension(), "m4a");
    }

    #[test]
    fn test_get_audio_extension_flac() {
        let downloader = Downloader::with_options(
            DownloadOptions::default().with_audio_format(AudioFormat::Flac)
        );
        assert_eq!(downloader.get_audio_extension(), "flac");
    }

    #[test]
    fn test_get_audio_extension_wav() {
        let downloader = Downloader::with_options(
            DownloadOptions::default().with_audio_format(AudioFormat::Wav)
        );
        assert_eq!(downloader.get_audio_extension(), "wav");
    }

    #[test]
    fn test_get_audio_extension_opus() {
        let downloader = Downloader::with_options(
            DownloadOptions::default().with_audio_format(AudioFormat::Opus)
        );
        assert_eq!(downloader.get_audio_extension(), "opus");
    }

    // ============== Downloader Filename Generation Tests ==============

    #[test]
    fn test_generate_filename_default_template() {
        let downloader = Downloader::new();
        let video_info = VideoInfo {
            id: "abc123".to_string(),
            title: "Test Video".to_string(),
            description: None,
            duration: 120,
            thumbnail_url: None,
            channel: None,
            publish_date: None,
            view_count: None,
            streams: vec![],
        };

        let filename = downloader.generate_filename(&video_info);

        assert!(filename.starts_with("Test Video"));
        assert!(filename.ends_with(".mp4"));
    }

    #[test]
    fn test_generate_filename_custom_template() {
        let downloader = Downloader::with_options(
            DownloadOptions::default()
                .with_template("{title}-{id}".to_string())
        );
        let video_info = VideoInfo {
            id: "abc123".to_string(),
            title: "Test Video".to_string(),
            description: None,
            duration: 120,
            thumbnail_url: None,
            channel: None,
            publish_date: None,
            view_count: None,
            streams: vec![],
        };

        let filename = downloader.generate_filename(&video_info);

        assert!(filename.contains("Test Video"));
        assert!(filename.contains("abc123"));
        assert!(filename.ends_with(".mp4"));
    }

    #[test]
    fn test_generate_filename_different_format() {
        let downloader = Downloader::with_options(
            DownloadOptions::default()
                .with_video_format(VideoFormat::Mkv)
        );
        let video_info = VideoInfo {
            id: "abc123".to_string(),
            title: "Test Video".to_string(),
            description: None,
            duration: 120,
            thumbnail_url: None,
            channel: None,
            publish_date: None,
            view_count: None,
            streams: vec![],
        };

        let filename = downloader.generate_filename(&video_info);

        assert!(filename.ends_with(".mkv"));
    }

    #[test]
    fn test_generate_audio_filename_default() {
        let downloader = Downloader::new();
        let video_info = VideoInfo {
            id: "abc123".to_string(),
            title: "Test Song".to_string(),
            description: None,
            duration: 180,
            thumbnail_url: None,
            channel: None,
            publish_date: None,
            view_count: None,
            streams: vec![],
        };

        let filename = downloader.generate_audio_filename(&video_info);

        assert!(filename.starts_with("Test Song"));
        assert!(filename.ends_with(".mp3"));
    }

    #[test]
    fn test_generate_audio_filename_flac() {
        let downloader = Downloader::with_options(
            DownloadOptions::default()
                .with_audio_format(AudioFormat::Flac)
        );
        let video_info = VideoInfo {
            id: "abc123".to_string(),
            title: "Test Song".to_string(),
            description: None,
            duration: 180,
            thumbnail_url: None,
            channel: None,
            publish_date: None,
            view_count: None,
            streams: vec![],
        };

        let filename = downloader.generate_audio_filename(&video_info);

        assert!(filename.ends_with(".flac"));
    }

    #[test]
    fn test_generate_filename_sanitizes_title() {
        let downloader = Downloader::new();
        let video_info = VideoInfo {
            id: "abc123".to_string(),
            title: "Test: Video? With <Invalid> Chars".to_string(),
            description: None,
            duration: 120,
            thumbnail_url: None,
            channel: None,
            publish_date: None,
            view_count: None,
            streams: vec![],
        };

        let filename = downloader.generate_filename(&video_info);

        assert!(!filename.contains(':'));
        assert!(!filename.contains('?'));
        assert!(!filename.contains('<'));
        assert!(!filename.contains('>'));
    }

    // ============== DownloadResult Tests ==============

    #[test]
    fn test_download_result_creation() {
        let result = DownloadResult {
            file_path: PathBuf::from("/downloads/video.mp4"),
            file_size: 1024 * 1024 * 100, // 100 MB
            video_id: "abc123".to_string(),
            video_title: "Test Video".to_string(),
        };

        assert_eq!(result.file_path, PathBuf::from("/downloads/video.mp4"));
        assert_eq!(result.file_size, 104857600);
        assert_eq!(result.video_id, "abc123");
        assert_eq!(result.video_title, "Test Video");
    }

    // ============== Multiple Downloader Instances ==============

    #[test]
    fn test_multiple_downloader_instances() {
        let downloader1 = Downloader::with_options(
            DownloadOptions::default().with_quality(VideoQuality::Q720p)
        );
        let downloader2 = Downloader::with_options(
            DownloadOptions::default().with_quality(VideoQuality::Q1080p)
        );

        assert!(matches!(downloader1.options().quality, VideoQuality::Q720p));
        assert!(matches!(downloader2.options().quality, VideoQuality::Q1080p));
    }

    // ============== Clone Tests ==============

    #[test]
    fn test_download_options_clone() {
        let options = DownloadOptions::default()
            .with_quality(VideoQuality::Q720p)
            .with_output_dir(PathBuf::from("/test"));

        let cloned = options.clone();

        assert!(matches!(cloned.quality, VideoQuality::Q720p));
        assert_eq!(cloned.output_dir, PathBuf::from("/test"));
    }

    #[test]
    fn test_download_options_clone_independent() {
        let options = DownloadOptions::default();
        let mut cloned = options.clone();

        cloned.silence = true;

        assert!(!options.silence);
        assert!(cloned.silence);
    }

    // ============== Debug Tests ==============

    #[test]
    fn test_download_options_debug() {
        let options = DownloadOptions::default();
        let debug_str = format!("{:?}", options);

        assert!(debug_str.contains("DownloadOptions"));
        assert!(debug_str.contains("output_dir"));
        assert!(debug_str.contains("quality"));
    }

    #[test]
    fn test_download_result_debug() {
        let result = DownloadResult {
            file_path: PathBuf::from("/test.mp4"),
            file_size: 1000,
            video_id: "test".to_string(),
            video_title: "Test".to_string(),
        };

        let debug_str = format!("{:?}", result);

        assert!(debug_str.contains("DownloadResult"));
        assert!(debug_str.contains("file_path"));
    }
}
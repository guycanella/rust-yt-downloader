//! Video format conversion and transcoding.
//!
//! This module provides video conversion capabilities including:
//! - Converting between video container formats (MP4, MKV, WebM, etc.)
//! - Stream copying for fast format changes
//! - Re-encoding with quality and codec control
//! - Resolution and framerate adjustments
//!
//! # Format Conversion Logic
//!
//! Video conversion supports two modes:
//!
//! 1. **Stream Copy** (`-c copy`): Fast, lossless container format change without re-encoding.
//!    Only works when codecs are compatible with the target container.
//!
//! 2. **Re-encoding**: Slower but works with any format combination. Allows quality,
//!    resolution, and codec customization.
//!
//! # Example
//!
//! ```no_run
//! use rust_yt_downloader::media::{VideoConverter, ConversionOptions, VideoFormat};
//!
//! // Fast conversion using stream copy
//! let options = ConversionOptions::fast(VideoFormat::Mkv);
//! VideoConverter::convert("video.mp4", "video.mkv", &options)?;
//!
//! // High quality re-encode
//! let options = ConversionOptions::high_quality(VideoFormat::Mp4);
//! VideoConverter::convert("video.avi", "video.mp4", &options)?;
//! # Ok::<(), rust_yt_downloader::error::AppError>(())
//! ```

use std::path::{Path, PathBuf};

use crate::error::{AppError, AppResult};
use crate::media::ffmpeg::FFmpeg;

/// Supported video container formats.
///
/// Each format represents a container that can hold various video and audio codecs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoFormat {
    /// MP4 container (H.264/AAC, widely compatible).
    Mp4,
    /// Matroska container (supports all codecs).
    Mkv,
    /// WebM container (VP8/VP9/Opus, web-optimized).
    Webm,
    /// AVI container (legacy format).
    Avi,
    /// QuickTime MOV container (similar to MP4).
    Mov,
}

impl VideoFormat {
    /// Returns the file extension for this video format.
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Mp4 => "mp4",
            Self::Mkv => "mkv",
            Self::Webm => "webm",
            Self::Avi => "avi",
            Self::Mov => "mov",
        }
    }

    /// Parses a video format from a file extension.
    ///
    /// # Arguments
    ///
    /// * `ext` - File extension (case-insensitive)
    ///
    /// # Returns
    ///
    /// `Some(VideoFormat)` if recognized, `None` otherwise.
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "mp4" => Some(Self::Mp4),
            "mkv" => Some(Self::Mkv),
            "webm" => Some(Self::Webm),
            "avi" => Some(Self::Avi),
            "mov" => Some(Self::Mov),
            _ => None,
        }
    }

    /// Returns the recommended FFmpeg video codec for this format.
    pub fn recommended_video_codec(&self) -> &'static str {
        match self {
            Self::Mp4 => "libx264",
            Self::Mkv => "libx264",
            Self::Webm => "libvpx-vp9",
            Self::Avi => "mpeg4",
            Self::Mov => "libx264",
        }
    }

    /// Returns the recommended FFmpeg audio codec for this format.
    pub fn recommended_audio_codec(&self) -> &'static str {
        match self {
            Self::Mp4 => "aac",
            Self::Mkv => "aac",
            Self::Webm => "libopus",
            Self::Avi => "mp3",
            Self::Mov => "aac",
        }
    }

    /// Checks if this format can receive stream copy from the source format.
    ///
    /// Stream copy is possible when container formats are compatible without re-encoding.
    /// MKV accepts streams from all formats. MP4 and MOV are mutually compatible.
    ///
    /// # Arguments
    ///
    /// * `source` - The source video format
    ///
    /// # Returns
    ///
    /// `true` if stream copy is supported, `false` if re-encoding is required.
    pub fn supports_stream_copy_from(&self, source: &VideoFormat) -> bool {
        match (source, self) {
            (Self::Mp4, Self::Mov) | (Self::Mov, Self::Mp4) => true,
            (_, Self::Mkv) => true,
            (a, b) if a == b => true,
            _ => false,
        }
    }
}

/// Video conversion options controlling encoding and quality.
///
/// Configures how video conversion is performed, including codec selection,
/// quality settings, and whether to use stream copy or re-encoding.
#[derive(Debug, Clone)]
pub struct ConversionOptions {
    /// Target video format.
    pub output_format: VideoFormat,
    /// Use stream copy (fast) vs re-encoding (slower but customizable).
    pub stream_copy: bool,
    /// Custom video codec (overrides recommended codec).
    pub video_codec: Option<String>,
    /// Custom audio codec (overrides recommended codec).
    pub audio_codec: Option<String>,
    /// Video quality (CRF: 0-51, lower = better quality, 18-28 typical).
    pub video_quality: Option<u8>,
    /// Audio bitrate (e.g., "192k", "320k").
    pub audio_bitrate: Option<String>,
    /// Output resolution (e.g., "1920x1080", "1280x720").
    pub resolution: Option<String>,
    /// Output framerate in fps (e.g., 30, 60).
    pub framerate: Option<u32>,
    /// Whether to overwrite existing output files.
    pub overwrite: bool,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            output_format: VideoFormat::Mp4,
            stream_copy: true,
            video_codec: None,
            audio_codec: None,
            video_quality: None,
            audio_bitrate: None,
            resolution: None,
            framerate: None,
            overwrite: true,
        }
    }
}

impl ConversionOptions {
    /// Creates options for fast conversion using stream copy.
    pub fn fast(format: VideoFormat) -> Self {
        Self {
            output_format: format,
            stream_copy: true,
            ..Default::default()
        }
    }

    /// Creates options for re-encoding with default codecs.
    pub fn reencode(format: VideoFormat) -> Self {
        Self {
            output_format: format,
            stream_copy: false,
            ..Default::default()
        }
    }

    /// Creates options for high-quality re-encoding (CRF 18, 320k audio).
    pub fn high_quality(format: VideoFormat) -> Self {
        Self {
            output_format: format,
            stream_copy: false,
            video_quality: Some(18), // CRF 18 = high quality
            audio_bitrate: Some("320k".to_string()),
            ..Default::default()
        }
    }

    /// Creates options for smaller file size (CRF 28, 128k audio).
    pub fn small_file(format: VideoFormat) -> Self {
        Self {
            output_format: format,
            stream_copy: false,
            video_quality: Some(28), // CRF 28 = smaller file
            audio_bitrate: Some("128k".to_string()),
            ..Default::default()
        }
    }

    /// Sets the output format (builder pattern).
    pub fn with_format(mut self, format: VideoFormat) -> Self {
        self.output_format = format;
        self
    }

    /// Enables or disables stream copy mode (builder pattern).
    pub fn with_stream_copy(mut self, stream_copy: bool) -> Self {
        self.stream_copy = stream_copy;
        self
    }

    /// Sets a custom video codec (builder pattern).
    pub fn with_video_codec(mut self, codec: impl Into<String>) -> Self {
        self.video_codec = Some(codec.into());
        self
    }

    /// Sets a custom audio codec (builder pattern).
    pub fn with_audio_codec(mut self, codec: impl Into<String>) -> Self {
        self.audio_codec = Some(codec.into());
        self
    }

    /// Sets video quality using CRF value (builder pattern).
    /// CRF range: 0-51, where 0 is lossless and 51 is worst quality.
    /// Typical values: 18 (high quality), 23 (default), 28 (low quality).
    pub fn with_quality(mut self, crf: u8) -> Self {
        self.video_quality = Some(crf);
        self
    }

    /// Sets audio bitrate (builder pattern).
    pub fn with_audio_bitrate(mut self, bitrate: impl Into<String>) -> Self {
        self.audio_bitrate = Some(bitrate.into());
        self
    }

    /// Sets output resolution (builder pattern).
    pub fn with_resolution(mut self, resolution: impl Into<String>) -> Self {
        self.resolution = Some(resolution.into());
        self
    }

    /// Sets output framerate (builder pattern).
    pub fn with_framerate(mut self, fps: u32) -> Self {
        self.framerate = Some(fps);
        self
    }
}

/// Video conversion and format detection utilities.
///
/// Provides methods to convert videos between formats, detect formats from file paths,
/// and determine if re-encoding is necessary.
pub struct VideoConverter;

impl VideoConverter {
    /// Converts a video file to a different format with custom options.
    ///
    /// This is the primary conversion method that builds FFmpeg commands based on the
    /// provided options, supporting both stream copy and re-encoding modes.
    ///
    /// # Arguments
    ///
    /// * `input` - Path to the input video file
    /// * `output` - Path to the output video file
    /// * `options` - Conversion options controlling codec, quality, etc.
    ///
    /// # Errors
    ///
    /// Returns an error if FFmpeg is not available or conversion fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rust_yt_downloader::media::{VideoConverter, ConversionOptions, VideoFormat};
    ///
    /// let options = ConversionOptions::high_quality(VideoFormat::Mp4);
    /// VideoConverter::convert("input.avi", "output.mp4", &options)?;
    /// # Ok::<(), rust_yt_downloader::error::AppError>(())
    /// ```
    pub fn convert<P: AsRef<Path>>(input: P, output: P, options: &ConversionOptions) -> AppResult<()> {
        FFmpeg::require()?;

        let input_str = input.as_ref().to_string_lossy();
        let output_str = output.as_ref().to_string_lossy();

        let mut args: Vec<String> = Vec::new();

        if options.overwrite {
            args.push("-y".to_string());
        }

        args.push("-i".to_string());
        args.push(input_str.to_string());

        if options.stream_copy {
            args.push("-c".to_string());
            args.push("copy".to_string());
        } else {
            args.push("-c:v".to_string());
            args.push(
                options
                    .video_codec
                    .clone()
                    .unwrap_or_else(|| options.output_format.recommended_video_codec().to_string()),
            );

            if let Some(crf) = options.video_quality {
                args.push("-crf".to_string());
                args.push(crf.to_string());
            }

            args.push("-c:a".to_string());
            args.push(
                options
                    .audio_codec
                    .clone()
                    .unwrap_or_else(|| options.output_format.recommended_audio_codec().to_string()),
            );

            if let Some(ref bitrate) = options.audio_bitrate {
                args.push("-b:a".to_string());
                args.push(bitrate.clone());
            }

            if let Some(ref resolution) = options.resolution {
                args.push("-s".to_string());
                args.push(resolution.clone());
            }

            if let Some(fps) = options.framerate {
                args.push("-r".to_string());
                args.push(fps.to_string());
            }
        }

        args.push(output_str.to_string());

        let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        FFmpeg::run(&args_ref)?;

        Ok(())
    }

    /// Converts a video using fast stream copy (no re-encoding).
    pub fn convert_fast<P: AsRef<Path>>(input: P, output: P) -> AppResult<()> {
        FFmpeg::convert(input, output)
    }

    /// Converts a video with re-encoding using default codecs.
    pub fn convert_reencode<P: AsRef<Path>>(input: P, output: P) -> AppResult<()> {
        FFmpeg::convert_reencode(input, output)
    }

    /// Detects video format from file extension.
    ///
    /// # Returns
    ///
    /// `Some(VideoFormat)` if the extension is recognized, `None` otherwise.
    pub fn detect_format<P: AsRef<Path>>(path: P) -> Option<VideoFormat> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(VideoFormat::from_extension)
    }

    /// Generates an output path by changing the file extension to match the target format.
    ///
    /// # Arguments
    ///
    /// * `input` - Original file path
    /// * `format` - Target video format
    ///
    /// # Returns
    ///
    /// Path with the extension changed to match the format.
    pub fn output_path_with_format<P: AsRef<Path>>(input: P, format: VideoFormat) -> PathBuf {
        let input_path = input.as_ref();
        let stem = input_path.file_stem().unwrap_or_default();

        input_path
            .parent()
            .unwrap_or(Path::new("."))
            .join(format!("{}.{}", stem.to_string_lossy(), format.extension()))
    }

    /// Determines if re-encoding is required for the conversion.
    ///
    /// Checks if the input and output formats support stream copy.
    /// If not, re-encoding is necessary.
    ///
    /// # Arguments
    ///
    /// * `input` - Input file path
    /// * `output_format` - Target video format
    ///
    /// # Returns
    ///
    /// `true` if re-encoding is required, `false` if stream copy can be used.
    pub fn needs_reencode<P: AsRef<Path>>(input: P, output_format: VideoFormat) -> bool {
        let input_format = Self::detect_format(&input);

        match input_format {
            Some(fmt) => !output_format.supports_stream_copy_from(&fmt),
            None => true,
        }
    }
}

/// Result information from a video conversion operation.
///
/// Contains details about the completed conversion including the output path,
/// format, and whether stream copy was used.
#[derive(Debug)]
pub struct ConversionResult {
    /// Path to the converted output file.
    pub output_path: PathBuf,
    /// Output video format.
    pub format: VideoFormat,
    /// Whether stream copy was used (fast) vs re-encoding (slow).
    pub used_stream_copy: bool,
}

// ==================================================
//          UNITARY TESTS
// ==================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ============== VideoFormat Extension Tests ==============

    #[test]
    fn test_video_format_extension_mp4() {
        assert_eq!(VideoFormat::Mp4.extension(), "mp4");
    }

    #[test]
    fn test_video_format_extension_mkv() {
        assert_eq!(VideoFormat::Mkv.extension(), "mkv");
    }

    #[test]
    fn test_video_format_extension_webm() {
        assert_eq!(VideoFormat::Webm.extension(), "webm");
    }

    #[test]
    fn test_video_format_extension_avi() {
        assert_eq!(VideoFormat::Avi.extension(), "avi");
    }

    #[test]
    fn test_video_format_extension_mov() {
        assert_eq!(VideoFormat::Mov.extension(), "mov");
    }

    // ============== VideoFormat from_extension Tests ==============

    #[test]
    fn test_video_format_from_extension_mp4() {
        assert_eq!(VideoFormat::from_extension("mp4"), Some(VideoFormat::Mp4));
    }

    #[test]
    fn test_video_format_from_extension_mkv() {
        assert_eq!(VideoFormat::from_extension("mkv"), Some(VideoFormat::Mkv));
    }

    #[test]
    fn test_video_format_from_extension_webm() {
        assert_eq!(VideoFormat::from_extension("webm"), Some(VideoFormat::Webm));
    }

    #[test]
    fn test_video_format_from_extension_avi() {
        assert_eq!(VideoFormat::from_extension("avi"), Some(VideoFormat::Avi));
    }

    #[test]
    fn test_video_format_from_extension_mov() {
        assert_eq!(VideoFormat::from_extension("mov"), Some(VideoFormat::Mov));
    }

    #[test]
    fn test_video_format_from_extension_unknown() {
        assert_eq!(VideoFormat::from_extension("xyz"), None);
    }

    #[test]
    fn test_video_format_from_extension_empty() {
        assert_eq!(VideoFormat::from_extension(""), None);
    }

    #[test]
    fn test_video_format_from_extension_case_insensitive() {
        assert_eq!(VideoFormat::from_extension("MP4"), Some(VideoFormat::Mp4));
        assert_eq!(VideoFormat::from_extension("MKV"), Some(VideoFormat::Mkv));
        assert_eq!(VideoFormat::from_extension("WebM"), Some(VideoFormat::Webm));
    }

    // ============== VideoFormat Codec Tests ==============

    #[test]
    fn test_video_format_recommended_video_codec_mp4() {
        assert_eq!(VideoFormat::Mp4.recommended_video_codec(), "libx264");
    }

    #[test]
    fn test_video_format_recommended_video_codec_mkv() {
        assert_eq!(VideoFormat::Mkv.recommended_video_codec(), "libx264");
    }

    #[test]
    fn test_video_format_recommended_video_codec_webm() {
        assert_eq!(VideoFormat::Webm.recommended_video_codec(), "libvpx-vp9");
    }

    #[test]
    fn test_video_format_recommended_video_codec_avi() {
        assert_eq!(VideoFormat::Avi.recommended_video_codec(), "mpeg4");
    }

    #[test]
    fn test_video_format_recommended_video_codec_mov() {
        assert_eq!(VideoFormat::Mov.recommended_video_codec(), "libx264");
    }

    #[test]
    fn test_video_format_recommended_audio_codec_mp4() {
        assert_eq!(VideoFormat::Mp4.recommended_audio_codec(), "aac");
    }

    #[test]
    fn test_video_format_recommended_audio_codec_mkv() {
        assert_eq!(VideoFormat::Mkv.recommended_audio_codec(), "aac");
    }

    #[test]
    fn test_video_format_recommended_audio_codec_webm() {
        assert_eq!(VideoFormat::Webm.recommended_audio_codec(), "libopus");
    }

    #[test]
    fn test_video_format_recommended_audio_codec_avi() {
        assert_eq!(VideoFormat::Avi.recommended_audio_codec(), "mp3");
    }

    #[test]
    fn test_video_format_recommended_audio_codec_mov() {
        assert_eq!(VideoFormat::Mov.recommended_audio_codec(), "aac");
    }

    // ============== VideoFormat Stream Copy Tests ==============

    #[test]
    fn test_supports_stream_copy_same_format() {
        assert!(VideoFormat::Mp4.supports_stream_copy_from(&VideoFormat::Mp4));
        assert!(VideoFormat::Mkv.supports_stream_copy_from(&VideoFormat::Mkv));
        assert!(VideoFormat::Webm.supports_stream_copy_from(&VideoFormat::Webm));
    }

    #[test]
    fn test_supports_stream_copy_mp4_to_mov() {
        assert!(VideoFormat::Mov.supports_stream_copy_from(&VideoFormat::Mp4));
    }

    #[test]
    fn test_supports_stream_copy_mov_to_mp4() {
        assert!(VideoFormat::Mp4.supports_stream_copy_from(&VideoFormat::Mov));
    }

    #[test]
    fn test_supports_stream_copy_to_mkv() {
        assert!(VideoFormat::Mkv.supports_stream_copy_from(&VideoFormat::Mp4));
        assert!(VideoFormat::Mkv.supports_stream_copy_from(&VideoFormat::Webm));
        assert!(VideoFormat::Mkv.supports_stream_copy_from(&VideoFormat::Avi));
        assert!(VideoFormat::Mkv.supports_stream_copy_from(&VideoFormat::Mov));
    }

    #[test]
    fn test_supports_stream_copy_incompatible() {
        assert!(!VideoFormat::Webm.supports_stream_copy_from(&VideoFormat::Mp4));
        assert!(!VideoFormat::Mp4.supports_stream_copy_from(&VideoFormat::Webm));
        assert!(!VideoFormat::Avi.supports_stream_copy_from(&VideoFormat::Webm));
    }

    // ============== VideoFormat Equality Tests ==============

    #[test]
    fn test_video_format_equality() {
        assert_eq!(VideoFormat::Mp4, VideoFormat::Mp4);
        assert_ne!(VideoFormat::Mp4, VideoFormat::Mkv);
    }

    #[test]
    fn test_video_format_clone() {
        let format = VideoFormat::Mp4;
        let cloned = format;
        assert_eq!(format, cloned);
    }

    #[test]
    fn test_video_format_copy() {
        let format = VideoFormat::Webm;
        let copied = format;
        assert_eq!(format, copied);
    }

    // ============== ConversionOptions Default Tests ==============

    #[test]
    fn test_conversion_options_default() {
        let options = ConversionOptions::default();

        assert_eq!(options.output_format, VideoFormat::Mp4);
        assert!(options.stream_copy);
        assert!(options.video_codec.is_none());
        assert!(options.audio_codec.is_none());
        assert!(options.video_quality.is_none());
        assert!(options.audio_bitrate.is_none());
        assert!(options.resolution.is_none());
        assert!(options.framerate.is_none());
        assert!(options.overwrite);
    }

    // ============== ConversionOptions Presets Tests ==============

    #[test]
    fn test_conversion_options_fast() {
        let options = ConversionOptions::fast(VideoFormat::Mkv);

        assert_eq!(options.output_format, VideoFormat::Mkv);
        assert!(options.stream_copy);
    }

    #[test]
    fn test_conversion_options_reencode() {
        let options = ConversionOptions::reencode(VideoFormat::Webm);

        assert_eq!(options.output_format, VideoFormat::Webm);
        assert!(!options.stream_copy);
    }

    #[test]
    fn test_conversion_options_high_quality() {
        let options = ConversionOptions::high_quality(VideoFormat::Mp4);

        assert_eq!(options.output_format, VideoFormat::Mp4);
        assert!(!options.stream_copy);
        assert_eq!(options.video_quality, Some(18));
        assert_eq!(options.audio_bitrate, Some("320k".to_string()));
    }

    #[test]
    fn test_conversion_options_small_file() {
        let options = ConversionOptions::small_file(VideoFormat::Mp4);

        assert_eq!(options.output_format, VideoFormat::Mp4);
        assert!(!options.stream_copy);
        assert_eq!(options.video_quality, Some(28));
        assert_eq!(options.audio_bitrate, Some("128k".to_string()));
    }

    // ============== ConversionOptions Builder Tests ==============

    #[test]
    fn test_conversion_options_with_format() {
        let options = ConversionOptions::default().with_format(VideoFormat::Webm);

        assert_eq!(options.output_format, VideoFormat::Webm);
    }

    #[test]
    fn test_conversion_options_with_stream_copy() {
        let options = ConversionOptions::default().with_stream_copy(false);

        assert!(!options.stream_copy);
    }

    #[test]
    fn test_conversion_options_with_video_codec() {
        let options = ConversionOptions::default().with_video_codec("libx265");

        assert_eq!(options.video_codec, Some("libx265".to_string()));
    }

    #[test]
    fn test_conversion_options_with_audio_codec() {
        let options = ConversionOptions::default().with_audio_codec("libopus");

        assert_eq!(options.audio_codec, Some("libopus".to_string()));
    }

    #[test]
    fn test_conversion_options_with_quality() {
        let options = ConversionOptions::default().with_quality(23);

        assert_eq!(options.video_quality, Some(23));
    }

    #[test]
    fn test_conversion_options_with_audio_bitrate() {
        let options = ConversionOptions::default().with_audio_bitrate("256k");

        assert_eq!(options.audio_bitrate, Some("256k".to_string()));
    }

    #[test]
    fn test_conversion_options_with_resolution() {
        let options = ConversionOptions::default().with_resolution("1920x1080");

        assert_eq!(options.resolution, Some("1920x1080".to_string()));
    }

    #[test]
    fn test_conversion_options_with_framerate() {
        let options = ConversionOptions::default().with_framerate(60);

        assert_eq!(options.framerate, Some(60));
    }

    #[test]
    fn test_conversion_options_builder_chain() {
        let options = ConversionOptions::default()
            .with_format(VideoFormat::Mkv)
            .with_stream_copy(false)
            .with_video_codec("libx265")
            .with_audio_codec("aac")
            .with_quality(20)
            .with_audio_bitrate("192k")
            .with_resolution("1280x720")
            .with_framerate(30);

        assert_eq!(options.output_format, VideoFormat::Mkv);
        assert!(!options.stream_copy);
        assert_eq!(options.video_codec, Some("libx265".to_string()));
        assert_eq!(options.audio_codec, Some("aac".to_string()));
        assert_eq!(options.video_quality, Some(20));
        assert_eq!(options.audio_bitrate, Some("192k".to_string()));
        assert_eq!(options.resolution, Some("1280x720".to_string()));
        assert_eq!(options.framerate, Some(30));
    }

    // ============== VideoConverter detect_format Tests ==============

    #[test]
    fn test_detect_format_mp4() {
        let format = VideoConverter::detect_format("video.mp4");
        assert_eq!(format, Some(VideoFormat::Mp4));
    }

    #[test]
    fn test_detect_format_mkv() {
        let format = VideoConverter::detect_format("video.mkv");
        assert_eq!(format, Some(VideoFormat::Mkv));
    }

    #[test]
    fn test_detect_format_webm() {
        let format = VideoConverter::detect_format("video.webm");
        assert_eq!(format, Some(VideoFormat::Webm));
    }

    #[test]
    fn test_detect_format_avi() {
        let format = VideoConverter::detect_format("video.avi");
        assert_eq!(format, Some(VideoFormat::Avi));
    }

    #[test]
    fn test_detect_format_mov() {
        let format = VideoConverter::detect_format("video.mov");
        assert_eq!(format, Some(VideoFormat::Mov));
    }

    #[test]
    fn test_detect_format_unknown() {
        let format = VideoConverter::detect_format("video.xyz");
        assert_eq!(format, None);
    }

    #[test]
    fn test_detect_format_no_extension() {
        let format = VideoConverter::detect_format("video");
        assert_eq!(format, None);
    }

    #[test]
    fn test_detect_format_with_path() {
        let format = VideoConverter::detect_format("/path/to/video.mp4");
        assert_eq!(format, Some(VideoFormat::Mp4));
    }

    #[test]
    fn test_detect_format_pathbuf() {
        let path = PathBuf::from("/home/user/videos/movie.mkv");
        let format = VideoConverter::detect_format(&path);
        assert_eq!(format, Some(VideoFormat::Mkv));
    }

    // ============== VideoConverter output_path_with_format Tests ==============

    #[test]
    fn test_output_path_with_format_simple() {
        let output = VideoConverter::output_path_with_format("video.mp4", VideoFormat::Mkv);
        assert_eq!(output, PathBuf::from("video.mkv"));
    }

    #[test]
    fn test_output_path_with_format_with_path() {
        let output =
            VideoConverter::output_path_with_format("/path/to/video.mp4", VideoFormat::Webm);
        assert_eq!(output, PathBuf::from("/path/to/video.webm"));
    }

    #[test]
    fn test_output_path_with_format_same_format() {
        let output = VideoConverter::output_path_with_format("video.mp4", VideoFormat::Mp4);
        assert_eq!(output, PathBuf::from("video.mp4"));
    }

    #[test]
    fn test_output_path_with_format_different_extensions() {
        let formats = vec![
            (VideoFormat::Mp4, "mp4"),
            (VideoFormat::Mkv, "mkv"),
            (VideoFormat::Webm, "webm"),
            (VideoFormat::Avi, "avi"),
            (VideoFormat::Mov, "mov"),
        ];

        for (format, expected_ext) in formats {
            let output = VideoConverter::output_path_with_format("video.xyz", format);
            assert!(
                output.to_string_lossy().ends_with(expected_ext),
                "Expected extension {} for format {:?}",
                expected_ext,
                format
            );
        }
    }

    #[test]
    fn test_output_path_preserves_filename() {
        let output =
            VideoConverter::output_path_with_format("my_awesome_video.mp4", VideoFormat::Mkv);
        assert!(output.to_string_lossy().contains("my_awesome_video"));
    }

    #[test]
    fn test_output_path_with_spaces() {
        let output =
            VideoConverter::output_path_with_format("my video file.mp4", VideoFormat::Mkv);
        assert!(output.to_string_lossy().contains("my video file"));
        assert!(output.to_string_lossy().ends_with(".mkv"));
    }

    #[test]
    fn test_output_path_with_unicode() {
        let output = VideoConverter::output_path_with_format("vídeo_música.mp4", VideoFormat::Mkv);
        assert!(output.to_string_lossy().contains("vídeo_música"));
    }

    // ============== VideoConverter needs_reencode Tests ==============

    #[test]
    fn test_needs_reencode_same_format() {
        assert!(!VideoConverter::needs_reencode("video.mp4", VideoFormat::Mp4));
        assert!(!VideoConverter::needs_reencode("video.mkv", VideoFormat::Mkv));
    }

    #[test]
    fn test_needs_reencode_to_mkv() {
        assert!(!VideoConverter::needs_reencode("video.mp4", VideoFormat::Mkv));
        assert!(!VideoConverter::needs_reencode("video.webm", VideoFormat::Mkv));
    }

    #[test]
    fn test_needs_reencode_mp4_to_mov() {
        assert!(!VideoConverter::needs_reencode("video.mp4", VideoFormat::Mov));
    }

    #[test]
    fn test_needs_reencode_mov_to_mp4() {
        assert!(!VideoConverter::needs_reencode("video.mov", VideoFormat::Mp4));
    }

    #[test]
    fn test_needs_reencode_incompatible() {
        assert!(VideoConverter::needs_reencode("video.mp4", VideoFormat::Webm));
        assert!(VideoConverter::needs_reencode("video.webm", VideoFormat::Mp4));
        assert!(VideoConverter::needs_reencode("video.avi", VideoFormat::Webm));
    }

    #[test]
    fn test_needs_reencode_unknown_format() {
        assert!(VideoConverter::needs_reencode("video.xyz", VideoFormat::Mp4));
    }

    #[test]
    fn test_needs_reencode_no_extension() {
        assert!(VideoConverter::needs_reencode("video", VideoFormat::Mp4));
    }

    // ============== ConversionResult Tests ==============

    #[test]
    fn test_conversion_result_creation() {
        let result = ConversionResult {
            output_path: PathBuf::from("/path/to/output.mkv"),
            format: VideoFormat::Mkv,
            used_stream_copy: true,
        };

        assert_eq!(result.output_path, PathBuf::from("/path/to/output.mkv"));
        assert_eq!(result.format, VideoFormat::Mkv);
        assert!(result.used_stream_copy);
    }

    #[test]
    fn test_conversion_result_debug() {
        let result = ConversionResult {
            output_path: PathBuf::from("output.mp4"),
            format: VideoFormat::Mp4,
            used_stream_copy: false,
        };

        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("ConversionResult"));
        assert!(debug_str.contains("output_path"));
    }

    // ============== ConversionOptions Clone Tests ==============

    #[test]
    fn test_conversion_options_clone() {
        let options = ConversionOptions::high_quality(VideoFormat::Mp4);
        let cloned = options.clone();

        assert_eq!(options.output_format, cloned.output_format);
        assert_eq!(options.video_quality, cloned.video_quality);
        assert_eq!(options.audio_bitrate, cloned.audio_bitrate);
    }

    #[test]
    fn test_conversion_options_clone_independent() {
        let options = ConversionOptions::default();
        let mut cloned = options.clone();

        cloned.stream_copy = false;

        assert!(options.stream_copy);
        assert!(!cloned.stream_copy);
    }

    // ============== Integration Tests (require FFmpeg) ==============

    mod integration {
        use super::*;

        fn skip_if_no_ffmpeg() -> bool {
            if !FFmpeg::is_available() {
                println!("Skipping test: FFmpeg not available");
                return true;
            }
            false
        }

        #[test]
        fn test_convert_nonexistent_file() {
            if skip_if_no_ffmpeg() {
                return;
            }

            let options = ConversionOptions::default();
            let result = VideoConverter::convert(
                "/nonexistent/input.mp4",
                "/nonexistent/output.mkv",
                &options,
            );

            assert!(result.is_err());
        }

        #[test]
        fn test_convert_fast_nonexistent_file() {
            if skip_if_no_ffmpeg() {
                return;
            }

            let result = VideoConverter::convert_fast(
                "/nonexistent/input.mp4",
                "/nonexistent/output.mkv",
            );

            assert!(result.is_err());
        }

        #[test]
        fn test_convert_reencode_nonexistent_file() {
            if skip_if_no_ffmpeg() {
                return;
            }

            let result = VideoConverter::convert_reencode(
                "/nonexistent/input.mp4",
                "/nonexistent/output.mkv",
            );

            assert!(result.is_err());
        }
    }

    // ============== Edge Cases ==============

    #[test]
    fn test_video_format_all_variants() {
        let formats = vec![
            VideoFormat::Mp4,
            VideoFormat::Mkv,
            VideoFormat::Webm,
            VideoFormat::Avi,
            VideoFormat::Mov,
        ];

        for format in formats {
            assert!(!format.extension().is_empty());
            assert!(!format.recommended_video_codec().is_empty());
            assert!(!format.recommended_audio_codec().is_empty());
        }
    }

    #[test]
    fn test_conversion_options_quality_range() {
        let options_low = ConversionOptions::default().with_quality(0);
        let options_high = ConversionOptions::default().with_quality(51);

        assert_eq!(options_low.video_quality, Some(0));
        assert_eq!(options_high.video_quality, Some(51));
    }

    #[test]
    fn test_conversion_options_various_bitrates() {
        let bitrates = vec!["64k", "128k", "192k", "256k", "320k"];

        for bitrate in bitrates {
            let options = ConversionOptions::default().with_audio_bitrate(bitrate);
            assert_eq!(options.audio_bitrate, Some(bitrate.to_string()));
        }
    }

    #[test]
    fn test_conversion_options_various_resolutions() {
        let resolutions = vec![
            "640x480",
            "1280x720",
            "1920x1080",
            "2560x1440",
            "3840x2160",
        ];

        for resolution in resolutions {
            let options = ConversionOptions::default().with_resolution(resolution);
            assert_eq!(options.resolution, Some(resolution.to_string()));
        }
    }

    #[test]
    fn test_conversion_options_various_framerates() {
        let framerates = vec![24, 25, 30, 50, 60, 120];

        for fps in framerates {
            let options = ConversionOptions::default().with_framerate(fps);
            assert_eq!(options.framerate, Some(fps));
        }
    }
}
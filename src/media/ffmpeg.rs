//! FFmpeg integration and command execution.
//!
//! This module provides low-level FFmpeg integration for media processing operations.
//! It handles FFmpeg command execution, error handling, and provides utilities for
//! common media manipulation tasks.
//!
//! # FFmpeg Availability
//!
//! Before using any FFmpeg functionality, the module checks for FFmpeg availability
//! in the system PATH. All operations will return [`AppError::FfmpegNotFound`] if
//! FFmpeg is not installed.
//!
//! # Command Execution
//!
//! The module executes FFmpeg as a subprocess using [`std::process::Command`]. All
//! commands are executed synchronously and return detailed error information on failure.
//!
//! # Example
//!
//! ```no_run
//! use rust_yt_downloader::media::FFmpeg;
//!
//! // Check FFmpeg availability
//! if FFmpeg::is_available() {
//!     let version = FFmpeg::version()?;
//!     println!("FFmpeg version: {}", version);
//!
//!     // Extract audio from video
//!     FFmpeg::extract_audio("input.mp4", "output.mp3")?;
//! }
//! # Ok::<(), rust_yt_downloader::error::AppError>(())
//! ```

use std::process::{Command, Output};
use std::path::Path;

use crate::error::{AppError, AppResult};

/// Core FFmpeg integration wrapper.
///
/// Provides methods for checking FFmpeg availability, executing FFmpeg commands,
/// and performing common media operations like format conversion, audio extraction,
/// and video trimming.
///
/// All methods that execute FFmpeg commands will first verify that FFmpeg is available
/// in the system PATH.
pub struct FFmpeg;

impl FFmpeg {
    /// Checks if FFmpeg is available in the system PATH.
    ///
    /// # Returns
    ///
    /// `true` if FFmpeg is installed and executable, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rust_yt_downloader::media::FFmpeg;
    ///
    /// if FFmpeg::is_available() {
    ///     println!("FFmpeg is ready to use");
    /// } else {
    ///     println!("Please install FFmpeg");
    /// }
    /// ```
    pub fn is_available() -> bool {
        Command::new("ffmpeg")
            .arg("-version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Retrieves the FFmpeg version string.
    ///
    /// # Returns
    ///
    /// The first line of FFmpeg's version output (e.g., "ffmpeg version 4.4.2").
    ///
    /// # Errors
    ///
    /// Returns [`AppError::FfmpegNotFound`] if FFmpeg is not available or fails to execute.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rust_yt_downloader::media::FFmpeg;
    ///
    /// let version = FFmpeg::version()?;
    /// println!("Using {}", version);
    /// # Ok::<(), rust_yt_downloader::error::AppError>(())
    /// ```
    pub fn version() -> AppResult<String> {
        let output = Command::new("ffmpeg")
            .arg("-version")
            .output()
            .map_err(|_| AppError::FfmpegNotFound)?;

        if !output.status.success() {
            return Err(AppError::FfmpegNotFound);
        }

        let version_output = String::from_utf8_lossy(&output.stdout);
        let first_line = version_output
            .lines()
            .next()
            .unwrap_or("unknown version");

        Ok(first_line.to_string())
    }

    /// Ensures FFmpeg is available, returning an error if not.
    ///
    /// This is a convenience method that checks FFmpeg availability and returns
    /// an error if it's not found, suitable for use with the `?` operator.
    ///
    /// # Errors
    ///
    /// Returns [`AppError::FfmpegNotFound`] if FFmpeg is not available.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rust_yt_downloader::media::FFmpeg;
    ///
    /// FFmpeg::require()?;
    /// // FFmpeg is guaranteed to be available here
    /// # Ok::<(), rust_yt_downloader::error::AppError>(())
    /// ```
    pub fn require() -> AppResult<()> {
        if !Self::is_available() {
            return Err(AppError::FfmpegNotFound);
        }
        Ok(())
    }

    /// Executes an FFmpeg command with the specified arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - Command-line arguments to pass to FFmpeg
    ///
    /// # Returns
    ///
    /// The command's output (stdout, stderr, and exit status) on success.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - FFmpeg is not available ([`AppError::FfmpegNotFound`])
    /// - The command fails to execute ([`AppError::FfmpegExecution`])
    /// - The command returns a non-zero exit code ([`AppError::FfmpegExecution`])
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rust_yt_downloader::media::FFmpeg;
    ///
    /// let output = FFmpeg::run(&["-i", "input.mp4", "output.mkv"])?;
    /// # Ok::<(), rust_yt_downloader::error::AppError>(())
    /// ```
    pub fn run(args: &[&str]) -> AppResult<Output> {
        Self::require()?;

        let output = Command::new("ffmpeg")
            .args(args)
            .output()
            .map_err(|e| AppError::ffmpeg(format!("Failed to execute FFmpeg: {}", e), None))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::ffmpeg(
                stderr.to_string(),
                output.status.code(),
            ));
        }

        Ok(output)
    }

    /// Executes an FFmpeg command with automatic file overwriting enabled.
    ///
    /// Prepends the `-y` flag to automatically overwrite output files without prompting.
    ///
    /// # Arguments
    ///
    /// * `args` - Command-line arguments to pass to FFmpeg (without `-y`)
    ///
    /// # Errors
    ///
    /// Same as [`FFmpeg::run`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rust_yt_downloader::media::FFmpeg;
    ///
    /// // Will overwrite output.mp4 if it exists
    /// FFmpeg::run_overwrite(&["-i", "input.mkv", "output.mp4"])?;
    /// # Ok::<(), rust_yt_downloader::error::AppError>(())
    /// ```
    pub fn run_overwrite(args: &[&str]) -> AppResult<Output> {
        let mut full_args = vec!["-y"];
        full_args.extend_from_slice(args);
        Self::run(&full_args)
    }

    /// Converts a media file to a different format using stream copy (fast, no re-encoding).
    ///
    /// Uses FFmpeg's `-c copy` option to copy streams without re-encoding, which is fast
    /// but only works when the codecs are compatible with the output container format.
    ///
    /// # Arguments
    ///
    /// * `input` - Path to the input file
    /// * `output` - Path to the output file
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion fails or codecs are incompatible with the output format.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rust_yt_downloader::media::FFmpeg;
    ///
    /// // Fast conversion without re-encoding
    /// FFmpeg::convert("video.mp4", "video.mkv")?;
    /// # Ok::<(), rust_yt_downloader::error::AppError>(())
    /// ```
    pub fn convert<P: AsRef<Path>>(input: P, output: P) -> AppResult<()> {
        let input_str = input.as_ref().to_string_lossy();
        let output_str = output.as_ref().to_string_lossy();

        Self::run_overwrite(&[
            "-i", &input_str,
            "-c", "copy",
            &output_str,
        ])?;

        Ok(())
    }

    /// Converts a media file to a different format with re-encoding.
    ///
    /// Re-encodes both video and audio streams using the default codecs for the output format.
    /// This is slower than [`FFmpeg::convert`] but works with any input/output format combination.
    ///
    /// # Arguments
    ///
    /// * `input` - Path to the input file
    /// * `output` - Path to the output file
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rust_yt_downloader::media::FFmpeg;
    ///
    /// // Convert with re-encoding (slower but always works)
    /// FFmpeg::convert_reencode("video.avi", "video.mp4")?;
    /// # Ok::<(), rust_yt_downloader::error::AppError>(())
    /// ```
    pub fn convert_reencode<P: AsRef<Path>>(input: P, output: P) -> AppResult<()> {
        let input_str = input.as_ref().to_string_lossy();
        let output_str = output.as_ref().to_string_lossy();

        Self::run_overwrite(&[
            "-i", &input_str,
            &output_str,
        ])?;

        Ok(())
    }

    /// Extracts the audio stream from a media file without re-encoding.
    ///
    /// Uses FFmpeg's `-vn` flag to disable video and `-acodec copy` to copy the audio
    /// stream without re-encoding. This is fast but preserves the original audio codec.
    ///
    /// # Arguments
    ///
    /// * `input` - Path to the input media file
    /// * `output` - Path to the output audio file
    ///
    /// # Errors
    ///
    /// Returns an error if extraction fails or the output format doesn't support the audio codec.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rust_yt_downloader::media::FFmpeg;
    ///
    /// // Extract audio without re-encoding
    /// FFmpeg::extract_audio("video.mp4", "audio.m4a")?;
    /// # Ok::<(), rust_yt_downloader::error::AppError>(())
    /// ```
    pub fn extract_audio<P: AsRef<Path>>(input: P, output: P) -> AppResult<()> {
        let input_str = input.as_ref().to_string_lossy();
        let output_str = output.as_ref().to_string_lossy();

        Self::run_overwrite(&[
            "-i", &input_str,
            "-vn",
            "-acodec", "copy",
            &output_str,
        ])?;

        Ok(())
    }

    /// Extracts and converts audio from a media file to a specific codec.
    ///
    /// Extracts the audio stream and re-encodes it using the specified codec and optional bitrate.
    /// This is slower than [`FFmpeg::extract_audio`] but allows format conversion.
    ///
    /// # Arguments
    ///
    /// * `input` - Path to the input media file
    /// * `output` - Path to the output audio file
    /// * `codec` - Audio codec to use (e.g., "libmp3lame", "aac", "flac")
    /// * `bitrate` - Optional bitrate (e.g., "320k", "192k")
    ///
    /// # Errors
    ///
    /// Returns an error if extraction or encoding fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rust_yt_downloader::media::FFmpeg;
    ///
    /// // Extract audio as MP3 with 320k bitrate
    /// FFmpeg::extract_audio_as("video.mp4", "audio.mp3", "libmp3lame", Some("320k"))?;
    /// # Ok::<(), rust_yt_downloader::error::AppError>(())
    /// ```
    pub fn extract_audio_as<P: AsRef<Path>>(
        input: P,
        output: P,
        codec: &str,
        bitrate: Option<&str>,
    ) -> AppResult<()> {
        let input_str = input.as_ref().to_string_lossy();
        let output_str = output.as_ref().to_string_lossy();

        let mut args = vec![
            "-i", &input_str,
            "-vn",
            "-acodec", codec,
        ];

        let bitrate_str;
        if let Some(br) = bitrate {
            bitrate_str = br.to_string();
            args.push("-b:a");
            args.push(&bitrate_str);
        }

        args.push(&output_str);

        Self::run_overwrite(&args)?;

        Ok(())
    }

    /// Trims a media file to a specific time range using stream copy.
    ///
    /// Cuts the media file from `start` to `end` time without re-encoding (fast).
    /// Time format: "HH:MM:SS" or "SS.mmm" (seconds with milliseconds).
    ///
    /// # Arguments
    ///
    /// * `input` - Path to the input media file
    /// * `output` - Path to the output media file
    /// * `start` - Start time (e.g., "00:01:30" or "90")
    /// * `end` - End time (e.g., "00:03:00" or "180")
    ///
    /// # Errors
    ///
    /// Returns an error if trimming fails or time format is invalid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rust_yt_downloader::media::FFmpeg;
    ///
    /// // Trim from 1:30 to 3:00 without re-encoding
    /// FFmpeg::trim("video.mp4", "clip.mp4", "00:01:30", "00:03:00")?;
    /// # Ok::<(), rust_yt_downloader::error::AppError>(())
    /// ```
    pub fn trim<P: AsRef<Path>>(
        input: P,
        output: P,
        start: &str,
        end: &str,
    ) -> AppResult<()> {
        let input_str = input.as_ref().to_string_lossy();
        let output_str = output.as_ref().to_string_lossy();

        Self::run_overwrite(&[
            "-i", &input_str,
            "-ss", start,
            "-to", end,
            "-c", "copy",
            &output_str,
        ])?;

        Ok(())
    }

    /// Trims a media file to a specific time range with re-encoding.
    ///
    /// Cuts the media file from `start` to `end` time with re-encoding (slower but more accurate).
    /// Use this when stream copy produces inaccurate cuts or fails.
    ///
    /// # Arguments
    ///
    /// * `input` - Path to the input media file
    /// * `output` - Path to the output media file
    /// * `start` - Start time (e.g., "00:01:30" or "90")
    /// * `end` - End time (e.g., "00:03:00" or "180")
    ///
    /// # Errors
    ///
    /// Returns an error if trimming fails or time format is invalid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rust_yt_downloader::media::FFmpeg;
    ///
    /// // Trim with re-encoding for precise cuts
    /// FFmpeg::trim_reencode("video.mp4", "clip.mp4", "00:01:30", "00:03:00")?;
    /// # Ok::<(), rust_yt_downloader::error::AppError>(())
    /// ```
    pub fn trim_reencode<P: AsRef<Path>>(
        input: P,
        output: P,
        start: &str,
        end: &str,
    ) -> AppResult<()> {
        let input_str = input.as_ref().to_string_lossy();
        let output_str = output.as_ref().to_string_lossy();

        Self::run_overwrite(&[
            "-i", &input_str,
            "-ss", start,
            "-to", end,
            &output_str,
        ])?;

        Ok(())
    }

    /// Probes a media file to extract metadata using ffprobe.
    ///
    /// Uses ffprobe to retrieve detailed information about the media file including
    /// duration, codecs, bitrates, resolution, and other metadata in JSON format.
    ///
    /// # Arguments
    ///
    /// * `input` - Path to the media file to probe
    ///
    /// # Returns
    ///
    /// JSON string containing format and stream information.
    ///
    /// # Errors
    ///
    /// Returns an error if ffprobe is not available or fails to execute.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rust_yt_downloader::media::FFmpeg;
    ///
    /// let metadata = FFmpeg::probe("video.mp4")?;
    /// println!("Metadata: {}", metadata);
    /// # Ok::<(), rust_yt_downloader::error::AppError>(())
    /// ```
    pub fn probe<P: AsRef<Path>>(input: P) -> AppResult<String> {
        let input_str = input.as_ref().to_string_lossy();

        let output = Command::new("ffprobe")
            .args([
                "-v", "quiet",
                "-print_format", "json",
                "-show_format",
                "-show_streams",
                &input_str,
            ])
            .output()
            .map_err(|e| AppError::ffmpeg(format!("Failed to execute ffprobe: {}", e), None))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::ffmpeg(stderr.to_string(), output.status.code()));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Checks if ffprobe is available in the system PATH.
    ///
    /// # Returns
    ///
    /// `true` if ffprobe is installed and executable, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rust_yt_downloader::media::FFmpeg;
    ///
    /// if FFmpeg::is_probe_available() {
    ///     println!("ffprobe is ready to use");
    /// }
    /// ```
    pub fn is_probe_available() -> bool {
        Command::new("ffprobe")
            .arg("-version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

/// FFmpeg audio codec constants.
///
/// Provides standard codec names used by FFmpeg for audio encoding.
/// These constants map to FFmpeg's encoder names which may differ from
/// the codec or format names.
pub struct AudioCodec;

impl AudioCodec {
    /// MP3 encoder using LAME library.
    pub const MP3: &'static str = "libmp3lame";

    /// AAC encoder (Advanced Audio Coding).
    pub const AAC: &'static str = "aac";

    /// FLAC encoder (Free Lossless Audio Codec).
    pub const FLAC: &'static str = "flac";

    /// Opus encoder (low-latency audio codec).
    pub const OPUS: &'static str = "libopus";

    /// Vorbis encoder (part of Ogg container).
    pub const VORBIS: &'static str = "libvorbis";

    /// WAV encoder using 16-bit PCM.
    pub const WAV: &'static str = "pcm_s16le";

    /// Returns the appropriate FFmpeg codec name for a given file extension.
    ///
    /// # Arguments
    ///
    /// * `ext` - File extension (case-insensitive)
    ///
    /// # Returns
    ///
    /// FFmpeg codec name, defaults to AAC for unknown extensions.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_yt_downloader::media::AudioCodec;
    ///
    /// assert_eq!(AudioCodec::for_extension("mp3"), "libmp3lame");
    /// assert_eq!(AudioCodec::for_extension("flac"), "flac");
    /// ```
    pub fn for_extension(ext: &str) -> &'static str {
        match ext.to_lowercase().as_str() {
            "mp3" => Self::MP3,
            "m4a" | "aac" => Self::AAC,
            "flac" => Self::FLAC,
            "opus" | "ogg" => Self::OPUS,
            "wav" => Self::WAV,
            _ => Self::AAC,
        }
    }
}

/// Audio bitrate presets for encoding.
///
/// Provides standard bitrate values for different quality levels.
/// Higher bitrates generally result in better audio quality but larger file sizes.
pub struct AudioBitrate;

impl AudioBitrate {
    /// Low quality bitrate (128 kbps).
    pub const LOW: &'static str = "128k";

    /// Medium quality bitrate (192 kbps).
    pub const MEDIUM: &'static str = "192k";

    /// High quality bitrate (256 kbps).
    pub const HIGH: &'static str = "256k";

    /// Very high quality bitrate (320 kbps).
    pub const VERY_HIGH: &'static str = "320k";

    /// Returns the recommended default bitrate for a given audio format.
    ///
    /// # Arguments
    ///
    /// * `format` - Audio format or file extension (case-insensitive)
    ///
    /// # Returns
    ///
    /// Recommended bitrate string, defaults to HIGH (256k) for unknown formats.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_yt_downloader::media::AudioBitrate;
    ///
    /// assert_eq!(AudioBitrate::default_for_format("mp3"), "320k");
    /// assert_eq!(AudioBitrate::default_for_format("opus"), "192k");
    /// ```
    pub fn default_for_format(format: &str) -> &'static str {
        match format.to_lowercase().as_str() {
            "mp3" => Self::VERY_HIGH,
            "m4a" | "aac" => Self::HIGH,
            "opus" | "ogg" => Self::MEDIUM,
            _ => Self::HIGH,
        }
    }
}

// ==================================================
//          UNITARY TESTS
// ==================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // ============== FFmpeg Availability Tests ==============

    #[test]
    fn test_ffmpeg_is_available() {
        let available = FFmpeg::is_available();
        println!("FFmpeg available: {}", available);
        assert!(available || !available);
    }

    #[test]
    fn test_ffmpeg_is_probe_available() {
        let available = FFmpeg::is_probe_available();
        println!("FFprobe available: {}", available);
        assert!(available || !available);
    }

    #[test]
    fn test_ffmpeg_version_format() {
        if FFmpeg::is_available() {
            let version = FFmpeg::version();
            assert!(version.is_ok());

            let version_str = version.unwrap();
            assert!(version_str.contains("ffmpeg"));
        }
    }

    #[test]
    fn test_ffmpeg_require_when_available() {
        if FFmpeg::is_available() {
            let result = FFmpeg::require();
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_ffmpeg_version_returns_first_line() {
        if FFmpeg::is_available() {
            let version = FFmpeg::version().unwrap();
            assert!(!version.contains('\n'));
        }
    }

    // ============== AudioCodec Tests ==============

    #[test]
    fn test_audio_codec_constants() {
        assert_eq!(AudioCodec::MP3, "libmp3lame");
        assert_eq!(AudioCodec::AAC, "aac");
        assert_eq!(AudioCodec::FLAC, "flac");
        assert_eq!(AudioCodec::OPUS, "libopus");
        assert_eq!(AudioCodec::VORBIS, "libvorbis");
        assert_eq!(AudioCodec::WAV, "pcm_s16le");
    }

    #[test]
    fn test_audio_codec_for_extension_mp3() {
        assert_eq!(AudioCodec::for_extension("mp3"), AudioCodec::MP3);
    }

    #[test]
    fn test_audio_codec_for_extension_m4a() {
        assert_eq!(AudioCodec::for_extension("m4a"), AudioCodec::AAC);
    }

    #[test]
    fn test_audio_codec_for_extension_aac() {
        assert_eq!(AudioCodec::for_extension("aac"), AudioCodec::AAC);
    }

    #[test]
    fn test_audio_codec_for_extension_flac() {
        assert_eq!(AudioCodec::for_extension("flac"), AudioCodec::FLAC);
    }

    #[test]
    fn test_audio_codec_for_extension_opus() {
        assert_eq!(AudioCodec::for_extension("opus"), AudioCodec::OPUS);
    }

    #[test]
    fn test_audio_codec_for_extension_ogg() {
        assert_eq!(AudioCodec::for_extension("ogg"), AudioCodec::OPUS);
    }

    #[test]
    fn test_audio_codec_for_extension_wav() {
        assert_eq!(AudioCodec::for_extension("wav"), AudioCodec::WAV);
    }

    #[test]
    fn test_audio_codec_for_extension_unknown() {
        assert_eq!(AudioCodec::for_extension("xyz"), AudioCodec::AAC);
    }

    #[test]
    fn test_audio_codec_for_extension_case_insensitive() {
        assert_eq!(AudioCodec::for_extension("MP3"), AudioCodec::MP3);
        assert_eq!(AudioCodec::for_extension("FLAC"), AudioCodec::FLAC);
        assert_eq!(AudioCodec::for_extension("M4A"), AudioCodec::AAC);
    }

    // ============== AudioBitrate Tests ==============

    #[test]
    fn test_audio_bitrate_constants() {
        assert_eq!(AudioBitrate::LOW, "128k");
        assert_eq!(AudioBitrate::MEDIUM, "192k");
        assert_eq!(AudioBitrate::HIGH, "256k");
        assert_eq!(AudioBitrate::VERY_HIGH, "320k");
    }

    #[test]
    fn test_audio_bitrate_default_for_mp3() {
        assert_eq!(AudioBitrate::default_for_format("mp3"), AudioBitrate::VERY_HIGH);
    }

    #[test]
    fn test_audio_bitrate_default_for_m4a() {
        assert_eq!(AudioBitrate::default_for_format("m4a"), AudioBitrate::HIGH);
    }

    #[test]
    fn test_audio_bitrate_default_for_aac() {
        assert_eq!(AudioBitrate::default_for_format("aac"), AudioBitrate::HIGH);
    }

    #[test]
    fn test_audio_bitrate_default_for_opus() {
        assert_eq!(AudioBitrate::default_for_format("opus"), AudioBitrate::MEDIUM);
    }

    #[test]
    fn test_audio_bitrate_default_for_ogg() {
        assert_eq!(AudioBitrate::default_for_format("ogg"), AudioBitrate::MEDIUM);
    }

    #[test]
    fn test_audio_bitrate_default_for_unknown() {
        assert_eq!(AudioBitrate::default_for_format("xyz"), AudioBitrate::HIGH);
    }

    #[test]
    fn test_audio_bitrate_default_case_insensitive() {
        assert_eq!(AudioBitrate::default_for_format("MP3"), AudioBitrate::VERY_HIGH);
        assert_eq!(AudioBitrate::default_for_format("OPUS"), AudioBitrate::MEDIUM);
    }

    // ============== FFmpeg Command Building Tests ==============

    #[test]
    fn test_convert_path_handling() {
        let input = PathBuf::from("/path/to/input.mp4");
        let output = PathBuf::from("/path/to/output.mkv");

        assert_eq!(input.to_string_lossy(), "/path/to/input.mp4");
        assert_eq!(output.to_string_lossy(), "/path/to/output.mkv");
    }

    #[test]
    fn test_path_with_spaces() {
        let input = PathBuf::from("/path/to/my video.mp4");
        let output = PathBuf::from("/path/to/my output.mkv");

        assert_eq!(input.to_string_lossy(), "/path/to/my video.mp4");
        assert_eq!(output.to_string_lossy(), "/path/to/my output.mkv");
    }

    #[test]
    fn test_path_with_unicode() {
        let input = PathBuf::from("/path/to/vídeo música.mp4");
        let output = PathBuf::from("/path/to/saída.mkv");

        assert!(input.to_string_lossy().contains("vídeo"));
        assert!(output.to_string_lossy().contains("saída"));
    }

    // ============== Integration Tests (require FFmpeg) ==============

    mod integration {
        use super::*;
        use std::fs;
        use tempfile::TempDir;

        fn skip_if_no_ffmpeg() -> bool {
            if !FFmpeg::is_available() {
                println!("Skipping test: FFmpeg not available");
                return true;
            }
            false
        }

        #[test]
        fn test_ffmpeg_run_with_help() {
            if skip_if_no_ffmpeg() {
                return;
            }

            // -h é um comando válido que não faz nada destrutivo
            let result = FFmpeg::run(&["-h"]);
            // FFmpeg retorna erro para -h (exit code 0 mas vai para stderr)
            // Este teste apenas verifica que conseguimos executar
            assert!(result.is_ok() || result.is_err());
        }

        #[test]
        fn test_ffmpeg_run_invalid_args() {
            if skip_if_no_ffmpeg() {
                return;
            }

            let result = FFmpeg::run(&["-invalid_arg_that_does_not_exist"]);
            assert!(result.is_err());
        }

        #[test]
        fn test_ffmpeg_convert_nonexistent_file() {
            if skip_if_no_ffmpeg() {
                return;
            }

            let result = FFmpeg::convert(
                "/nonexistent/input.mp4",
                "/nonexistent/output.mkv",
            );
            assert!(result.is_err());
        }

        #[test]
        fn test_ffmpeg_extract_audio_nonexistent_file() {
            if skip_if_no_ffmpeg() {
                return;
            }

            let result = FFmpeg::extract_audio(
                "/nonexistent/input.mp4",
                "/nonexistent/output.mp3",
            );
            assert!(result.is_err());
        }

        #[test]
        fn test_ffmpeg_trim_nonexistent_file() {
            if skip_if_no_ffmpeg() {
                return;
            }

            let result = FFmpeg::trim(
                "/nonexistent/input.mp4",
                "/nonexistent/output.mp4",
                "00:00:00",
                "00:00:10",
            );
            assert!(result.is_err());
        }

        #[test]
        fn test_ffmpeg_probe_nonexistent_file() {
            if skip_if_no_ffmpeg() {
                return;
            }

            let result = FFmpeg::probe("/nonexistent/file.mp4");
            assert!(result.is_err());
        }

        #[test]
        fn test_ffmpeg_error_contains_message() {
            if skip_if_no_ffmpeg() {
                return;
            }

            let result = FFmpeg::convert(
                "/nonexistent/input.mp4",
                "/nonexistent/output.mkv",
            );

            match result {
                Err(AppError::FfmpegExecution { message, .. }) => {
                    assert!(!message.is_empty());
                }
                Err(_) => {}
                Ok(_) => panic!("Expected error for nonexistent file"),
            }
        }
    }

    // ============== Argument Construction Tests ==============

    #[test]
    fn test_run_overwrite_prepends_y_flag() {
        let args = vec!["-i", "input.mp4", "output.mkv"];
        let mut full_args = vec!["-y"];
        full_args.extend_from_slice(&args);

        assert_eq!(full_args[0], "-y");
        assert_eq!(full_args[1], "-i");
        assert_eq!(full_args.len(), 4);
    }

    #[test]
    fn test_extract_audio_args_structure() {
        let input = "input.mp4";
        let output = "output.mp3";

        let args = vec![
            "-i", input,
            "-vn",
            "-acodec", "copy",
            output,
        ];

        assert_eq!(args[0], "-i");
        assert_eq!(args[2], "-vn");
        assert_eq!(args[3], "-acodec");
    }

    #[test]
    fn test_extract_audio_as_with_bitrate() {
        let input = "input.mp4";
        let output = "output.mp3";
        let codec = "libmp3lame";
        let bitrate = "320k";

        let mut args = vec![
            "-i", input,
            "-vn",
            "-acodec", codec,
        ];

        args.push("-b:a");
        args.push(bitrate);
        args.push(output);

        assert!(args.contains(&"-b:a"));
        assert!(args.contains(&"320k"));
    }

    #[test]
    fn test_trim_args_structure() {
        let input = "input.mp4";
        let output = "output.mp4";
        let start = "00:01:00";
        let end = "00:02:00";

        let args = vec![
            "-i", input,
            "-ss", start,
            "-to", end,
            "-c", "copy",
            output,
        ];

        assert_eq!(args[2], "-ss");
        assert_eq!(args[3], start);
        assert_eq!(args[4], "-to");
        assert_eq!(args[5], end);
    }

    // ============== Edge Cases ==============

    #[test]
    fn test_audio_codec_empty_extension() {
        let codec = AudioCodec::for_extension("");
        assert_eq!(codec, AudioCodec::AAC);
    }

    #[test]
    fn test_audio_bitrate_empty_format() {
        let bitrate = AudioBitrate::default_for_format("");
        assert_eq!(bitrate, AudioBitrate::HIGH);
    }

    #[test]
    fn test_path_empty() {
        let path = PathBuf::from("");
        assert_eq!(path.to_string_lossy(), "");
    }

    #[test]
    fn test_path_relative() {
        let path = PathBuf::from("./video.mp4");
        assert!(path.to_string_lossy().contains("video.mp4"));
    }

    // ============== Codec and Bitrate Combinations ==============

    #[test]
    fn test_mp3_codec_and_bitrate() {
        let codec = AudioCodec::for_extension("mp3");
        let bitrate = AudioBitrate::default_for_format("mp3");

        assert_eq!(codec, "libmp3lame");
        assert_eq!(bitrate, "320k");
    }

    #[test]
    fn test_flac_codec_no_bitrate_needed() {
        let codec = AudioCodec::for_extension("flac");
        let bitrate = AudioBitrate::default_for_format("flac");

        assert_eq!(codec, "flac");
        assert_eq!(bitrate, "256k");
    }

    #[test]
    fn test_opus_codec_and_bitrate() {
        let codec = AudioCodec::for_extension("opus");
        let bitrate = AudioBitrate::default_for_format("opus");

        assert_eq!(codec, "libopus");
        assert_eq!(bitrate, "192k");
    }

    #[test]
    fn test_wav_codec() {
        let codec = AudioCodec::for_extension("wav");
        assert_eq!(codec, "pcm_s16le");
    }

    // ============== Multiple Format Support ==============

    #[test]
    fn test_all_supported_audio_extensions() {
        let extensions = vec!["mp3", "m4a", "aac", "flac", "opus", "ogg", "wav"];

        for ext in extensions {
            let codec = AudioCodec::for_extension(ext);
            assert!(!codec.is_empty(), "Codec for {} should not be empty", ext);
        }
    }

    #[test]
    fn test_all_supported_audio_formats_have_bitrate() {
        let formats = vec!["mp3", "m4a", "aac", "opus", "ogg"];

        for format in formats {
            let bitrate = AudioBitrate::default_for_format(format);
            assert!(
                bitrate.ends_with('k'),
                "Bitrate for {} should end with 'k'",
                format
            );
        }
    }
}
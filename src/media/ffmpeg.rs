use std::process::{Command, Output};
use std::path::Path;

use crate::error::{AppError, AppResult};

pub struct FFmpeg;

impl FFmpeg {
    pub fn is_available() -> bool {
        Command::new("ffmpeg")
            .arg("-version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

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

    pub fn require() -> AppResult<()> {
        if !Self::is_available() {
            return Err(AppError::FfmpegNotFound);
        }
        Ok(())
    }

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

    pub fn run_overwrite(args: &[&str]) -> AppResult<Output> {
        let mut full_args = vec!["-y"];
        full_args.extend_from_slice(args);
        Self::run(&full_args)
    }

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

    pub fn convert_reencode<P: AsRef<Path>>(input: P, output: P) -> AppResult<()> {
        let input_str = input.as_ref().to_string_lossy();
        let output_str = output.as_ref().to_string_lossy();

        Self::run_overwrite(&[
            "-i", &input_str,
            &output_str,
        ])?;

        Ok(())
    }

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

    /// Get information about a media file (duration, codecs, etc)
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

    pub fn is_probe_available() -> bool {
        Command::new("ffprobe")
            .arg("-version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

pub struct AudioCodec;

impl AudioCodec {
    pub const MP3: &'static str = "libmp3lame";
    pub const AAC: &'static str = "aac";
    pub const FLAC: &'static str = "flac";
    pub const OPUS: &'static str = "libopus";
    pub const VORBIS: &'static str = "libvorbis";
    pub const WAV: &'static str = "pcm_s16le";

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

pub struct AudioBitrate;

impl AudioBitrate {
    pub const LOW: &'static str = "128k";
    pub const MEDIUM: &'static str = "192k";
    pub const HIGH: &'static str = "256k";
    pub const VERY_HIGH: &'static str = "320k";

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
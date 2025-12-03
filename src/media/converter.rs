// src/media/converter.rs

use std::path::{Path, PathBuf};

use crate::error::{AppError, AppResult};
use crate::media::ffmpeg::FFmpeg;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoFormat {
    Mp4,
    Mkv,
    Webm,
    Avi,
    Mov,
}

impl VideoFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Mp4 => "mp4",
            Self::Mkv => "mkv",
            Self::Webm => "webm",
            Self::Avi => "avi",
            Self::Mov => "mov",
        }
    }

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

    pub fn recommended_video_codec(&self) -> &'static str {
        match self {
            Self::Mp4 => "libx264",
            Self::Mkv => "libx264",
            Self::Webm => "libvpx-vp9",
            Self::Avi => "mpeg4",
            Self::Mov => "libx264",
        }
    }

    pub fn recommended_audio_codec(&self) -> &'static str {
        match self {
            Self::Mp4 => "aac",
            Self::Mkv => "aac",
            Self::Webm => "libopus",
            Self::Avi => "mp3",
            Self::Mov => "aac",
        }
    }

    pub fn supports_stream_copy_from(&self, source: &VideoFormat) -> bool {
        match (source, self) {
            (Self::Mp4, Self::Mov) | (Self::Mov, Self::Mp4) => true,
            (_, Self::Mkv) => true,
            (a, b) if a == b => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConversionOptions {
    pub output_format: VideoFormat,
    pub stream_copy: bool,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub video_quality: Option<u8>,
    pub audio_bitrate: Option<String>,
    pub resolution: Option<String>,
    pub framerate: Option<u32>,
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
    pub fn fast(format: VideoFormat) -> Self {
        Self {
            output_format: format,
            stream_copy: true,
            ..Default::default()
        }
    }

    pub fn reencode(format: VideoFormat) -> Self {
        Self {
            output_format: format,
            stream_copy: false,
            ..Default::default()
        }
    }

    pub fn high_quality(format: VideoFormat) -> Self {
        Self {
            output_format: format,
            stream_copy: false,
            video_quality: Some(18), // CRF 18 = high quality
            audio_bitrate: Some("320k".to_string()),
            ..Default::default()
        }
    }

    pub fn small_file(format: VideoFormat) -> Self {
        Self {
            output_format: format,
            stream_copy: false,
            video_quality: Some(28), // CRF 28 = smaller file
            audio_bitrate: Some("128k".to_string()),
            ..Default::default()
        }
    }

    pub fn with_format(mut self, format: VideoFormat) -> Self {
        self.output_format = format;
        self
    }

    pub fn with_stream_copy(mut self, stream_copy: bool) -> Self {
        self.stream_copy = stream_copy;
        self
    }

    pub fn with_video_codec(mut self, codec: impl Into<String>) -> Self {
        self.video_codec = Some(codec.into());
        self
    }
    pub fn with_audio_codec(mut self, codec: impl Into<String>) -> Self {
        self.audio_codec = Some(codec.into());
        self
    }

    pub fn with_quality(mut self, crf: u8) -> Self {
        self.video_quality = Some(crf);
        self
    }

    pub fn with_audio_bitrate(mut self, bitrate: impl Into<String>) -> Self {
        self.audio_bitrate = Some(bitrate.into());
        self
    }

    pub fn with_resolution(mut self, resolution: impl Into<String>) -> Self {
        self.resolution = Some(resolution.into());
        self
    }

    pub fn with_framerate(mut self, fps: u32) -> Self {
        self.framerate = Some(fps);
        self
    }
}

pub struct VideoConverter;

impl VideoConverter {
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

    pub fn convert_fast<P: AsRef<Path>>(input: P, output: P) -> AppResult<()> {
        FFmpeg::convert(input, output)
    }

    pub fn convert_reencode<P: AsRef<Path>>(input: P, output: P) -> AppResult<()> {
        FFmpeg::convert_reencode(input, output)
    }

    pub fn detect_format<P: AsRef<Path>>(path: P) -> Option<VideoFormat> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(VideoFormat::from_extension)
    }

    pub fn output_path_with_format<P: AsRef<Path>>(input: P, format: VideoFormat) -> PathBuf {
        let input_path = input.as_ref();
        let stem = input_path.file_stem().unwrap_or_default();

        input_path
            .parent()
            .unwrap_or(Path::new("."))
            .join(format!("{}.{}", stem.to_string_lossy(), format.extension()))
    }

    pub fn needs_reencode<P: AsRef<Path>>(input: P, output_format: VideoFormat) -> bool {
        let input_format = Self::detect_format(&input);

        match input_format {
            Some(fmt) => !output_format.supports_stream_copy_from(&fmt),
            None => true,
        }
    }
}

#[derive(Debug)]
pub struct ConversionResult {
    pub output_path: PathBuf,
    pub format: VideoFormat,
    pub used_stream_copy: bool,
}
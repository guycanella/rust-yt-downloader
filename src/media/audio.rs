use std::path::{Path, PathBuf};

use crate::error::{AppError, AppResult};
use crate::media::ffmpeg::{AudioBitrate, AudioCodec, FFmpeg};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Mp3,
    M4a,
    Aac,
    Flac,
    Wav,
    Opus,
    Ogg,
}

impl AudioFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Mp3 => "mp3",
            Self::M4a => "m4a",
            Self::Aac => "aac",
            Self::Flac => "flac",
            Self::Wav => "wav",
            Self::Opus => "opus",
            Self::Ogg => "ogg",
        }
    }

    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "mp3" => Some(Self::Mp3),
            "m4a" => Some(Self::M4a),
            "aac" => Some(Self::Aac),
            "flac" => Some(Self::Flac),
            "wav" => Some(Self::Wav),
            "opus" => Some(Self::Opus),
            "ogg" => Some(Self::Ogg),
            _ => None,
        }
    }

    pub fn codec(&self) -> &'static str {
        match self {
            Self::Mp3 => AudioCodec::MP3,
            Self::M4a | Self::Aac => AudioCodec::AAC,
            Self::Flac => AudioCodec::FLAC,
            Self::Wav => AudioCodec::WAV,
            Self::Opus => AudioCodec::OPUS,
            Self::Ogg => AudioCodec::VORBIS,
        }
    }

    pub fn default_bitrate(&self) -> &'static str {
        match self {
            Self::Mp3 => AudioBitrate::VERY_HIGH,
            Self::M4a | Self::Aac => AudioBitrate::HIGH,
            Self::Opus | Self::Ogg => AudioBitrate::MEDIUM,
            Self::Flac | Self::Wav => "0",
        }
    }

    pub fn is_lossless(&self) -> bool {
        matches!(self, Self::Flac | Self::Wav)
    }

    pub fn supports_vbr(&self) -> bool {
        matches!(self, Self::Mp3 | Self::Opus | Self::Ogg)
    }
}

#[derive(Debug, Clone)]
pub struct AudioOptions {
    pub format: AudioFormat,
    pub bitrate: Option<String>,
    pub sample_rate: Option<u32>,
    pub channels: Option<u8>,
    pub overwrite: bool,
}

impl Default for AudioOptions {
    fn default() -> Self {
        Self {
            format: AudioFormat::Mp3,
            bitrate: None,
            sample_rate: None,
            channels: None,
            overwrite: true,
        }
    }
}

impl AudioOptions {
    pub fn mp3_high_quality() -> Self {
        Self {
            format: AudioFormat::Mp3,
            bitrate: Some(AudioBitrate::VERY_HIGH.to_string()),
            ..Default::default()
        }
    }

    pub fn flac() -> Self {
        Self {
            format: AudioFormat::Flac,
            bitrate: None,
            ..Default::default()
        }
    }

    pub fn m4a(bitrate: &str) -> Self {
        Self {
            format: AudioFormat::M4a,
            bitrate: Some(bitrate.to_string()),
            ..Default::default()
        }
    }

    pub fn opus() -> Self {
        Self {
            format: AudioFormat::Opus,
            bitrate: Some(AudioBitrate::MEDIUM.to_string()),
            ..Default::default()
        }
    }

    pub fn with_format(mut self, format: AudioFormat) -> Self {
        self.format = format;
        self
    }

    pub fn with_bitrate(mut self, bitrate: impl Into<String>) -> Self {
        self.bitrate = Some(bitrate.into());
        self
    }

    pub fn with_sample_rate(mut self, rate: u32) -> Self {
        self.sample_rate = Some(rate);
        self
    }

    pub fn with_channels(mut self, channels: u8) -> Self {
        self.channels = Some(channels);
        self
    }

    pub fn effective_bitrate(&self) -> Option<&str> {
        if self.format.is_lossless() {
            None
        } else {
            Some(
                self.bitrate
                    .as_deref()
                    .unwrap_or_else(|| self.format.default_bitrate()),
            )
        }
    }
}

pub struct AudioExtractor;

impl AudioExtractor {
    pub fn extract<P: AsRef<Path>>(input: P, output: P, options: &AudioOptions) -> AppResult<()> {
        FFmpeg::require()?;

        let input_str = input.as_ref().to_string_lossy();
        let output_str = output.as_ref().to_string_lossy();

        let mut args: Vec<String> = Vec::new();

        if options.overwrite {
            args.push("-y".to_string());
        }

        args.push("-i".to_string());
        args.push(input_str.to_string());

        args.push("-vn".to_string());

        args.push("-acodec".to_string());
        args.push(options.format.codec().to_string());

        if let Some(bitrate) = options.effective_bitrate() {
            args.push("-b:a".to_string());
            args.push(bitrate.to_string());
        }

        if let Some(rate) = options.sample_rate {
            args.push("-ar".to_string());
            args.push(rate.to_string());
        }

        if let Some(channels) = options.channels {
            args.push("-ac".to_string());
            args.push(channels.to_string());
        }

        args.push(output_str.to_string());

        let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        FFmpeg::run(&args_ref)?;

        Ok(())
    }

    pub fn extract_default<P: AsRef<Path>>(input: P, output: P) -> AppResult<()> {
        FFmpeg::extract_audio(input, output)
    }

    pub fn extract_as_mp3<P: AsRef<Path>>(input: P, output: P) -> AppResult<()> {
        Self::extract(input, output, &AudioOptions::mp3_high_quality())
    }

    pub fn extract_as_flac<P: AsRef<Path>>(input: P, output: P) -> AppResult<()> {
        Self::extract(input, output, &AudioOptions::flac())
    }

    pub fn convert<P: AsRef<Path>>(input: P, output: P, options: &AudioOptions) -> AppResult<()> {
        Self::extract(input, output, options)
    }

    pub fn detect_format<P: AsRef<Path>>(path: P) -> Option<AudioFormat> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(AudioFormat::from_extension)
    }

    pub fn output_path_with_format<P: AsRef<Path>>(input: P, format: AudioFormat) -> PathBuf {
        let input_path = input.as_ref();
        let stem = input_path.file_stem().unwrap_or_default();

        input_path
            .parent()
            .unwrap_or(Path::new(""))
            .join(format!("{}.{}", stem.to_string_lossy(), format.extension()))
    }
}

#[derive(Debug, Clone)]
pub struct AudioInfo {
    pub format: Option<AudioFormat>,
    pub duration: Option<f64>,
    pub bitrate: Option<u32>,
    pub sample_rate: Option<u32>,
    pub channels: Option<u8>,
    pub codec: Option<String>,
}

impl AudioInfo {
    pub fn empty() -> Self {
        Self {
            format: None,
            duration: None,
            bitrate: None,
            sample_rate: None,
            channels: None,
            codec: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.format.is_none()
            && self.duration.is_none()
            && self.bitrate.is_none()
            && self.sample_rate.is_none()
            && self.channels.is_none()
            && self.codec.is_none()
    }
}

// ==================================================
//          UNITARY TESTS
// ==================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ============== AudioFormat Extension Tests ==============

    #[test]
    fn test_audio_format_extension_mp3() {
        assert_eq!(AudioFormat::Mp3.extension(), "mp3");
    }

    #[test]
    fn test_audio_format_extension_m4a() {
        assert_eq!(AudioFormat::M4a.extension(), "m4a");
    }

    #[test]
    fn test_audio_format_extension_aac() {
        assert_eq!(AudioFormat::Aac.extension(), "aac");
    }

    #[test]
    fn test_audio_format_extension_flac() {
        assert_eq!(AudioFormat::Flac.extension(), "flac");
    }

    #[test]
    fn test_audio_format_extension_wav() {
        assert_eq!(AudioFormat::Wav.extension(), "wav");
    }

    #[test]
    fn test_audio_format_extension_opus() {
        assert_eq!(AudioFormat::Opus.extension(), "opus");
    }

    #[test]
    fn test_audio_format_extension_ogg() {
        assert_eq!(AudioFormat::Ogg.extension(), "ogg");
    }

    // ============== AudioFormat from_extension Tests ==============

    #[test]
    fn test_audio_format_from_extension_mp3() {
        assert_eq!(AudioFormat::from_extension("mp3"), Some(AudioFormat::Mp3));
    }

    #[test]
    fn test_audio_format_from_extension_m4a() {
        assert_eq!(AudioFormat::from_extension("m4a"), Some(AudioFormat::M4a));
    }

    #[test]
    fn test_audio_format_from_extension_aac() {
        assert_eq!(AudioFormat::from_extension("aac"), Some(AudioFormat::Aac));
    }

    #[test]
    fn test_audio_format_from_extension_flac() {
        assert_eq!(AudioFormat::from_extension("flac"), Some(AudioFormat::Flac));
    }

    #[test]
    fn test_audio_format_from_extension_wav() {
        assert_eq!(AudioFormat::from_extension("wav"), Some(AudioFormat::Wav));
    }

    #[test]
    fn test_audio_format_from_extension_opus() {
        assert_eq!(AudioFormat::from_extension("opus"), Some(AudioFormat::Opus));
    }

    #[test]
    fn test_audio_format_from_extension_ogg() {
        assert_eq!(AudioFormat::from_extension("ogg"), Some(AudioFormat::Ogg));
    }

    #[test]
    fn test_audio_format_from_extension_unknown() {
        assert_eq!(AudioFormat::from_extension("xyz"), None);
    }

    #[test]
    fn test_audio_format_from_extension_empty() {
        assert_eq!(AudioFormat::from_extension(""), None);
    }

    #[test]
    fn test_audio_format_from_extension_case_insensitive() {
        assert_eq!(AudioFormat::from_extension("MP3"), Some(AudioFormat::Mp3));
        assert_eq!(AudioFormat::from_extension("FLAC"), Some(AudioFormat::Flac));
        assert_eq!(AudioFormat::from_extension("Opus"), Some(AudioFormat::Opus));
    }

    // ============== AudioFormat Codec Tests ==============

    #[test]
    fn test_audio_format_codec_mp3() {
        assert_eq!(AudioFormat::Mp3.codec(), AudioCodec::MP3);
    }

    #[test]
    fn test_audio_format_codec_m4a() {
        assert_eq!(AudioFormat::M4a.codec(), AudioCodec::AAC);
    }

    #[test]
    fn test_audio_format_codec_aac() {
        assert_eq!(AudioFormat::Aac.codec(), AudioCodec::AAC);
    }

    #[test]
    fn test_audio_format_codec_flac() {
        assert_eq!(AudioFormat::Flac.codec(), AudioCodec::FLAC);
    }

    #[test]
    fn test_audio_format_codec_wav() {
        assert_eq!(AudioFormat::Wav.codec(), AudioCodec::WAV);
    }

    #[test]
    fn test_audio_format_codec_opus() {
        assert_eq!(AudioFormat::Opus.codec(), AudioCodec::OPUS);
    }

    #[test]
    fn test_audio_format_codec_ogg() {
        assert_eq!(AudioFormat::Ogg.codec(), AudioCodec::VORBIS);
    }

    // ============== AudioFormat Bitrate Tests ==============

    #[test]
    fn test_audio_format_default_bitrate_mp3() {
        assert_eq!(AudioFormat::Mp3.default_bitrate(), AudioBitrate::VERY_HIGH);
    }

    #[test]
    fn test_audio_format_default_bitrate_m4a() {
        assert_eq!(AudioFormat::M4a.default_bitrate(), AudioBitrate::HIGH);
    }

    #[test]
    fn test_audio_format_default_bitrate_aac() {
        assert_eq!(AudioFormat::Aac.default_bitrate(), AudioBitrate::HIGH);
    }

    #[test]
    fn test_audio_format_default_bitrate_opus() {
        assert_eq!(AudioFormat::Opus.default_bitrate(), AudioBitrate::MEDIUM);
    }

    #[test]
    fn test_audio_format_default_bitrate_ogg() {
        assert_eq!(AudioFormat::Ogg.default_bitrate(), AudioBitrate::MEDIUM);
    }

    #[test]
    fn test_audio_format_default_bitrate_flac() {
        assert_eq!(AudioFormat::Flac.default_bitrate(), "0");
    }

    #[test]
    fn test_audio_format_default_bitrate_wav() {
        assert_eq!(AudioFormat::Wav.default_bitrate(), "0");
    }

    // ============== AudioFormat is_lossless Tests ==============

    #[test]
    fn test_audio_format_is_lossless_flac() {
        assert!(AudioFormat::Flac.is_lossless());
    }

    #[test]
    fn test_audio_format_is_lossless_wav() {
        assert!(AudioFormat::Wav.is_lossless());
    }

    #[test]
    fn test_audio_format_is_lossless_mp3() {
        assert!(!AudioFormat::Mp3.is_lossless());
    }

    #[test]
    fn test_audio_format_is_lossless_m4a() {
        assert!(!AudioFormat::M4a.is_lossless());
    }

    #[test]
    fn test_audio_format_is_lossless_opus() {
        assert!(!AudioFormat::Opus.is_lossless());
    }

    // ============== AudioFormat supports_vbr Tests ==============

    #[test]
    fn test_audio_format_supports_vbr_mp3() {
        assert!(AudioFormat::Mp3.supports_vbr());
    }

    #[test]
    fn test_audio_format_supports_vbr_opus() {
        assert!(AudioFormat::Opus.supports_vbr());
    }

    #[test]
    fn test_audio_format_supports_vbr_ogg() {
        assert!(AudioFormat::Ogg.supports_vbr());
    }

    #[test]
    fn test_audio_format_supports_vbr_flac() {
        assert!(!AudioFormat::Flac.supports_vbr());
    }

    #[test]
    fn test_audio_format_supports_vbr_wav() {
        assert!(!AudioFormat::Wav.supports_vbr());
    }

    #[test]
    fn test_audio_format_supports_vbr_m4a() {
        assert!(!AudioFormat::M4a.supports_vbr());
    }

    // ============== AudioFormat Equality Tests ==============

    #[test]
    fn test_audio_format_equality() {
        assert_eq!(AudioFormat::Mp3, AudioFormat::Mp3);
        assert_ne!(AudioFormat::Mp3, AudioFormat::Flac);
    }

    #[test]
    fn test_audio_format_clone() {
        let format = AudioFormat::Mp3;
        let cloned = format;
        assert_eq!(format, cloned);
    }

    // ============== AudioOptions Default Tests ==============

    #[test]
    fn test_audio_options_default() {
        let options = AudioOptions::default();

        assert_eq!(options.format, AudioFormat::Mp3);
        assert!(options.bitrate.is_none());
        assert!(options.sample_rate.is_none());
        assert!(options.channels.is_none());
        assert!(options.overwrite);
    }

    // ============== AudioOptions Presets Tests ==============

    #[test]
    fn test_audio_options_mp3_high_quality() {
        let options = AudioOptions::mp3_high_quality();

        assert_eq!(options.format, AudioFormat::Mp3);
        assert_eq!(options.bitrate, Some(AudioBitrate::VERY_HIGH.to_string()));
    }

    #[test]
    fn test_audio_options_flac() {
        let options = AudioOptions::flac();

        assert_eq!(options.format, AudioFormat::Flac);
        assert!(options.bitrate.is_none());
    }

    #[test]
    fn test_audio_options_m4a() {
        let options = AudioOptions::m4a("256k");

        assert_eq!(options.format, AudioFormat::M4a);
        assert_eq!(options.bitrate, Some("256k".to_string()));
    }

    #[test]
    fn test_audio_options_opus() {
        let options = AudioOptions::opus();

        assert_eq!(options.format, AudioFormat::Opus);
        assert_eq!(options.bitrate, Some(AudioBitrate::MEDIUM.to_string()));
    }

    // ============== AudioOptions Builder Tests ==============

    #[test]
    fn test_audio_options_with_format() {
        let options = AudioOptions::default().with_format(AudioFormat::Flac);

        assert_eq!(options.format, AudioFormat::Flac);
    }

    #[test]
    fn test_audio_options_with_bitrate() {
        let options = AudioOptions::default().with_bitrate("256k");

        assert_eq!(options.bitrate, Some("256k".to_string()));
    }

    #[test]
    fn test_audio_options_with_sample_rate() {
        let options = AudioOptions::default().with_sample_rate(48000);

        assert_eq!(options.sample_rate, Some(48000));
    }

    #[test]
    fn test_audio_options_with_channels() {
        let options = AudioOptions::default().with_channels(2);

        assert_eq!(options.channels, Some(2));
    }

    #[test]
    fn test_audio_options_with_channels_mono() {
        let options = AudioOptions::default().with_channels(1);

        assert_eq!(options.channels, Some(1));
    }

    #[test]
    fn test_audio_options_builder_chain() {
        let options = AudioOptions::default()
            .with_format(AudioFormat::Opus)
            .with_bitrate("192k")
            .with_sample_rate(48000)
            .with_channels(2);

        assert_eq!(options.format, AudioFormat::Opus);
        assert_eq!(options.bitrate, Some("192k".to_string()));
        assert_eq!(options.sample_rate, Some(48000));
        assert_eq!(options.channels, Some(2));
    }

    // ============== AudioOptions effective_bitrate Tests ==============

    #[test]
    fn test_effective_bitrate_specified() {
        let options = AudioOptions::default().with_bitrate("256k");

        assert_eq!(options.effective_bitrate(), Some("256k"));
    }

    #[test]
    fn test_effective_bitrate_default_mp3() {
        let options = AudioOptions::default().with_format(AudioFormat::Mp3);

        assert_eq!(options.effective_bitrate(), Some(AudioBitrate::VERY_HIGH));
    }

    #[test]
    fn test_effective_bitrate_default_opus() {
        let options = AudioOptions::default().with_format(AudioFormat::Opus);

        assert_eq!(options.effective_bitrate(), Some(AudioBitrate::MEDIUM));
    }

    #[test]
    fn test_effective_bitrate_lossless_flac() {
        let options = AudioOptions::default().with_format(AudioFormat::Flac);

        assert!(options.effective_bitrate().is_none());
    }

    #[test]
    fn test_effective_bitrate_lossless_wav() {
        let options = AudioOptions::default().with_format(AudioFormat::Wav);

        assert!(options.effective_bitrate().is_none());
    }

    #[test]
    fn test_effective_bitrate_lossless_ignores_specified() {
        let options = AudioOptions::default()
            .with_format(AudioFormat::Flac)
            .with_bitrate("320k");

        // Lossless ignora bitrate especificado
        assert!(options.effective_bitrate().is_none());
    }

    // ============== AudioOptions Clone Tests ==============

    #[test]
    fn test_audio_options_clone() {
        let options = AudioOptions::mp3_high_quality();
        let cloned = options.clone();

        assert_eq!(options.format, cloned.format);
        assert_eq!(options.bitrate, cloned.bitrate);
    }

    #[test]
    fn test_audio_options_clone_independent() {
        let options = AudioOptions::default();
        let mut cloned = options.clone();

        cloned.overwrite = false;

        assert!(options.overwrite);
        assert!(!cloned.overwrite);
    }

    // ============== AudioExtractor detect_format Tests ==============

    #[test]
    fn test_detect_format_mp3() {
        let format = AudioExtractor::detect_format("audio.mp3");
        assert_eq!(format, Some(AudioFormat::Mp3));
    }

    #[test]
    fn test_detect_format_m4a() {
        let format = AudioExtractor::detect_format("audio.m4a");
        assert_eq!(format, Some(AudioFormat::M4a));
    }

    #[test]
    fn test_detect_format_flac() {
        let format = AudioExtractor::detect_format("audio.flac");
        assert_eq!(format, Some(AudioFormat::Flac));
    }

    #[test]
    fn test_detect_format_wav() {
        let format = AudioExtractor::detect_format("audio.wav");
        assert_eq!(format, Some(AudioFormat::Wav));
    }

    #[test]
    fn test_detect_format_opus() {
        let format = AudioExtractor::detect_format("audio.opus");
        assert_eq!(format, Some(AudioFormat::Opus));
    }

    #[test]
    fn test_detect_format_ogg() {
        let format = AudioExtractor::detect_format("audio.ogg");
        assert_eq!(format, Some(AudioFormat::Ogg));
    }

    #[test]
    fn test_detect_format_unknown() {
        let format = AudioExtractor::detect_format("audio.xyz");
        assert_eq!(format, None);
    }

    #[test]
    fn test_detect_format_no_extension() {
        let format = AudioExtractor::detect_format("audio");
        assert_eq!(format, None);
    }

    #[test]
    fn test_detect_format_with_path() {
        let format = AudioExtractor::detect_format("/path/to/audio.flac");
        assert_eq!(format, Some(AudioFormat::Flac));
    }

    #[test]
    fn test_detect_format_pathbuf() {
        let path = PathBuf::from("/home/user/music/song.mp3");
        let format = AudioExtractor::detect_format(&path);
        assert_eq!(format, Some(AudioFormat::Mp3));
    }

    // ============== AudioExtractor output_path_with_format Tests ==============

    #[test]
    fn test_output_path_with_format_simple() {
        let output = AudioExtractor::output_path_with_format("audio.wav", AudioFormat::Mp3);
        assert_eq!(output, PathBuf::from("audio.mp3"));
    }

    #[test]
    fn test_output_path_with_format_with_path() {
        let output =
            AudioExtractor::output_path_with_format("/path/to/audio.wav", AudioFormat::Flac);
        assert_eq!(output, PathBuf::from("/path/to/audio.flac"));
    }

    #[test]
    fn test_output_path_with_format_same_format() {
        let output = AudioExtractor::output_path_with_format("audio.mp3", AudioFormat::Mp3);
        assert_eq!(output, PathBuf::from("audio.mp3"));
    }

    #[test]
    fn test_output_path_with_format_all_formats() {
        let formats = vec![
            (AudioFormat::Mp3, "mp3"),
            (AudioFormat::M4a, "m4a"),
            (AudioFormat::Aac, "aac"),
            (AudioFormat::Flac, "flac"),
            (AudioFormat::Wav, "wav"),
            (AudioFormat::Opus, "opus"),
            (AudioFormat::Ogg, "ogg"),
        ];

        for (format, expected_ext) in formats {
            let output = AudioExtractor::output_path_with_format("audio.xyz", format);
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
            AudioExtractor::output_path_with_format("my_awesome_song.wav", AudioFormat::Mp3);
        assert!(output.to_string_lossy().contains("my_awesome_song"));
    }

    #[test]
    fn test_output_path_with_spaces() {
        let output =
            AudioExtractor::output_path_with_format("my song file.wav", AudioFormat::Mp3);
        assert!(output.to_string_lossy().contains("my song file"));
        assert!(output.to_string_lossy().ends_with(".mp3"));
    }

    #[test]
    fn test_output_path_with_unicode() {
        let output = AudioExtractor::output_path_with_format("música_brasileira.wav", AudioFormat::Mp3);
        assert!(output.to_string_lossy().contains("música_brasileira"));
    }

    // ============== AudioInfo Tests ==============

    #[test]
    fn test_audio_info_empty() {
        let info = AudioInfo::empty();

        assert!(info.format.is_none());
        assert!(info.duration.is_none());
        assert!(info.bitrate.is_none());
        assert!(info.sample_rate.is_none());
        assert!(info.channels.is_none());
        assert!(info.codec.is_none());
    }

    #[test]
    fn test_audio_info_is_empty_true() {
        let info = AudioInfo::empty();
        assert!(info.is_empty());
    }

    #[test]
    fn test_audio_info_is_empty_false_with_format() {
        let mut info = AudioInfo::empty();
        info.format = Some(AudioFormat::Mp3);
        assert!(!info.is_empty());
    }

    #[test]
    fn test_audio_info_is_empty_false_with_duration() {
        let mut info = AudioInfo::empty();
        info.duration = Some(180.5);
        assert!(!info.is_empty());
    }

    #[test]
    fn test_audio_info_is_empty_false_with_bitrate() {
        let mut info = AudioInfo::empty();
        info.bitrate = Some(320);
        assert!(!info.is_empty());
    }

    #[test]
    fn test_audio_info_is_empty_false_with_sample_rate() {
        let mut info = AudioInfo::empty();
        info.sample_rate = Some(44100);
        assert!(!info.is_empty());
    }

    #[test]
    fn test_audio_info_is_empty_false_with_channels() {
        let mut info = AudioInfo::empty();
        info.channels = Some(2);
        assert!(!info.is_empty());
    }

    #[test]
    fn test_audio_info_is_empty_false_with_codec() {
        let mut info = AudioInfo::empty();
        info.codec = Some("mp3".to_string());
        assert!(!info.is_empty());
    }

    #[test]
    fn test_audio_info_full() {
        let info = AudioInfo {
            format: Some(AudioFormat::Mp3),
            duration: Some(240.5),
            bitrate: Some(320),
            sample_rate: Some(44100),
            channels: Some(2),
            codec: Some("libmp3lame".to_string()),
        };

        assert!(!info.is_empty());
        assert_eq!(info.format, Some(AudioFormat::Mp3));
        assert_eq!(info.duration, Some(240.5));
        assert_eq!(info.bitrate, Some(320));
        assert_eq!(info.sample_rate, Some(44100));
        assert_eq!(info.channels, Some(2));
        assert_eq!(info.codec, Some("libmp3lame".to_string()));
    }

    #[test]
    fn test_audio_info_clone() {
        let info = AudioInfo {
            format: Some(AudioFormat::Flac),
            duration: Some(300.0),
            bitrate: None,
            sample_rate: Some(48000),
            channels: Some(2),
            codec: Some("flac".to_string()),
        };

        let cloned = info.clone();

        assert_eq!(info.format, cloned.format);
        assert_eq!(info.duration, cloned.duration);
        assert_eq!(info.sample_rate, cloned.sample_rate);
    }

    #[test]
    fn test_audio_info_debug() {
        let info = AudioInfo::empty();
        let debug_str = format!("{:?}", info);

        assert!(debug_str.contains("AudioInfo"));
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
        fn test_extract_nonexistent_file() {
            if skip_if_no_ffmpeg() {
                return;
            }

            let options = AudioOptions::default();
            let result = AudioExtractor::extract(
                "/nonexistent/video.mp4",
                "/nonexistent/audio.mp3",
                &options,
            );

            assert!(result.is_err());
        }

        #[test]
        fn test_extract_default_nonexistent_file() {
            if skip_if_no_ffmpeg() {
                return;
            }

            let result = AudioExtractor::extract_default(
                "/nonexistent/video.mp4",
                "/nonexistent/audio.mp3",
            );

            assert!(result.is_err());
        }

        #[test]
        fn test_extract_as_mp3_nonexistent_file() {
            if skip_if_no_ffmpeg() {
                return;
            }

            let result = AudioExtractor::extract_as_mp3(
                "/nonexistent/video.mp4",
                "/nonexistent/audio.mp3",
            );

            assert!(result.is_err());
        }

        #[test]
        fn test_extract_as_flac_nonexistent_file() {
            if skip_if_no_ffmpeg() {
                return;
            }

            let result = AudioExtractor::extract_as_flac(
                "/nonexistent/video.mp4",
                "/nonexistent/audio.flac",
            );

            assert!(result.is_err());
        }

        #[test]
        fn test_convert_nonexistent_file() {
            if skip_if_no_ffmpeg() {
                return;
            }

            let options = AudioOptions::default();
            let result = AudioExtractor::convert(
                "/nonexistent/audio.wav",
                "/nonexistent/audio.mp3",
                &options,
            );

            assert!(result.is_err());
        }
    }

    // ============== Edge Cases ==============

    #[test]
    fn test_audio_format_all_variants() {
        let formats = vec![
            AudioFormat::Mp3,
            AudioFormat::M4a,
            AudioFormat::Aac,
            AudioFormat::Flac,
            AudioFormat::Wav,
            AudioFormat::Opus,
            AudioFormat::Ogg,
        ];

        for format in formats {
            assert!(!format.extension().is_empty());
            assert!(!format.codec().is_empty());
            assert!(!format.default_bitrate().is_empty());
        }
    }

    #[test]
    fn test_audio_options_various_bitrates() {
        let bitrates = vec!["64k", "128k", "192k", "256k", "320k"];

        for bitrate in bitrates {
            let options = AudioOptions::default().with_bitrate(bitrate);
            assert_eq!(options.bitrate, Some(bitrate.to_string()));
        }
    }

    #[test]
    fn test_audio_options_various_sample_rates() {
        let rates = vec![22050, 44100, 48000, 96000];

        for rate in rates {
            let options = AudioOptions::default().with_sample_rate(rate);
            assert_eq!(options.sample_rate, Some(rate));
        }
    }

    #[test]
    fn test_audio_options_channel_configurations() {
        let channels = vec![1, 2, 6, 8]; // Mono, Stereo, 5.1, 7.1

        for ch in channels {
            let options = AudioOptions::default().with_channels(ch);
            assert_eq!(options.channels, Some(ch));
        }
    }

    #[test]
    fn test_roundtrip_format_extension() {
        let formats = vec![
            AudioFormat::Mp3,
            AudioFormat::M4a,
            AudioFormat::Aac,
            AudioFormat::Flac,
            AudioFormat::Wav,
            AudioFormat::Opus,
            AudioFormat::Ogg,
        ];

        for format in formats {
            let ext = format.extension();
            let parsed = AudioFormat::from_extension(ext);
            assert_eq!(parsed, Some(format));
        }
    }
}
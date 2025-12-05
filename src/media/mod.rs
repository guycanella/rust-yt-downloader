//! Media processing module for video and audio manipulation.
//!
//! This module provides comprehensive video and audio processing capabilities using FFmpeg as the
//! underlying engine. It offers high-level abstractions for common media operations including:
//!
//! - **Audio extraction**: Extract audio streams from video files
//! - **Format conversion**: Convert between different video and audio formats
//! - **FFmpeg integration**: Low-level FFmpeg command execution and management
//!
//! # Architecture
//!
//! The module is organized into three main submodules:
//!
//! - [`ffmpeg`]: Core FFmpeg integration and command execution
//! - [`audio`]: Audio extraction, conversion, and format management
//! - [`converter`]: Video format conversion and transcoding
//!
//! # Requirements
//!
//! FFmpeg must be installed and available in the system PATH. The module will automatically
//! check for FFmpeg availability before executing operations.
//!
//! # Example
//!
//! ```no_run
//! use rust_yt_downloader::media::{AudioExtractor, AudioOptions, FFmpeg};
//!
//! // Check if FFmpeg is available
//! if FFmpeg::is_available() {
//!     // Extract audio with custom options
//!     let options = AudioOptions::mp3_high_quality();
//!     AudioExtractor::extract("video.mp4", "audio.mp3", &options)?;
//! }
//! # Ok::<(), rust_yt_downloader::error::AppError>(())
//! ```

pub mod audio;
pub mod converter;
pub mod ffmpeg;

pub use audio::{AudioExtractor, AudioFormat, AudioInfo, AudioOptions};
pub use converter::{ConversionOptions, ConversionResult, VideoConverter, VideoFormat};
pub use ffmpeg::{AudioBitrate, AudioCodec, FFmpeg};

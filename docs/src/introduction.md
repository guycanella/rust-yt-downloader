# YouTube Downloader (ytdl)

A professional, feature-rich CLI tool for downloading YouTube videos and audio, built in Rust.

## Features

- 🎥 **Video Downloads**: Download videos in multiple quality options (144p to 4K)
- 🎵 **Audio Extraction**: Extract audio in various formats (MP3, FLAC, M4A, WAV, Opus)
- 📋 **Playlist Support**: Download entire playlists with a single command
- ⚙️ **Configuration Management**: Persistent settings via TOML configuration file
- 📊 **Progress Tracking**: Real-time progress bars with speed and ETA
- 🔄 **Format Conversion**: Convert between video formats using FFmpeg
- ℹ️ **Video Information**: Display metadata without downloading
- 🔁 **Automatic Retry**: Intelligent retry logic for network failures

## Why ytdl?

- **Fast**: Built in Rust for maximum performance
- **Reliable**: Comprehensive error handling and automatic retry
- **User-Friendly**: Clear progress indicators and colored output
- **Flexible**: Extensive configuration options
- **Well-Documented**: Complete API documentation and user guide

## Quick Example

```bash
# Download a video
ytdl download https://youtube.com/watch?v=dQw4w9WgXcQ

# Extract audio as MP3
ytdl audio https://youtube.com/watch?v=dQw4w9WgXcQ -f mp3

# Download a playlist
ytdl playlist https://youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf
```

## System Requirements

- **Rust**: 1.75.0 or higher (for building from source)
- **FFmpeg**: Required for audio extraction and format conversion
- **Platform**: Linux, macOS, or Windows

## License

MIT License - see LICENSE file for details

## Project Status

This project is actively maintained and ready for production use. All core features are implemented and thoroughly tested.

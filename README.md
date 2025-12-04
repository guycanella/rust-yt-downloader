# YouTube Downloader (ytdl)

[![CI](https://github.com/guycanella/rust-yt-downloader/workflows/CI/badge.svg)](https://github.com/guycanella/rust-yt-downloader/actions)
[![codecov](https://codecov.io/gh/guycanella/rust-yt-downloader/branch/main/graph/badge.svg)](https://codecov.io/gh/guycanella/rust-yt-downloader)
[![Crates.io](https://img.shields.io/crates/v/ytdl.svg)](https://crates.io/crates/ytdl)
[![Documentation](https://docs.rs/ytdl/badge.svg)](https://docs.rs/ytdl)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg?maxAge=3600)](https://github.com/rust-lang/rust)

A professional, feature-rich CLI tool for downloading YouTube videos and audio, built in Rust.

## ✨ Features

- 🎥 **Video Downloads**: Download videos in multiple quality options (144p to 4K)
- 🎵 **Audio Extraction**: Extract audio in various formats (MP3, FLAC, M4A, WAV, Opus)
- 📋 **Playlist Support**: Download entire playlists with a single command
- ⚙️ **Configuration Management**: Persistent settings via TOML configuration file
- 📊 **Progress Tracking**: Real-time progress bars with speed and ETA
- 🔄 **Format Conversion**: Convert between video formats using FFmpeg
- ℹ️ **Video Information**: Display metadata without downloading
- 🔁 **Automatic Retry**: Intelligent retry logic for network failures
- 🚀 **Fast & Reliable**: Built in Rust for maximum performance
- 📚 **Well-Documented**: Complete API documentation and user guide

## 📦 Installation

### From crates.io (Recommended)

```bash
cargo install ytdl
```

### From Source

```bash
git clone https://github.com/guycanella/rust-yt-downloader.git
cd rust-yt-downloader
cargo install --path .
```

### Dependencies

**FFmpeg** is required for audio extraction and format conversion:

- **Linux**: `sudo apt-get install ffmpeg`
- **macOS**: `brew install ffmpeg`
- **Windows**: `choco install ffmpeg` or download from [ffmpeg.org](https://ffmpeg.org)

## 🚀 Quick Start

### Download a video

```bash
ytdl download https://youtube.com/watch?v=dQw4w9WgXcQ
```

### Extract audio as MP3

```bash
ytdl audio https://youtube.com/watch?v=dQw4w9WgXcQ -f mp3
```

### Download with specific quality

```bash
ytdl download https://youtube.com/watch?v=dQw4w9WgXcQ -q 1080p -f mkv
```

### Download a playlist

```bash
ytdl playlist https://youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf
```

### View video information

```bash
ytdl info https://youtube.com/watch?v=dQw4w9WgXcQ
```

## 📖 Usage

### Commands

| Command | Description |
|---------|-------------|
| `download` | Download a YouTube video |
| `audio` | Extract and download only audio |
| `playlist` | Download multiple videos from playlists |
| `info` | Display video information without downloading |
| `config` | Manage application configuration |

### Quality Options

- `144p`, `240p`, `360p`, `480p` - Standard definition
- `720p` - HD
- `1080p` - Full HD
- `1440p` - 2K
- `4k` - Ultra HD
- `best` - Highest available (default)
- `worst` - Lowest available

### Audio Formats

- `mp3` - MP3 (lossy, widely compatible) - **default**
- `flac` - FLAC (lossless, best quality)
- `m4a` - M4A/AAC (lossy, good quality)
- `wav` - WAV (uncompressed, largest size)
- `opus` - Opus (modern codec, efficient)

### Video Formats

- `mp4` - MP4 (widely compatible) - **default**
- `mkv` - Matroska (supports more codecs)
- `webm` - WebM (open format)

## ⚙️ Configuration

Configuration file location: `~/.config/rust-yt-downloader/config.toml`

### View current configuration

```bash
ytdl config show
```

### Set a configuration value

```bash
ytdl config set general.default_quality 1080p
ytdl config set audio.format flac
```

### Get a configuration value

```bash
ytdl config get general.output_dir
```

### Reset to defaults

```bash
ytdl config reset
```

### Configuration Options

```toml
[general]
output_dir = "~/Downloads"
default_quality = "best"
max_parallel_downloads = 3

[audio]
format = "mp3"
bitrate = "320k"

[video]
format = "mp4"
include_thumbnail = true
include_subtitles = true

[network]
retry_attempts = 3
timeout = 300
```

## 📚 Documentation

- **User Guide**: Run `mdbook serve` in the `docs/` directory
- **API Documentation**: Run `cargo doc --open`
- **Online Documentation**: [docs.rs/ytdl](https://docs.rs/ytdl) (after publishing)

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test '*'

# Run with coverage
cargo tarpaulin --verbose --all-features --workspace
```

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](docs/src/dev/contributing.md) for guidelines.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [rustube](https://github.com/DzenanJupic/rustube) for YouTube API integration
- FFmpeg for media processing
- [clap](https://github.com/clap-rs/clap) for CLI argument parsing
- [indicatif](https://github.com/console-rs/indicatif) for progress bars

## 📊 Project Status

- ✅ **Production Ready**
- ✅ **700+ Tests** (unit + integration)
- ✅ **Comprehensive Documentation**
- ✅ **Active Development**

## 🐛 Bug Reports

Found a bug? Please [open an issue](https://github.com/guycanella/rust-yt-downloader/issues/new) with:
- Steps to reproduce
- Expected vs actual behavior
- Your environment (OS, Rust version, FFmpeg version)

## 💬 Support

- 📖 [User Guide](docs/src/guide/)
- 💻 [Developer Documentation](docs/src/dev/)
- 🐛 [Issue Tracker](https://github.com/guycanella/rust-yt-downloader/issues)

---

Made with ❤️ and 🦀 by Guilherme Canella

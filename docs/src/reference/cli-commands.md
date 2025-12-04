# CLI Commands Reference

Complete reference documentation for all `ytdl` command-line interface commands, arguments, and flags.

## Synopsis

```
ytdl <COMMAND> [OPTIONS] [ARGS]
```

## Global Options

These options are available for all commands:

| Option | Description |
|--------|-------------|
| `-h, --help` | Print help information |
| `-V, --version` | Print version information |

## Commands

### `download`

Download a YouTube video with specified quality and format.

**Synopsis**:
```bash
ytdl download [OPTIONS] <URL>
```

**Arguments**:

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `<URL>` | String | Yes | YouTube video URL |

**Options**:

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--quality` | `-q` | Quality | `best` | Video quality/resolution |
| `--format` | `-f` | Format | `mp4` | Output container format |
| `--output` | `-o` | Path | `.` | Output directory |
| `--silence` | `-s` | Flag | `false` | Suppress progress output |
| `--verbose` | `-v` | Flag | `false` | Enable verbose logging |

**Quality values**:
- `144p`, `240p`, `360p`, `480p`, `720p`, `1080p`, `1440p`, `4k`
- `best` - Highest available quality (default)
- `worst` - Lowest available quality

**Format values**:
- `mp4` - MP4 container (default)
- `mkv` - Matroska container
- `webm` - WebM container

**Examples**:

```bash
# Download with default settings (best quality, MP4)
ytdl download https://youtube.com/watch?v=abc123

# Download specific quality and format
ytdl download https://youtube.com/watch?v=abc123 -q 1080p -f mkv

# Download to specific directory
ytdl download https://youtube.com/watch?v=abc123 -o ~/Videos/YouTube

# Download with verbose output
ytdl download https://youtube.com/watch?v=abc123 -v

# Download quietly (no progress bars)
ytdl download https://youtube.com/watch?v=abc123 -s

# Combine multiple options
ytdl download https://youtube.com/watch?v=abc123 -q 720p -f mp4 -o ~/Videos -v
```

**Exit codes**:
- `0` - Success
- `1` - Download error
- `2` - Network error
- `3` - Invalid arguments

---

### `audio`

Extract and download only audio from a YouTube video.

**Synopsis**:
```bash
ytdl audio [OPTIONS] <URL>
```

**Arguments**:

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `<URL>` | String | Yes | YouTube video URL |

**Options**:

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--format` | `-f` | Format | `mp3` | Audio output format |
| `--output` | `-o` | Path | `.` | Output directory |
| `--silence` | `-s` | Flag | `false` | Suppress progress output |
| `--verbose` | `-v` | Flag | `false` | Enable verbose logging |

**Format values**:
- `mp3` - MP3 format (default)
- `m4a` - M4A/AAC format
- `flac` - FLAC lossless format
- `wav` - WAV uncompressed format
- `opus` - Opus format

**Examples**:

```bash
# Extract as MP3 (default, 320k bitrate)
ytdl audio https://youtube.com/watch?v=abc123

# Extract as FLAC (lossless)
ytdl audio https://youtube.com/watch?v=abc123 -f flac

# Extract as Opus
ytdl audio https://youtube.com/watch?v=abc123 -f opus

# Extract to specific directory
ytdl audio https://youtube.com/watch?v=abc123 -o ~/Music

# Multiple options
ytdl audio https://youtube.com/watch?v=abc123 -f m4a -o ~/Music -v
```

**Exit codes**:
- `0` - Success
- `1` - Extraction error
- `2` - Network error
- `3` - Invalid arguments
- `4` - FFmpeg error

---

### `playlist`

Download multiple videos from YouTube playlists.

**Synopsis**:
```bash
ytdl playlist [OPTIONS] <URL>...
```

**Arguments**:

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `<URL>...` | String(s) | Yes | One or more playlist URLs |

**Options**:

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--quality` | `-q` | Quality | `best` | Video quality for downloads |
| `--format` | `-f` | Format | `mp4` | Video container format |
| `--audio-only` | - | Flag | `false` | Download audio only |
| `--audio-format` | - | Format | `mp3` | Audio format when `--audio-only` |
| `--output` | `-o` | Path | `.` | Output directory |
| `--silence` | `-s` | Flag | `false` | Suppress progress output |
| `--verbose` | `-v` | Flag | `false` | Enable verbose logging |

**Quality values**: Same as `download` command

**Format values**: Same as `download` command

**Audio format values**: Same as `audio` command

**Examples**:

```bash
# Download entire playlist as videos
ytdl playlist https://youtube.com/playlist?list=PL123

# Download multiple playlists
ytdl playlist URL1 URL2 URL3

# Download playlist with specific quality
ytdl playlist https://youtube.com/playlist?list=PL123 -q 720p

# Download playlist as audio only
ytdl playlist https://youtube.com/playlist?list=PL123 --audio-only

# Download playlist as audio with specific format
ytdl playlist https://youtube.com/playlist?list=PL123 --audio-only --audio-format flac

# Download to specific directory with verbose output
ytdl playlist https://youtube.com/playlist?list=PL123 -o ~/Videos/Playlists -v

# Combine options
ytdl playlist https://youtube.com/playlist?list=PL123 -q 1080p -f mkv -o ~/Videos
```

**Behavior notes**:
- Downloads all videos in the playlist sequentially
- Respects `max_parallel_downloads` config setting (default: 3)
- Creates subdirectory for each playlist
- Continues on individual video errors
- Shows overall progress

**Exit codes**:
- `0` - All videos successful
- `1` - Some videos failed
- `2` - Network error
- `3` - Invalid arguments
- `5` - Playlist access error

---

### `info`

Display information about a YouTube video without downloading.

**Synopsis**:
```bash
ytdl info <URL>
```

**Arguments**:

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `<URL>` | String | Yes | YouTube video URL |

**Options**:

This command has no additional options.

**Output format**:

```
Title: Video Title
Channel: Channel Name
Duration: HH:MM:SS
Upload Date: YYYY-MM-DD
Views: N views
Likes: N likes

Description:
Video description text...

Available Qualities:
  - 4k (3840×2160) - VP9/Opus - 500 MB
  - 1440p (2560×1440) - VP9/Opus - 280 MB
  - 1080p (1920×1080) - H.264/AAC - 150 MB
  - 720p (1280×720) - H.264/AAC - 80 MB
  - 480p (854×480) - H.264/AAC - 45 MB

Available Audio Formats:
  - Opus - 160 kbps
  - AAC - 128 kbps
  - MP3 - 128 kbps

Subtitles:
  - English (auto-generated)
  - Spanish
  - French
```

**Examples**:

```bash
# Get video information
ytdl info https://youtube.com/watch?v=abc123

# Pipe to file
ytdl info https://youtube.com/watch?v=abc123 > video-info.txt

# Extract specific info with grep
ytdl info URL | grep "Duration:"
```

**Exit codes**:
- `0` - Success
- `1` - Video not found
- `2` - Network error
- `3` - Invalid URL

---

### `config`

Manage application configuration.

**Synopsis**:
```bash
ytdl config <SUBCOMMAND>
```

**Subcommands**:

#### `config show`

Display all current configuration settings.

**Synopsis**:
```bash
ytdl config show
```

**Output format**:
```toml
[general]
output_dir = "/Users/username/Downloads/YouTube"
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
rate_limit = null
retry_attempts = 3
timeout = 300
```

**Examples**:
```bash
# Show all settings
ytdl config show

# Filter specific section
ytdl config show | grep -A5 "\[audio\]"

# Save to file
ytdl config show > my-config.toml
```

#### `config set`

Set a configuration value.

**Synopsis**:
```bash
ytdl config set <KEY> <VALUE>
```

**Arguments**:

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `<KEY>` | String | Yes | Configuration key (dot notation) |
| `<VALUE>` | String | Yes | Value to set |

**Valid keys**: See [Configuration Options](./config-options.md)

**Examples**:
```bash
# Set output directory
ytdl config set general.output_dir ~/Videos/YouTube

# Set default quality
ytdl config set general.default_quality 1080p

# Set audio format
ytdl config set audio.format flac

# Set audio bitrate
ytdl config set audio.bitrate 256k

# Set video format
ytdl config set video.format mkv

# Enable/disable features
ytdl config set video.include_thumbnail true
ytdl config set video.include_subtitles false

# Network settings
ytdl config set network.retry_attempts 5
ytdl config set network.timeout 600
ytdl config set network.rate_limit "5M"

# Parallel downloads
ytdl config set general.max_parallel_downloads 5
```

**Exit codes**:
- `0` - Success
- `1` - Invalid key
- `2` - Invalid value
- `3` - Write error

#### `config get`

Get a specific configuration value.

**Synopsis**:
```bash
ytdl config get <KEY>
```

**Arguments**:

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `<KEY>` | String | Yes | Configuration key (dot notation) |

**Examples**:
```bash
# Get output directory
ytdl config get general.output_dir
# Output: /Users/username/Downloads/YouTube

# Get default quality
ytdl config get general.default_quality
# Output: best

# Get audio format
ytdl config get audio.format
# Output: mp3

# Use in scripts
OUTPUT_DIR=$(ytdl config get general.output_dir)
echo "Downloads go to: $OUTPUT_DIR"
```

**Exit codes**:
- `0` - Success
- `1` - Key not found
- `2` - Invalid key format

#### `config reset`

Reset configuration to default values.

**Synopsis**:
```bash
ytdl config reset
```

**Behavior**:
- Deletes current configuration file
- Next run will create fresh config with defaults
- Irreversible operation (prompts for confirmation)

**Examples**:
```bash
# Reset to defaults
ytdl config reset

# With confirmation
$ ytdl config reset
This will reset all configuration to defaults. Continue? [y/N]: y
Configuration reset to defaults.
```

**Exit codes**:
- `0` - Success
- `1` - Reset cancelled
- `2` - Write error

#### `config path`

Show the path to the configuration file.

**Synopsis**:
```bash
ytdl config path
```

**Output**:
- Linux/macOS: `~/.config/rust-yt-downloader/config.toml`
- Windows: `%APPDATA%\rust-yt-downloader\config.toml`

**Examples**:
```bash
# Show config file path
ytdl config path
# Output: /Users/username/.config/rust-yt-downloader/config.toml

# Open config in editor
vim $(ytdl config path)
code $(ytdl config path)
nano $(ytdl config path)

# View config file
cat $(ytdl config path)
```

**Exit codes**:
- `0` - Success
- `1` - Config directory not found

---

## URL Formats

### Supported YouTube URL Formats

All commands accept standard YouTube URLs:

**Video URLs**:
```
https://www.youtube.com/watch?v=VIDEO_ID
https://youtube.com/watch?v=VIDEO_ID
https://youtu.be/VIDEO_ID
https://m.youtube.com/watch?v=VIDEO_ID
```

**Playlist URLs**:
```
https://www.youtube.com/playlist?list=PLAYLIST_ID
https://youtube.com/playlist?list=PLAYLIST_ID
```

**Channel URLs** (for playlist command):
```
https://www.youtube.com/c/CHANNEL_NAME
https://youtube.com/@USERNAME
```

### URL Validation

- Invalid URLs result in exit code 3
- Private/deleted videos result in exit code 1
- Region-restricted content may fail with exit code 5

---

## Output Control

### Progress Display

**Default behavior**:
- Shows progress bars for downloads
- Displays current status
- Shows estimated time remaining
- Updates in real-time

**Silence mode** (`-s` / `--silence`):
- Suppresses progress bars
- Shows only errors
- Useful for scripting
- Exit codes still indicate status

**Verbose mode** (`-v` / `--verbose`):
- Shows detailed logging
- Displays HTTP requests
- Shows FFmpeg commands
- Useful for debugging

### Output Examples

**Normal output**:
```
Downloading: My Video Title
Progress: [████████████████████████░░░░░░] 80% - 2.5 MB/s - ETA 30s
```

**Verbose output**:
```
[DEBUG] Fetching video metadata: https://youtube.com/watch?v=abc123
[DEBUG] Available qualities: 1080p, 720p, 480p, 360p
[DEBUG] Selected quality: 1080p (1920×1080)
[DEBUG] Downloading video stream: format 137
[DEBUG] Downloading audio stream: format 140
[INFO] Download complete: video.mp4 (150 MB)
[DEBUG] Running FFmpeg: ffmpeg -i video.mp4 -i audio.m4a -c copy output.mp4
[INFO] Processing complete: My Video Title.mp4
```

**Silent output**:
```
(No output unless error occurs)
```

---

## Environment Variables

### Supported Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `YTDL_CONFIG` | Override config file path | `~/.ytdl.toml` |
| `YTDL_OUTPUT` | Default output directory | `~/Videos` |
| `YTDL_QUALITY` | Default video quality | `1080p` |
| `YTDL_VERBOSE` | Enable verbose mode | `1` or `true` |

**Usage**:
```bash
# Set output directory via environment
export YTDL_OUTPUT=~/Videos/YouTube
ytdl download URL  # Uses ~/Videos/YouTube

# Use custom config file
export YTDL_CONFIG=~/.config/ytdl-custom.toml
ytdl download URL

# Enable verbose mode globally
export YTDL_VERBOSE=1
ytdl download URL  # Always verbose
```

**Priority** (highest to lowest):
1. Command-line flags
2. Environment variables
3. Configuration file
4. Built-in defaults

---

## Exit Codes

### Standard Exit Codes

| Code | Meaning | Description |
|------|---------|-------------|
| `0` | Success | Operation completed successfully |
| `1` | General error | Download, extraction, or processing error |
| `2` | Network error | Connection timeout, DNS failure, HTTP error |
| `3` | Invalid arguments | Bad URL, invalid flag, missing required arg |
| `4` | FFmpeg error | FFmpeg not found, encoding error |
| `5` | Access error | Private video, region restriction, age gate |
| `6` | Filesystem error | Permission denied, disk full, path not found |

### Usage in Scripts

```bash
#!/bin/bash

ytdl download "$URL"
EXIT_CODE=$?

case $EXIT_CODE in
  0)
    echo "Download successful"
    ;;
  1)
    echo "Download failed"
    exit 1
    ;;
  2)
    echo "Network error, retrying..."
    sleep 5
    ytdl download "$URL"
    ;;
  3)
    echo "Invalid URL: $URL"
    exit 1
    ;;
  *)
    echo "Unknown error: $EXIT_CODE"
    exit $EXIT_CODE
    ;;
esac
```

---

## Shell Completion

### Generating Completion Scripts

```bash
# Bash
ytdl completions bash > ~/.local/share/bash-completion/completions/ytdl

# Zsh
ytdl completions zsh > ~/.zsh/completions/_ytdl

# Fish
ytdl completions fish > ~/.config/fish/completions/ytdl.fish

# PowerShell
ytdl completions powershell > ytdl.ps1
```

### Features

Completion provides:
- Command name completion
- Subcommand completion
- Flag/option completion
- Quality value completion
- Format value completion

---

## Related Documentation

- [Configuration Options](./config-options.md) - All configuration settings
- [Supported Formats](./formats.md) - Video and audio formats
- [Quick Start Guide](../guide/quick-start.md) - Getting started

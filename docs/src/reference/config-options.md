# Configuration Options Reference

Complete reference for all configuration options in `ytdl`, including default values, valid ranges, and usage examples.

## Configuration File Location

The configuration file is stored in TOML format at:

- **Linux/macOS**: `~/.config/rust-yt-downloader/config.toml`
- **Windows**: `%APPDATA%\rust-yt-downloader\config.toml`

**Find your config path**:
```bash
ytdl config path
```

## Configuration Structure

The configuration file is organized into four main sections:

```toml
[general]   # Application-wide settings
[audio]     # Audio download and conversion settings
[video]     # Video download settings
[network]   # Network and connection settings
```

---

## [general] Section

Application-wide settings that affect overall behavior.

### output_dir

**Type**: String (path)
**Default**: `~/Downloads/YouTube` (platform-specific)
**Description**: Default output directory for all downloads

**Valid values**:
- Absolute path: `/home/user/Videos/YouTube`
- Home-relative: `~/Videos/YouTube`
- Relative path: `./downloads` (relative to current directory)

**Examples**:
```toml
[general]
output_dir = "~/Videos/YouTube"
```

```bash
# Set via CLI
ytdl config set general.output_dir ~/Videos/YouTube

# Get current value
ytdl config get general.output_dir
```

**Notes**:
- Directory will be created if it doesn't exist
- Supports tilde expansion (`~`)
- Can be overridden with `-o` flag per download

---

### default_quality

**Type**: String
**Default**: `"best"`
**Description**: Default video quality preference

**Valid values**:
- Resolution presets: `"144p"`, `"240p"`, `"360p"`, `"480p"`, `"720p"`, `"1080p"`, `"1440p"`, `"4k"`
- Auto-selection: `"best"`, `"worst"`

**Examples**:
```toml
[general]
default_quality = "1080p"
```

```bash
# Set default quality
ytdl config set general.default_quality 1080p

# Common settings
ytdl config set general.default_quality best      # Highest available
ytdl config set general.default_quality 720p      # HD balance
ytdl config set general.default_quality worst     # Smallest files
```

**Notes**:
- Can be overridden with `-q` flag per download
- If requested quality unavailable, closest quality is selected
- "best" always selects highest available quality

---

### max_parallel_downloads

**Type**: Unsigned integer (u32)
**Default**: `3`
**Description**: Maximum number of simultaneous downloads for playlists

**Valid range**: `1` to `16`
**Recommended**: `2` to `5`

**Examples**:
```toml
[general]
max_parallel_downloads = 3
```

```bash
# Set parallel download limit
ytdl config set general.max_parallel_downloads 5

# Conservative (slow connection)
ytdl config set general.max_parallel_downloads 2

# Aggressive (fast connection)
ytdl config set general.max_parallel_downloads 8
```

**Notes**:
- Only applies to playlist downloads
- Higher values = faster but more bandwidth/CPU usage
- Consider system resources and connection speed
- YouTube may rate-limit excessive parallelism

**Performance impact**:
| Value | Playlist Speed | CPU Usage | Bandwidth |
|-------|----------------|-----------|-----------|
| 1 | Slowest | Low | Low |
| 3 | Balanced | Medium | Medium |
| 5 | Fast | High | High |
| 8+ | Fastest | Very High | Very High |

---

### filename_template

**Type**: String (template)
**Default**: `"{title}.{ext}"`
**Description**: Template for output filenames

**Available variables**:
- `{title}` - Video title
- `{channel}` - Channel name
- `{video_id}` - YouTube video ID
- `{ext}` - File extension
- `{quality}` - Quality (e.g., "1080p")
- `{upload_date}` - Upload date (YYYYMMDD)
- More in [Templates Guide](../advanced/templates.md)

**Examples**:
```toml
[general]
filename_template = "{channel} - {title}.{ext}"
```

```bash
# Set custom template
ytdl config set general.filename_template "{upload_date} - {title}.{ext}"

# Include quality in filename
ytdl config set general.filename_template "{title} [{quality}].{ext}"

# Organize by channel
ytdl config set general.filename_template "{channel}/{title}.{ext}"
```

**Template examples**:
```toml
# Simple (default)
"{title}.{ext}"

# With channel
"{channel} - {title}.{ext}"

# With date
"{upload_date}_{title}.{ext}"

# Complete metadata
"[{upload_date}] {channel} - {title} [{quality}].{ext}"
```

---

### playlist_template

**Type**: String (template)
**Default**: `"{playlist}/{playlist_index:02} - {title}.{ext}"`
**Description**: Template for playlist downloads

**Additional variables** (vs. filename_template):
- `{playlist}` - Playlist name
- `{playlist_index}` - Video position in playlist
- `{playlist_id}` - Playlist ID

**Examples**:
```toml
[general]
playlist_template = "{playlist}/{playlist_index:02} - {title}.{ext}"
```

```bash
# Numbered playlist
ytdl config set general.playlist_template "{playlist_index:02}. {title}.{ext}"

# Nested structure
ytdl config set general.playlist_template "{channel}/{playlist}/{title}.{ext}"
```

---

## [audio] Section

Settings for audio downloads and extraction.

### format

**Type**: String
**Default**: `"mp3"`
**Description**: Default audio output format

**Valid values**:
- `"mp3"` - MP3 (lossy, universal compatibility)
- `"m4a"` - M4A/AAC (lossy, better quality than MP3)
- `"flac"` - FLAC (lossless compression)
- `"wav"` - WAV (uncompressed)
- `"opus"` - Opus (modern lossy codec)

**Examples**:
```toml
[audio]
format = "mp3"
```

```bash
# Set audio format
ytdl config set audio.format mp3

# Lossless archival
ytdl config set audio.format flac

# Modern efficient codec
ytdl config set audio.format opus

# Maximum compatibility
ytdl config set audio.format mp3
```

**Format comparison**:
| Format | Type | Quality | File Size | Compatibility |
|--------|------|---------|-----------|---------------|
| mp3 | Lossy | Good | Medium | Universal |
| m4a | Lossy | Better | Medium | Very High |
| flac | Lossless | Perfect | Large | High |
| wav | Uncompressed | Perfect | Largest | Universal |
| opus | Lossy | Best | Small | Modern |

---

### bitrate

**Type**: String
**Default**: `"320k"`
**Description**: Audio bitrate for lossy formats

**Valid values**: `"128k"`, `"192k"`, `"256k"`, `"320k"`
**Ignored for**: FLAC, WAV (lossless/uncompressed)

**Examples**:
```toml
[audio]
bitrate = "320k"
```

```bash
# Set bitrate
ytdl config set audio.bitrate 320k

# Quality options
ytdl config set audio.bitrate 128k  # Low quality, small files
ytdl config set audio.bitrate 192k  # Acceptable quality
ytdl config set audio.bitrate 256k  # Good quality
ytdl config set audio.bitrate 320k  # High quality (default)
```

**Bitrate guide**:
| Bitrate | Quality | File Size (per minute) | Use Case |
|---------|---------|------------------------|----------|
| 128k | Low | ~1 MB | Podcasts, speech |
| 192k | Acceptable | ~1.5 MB | General listening |
| 256k | Good | ~2 MB | Music, balanced |
| 320k | High | ~2.5 MB | Music, archival |

---

### normalize_audio

**Type**: Boolean
**Default**: `false`
**Description**: Normalize audio volume levels

**Examples**:
```toml
[audio]
normalize_audio = true
```

```bash
# Enable normalization
ytdl config set audio.normalize_audio true

# Disable
ytdl config set audio.normalize_audio false
```

**Notes**:
- Useful for playlists with varying volume levels
- Uses FFmpeg loudnorm filter
- Slightly slower processing

---

## [video] Section

Settings specific to video downloads.

### format

**Type**: String
**Default**: `"mp4"`
**Description**: Default video container format

**Valid values**:
- `"mp4"` - MP4 container (maximum compatibility)
- `"mkv"` - Matroska container (feature-rich)
- `"webm"` - WebM container (open format)

**Examples**:
```toml
[video]
format = "mp4"
```

```bash
# Set video format
ytdl config set video.format mp4

# Format recommendations
ytdl config set video.format mp4   # Maximum compatibility
ytdl config set video.format mkv   # Feature-rich, archival
ytdl config set video.format webm  # Web-optimized, open
```

**Format comparison**:
| Format | Compatibility | Features | Codecs Supported |
|--------|---------------|----------|------------------|
| mp4 | Highest | Basic | H.264, AAC |
| mkv | High | Advanced | All codecs |
| webm | Medium | Web-focused | VP8/VP9, Opus |

---

### include_thumbnail

**Type**: Boolean
**Default**: `true`
**Description**: Download video thumbnail image

**Examples**:
```toml
[video]
include_thumbnail = true
```

```bash
# Enable thumbnail download
ytdl config set video.include_thumbnail true

# Disable
ytdl config set video.include_thumbnail false
```

**Notes**:
- Thumbnail saved as separate `.jpg` file
- Same filename as video
- Useful for media library organization

---

### include_subtitles

**Type**: Boolean
**Default**: `true`
**Description**: Download available subtitles

**Examples**:
```toml
[video]
include_subtitles = true
```

```bash
# Enable subtitle download
ytdl config set video.include_subtitles true

# Disable
ytdl config set video.include_subtitles false
```

**Notes**:
- Downloads all available subtitle languages
- Saved as `.vtt` or `.srt` files
- Auto-generated and manual subtitles both included

---

### embed_thumbnail

**Type**: Boolean
**Default**: `false`
**Description**: Embed thumbnail in video file metadata

**Examples**:
```toml
[video]
embed_thumbnail = true
```

```bash
# Enable embedding
ytdl config set video.embed_thumbnail true
```

**Notes**:
- Requires `include_thumbnail = true`
- Supported by MP4 and MKV
- Visible in media players

---

### embed_subtitles

**Type**: Boolean
**Default**: `false`
**Description**: Embed subtitles in video file

**Examples**:
```toml
[video]
embed_subtitles = true
```

```bash
# Enable embedding
ytdl config set video.embed_subtitles true
```

**Notes**:
- Requires `include_subtitles = true`
- MKV supports all subtitle formats
- MP4 requires conversion to mov_text

---

### embed_metadata

**Type**: Boolean
**Default**: `true`
**Description**: Embed video metadata (title, artist, date)

**Examples**:
```toml
[video]
embed_metadata = true
```

```bash
# Enable metadata embedding
ytdl config set video.embed_metadata true

# Disable
ytdl config set video.embed_metadata false
```

**Metadata fields**:
- Title (video title)
- Artist (channel name)
- Date (upload date)
- Comment (video URL)
- Description

---

## [network] Section

Network and connection settings.

### rate_limit

**Type**: String (optional)
**Default**: `null` (no limit)
**Description**: Download rate limit

**Valid values**:
- `null` or `"none"` - No limit
- `"<number>K"` - KB/s (e.g., `"500K"`)
- `"<number>M"` - MB/s (e.g., `"5M"`)

**Examples**:
```toml
[network]
rate_limit = "5M"
```

```bash
# Set rate limit
ytdl config set network.rate_limit "5M"   # 5 MB/s
ytdl config set network.rate_limit "500K" # 500 KB/s

# Remove limit
ytdl config set network.rate_limit none
```

**Use cases**:
```bash
# Metered connection
ytdl config set network.rate_limit "1M"

# Background downloading
ytdl config set network.rate_limit "500K"

# Maximum speed
ytdl config set network.rate_limit none
```

---

### retry_attempts

**Type**: Unsigned integer (u32)
**Default**: `3`
**Description**: Number of retry attempts for failed downloads

**Valid range**: `0` to `10`
**Recommended**: `3` to `5`

**Examples**:
```toml
[network]
retry_attempts = 3
```

```bash
# Set retry attempts
ytdl config set network.retry_attempts 3

# More aggressive (unstable connection)
ytdl config set network.retry_attempts 5

# No retries (fail fast)
ytdl config set network.retry_attempts 0
```

**Notes**:
- Only retries transient errors (network timeouts, HTTP 5xx)
- Does not retry permanent errors (HTTP 404, 403)
- Exponential backoff between retries

---

### timeout

**Type**: Unsigned integer (u64)
**Default**: `300` (5 minutes)
**Description**: Connection timeout in seconds

**Valid range**: `30` to `3600` (1 minute to 1 hour)
**Recommended**: `300` (5 minutes)

**Examples**:
```toml
[network]
timeout = 300
```

```bash
# Set timeout
ytdl config set network.timeout 300

# Short timeout (fast connection)
ytdl config set network.timeout 60

# Long timeout (slow connection)
ytdl config set network.timeout 600

# Very long (large 4K files)
ytdl config set network.timeout 1800
```

**Timeout guide**:
| Timeout | Use Case |
|---------|----------|
| 60s | Fast connection, small files |
| 300s | Default, balanced |
| 600s | Slow connection, large files |
| 1800s | Very slow connection, 4K |

---

### proxy

**Type**: String (optional)
**Default**: `null` (no proxy)
**Description**: HTTP/HTTPS proxy server

**Valid values**:
- `null` - No proxy
- `"http://host:port"` - HTTP proxy
- `"https://host:port"` - HTTPS proxy
- `"socks5://host:port"` - SOCKS5 proxy

**Examples**:
```toml
[network]
proxy = "http://proxy.example.com:8080"
```

```bash
# Set HTTP proxy
ytdl config set network.proxy "http://proxy.example.com:8080"

# Set SOCKS5 proxy
ytdl config set network.proxy "socks5://localhost:1080"

# Remove proxy
ytdl config set network.proxy none
```

**With authentication**:
```toml
proxy = "http://user:pass@proxy.example.com:8080"
```

---

### user_agent

**Type**: String
**Default**: Auto-generated (e.g., `"ytdl/1.0.0"`)
**Description**: HTTP User-Agent header

**Examples**:
```toml
[network]
user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64)"
```

```bash
# Set custom user agent
ytdl config set network.user_agent "Mozilla/5.0 ..."

# Reset to default
ytdl config set network.user_agent default
```

---

## [ffmpeg] Section

FFmpeg configuration (advanced users).

### binary_path

**Type**: String (optional)
**Default**: Auto-detect from PATH
**Description**: Path to FFmpeg binary

**Examples**:
```toml
[ffmpeg]
binary_path = "/usr/local/bin/ffmpeg"
```

```bash
# Set custom FFmpeg path
ytdl config set ffmpeg.binary_path /opt/ffmpeg/bin/ffmpeg

# Auto-detect (default)
ytdl config set ffmpeg.binary_path auto
```

---

### preset

**Type**: String
**Default**: `"medium"`
**Description**: FFmpeg encoding preset

**Valid values**:
- `"ultrafast"`, `"superfast"`, `"veryfast"`, `"faster"`, `"fast"`
- `"medium"` (default)
- `"slow"`, `"slower"`, `"veryslow"`

**Examples**:
```toml
[ffmpeg]
preset = "medium"
```

```bash
# Set preset
ytdl config set ffmpeg.preset medium

# Fast encoding (larger files)
ytdl config set ffmpeg.preset fast

# Slow encoding (smaller files, better quality)
ytdl config set ffmpeg.preset slow
```

**Preset comparison**:
| Preset | Speed | File Size | Quality |
|--------|-------|-----------|---------|
| ultrafast | Fastest | Largest | Good |
| fast | Fast | Large | Good |
| medium | Balanced | Medium | Very Good |
| slow | Slow | Small | Excellent |
| veryslow | Slowest | Smallest | Excellent |

---

### threads

**Type**: Unsigned integer (u32)
**Default**: `0` (auto-detect)
**Description**: Number of FFmpeg encoding threads

**Valid range**: `0` to `64`
**Recommended**: `0` (auto) or number of CPU cores

**Examples**:
```toml
[ffmpeg]
threads = 0
```

```bash
# Auto-detect (recommended)
ytdl config set ffmpeg.threads 0

# Manual setting
ytdl config set ffmpeg.threads 4
```

---

## Complete Configuration Example

```toml
# ~/.config/rust-yt-downloader/config.toml

[general]
output_dir = "~/Videos/YouTube"
default_quality = "1080p"
max_parallel_downloads = 3
filename_template = "{channel} - {title}.{ext}"
playlist_template = "{playlist}/{playlist_index:02} - {title}.{ext}"

[audio]
format = "mp3"
bitrate = "320k"
normalize_audio = false

[video]
format = "mp4"
include_thumbnail = true
include_subtitles = true
embed_thumbnail = false
embed_subtitles = false
embed_metadata = true

[network]
rate_limit = null
retry_attempts = 3
timeout = 300
proxy = null
user_agent = "ytdl/1.0.0"

[ffmpeg]
binary_path = null
preset = "medium"
threads = 0
```

---

## Validation Rules

### General Rules

- All paths are validated and expanded (tilde, environment variables)
- Boolean values must be `true` or `false`
- Numeric values must be within valid ranges
- Invalid keys are rejected with error messages

### Type Validation

**String values**: Must be quoted in TOML
```toml
format = "mp4"      # Correct
format = mp4        # Error (unless bare strings allowed)
```

**Boolean values**: Unquoted true/false
```toml
include_thumbnail = true   # Correct
include_thumbnail = "true" # Error
```

**Numeric values**: Unquoted integers
```toml
timeout = 300      # Correct
timeout = "300"    # Error
```

---

## Environment Variable Override

Configuration values can be overridden with environment variables:

```bash
# Override output directory
export YTDL_OUTPUT_DIR=~/Videos

# Override quality
export YTDL_DEFAULT_QUALITY=720p

# Override audio format
export YTDL_AUDIO_FORMAT=flac
```

**Priority** (highest to lowest):
1. Command-line flags (`-q`, `-f`, `-o`)
2. Environment variables (`YTDL_*`)
3. Configuration file (`config.toml`)
4. Built-in defaults

---

## Related Documentation

- [CLI Commands](./cli-commands.md) - Command-line usage
- [Templates Guide](../advanced/templates.md) - Filename templating
- [FFmpeg Integration](../advanced/ffmpeg.md) - FFmpeg configuration

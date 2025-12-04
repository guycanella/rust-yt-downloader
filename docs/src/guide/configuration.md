# Configuration

This comprehensive guide covers the `ytdl` configuration system, allowing you to set default preferences and customize behavior to suit your needs.

## Configuration Overview

Instead of typing the same options repeatedly, you can configure defaults in a configuration file. For example, if you always download in 720p MP4 format to a specific folder, you can set these as defaults.

**Priority Order:**
1. **Command-line arguments** (highest priority)
2. **Configuration file**
3. **Built-in defaults** (lowest priority)

This means CLI arguments always override config file settings, which override built-in defaults.

## Configuration File Location

The configuration file is automatically created the first time you use a `config` command.

### File Path by Operating System

**Linux:**
```
~/.config/rust-yt-downloader/config.toml
```

**macOS:**
```
~/.config/rust-yt-downloader/config.toml
```

**Windows:**
```
%APPDATA%\rust-yt-downloader\config.toml
```

### Finding Your Config File

Use the `path` command to display the exact location:

```bash
ytdl config path
```

Output:
```
Configuration file: /home/username/.config/rust-yt-downloader/config.toml
```

## Configuration Commands

### Show All Settings

Display your current configuration:

```bash
ytdl config show
```

Example output:
```toml
[general]
output_dir = "/home/username/Downloads/YouTube"
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
rate_limit = ""
retry_attempts = 3
timeout = 300
```

### Get a Specific Setting

Retrieve the value of a single configuration option:

```bash
ytdl config get KEY
```

Examples:
```bash
# Get default video quality
ytdl config get general.default_quality

# Get audio format
ytdl config get audio.format

# Get output directory
ytdl config get general.output_dir
```

### Set a Configuration Value

Change a configuration setting:

```bash
ytdl config set KEY VALUE
```

Examples:
```bash
# Set default quality to 1080p
ytdl config set general.default_quality 1080p

# Change output directory
ytdl config set general.output_dir ~/Videos/YouTube

# Set audio format to FLAC
ytdl config set audio.format flac

# Set audio bitrate
ytdl config set audio.bitrate 192k

# Set video format to MKV
ytdl config set video.format mkv
```

### Reset to Defaults

Reset all settings to their default values:

```bash
ytdl config reset
```

This will prompt for confirmation:
```
Warning: This will reset all configuration to default values.
Continue? (y/n):
```

Type `y` to confirm or `n` to cancel.

## Configuration Sections

The configuration is organized into four sections: `general`, `audio`, `video`, and `network`.

---

## General Settings

General application-wide settings.

### `general.output_dir`

**Default:** `~/Downloads/YouTube`

**Description:** Default directory for downloaded files

**Type:** String (file path)

**Examples:**
```bash
# Set to home Downloads folder
ytdl config set general.output_dir ~/Downloads

# Set to specific Videos folder
ytdl config set general.output_dir ~/Videos/YouTube

# Windows example
ytdl config set general.output_dir C:\Users\Username\Videos
```

**When to use:**
- You always download to the same location
- You want to organize downloads in a dedicated folder

---

### `general.default_quality`

**Default:** `"best"`

**Description:** Default video quality for downloads

**Type:** String

**Valid values:** `"144p"`, `"240p"`, `"360p"`, `"480p"`, `"720p"`, `"1080p"`, `"1440p"`, `"4k"`, `"best"`, `"worst"`

**Examples:**
```bash
# Always download in 720p
ytdl config set general.default_quality 720p

# Always download best quality
ytdl config set general.default_quality best

# Conservative quality for limited storage
ytdl config set general.default_quality 480p
```

**When to use:**
- You consistently prefer a specific quality level
- Your display or storage constraints favor a certain resolution
- You want to conserve bandwidth with lower defaults

**Note:** CLI `-q` flag overrides this setting.

---

### `general.max_parallel_downloads`

**Default:** `3`

**Description:** Maximum number of simultaneous downloads (for playlists)

**Type:** Integer (1-10 recommended)

**Examples:**
```bash
# Download one at a time (slower but safer)
ytdl config set general.max_parallel_downloads 1

# Download 5 at a time (faster with good connection)
ytdl config set general.max_parallel_downloads 5

# Conservative setting
ytdl config set general.max_parallel_downloads 2
```

**When to use:**
- Adjust based on internet connection speed
- Lower values for stability
- Higher values for faster playlist downloads (if your connection can handle it)

**Recommendations:**
- **Slow connection (<5 Mbps):** 1-2
- **Medium connection (5-25 Mbps):** 2-3
- **Fast connection (>25 Mbps):** 3-5

---

## Audio Settings

Settings specific to audio extraction and conversion.

### `audio.format`

**Default:** `"mp3"`

**Description:** Default format for audio extraction

**Type:** String

**Valid values:** `"mp3"`, `"flac"`, `"m4a"`, `"wav"`, `"opus"`

**Examples:**
```bash
# Use FLAC for lossless archival
ytdl config set audio.format flac

# Use M4A for Apple devices
ytdl config set audio.format m4a

# Use Opus for smallest files
ytdl config set audio.format opus
```

**When to use:**
- You consistently prefer one audio format
- Building a library in a specific format
- Optimizing for specific devices

**Note:** CLI `-f` flag overrides this setting.

**Format Guide:**
- `mp3` - Most compatible, good quality
- `flac` - Lossless, large files, archival
- `m4a` - Great for Apple ecosystem
- `wav` - Uncompressed, professional use
- `opus` - Best compression, smallest files

---

### `audio.bitrate`

**Default:** `"320k"`

**Description:** Default bitrate for lossy audio formats (MP3, M4A, Opus)

**Type:** String

**Valid values:** `"128k"`, `"192k"`, `"256k"`, `"320k"`, or any value ending in `k`

**Examples:**
```bash
# Maximum MP3 quality
ytdl config set audio.bitrate 320k

# Balanced quality and size
ytdl config set audio.bitrate 192k

# For podcasts/speech
ytdl config set audio.bitrate 128k
```

**When to use:**
- You have preferences about file size vs. quality
- Building a consistent library
- Optimizing for specific use cases (music vs. podcasts)

**Note:**
- Only affects lossy formats (MP3, M4A, Opus)
- FLAC and WAV ignore this setting (they're lossless/uncompressed)
- CLI `-b` flag overrides this setting

**Bitrate Guide:**
- `128k` - Acceptable for speech, podcasts
- `192k` - Good for casual music listening
- `256k` - Very good quality
- `320k` - Maximum MP3 quality (transparent)

---

## Video Settings

Settings specific to video downloads.

### `video.format`

**Default:** `"mp4"`

**Description:** Default container format for video downloads

**Type:** String

**Valid values:** `"mp4"`, `"mkv"`, `"webm"`

**Examples:**
```bash
# Use MP4 for maximum compatibility
ytdl config set video.format mp4

# Use MKV for advanced features
ytdl config set video.format mkv

# Use WebM for web content
ytdl config set video.format webm
```

**When to use:**
- You consistently prefer one container format
- Your devices work best with a specific format
- You're building a video library

**Note:** CLI `-f` flag overrides this setting.

**Format Guide:**
- `mp4` - Universal compatibility (recommended)
- `mkv` - More features, better for advanced users
- `webm` - Open format, good for web use

---

### `video.include_thumbnail`

**Default:** `true`

**Description:** Download video thumbnail image along with video

**Type:** Boolean (`true` or `false`)

**Examples:**
```bash
# Download thumbnails (default)
ytdl config set video.include_thumbnail true

# Skip thumbnails
ytdl config set video.include_thumbnail false
```

**When to use:**
- `true` - If you want thumbnail images for video library organization
- `false` - If you want to save minimal disk space and don't need thumbnails

**Note:** Thumbnails are typically small (50-500 KB) and useful for media library software.

---

### `video.include_subtitles`

**Default:** `true`

**Description:** Download available subtitles along with video

**Type:** Boolean (`true` or `false`)

**Examples:**
```bash
# Download subtitles (default)
ytdl config set video.include_subtitles true

# Skip subtitles
ytdl config set video.include_subtitles false
```

**When to use:**
- `true` - If you want subtitles for accessibility or language learning
- `false` - If you don't need subtitles and want to save disk space

**Note:** Subtitles are typically very small (10-100 KB) and saved as separate `.srt` or `.vtt` files.

---

## Network Settings

Settings related to network connections and downloads.

### `network.rate_limit`

**Default:** `""` (empty, no limit)

**Description:** Bandwidth limit for downloads

**Type:** String

**Valid values:** Empty string (no limit), or values like `"5M"`, `"1M"`, `"500K"`

**Examples:**
```bash
# Limit to 5 megabytes per second
ytdl config set network.rate_limit 5M

# Limit to 1 megabyte per second
ytdl config set network.rate_limit 1M

# Limit to 500 kilobytes per second
ytdl config set network.rate_limit 500K

# Remove limit
ytdl config set network.rate_limit ""
```

**When to use:**
- Prevent downloads from consuming all bandwidth
- Leave bandwidth for other applications
- Avoid ISP throttling on metered connections
- Download overnight at lower speeds

**Format:**
- Numbers + `K` for kilobytes/second (e.g., `500K`)
- Numbers + `M` for megabytes/second (e.g., `5M`)
- Empty string for no limit

---

### `network.retry_attempts`

**Default:** `3`

**Description:** Number of times to retry failed downloads

**Type:** Integer (0-10 recommended)

**Examples:**
```bash
# Retry up to 5 times
ytdl config set network.retry_attempts 5

# No retries (fail immediately)
ytdl config set network.retry_attempts 0

# Conservative retry (default)
ytdl config set network.retry_attempts 3
```

**When to use:**
- Increase for unreliable connections
- Decrease if you prefer faster failures
- Set to 0 for debugging connection issues

**Recommendations:**
- **Stable connection:** 2-3
- **Unstable connection:** 5-7
- **Debugging:** 0

---

### `network.timeout`

**Default:** `300` (5 minutes)

**Description:** Connection timeout in seconds

**Type:** Integer (seconds)

**Examples:**
```bash
# 10 minute timeout for slow connections
ytdl config set network.timeout 600

# 2 minute timeout for fast connections
ytdl config set network.timeout 120

# Very patient timeout (30 minutes)
ytdl config set network.timeout 1800
```

**When to use:**
- Increase for slow/unstable connections
- Increase for very large files (4K videos)
- Decrease for fast connections to fail faster

**Recommendations:**
- **Fast connection:** 120-300 seconds
- **Medium connection:** 300-600 seconds
- **Slow/unstable connection:** 600-1800 seconds

---

## Example Configurations

Here are complete example configurations for different use cases:

### Example 1: Music Collector

Optimized for downloading music in high quality:

```bash
ytdl config set general.output_dir ~/Music/YouTube
ytdl config set general.default_quality best
ytdl config set audio.format flac
ytdl config set audio.bitrate 320k
ytdl config set network.retry_attempts 5
```

**Result:** Downloads to Music folder, always best quality, FLAC format, patient retries.

### Example 2: Podcast Listener

Optimized for podcasts and spoken content:

```bash
ytdl config set general.output_dir ~/Podcasts
ytdl config set audio.format mp3
ytdl config set audio.bitrate 128k
ytdl config set video.include_thumbnail true
ytdl config set video.include_subtitles false
```

**Result:** Downloads to Podcasts folder, small MP3 files, thumbnails for organization.

### Example 3: Educational Content

Optimized for courses and tutorials:

```bash
ytdl config set general.output_dir ~/Education
ytdl config set general.default_quality 720p
ytdl config set video.format mp4
ytdl config set video.include_subtitles true
ytdl config set network.timeout 600
```

**Result:** Downloads to Education folder, good quality (readable text), subtitles for learning.

### Example 4: Limited Storage

Optimized for minimal disk space usage:

```bash
ytdl config set general.default_quality 480p
ytdl config set audio.format opus
ytdl config set audio.bitrate 128k
ytdl config set video.format webm
ytdl config set video.include_thumbnail false
ytdl config set video.include_subtitles false
```

**Result:** Smaller files, efficient formats, no extras.

### Example 5: Archival Quality

Optimized for long-term archival:

```bash
ytdl config set general.output_dir ~/Videos/Archive
ytdl config set general.default_quality best
ytdl config set audio.format flac
ytdl config set video.format mkv
ytdl config set video.include_thumbnail true
ytdl config set video.include_subtitles true
```

**Result:** Best quality, flexible formats, all metadata preserved.

### Example 6: Slow Connection

Optimized for slow/unreliable internet:

```bash
ytdl config set general.default_quality 360p
ytdl config set general.max_parallel_downloads 1
ytdl config set network.rate_limit 1M
ytdl config set network.retry_attempts 7
ytdl config set network.timeout 900
```

**Result:** Lower quality, one at a time, limited bandwidth, patient retries.

## Configuration File Format

The configuration file is in TOML format. You can also edit it directly with a text editor:

```toml
[general]
output_dir = "/home/username/Downloads/YouTube"
default_quality = "1080p"
max_parallel_downloads = 3

[audio]
format = "mp3"
bitrate = "320k"

[video]
format = "mp4"
include_thumbnail = true
include_subtitles = true

[network]
rate_limit = ""
retry_attempts = 3
timeout = 300
```

**To edit manually:**

```bash
# Linux/macOS
nano ~/.config/rust-yt-downloader/config.toml

# Windows (PowerShell)
notepad $env:APPDATA\rust-yt-downloader\config.toml
```

> **Warning:** Make sure your syntax is correct when editing manually. Invalid TOML will cause errors.

## Configuration Key Reference

Quick reference table of all configuration keys:

| Key | Type | Default | Valid Values |
|-----|------|---------|--------------|
| `general.output_dir` | String | `~/Downloads/YouTube` | Any file path |
| `general.default_quality` | String | `"best"` | 144p, 240p, 360p, 480p, 720p, 1080p, 1440p, 4k, best, worst |
| `general.max_parallel_downloads` | Integer | `3` | 1-10 |
| `audio.format` | String | `"mp3"` | mp3, flac, m4a, wav, opus |
| `audio.bitrate` | String | `"320k"` | 128k, 192k, 256k, 320k, etc. |
| `video.format` | String | `"mp4"` | mp4, mkv, webm |
| `video.include_thumbnail` | Boolean | `true` | true, false |
| `video.include_subtitles` | Boolean | `true` | true, false |
| `network.rate_limit` | String | `""` | Empty or "5M", "1M", "500K", etc. |
| `network.retry_attempts` | Integer | `3` | 0-10 |
| `network.timeout` | Integer | `300` | Any positive integer (seconds) |

## Troubleshooting

### Config File Not Found

If you get "config file not found" errors:

1. **Create it:** Run any `config set` command to create the file automatically
2. **Check path:** Use `ytdl config path` to verify the location
3. **Check permissions:** Ensure you have write access to the config directory

### Invalid Configuration

If you get "invalid configuration" errors:

1. **Reset to defaults:** `ytdl config reset`
2. **Check syntax:** Ensure TOML syntax is valid if edited manually
3. **Verify values:** Use valid values from the reference table above

### Settings Not Taking Effect

If your config settings aren't being used:

1. **Check CLI override:** CLI arguments always override config settings
2. **Verify the key:** Use `ytdl config get KEY` to check the actual value
3. **Reload config:** Some settings may require restarting long-running processes

## Best Practices

1. **Start with defaults** - Only change settings you actually need to customize
2. **Use `config show`** - Review all settings periodically to ensure they match your needs
3. **Back up your config** - Copy the config file when you have a working setup
4. **Test changes** - After changing settings, test with a small download to verify
5. **Document custom values** - Add comments in the TOML file (lines starting with `#`) to remember why you set certain values
6. **Reset when in doubt** - Use `ytdl config reset` if you get confused about your settings

## Next Steps

Now that you understand configuration, you can:

- **Customize your defaults** to match your workflow
- **Experiment with different profiles** by backing up and swapping config files
- **Optimize for specific use cases** using the example configurations above

Return to the guides to apply your configuration:
- **[Quick Start](./quick-start.md)** - Basic usage with your new defaults
- **[Downloading Videos](./downloading-videos.md)** - Video downloads with custom settings
- **[Extracting Audio](./extracting-audio.md)** - Audio extraction with your preferences
- **[Playlists](./playlists.md)** - Bulk downloads using configured defaults

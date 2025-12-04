# Downloading Videos

This comprehensive guide covers everything you need to know about downloading YouTube videos with `ytdl`, including quality selection, format options, and advanced features.

## Basic Video Download

The simplest way to download a video is:

```bash
ytdl download https://www.youtube.com/watch?v=VIDEO_ID
```

This downloads the video in the best available quality as an MP4 file to your current directory.

### Supported URL Formats

`ytdl` accepts various YouTube URL formats:

```bash
# Standard format
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw

# Short format
ytdl download https://youtu.be/jNQXAC9IVRw

# With www subdomain
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw

# Mobile format
ytdl download https://m.youtube.com/watch?v=jNQXAC9IVRw
```

All of these will work identically.

## Quality Selection

YouTube videos are available in various quality levels, from 144p up to 4K. You can specify your desired quality using the `-q` or `--quality` flag.

### Available Quality Options

| Quality | Resolution | Description | Typical Use Case |
|---------|-----------|-------------|------------------|
| `144p` | 256×144 | Lowest quality | Minimal bandwidth, quick previews |
| `240p` | 426×240 | Very low | Mobile data saving |
| `360p` | 640×360 | Standard definition | Older devices, limited storage |
| `480p` | 854×480 | Standard definition | DVD-like quality |
| `720p` | 1280×720 | HD (High Definition) | Good balance of quality/size |
| `1080p` | 1920×1080 | Full HD | High quality, modern standard |
| `1440p` | 2560×1440 | 2K / Quad HD | High-end displays |
| `4k` | 3840×2160 | Ultra HD / 4K | Best quality, largest files |
| `best` | Variable | Highest available (default) | Maximum quality |
| `worst` | Variable | Lowest available | Minimal file size |

### Quality Examples

**Download in 720p (HD):**
```bash
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw -q 720p
```

**Download in 1080p (Full HD):**
```bash
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw -q 1080p
```

**Download in 4K (if available):**
```bash
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw -q 4k
```

**Download in lowest quality (smallest file):**
```bash
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw -q worst
```

**Download in best available quality:**
```bash
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw -q best
```

> **Note:** If the requested quality isn't available, `ytdl` will automatically select the closest available quality and notify you.

### Quality vs. File Size

Here's a rough guide to file sizes for a 10-minute video:

- **144p**: ~10-20 MB
- **240p**: ~20-40 MB
- **360p**: ~40-80 MB
- **480p**: ~80-150 MB
- **720p**: ~150-300 MB
- **1080p**: ~300-600 MB
- **1440p**: ~600-1200 MB
- **4K**: ~1200-2500 MB

Actual sizes vary based on video content, compression, and frame rate.

## Format Selection

`ytdl` supports three video container formats. Use the `-f` or `--format` flag to specify your preference.

### Available Video Formats

#### MP4 (Default - Recommended)
- **Compatibility**: Excellent (plays on virtually all devices)
- **Quality**: High
- **File Size**: Medium
- **Best For**: General use, sharing, web playback

```bash
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw -f mp4
```

#### MKV (Matroska)
- **Compatibility**: Good (modern devices and players)
- **Quality**: Excellent (supports more codecs)
- **File Size**: Medium to large
- **Best For**: Archival, advanced features (multiple audio tracks, subtitles)

```bash
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw -f mkv
```

#### WebM
- **Compatibility**: Good (all modern browsers, some devices)
- **Quality**: High
- **File Size**: Small to medium
- **Best For**: Web use, open-source enthusiasts

```bash
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw -f webm
```

### Combining Quality and Format

You can specify both quality and format together:

```bash
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw -q 1080p -f mkv
```

This downloads in 1080p quality as an MKV file.

## Output Directory

By default, videos are saved to your current directory. Use `-o` or `--output` to specify a different location.

### Basic Output Directory

```bash
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw -o ~/Videos
```

This saves the video to your home Videos folder.

### Platform-Specific Paths

**Linux/macOS:**
```bash
# Home directory
ytdl download URL -o ~/Downloads/YouTube

# Absolute path
ytdl download URL -o /home/username/Videos/YouTube
```

**Windows:**
```bash
# User folder
ytdl download URL -o C:\Users\Username\Videos\YouTube

# Using environment variables (PowerShell)
ytdl download URL -o $env:USERPROFILE\Videos\YouTube
```

### Creating Organized Folders

Organize downloads by creating subdirectories:

```bash
# Music videos
ytdl download URL -o ~/Videos/Music

# Tutorials
ytdl download URL -o ~/Videos/Tutorials

# Entertainment
ytdl download URL -o ~/Videos/Entertainment
```

> **Tip:** The directory will be created automatically if it doesn't exist.

## Advanced Options

### Silent Mode

Suppress progress bars and non-error output:

```bash
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw --silent
```

Useful for:
- Scripts and automation
- Background downloads
- Logging only errors

### Combining All Options

Here's a complete example using multiple options:

```bash
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw \
  -q 1080p \
  -f mkv \
  -o ~/Videos/YouTube \
  --silent
```

This command:
- Downloads in 1080p quality
- Saves as MKV format
- Stores in `~/Videos/YouTube/`
- Runs silently (no progress bars)

## Real-World Examples

### Example 1: Download a Tutorial

Download a coding tutorial in good quality:

```bash
ytdl download https://www.youtube.com/watch?v=example123 \
  -q 1080p \
  -f mp4 \
  -o ~/Videos/Tutorials
```

### Example 2: Download for Mobile Device

Download a smaller file for transfer to a phone:

```bash
ytdl download https://www.youtube.com/watch?v=example123 \
  -q 480p \
  -f mp4 \
  -o ~/Videos/Mobile
```

### Example 3: Archive in Highest Quality

Archive a video in the best possible quality:

```bash
ytdl download https://www.youtube.com/watch?v=example123 \
  -q 4k \
  -f mkv \
  -o ~/Videos/Archive
```

### Example 4: Quick Low-Quality Preview

Download a quick preview to check content:

```bash
ytdl download https://www.youtube.com/watch?v=example123 \
  -q 360p \
  -f mp4 \
  -o ~/Videos/Previews
```

### Example 5: Batch Download Script

Create a simple bash script for downloading multiple videos:

```bash
#!/bin/bash
# download-videos.sh

URLS=(
  "https://www.youtube.com/watch?v=example1"
  "https://www.youtube.com/watch?v=example2"
  "https://www.youtube.com/watch?v=example3"
)

for url in "${URLS[@]}"; do
  ytdl download "$url" -q 720p -f mp4 -o ~/Videos/Batch
done
```

Make it executable and run:

```bash
chmod +x download-videos.sh
./download-videos.sh
```

## Troubleshooting

### Video Quality Not Available

If you request a quality that isn't available:

```
Warning: 4K quality not available for this video
Downloading in best available quality: 1080p
```

Solution: Use `ytdl info URL` to see available qualities before downloading.

### Insufficient Disk Space

If you run out of space during download:

```
Error: Insufficient disk space
Required: 1.2 GB, Available: 800 MB
```

Solutions:
- Free up disk space
- Choose a lower quality (`-q 720p` instead of `-q 4k`)
- Download to a different drive with more space

### Slow Download Speeds

If downloads are slow:

1. **Check your internet connection**
2. **Try a different time of day** (YouTube may throttle during peak hours)
3. **Lower the quality** to reduce file size
4. **Check rate limiting** in your config (see [Configuration Guide](./configuration.md))

### File Already Exists

If a file with the same name exists:

```
Warning: File already exists: Me at the zoo.mp4
Overwrite? (y/n):
```

Options:
- Type `y` to overwrite
- Type `n` to cancel
- Move the existing file to a different location

## Best Practices

1. **Start with `info` command** - Check available qualities before downloading
2. **Choose appropriate quality** - Don't download 4K if you'll watch on a phone
3. **Use sensible output directories** - Keep downloads organized
4. **Test before batch operations** - Download one video to verify settings
5. **Consider storage space** - 4K videos can be several gigabytes
6. **Use MP4 for compatibility** - Unless you specifically need MKV/WebM features

## Next Steps

- **[Extracting Audio](./extracting-audio.md)** - Learn how to extract audio from videos
- **[Playlists](./playlists.md)** - Download multiple videos at once
- **[Configuration](./configuration.md)** - Set default quality and format preferences

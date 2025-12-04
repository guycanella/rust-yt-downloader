# FFmpeg Integration

This guide provides comprehensive coverage of FFmpeg integration in `ytdl`, including installation, configuration, advanced options, and troubleshooting.

## Overview

FFmpeg is the industry-standard multimedia framework that `ytdl` uses for:
- **Format conversion** - Converting between video/audio formats
- **Stream merging** - Combining separate video and audio streams
- **Codec transcoding** - Re-encoding to different codecs
- **Metadata embedding** - Adding thumbnails, titles, and tags
- **Stream processing** - Trimming, filtering, and optimization

## Installation

### Verifying FFmpeg

Check if FFmpeg is already installed:

```bash
# Check version
ffmpeg -version

# Expected output:
# ffmpeg version 6.0 Copyright (c) 2000-2023 the FFmpeg developers
# ...
```

### Installation by Platform

#### macOS

**Using Homebrew** (recommended):
```bash
brew install ffmpeg
```

**With additional codecs**:
```bash
brew install ffmpeg --with-fdk-aac --with-opus --with-libvpx
```

#### Linux

**Ubuntu/Debian**:
```bash
sudo apt update
sudo apt install ffmpeg
```

**Fedora/RHEL**:
```bash
sudo dnf install ffmpeg
```

**Arch Linux**:
```bash
sudo pacman -S ffmpeg
```

**From source** (for latest features):
```bash
git clone https://git.ffmpeg.org/ffmpeg.git
cd ffmpeg
./configure --enable-gpl --enable-libx264 --enable-libx265 --enable-libvpx
make
sudo make install
```

#### Windows

**Using Chocolatey**:
```powershell
choco install ffmpeg
```

**Using Scoop**:
```powershell
scoop install ffmpeg
```

**Manual installation**:
1. Download from [ffmpeg.org/download.html](https://ffmpeg.org/download.html)
2. Extract to `C:\ffmpeg`
3. Add `C:\ffmpeg\bin` to system PATH
4. Verify: `ffmpeg -version`

### Verifying Installation

After installation, verify FFmpeg is in PATH:

```bash
# Should show path to ffmpeg binary
which ffmpeg    # Unix/macOS
where ffmpeg    # Windows

# Test functionality
ffmpeg -codecs | grep h264
ffmpeg -codecs | grep vp9
```

## FFmpeg Configuration

### Global Settings

Configure FFmpeg behavior in `ytdl`:

```toml
[ffmpeg]
# Path to ffmpeg binary (auto-detected if in PATH)
binary_path = "/usr/local/bin/ffmpeg"

# Default encoding preset
preset = "medium"  # ultrafast, fast, medium, slow, veryslow

# Hardware acceleration
hwaccel = "auto"   # auto, none, cuda, qsv, vaapi

# Thread count (0 = auto)
threads = 0

# Enable verbose FFmpeg output
verbose = false
```

### Configuration Commands

```bash
# Set FFmpeg path manually
ytdl config set ffmpeg.binary_path /usr/local/bin/ffmpeg

# Set encoding preset
ytdl config set ffmpeg.preset slow

# Enable hardware acceleration
ytdl config set ffmpeg.hwaccel cuda

# View current settings
ytdl config show | grep ffmpeg
```

## Video Processing

### Stream Merging

YouTube often provides video and audio as separate streams. FFmpeg merges them:

```bash
# Automatic merging (default)
ytdl download URL

# FFmpeg command (internal):
# ffmpeg -i video.mp4 -i audio.m4a -c copy output.mp4
```

**Merging process**:
1. Download video stream
2. Download audio stream
3. Merge using FFmpeg (copy codecs, no re-encoding)
4. Delete temporary files

### Container Conversion (Remuxing)

Change container without re-encoding:

```bash
# WebM to MP4 (if codecs compatible)
ytdl download URL -f mp4

# FFmpeg command (internal):
# ffmpeg -i input.webm -c copy output.mp4
```

**Supported remuxing**:
- WebM (VP9/Opus) → MKV
- MP4 (H.264/AAC) → MKV
- Any compatible codec → appropriate container

### Codec Transcoding

Re-encode to different codec:

```bash
# VP9 to H.264 (for MP4 compatibility)
ytdl download URL -f mp4 -q 1080p

# FFmpeg command (internal):
# ffmpeg -i input.webm -c:v libx264 -preset medium -crf 23 \
#        -c:a aac -b:a 192k output.mp4
```

**Transcoding parameters**:
- **CRF** (Constant Rate Factor): 18-28 (lower = better quality)
- **Preset**: Speed vs. compression trade-off
- **Bitrate**: Target output bitrate

## Audio Processing

### Audio Extraction

Extract audio stream:

```bash
# Extract as MP3
ytdl audio URL -f mp3

# FFmpeg command (internal):
# ffmpeg -i input.mp4 -vn -c:a libmp3lame -b:a 320k output.mp3
```

**Extraction methods**:
- **Copy** (if format matches): Near-instant
- **Transcode** (if conversion needed): Slower, configurable quality

### Format-Specific Processing

**MP3** (lossy):
```bash
ytdl audio URL -f mp3

# FFmpeg encoding:
# -c:a libmp3lame        # MP3 encoder
# -b:a 320k              # Bitrate from config
# -q:a 0                 # Variable bitrate quality (0 = best)
```

**FLAC** (lossless):
```bash
ytdl audio URL -f flac

# FFmpeg encoding:
# -c:a flac              # FLAC encoder
# -compression_level 8   # Maximum compression
```

**Opus** (modern):
```bash
ytdl audio URL -f opus

# FFmpeg encoding:
# -c:a libopus           # Opus encoder
# -b:a 256k              # Bitrate
# -vbr on                # Variable bitrate
```

**AAC/M4A** (high quality):
```bash
ytdl audio URL -f m4a

# FFmpeg encoding:
# -c:a aac               # Native AAC encoder
# -b:a 256k              # Bitrate
# -movflags +faststart   # Optimize for streaming
```

**WAV** (uncompressed):
```bash
ytdl audio URL -f wav

# FFmpeg encoding:
# -c:a pcm_s16le         # PCM 16-bit little-endian
# -ar 48000              # Sample rate
```

## Advanced Features

### Metadata Embedding

Embed video metadata:

```bash
# Automatic metadata embedding
ytdl download URL

# FFmpeg metadata:
# -metadata title="Video Title"
# -metadata artist="Channel Name"
# -metadata date="2024"
# -metadata comment="YouTube Video ID: abc123"
```

**Embedded metadata**:
- Title
- Artist (channel name)
- Album (playlist name, if applicable)
- Date (upload date)
- Comment (video URL)
- Description

### Thumbnail Embedding

Embed video thumbnail:

```toml
[video]
include_thumbnail = true  # Download thumbnail
embed_thumbnail = true    # Embed in video file
```

**FFmpeg thumbnail embedding**:
```bash
# MP4
ffmpeg -i video.mp4 -i thumbnail.jpg \
  -map 0 -map 1 -c copy -disposition:v:1 attached_pic \
  output.mp4

# MKV
ffmpeg -i video.mkv -attach thumbnail.jpg \
  -metadata:s:t mimetype=image/jpeg \
  output.mkv
```

### Subtitle Embedding

Embed subtitles:

```toml
[video]
include_subtitles = true  # Download subtitles
embed_subtitles = true    # Embed in video
```

**FFmpeg subtitle embedding**:
```bash
# MP4 (requires mov_text)
ffmpeg -i video.mp4 -i subtitles.vtt \
  -c copy -c:s mov_text \
  output.mp4

# MKV (supports SRT, ASS, VTT)
ffmpeg -i video.mkv -i subtitles.srt \
  -c copy -c:s srt \
  output.mkv
```

### Chapter Markers

Add chapter markers (for long videos):

```bash
# FFmpeg chapters from metadata file
ffmpeg -i input.mp4 -i chapters.txt \
  -map_metadata 1 -c copy output.mp4
```

**Chapter file format** (`chapters.txt`):
```ini
;FFMETADATA1
[CHAPTER]
TIMEBASE=1/1000
START=0
END=180000
title=Introduction

[CHAPTER]
TIMEBASE=1/1000
START=180000
END=600000
title=Main Content
```

## Hardware Acceleration

### Supported Acceleration

- **NVIDIA CUDA** (NVENC/NVDEC)
- **Intel Quick Sync Video** (QSV)
- **AMD AMF**
- **Apple VideoToolbox** (macOS)
- **VAAPI** (Linux)

### Enabling Hardware Acceleration

```toml
[ffmpeg]
hwaccel = "cuda"  # or qsv, vaapi, videotoolbox
```

**NVIDIA CUDA example**:
```bash
# H.264 encoding with NVENC
ffmpeg -hwaccel cuda -i input.mp4 \
  -c:v h264_nvenc -preset fast \
  output.mp4
```

**Intel QSV example**:
```bash
# H.264 encoding with Quick Sync
ffmpeg -hwaccel qsv -c:v h264_qsv -i input.mp4 \
  -c:v h264_qsv -preset medium \
  output.mp4
```

### Performance Benefits

| Operation | CPU Only | With HW Accel | Speedup |
|-----------|----------|---------------|---------|
| H.264 encode (1080p) | 15 min | 2 min | 7.5x |
| H.265 encode (1080p) | 30 min | 4 min | 7.5x |
| VP9 encode (1080p) | 45 min | N/A* | - |

*VP9 hardware encoding support is limited

## Performance Optimization

### Encoding Presets

Control speed vs. compression trade-off:

```toml
[ffmpeg]
preset = "medium"  # ultrafast, superfast, veryfast, faster, fast,
                   # medium, slow, slower, veryslow, placebo
```

**Preset comparison** (H.264, 1080p, 10 min video):

| Preset | Encoding Time | File Size | Quality |
|--------|---------------|-----------|---------|
| ultrafast | 1 min | 250 MB | Good |
| fast | 3 min | 180 MB | Good |
| medium | 6 min | 150 MB | Very Good |
| slow | 12 min | 135 MB | Excellent |
| veryslow | 25 min | 125 MB | Excellent |

**Recommendation**: `medium` for general use, `slow` for archival

### Thread Configuration

Optimize CPU usage:

```toml
[ffmpeg]
threads = 0  # Auto-detect (recommended)
# threads = 4  # Manual setting
```

**Thread scaling**:
- 0 = Auto (uses all available cores)
- 1 = Single-threaded (slowest)
- N = Use N threads

### Two-Pass Encoding

Better quality at target bitrate (slower):

```bash
# Internal FFmpeg two-pass VP9
# Pass 1: Analysis
ffmpeg -i input.mp4 -c:v libvpx-vp9 -b:v 2M -pass 1 -f null /dev/null

# Pass 2: Encoding
ffmpeg -i input.mp4 -c:v libvpx-vp9 -b:v 2M -pass 2 output.webm
```

## Troubleshooting

### FFmpeg Not Found

**Error**: `FFmpeg binary not found in PATH`

**Diagnosis**:
```bash
# Check if installed
ffmpeg -version

# Check PATH
echo $PATH | grep ffmpeg
```

**Solutions**:
```bash
# Add to PATH (Unix/macOS)
export PATH=$PATH:/usr/local/bin

# Or set manually in config
ytdl config set ffmpeg.binary_path /usr/local/bin/ffmpeg

# Verify
ytdl download URL -v  # Should show FFmpeg path
```

### Codec Not Supported

**Error**: `Encoder 'libx264' not found`

**Diagnosis**:
```bash
# Check available encoders
ffmpeg -encoders | grep 264
```

**Solution**:
```bash
# Reinstall FFmpeg with additional codecs
brew reinstall ffmpeg --with-fdk-aac --with-libvpx  # macOS

# Or compile from source with --enable-libx264
```

### Slow Conversion

**Problem**: Format conversion takes too long

**Solutions**:

1. **Use faster preset**:
   ```bash
   ytdl config set ffmpeg.preset fast
   ```

2. **Enable hardware acceleration**:
   ```bash
   ytdl config set ffmpeg.hwaccel cuda
   ```

3. **Avoid transcoding** (use remuxing):
   ```bash
   # Check source format first
   ytdl info URL

   # Request matching format
   ytdl download URL -f mkv  # If source is VP9
   ```

4. **Reduce quality**:
   ```bash
   ytdl download URL -q 720p  # Instead of 1080p
   ```

### Quality Loss

**Problem**: Output quality lower than expected

**Diagnosis**:
```bash
# Check FFmpeg encoding parameters
ytdl download URL -v  # Shows FFmpeg command
```

**Solutions**:

1. **Increase CRF quality**:
   ```toml
   [ffmpeg]
   crf = 18  # Lower = better (default: 23)
   ```

2. **Use slower preset**:
   ```toml
   [ffmpeg]
   preset = "slow"
   ```

3. **Increase bitrate**:
   ```toml
   [audio]
   bitrate = "320k"  # For audio
   ```

### Audio/Video Desync

**Problem**: Audio and video out of sync

**Causes**:
- Variable frame rate (VFR) video
- Timestamp issues during merging

**Solution**:
```bash
# Force constant frame rate
ytdl download URL --ffmpeg-args "-vsync cfr"
```

### Out of Memory

**Problem**: FFmpeg crashes with large files

**Solutions**:

1. **Reduce thread count**:
   ```toml
   [ffmpeg]
   threads = 2
   ```

2. **Use streaming mode**:
   ```toml
   [ffmpeg]
   stream_copy = true
   ```

3. **Download in parts** (for very long videos)

## Command-Line Override

### Custom FFmpeg Arguments

Pass custom arguments to FFmpeg:

```bash
# Custom CRF
ytdl download URL --ffmpeg-args "-crf 20"

# Custom preset and tune
ytdl download URL --ffmpeg-args "-preset slow -tune film"

# Multiple arguments
ytdl download URL --ffmpeg-args "-crf 18 -preset veryslow -profile:v high"
```

### Common FFmpeg Arguments

**Video encoding**:
```bash
-crf 18                    # Quality (18 = visually lossless)
-preset slow               # Speed vs. compression
-tune film                 # Optimize for film content
-profile:v high            # H.264 profile
-level 4.1                 # H.264 level
```

**Audio encoding**:
```bash
-b:a 320k                  # Bitrate
-q:a 0                     # VBR quality (0 = best)
-ar 48000                  # Sample rate (48 kHz)
-ac 2                      # Channels (stereo)
```

**Filtering**:
```bash
-vf scale=1920:1080        # Resize
-af volume=2.0             # Increase volume
-ss 00:01:00 -t 00:02:00   # Trim (start + duration)
```

## Best Practices

1. **Keep FFmpeg updated**: New versions have better codecs and performance
2. **Use hardware acceleration**: Dramatically faster encoding
3. **Choose appropriate presets**: Balance speed and quality
4. **Avoid multiple transcoding**: Each generation loses quality
5. **Use lossless for intermediate files**: Preserve quality
6. **Verify output**: Check file integrity after conversion
7. **Monitor resource usage**: Adjust threads for system performance

## Related Documentation

- [Format Conversion](./format-conversion.md) - Format conversion guide
- [Quality Selection](./quality-selection.md) - Video quality options
- [Supported Formats](../reference/formats.md) - Complete format reference

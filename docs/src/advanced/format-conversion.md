# Format Conversion

This guide covers format conversion capabilities powered by FFmpeg, including container remuxing, codec transcoding, audio extraction, and optimization techniques.

## Overview

`ytdl` uses FFmpeg to convert between video and audio formats. Understanding the difference between **remuxing** (container changes) and **transcoding** (codec changes) is crucial for efficient format conversion.

## Container Formats

### Supported Video Containers

| Format | Extension | Codecs Supported | Use Case |
|--------|-----------|-----------------|----------|
| MP4 | `.mp4` | H.264, AAC | Maximum compatibility |
| MKV | `.mkv` | H.264, VP9, AV1, FLAC, Opus | Feature-rich, archival |
| WebM | `.webm` | VP8, VP9, Vorbis, Opus | Web playback, open format |

### Format Selection

```bash
# Download as MP4 (default)
ytdl download URL -f mp4

# Download as MKV
ytdl download URL -f mkv

# Download as WebM
ytdl download URL -f webm
```

### Container Capabilities

**MP4**:
- ✅ Widely compatible (all devices/players)
- ✅ Hardware decoding support
- ✅ Fast seeking
- ❌ Limited codec support
- ❌ No FLAC or Opus audio

**MKV**:
- ✅ Supports all codecs
- ✅ Multiple audio/subtitle tracks
- ✅ Chapter markers
- ✅ Metadata embedding
- ❌ Less compatible (some devices)

**WebM**:
- ✅ Open format
- ✅ Optimized for web
- ✅ Good compression
- ❌ Limited device support
- ❌ Primarily VP8/VP9 codecs

## Audio Formats

### Supported Audio Formats

| Format | Type | Quality | File Size | Compatibility |
|--------|------|---------|-----------|---------------|
| MP3 | Lossy | Good | Medium | Universal |
| M4A/AAC | Lossy | Better | Medium | Very High |
| FLAC | Lossless | Perfect | Large | High |
| WAV | Uncompressed | Perfect | Largest | Universal |
| Opus | Lossy | Best | Small | Modern devices |

### Audio Extraction Examples

```bash
# Extract as MP3 (default, 320k bitrate)
ytdl audio URL

# Extract as FLAC (lossless)
ytdl audio URL -f flac

# Extract as Opus (modern, efficient)
ytdl audio URL -f opus

# Extract as M4A (AAC)
ytdl audio URL -f m4a

# Extract as WAV (uncompressed)
ytdl audio URL -f wav
```

## Remuxing vs. Transcoding

### Remuxing (Fast)

**Definition**: Changing the container without re-encoding the video/audio streams.

**Characteristics**:
- ⚡ Very fast (near-instant)
- 🎯 No quality loss
- 💾 Same file size
- ✅ Lossless operation

**Example scenario**:
```bash
# Source: H.264/AAC in WebM container
# Request: MP4 container
ytdl download URL -f mp4
# Result: H.264/AAC remuxed to MP4 (fast, no quality loss)
```

### Transcoding (Slow)

**Definition**: Re-encoding video/audio to different codecs.

**Characteristics**:
- 🐌 Slow (CPU-intensive)
- 📉 Potential quality loss
- 💾 File size varies
- ⚠️ Lossy operation

**Example scenario**:
```bash
# Source: VP9/Opus
# Request: MP4 (requires H.264/AAC)
ytdl download URL -f mp4
# Result: Transcoded to H.264/AAC (slow, quality trade-off)
```

## Audio Conversion Details

### Bitrate Configuration

Configure default audio bitrate in the config file:

```toml
[audio]
format = "mp3"
bitrate = "320k"  # 320 kbps
```

**Common bitrates**:
- `128k` - Low quality, small files
- `192k` - Acceptable quality
- `256k` - Good quality
- `320k` - High quality (recommended)

### Lossless vs. Lossy

**Lossy formats** (MP3, M4A, Opus):
- Use compression algorithms that discard data
- Smaller file sizes
- Some quality loss (minimal at high bitrates)
- Configurable bitrate

**Lossless formats** (FLAC, WAV):
- Perfect audio reproduction
- Larger file sizes
- No quality loss
- Bitrate not applicable (FLAC) or fixed (WAV)

### Format-Specific Notes

**MP3**:
```bash
ytdl audio URL -f mp3
# Default: 320k bitrate
# Compatibility: Universal
# File size: ~2.5 MB per minute
```

**FLAC**:
```bash
ytdl audio URL -f flac
# Lossless compression
# File size: ~4-5 MB per minute
# Best for archival
```

**Opus**:
```bash
ytdl audio URL -f opus
# Modern codec, efficient
# Better quality than MP3 at same bitrate
# File size: ~2 MB per minute at 256k
```

**M4A/AAC**:
```bash
ytdl audio URL -f m4a
# Better than MP3 at equivalent bitrates
# Apple device friendly
# File size: ~2.5 MB per minute
```

**WAV**:
```bash
ytdl audio URL -f wav
# Uncompressed PCM
# File size: ~10 MB per minute
# Editing-friendly
```

## FFmpeg Integration

### FFmpeg Requirements

`ytdl` requires FFmpeg to be installed and available in your system PATH:

**Check FFmpeg installation**:
```bash
ffmpeg -version
```

**Install FFmpeg**:
```bash
# macOS
brew install ffmpeg

# Ubuntu/Debian
sudo apt-get install ffmpeg

# Fedora
sudo dnf install ffmpeg

# Windows (using Chocolatey)
choco install ffmpeg

# Windows (manual)
# Download from https://ffmpeg.org/download.html
```

### FFmpeg Codec Support

Verify FFmpeg has necessary codecs:

```bash
# List all supported codecs
ffmpeg -codecs

# Check specific codec
ffmpeg -codecs | grep h264
ffmpeg -codecs | grep vp9
ffmpeg -codecs | grep opus
```

### Conversion Quality Settings

FFmpeg uses different quality settings per codec:

**H.264 (Video)**:
- CRF: 18-23 (18 = visually lossless, 23 = good quality)
- Preset: slow, medium, fast (speed vs. compression trade-off)

**VP9 (Video)**:
- CRF: 15-35 (lower = better quality)
- Two-pass encoding for optimal compression

**Opus (Audio)**:
- VBR: 128-320 kbps
- Optimized for voice and music

## Conversion Performance

### Optimization Tips

1. **Use remuxing when possible**:
   ```bash
   # If source is H.264, MP4 remux is instant
   ytdl download URL -f mp4
   ```

2. **Avoid unnecessary conversions**:
   ```bash
   # Check source format first
   ytdl info URL

   # Request matching format
   ytdl download URL -f mkv  # If source is VP9
   ```

3. **Batch conversions**:
   ```bash
   # Convert entire playlist
   ytdl playlist URL --audio-only --audio-format flac
   ```

4. **Set permanent defaults**:
   ```bash
   ytdl config set audio.format mp3
   ytdl config set audio.bitrate 256k
   ytdl config set video.format mp4
   ```

### Performance Benchmarks

Approximate conversion times for 10-minute 1080p video:

| Operation | Time | CPU Usage |
|-----------|------|-----------|
| Remux (container change) | 5-10 seconds | Low |
| H.264 → H.265 transcode | 5-15 minutes | Very High |
| VP9 → H.264 transcode | 10-20 minutes | Very High |
| Audio extraction (remux) | 5-10 seconds | Low |
| Audio extraction (transcode) | 1-2 minutes | Medium |

*Note: Times vary based on CPU performance and FFmpeg version.*

## Advanced Use Cases

### High-Quality Archival

For preserving content with maximum quality:

```bash
# Video: MKV with VP9, highest quality
ytdl download URL -q best -f mkv

# Audio: FLAC for lossless
ytdl audio URL -f flac
```

### Compatibility-First

For playback on all devices:

```bash
# Video: MP4 with H.264 (universal)
ytdl download URL -q 1080p -f mp4

# Audio: MP3 at high bitrate
ytdl audio URL -f mp3
```

### Size-Optimized

Minimize file sizes while maintaining acceptable quality:

```bash
# Video: WebM with VP9 (efficient compression)
ytdl download URL -q 720p -f webm

# Audio: Opus (best compression)
ytdl audio URL -f opus
```

### Editing Workflow

Prepare files for video editing:

```bash
# Video: MKV (all features)
ytdl download URL -f mkv

# Audio: WAV (uncompressed, no artifacts)
ytdl audio URL -f wav
```

## Configuration Examples

### Config File Setup

```toml
[general]
output_dir = "~/Videos/YouTube"
default_quality = "1080p"

[audio]
format = "mp3"      # or "flac" for lossless
bitrate = "320k"    # ignored for lossless

[video]
format = "mp4"      # mp4, mkv, or webm
include_thumbnail = true
include_subtitles = true
```

### Profile-Based Workflows

Create different configurations for different needs:

**Standard quality** (default):
```bash
ytdl config set video.format mp4
ytdl config set audio.format mp3
ytdl config set general.default_quality 1080p
```

**Archival quality**:
```bash
ytdl config set video.format mkv
ytdl config set audio.format flac
ytdl config set general.default_quality best
```

**Mobile-optimized**:
```bash
ytdl config set video.format mp4
ytdl config set audio.format m4a
ytdl config set general.default_quality 720p
```

## Troubleshooting

### FFmpeg Not Found

**Error**: `FFmpeg not found in PATH`

**Solution**:
```bash
# Verify installation
which ffmpeg  # Unix/macOS
where ffmpeg  # Windows

# Install if missing (see FFmpeg Requirements above)

# Add to PATH if installed but not found
export PATH=$PATH:/path/to/ffmpeg/bin  # Unix/macOS
```

### Codec Not Supported

**Error**: `Codec not supported by container`

**Solution**:
```bash
# Check source codecs
ytdl info URL

# Use compatible container
# VP9/Opus → use MKV or WebM
# H.264/AAC → use MP4 or MKV
```

### Slow Conversion

**Problem**: Format conversion takes too long

**Solutions**:
1. Use remuxing (same codec, different container)
2. Request format matching source
3. Disable unnecessary processing
4. Use hardware acceleration (if available)

### Quality Loss

**Problem**: Converted file has lower quality

**Cause**: Transcoding between lossy formats compounds quality loss

**Prevention**:
```bash
# Always start from highest quality
ytdl download URL -q best -f mkv

# Use lossless for audio
ytdl audio URL -f flac

# Avoid repeated transcoding
```

## Related Documentation

- [Quality Selection](./quality-selection.md) - Choosing video quality
- [FFmpeg Integration](./ffmpeg.md) - Detailed FFmpeg usage
- [Supported Formats Reference](../reference/formats.md) - Complete format list

# Quality Selection

This guide provides in-depth coverage of video quality selection strategies, including resolution preferences, codec considerations, and quality negotiation when your preferred option isn't available.

## Overview

YouTube provides videos in multiple qualities and formats. The `ytdl` tool allows you to select your preferred quality with intelligent fallback behavior when exact matches aren't available.

## Quality Options

### Predefined Quality Levels

The following quality presets are available via the `-q/--quality` flag:

| Quality | Resolution | Typical Use Case |
|---------|-----------|------------------|
| `144p` | 256×144 | Minimal bandwidth, testing |
| `240p` | 426×240 | Very low bandwidth scenarios |
| `360p` | 640×360 | Standard definition, mobile |
| `480p` | 854×480 | Standard definition |
| `720p` | 1280×720 | HD, balanced quality/size |
| `1080p` | 1920×1080 | Full HD, recommended default |
| `1440p` | 2560×1440 | 2K, high quality |
| `4k` | 3840×2160 | Ultra HD, maximum quality |
| `best` | Highest available | Automatic selection (default) |
| `worst` | Lowest available | Minimum file size |

### Examples

```bash
# Download best available quality (default)
ytdl download https://youtube.com/watch?v=abc123

# Explicitly request best quality
ytdl download https://youtube.com/watch?v=abc123 -q best

# Request specific resolution
ytdl download https://youtube.com/watch?v=abc123 -q 1080p

# Download lowest quality (smallest file)
ytdl download https://youtube.com/watch?v=abc123 -q worst

# 4K video for maximum quality
ytdl download https://youtube.com/watch?v=abc123 -q 4k
```

## Quality Negotiation

### Fallback Behavior

When your requested quality isn't available, `ytdl` employs intelligent fallback logic:

1. **Exact match**: If the requested quality exists, it's selected
2. **Closest lower**: If unavailable, the next lower quality is chosen
3. **Closest higher**: If no lower quality exists, the next higher quality is used
4. **Best available**: As last resort, the best available quality is selected

**Example scenario**:
```bash
# Request 1440p but video only has: 144p, 360p, 720p, 1080p
ytdl download URL -q 1440p
# Result: Downloads 1080p (closest lower quality)
```

### Inspecting Available Qualities

Before downloading, use the `info` command to see all available qualities:

```bash
ytdl info https://youtube.com/watch?v=abc123
```

**Sample output**:
```
Title: Example Video
Duration: 10:30
Channel: Example Channel

Available Qualities:
  - 4k (3840×2160) - VP9/Opus - 500 MB
  - 1440p (2560×1440) - VP9/Opus - 280 MB
  - 1080p (1920×1080) - H.264/AAC - 150 MB
  - 720p (1280×720) - H.264/AAC - 80 MB
  - 480p (854×480) - H.264/AAC - 45 MB
  - 360p (640×360) - H.264/AAC - 25 MB
```

## Codec Considerations

### Video Codecs

YouTube typically provides videos in multiple codec formats:

- **H.264/AVC** (`.mp4`): Maximum compatibility, hardware decoding support
- **VP9** (`.webm`): Better compression, higher quality per bitrate
- **AV1** (`.webm`): Newest codec, best compression (limited availability)

### Codec Selection Strategy

The tool automatically selects the best codec for your requested format:

| Format | Preferred Codec | Fallback |
|--------|----------------|----------|
| `mp4` | H.264 | VP9 → transcode |
| `mkv` | VP9 | H.264, AV1 |
| `webm` | VP9 | AV1, H.264 → transcode |

**Performance note**: Transcoding (converting between codecs) is CPU-intensive and may take longer than direct downloads.

## Configuration Defaults

Set your preferred quality in the config file to avoid repeating the flag:

```toml
[general]
default_quality = "1080p"
```

**Configuration file location**:
```bash
ytdl config path
```

**Set via CLI**:
```bash
ytdl config set general.default_quality 1080p
```

**Verify setting**:
```bash
ytdl config get general.default_quality
```

## Advanced Scenarios

### Bandwidth-Constrained Downloads

For limited bandwidth, use lower qualities to reduce download time:

```bash
# Mobile data - ~25 MB file
ytdl download URL -q 360p

# Slow connection - ~45 MB file
ytdl download URL -q 480p
```

### Storage-Optimized Downloads

Balance quality and disk space:

```bash
# Good quality, moderate size (~80 MB)
ytdl download URL -q 720p

# Archive quality (~150 MB)
ytdl download URL -q 1080p
```

### Maximum Quality Archival

For archival purposes, always use best quality:

```bash
# Auto-select highest
ytdl download URL -q best

# Explicitly request 4K
ytdl download URL -q 4k -f mkv
```

### Playlist Quality Settings

Apply quality settings to entire playlists:

```bash
# All videos at 720p
ytdl playlist URL -q 720p

# Best available for each video
ytdl playlist URL -q best
```

## Quality vs. File Size

Approximate file sizes for a 10-minute video:

| Quality | File Size | Bitrate | Data Rate |
|---------|-----------|---------|-----------|
| 144p | ~10 MB | 128 kbps | Low |
| 240p | ~20 MB | 256 kbps | Low |
| 360p | ~40 MB | 512 kbps | Medium |
| 480p | ~70 MB | 1 Mbps | Medium |
| 720p | ~120 MB | 2.5 Mbps | High |
| 1080p | ~250 MB | 4-5 Mbps | High |
| 1440p | ~450 MB | 8-10 Mbps | Very High |
| 4K | ~800 MB | 15-20 Mbps | Ultra High |

*Note: Actual sizes vary based on content complexity, codec, and YouTube's encoding.*

## Quality Selection Best Practices

### Recommended by Use Case

1. **Streaming re-upload**: `720p` or `1080p` (good balance)
2. **Archive/preservation**: `best` or `4k` (maximum quality)
3. **Mobile viewing**: `360p` or `480p` (compatible, smaller)
4. **Offline playlist**: `720p` (storage-efficient HD)
5. **Slow connection**: `360p` or `worst` (fastest download)

### Performance Tips

- **Use `info` first**: Check available qualities before downloading
- **Set config defaults**: Avoid repeating `-q` flag for common workflows
- **Consider format**: MKV container supports more codecs than MP4
- **Monitor bandwidth**: Use lower qualities on metered connections
- **Batch downloads**: Use playlists with consistent quality settings

## Troubleshooting

### Requested Quality Unavailable

**Problem**: Video doesn't have your requested quality

**Solution**:
```bash
# Check what's available
ytdl info URL

# Use fallback logic with closest match
ytdl download URL -q 1080p  # Will select best available if 1080p missing

# Or use 'best' to always get highest
ytdl download URL -q best
```

### Quality Lower Than Expected

**Problem**: Download quality is lower than requested

**Possible causes**:
1. Video was uploaded in lower quality (check original source)
2. YouTube hasn't finished processing higher qualities (wait 30-60 minutes)
3. Regional restrictions limit available qualities

**Verification**:
```bash
ytdl info URL  # Shows all available qualities
```

### Large File Sizes

**Problem**: 4K/1440p downloads are too large

**Solutions**:
```bash
# Use 1080p for good balance
ytdl download URL -q 1080p

# Set permanent default
ytdl config set general.default_quality 720p
```

## Related Documentation

- [Format Conversion](./format-conversion.md) - Converting between formats
- [CLI Commands Reference](../reference/cli-commands.md) - Full command documentation
- [Configuration Options](../reference/config-options.md) - All config settings

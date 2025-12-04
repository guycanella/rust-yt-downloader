# Quick Start Guide

This guide will get you up and running with `ytdl` in just a few minutes. We'll cover the most common use cases to help you start downloading YouTube content right away.

## Your First Download

The simplest way to download a YouTube video is with the `download` command:

```bash
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw
```

This downloads the video in the best available quality to your current directory. The file will be saved with the video's title as the filename.

### Example Output

```
Downloading: "Me at the zoo"
Quality: 1080p (Best available)
Format: MP4
[████████████████████████████████] 100% - 5.2 MB/s
✓ Download complete: Me at the zoo.mp4
```

## Extracting Audio Only

Want just the audio from a video? Use the `audio` command:

```bash
ytdl audio https://www.youtube.com/watch?v=jNQXAC9IVRw
```

This extracts the audio and saves it as an MP3 file (320kbps by default) in your current directory.

### Choosing a Different Audio Format

You can specify different audio formats like FLAC for lossless quality:

```bash
ytdl audio https://www.youtube.com/watch?v=jNQXAC9IVRw -f flac
```

Available formats:
- `mp3` - Most compatible (default)
- `flac` - Lossless quality, larger files
- `m4a` - Good quality, smaller than FLAC
- `wav` - Uncompressed, largest files
- `opus` - Modern codec, excellent quality

## Downloading a Playlist

To download an entire playlist, use the `playlist` command:

```bash
ytdl playlist https://www.youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf
```

This downloads all videos from the playlist to your current directory.

### Audio-Only from Playlists

Extract audio from all videos in a playlist:

```bash
ytdl playlist https://www.youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf --audio-only
```

You can also specify the audio format:

```bash
ytdl playlist https://www.youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf --audio-only -f mp3
```

## Viewing Video Information

Before downloading, you might want to see what's available. Use the `info` command:

```bash
ytdl info https://www.youtube.com/watch?v=jNQXAC9IVRw
```

This displays detailed information about the video without downloading anything:

```
Title: Me at the zoo
Duration: 0:19
Uploader: jawed
Views: 280,000,000

Available Qualities:
  ✓ 4K (2160p)
  ✓ 1440p
  ✓ 1080p (Full HD)
  ✓ 720p (HD)
  ✓ 480p
  ✓ 360p
  ✓ 240p
  ✓ 144p

Available Formats:
  Video: MP4, WebM, MKV
  Audio: MP3, FLAC, M4A, WAV, Opus

File Size (estimate):
  Best Quality: ~45 MB
  Audio Only (MP3): ~1.2 MB
```

## Common Options

Here are some frequently used options that work with most commands:

### Specify Output Directory

Save downloads to a specific folder:

```bash
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw -o ~/Videos/YouTube
```

### Choose Video Quality

Download in a specific quality:

```bash
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw -q 720p
```

Available qualities: `144p`, `240p`, `360p`, `480p`, `720p`, `1080p`, `1440p`, `4k`, `best`, `worst`

### Choose Video Format

Download in a specific container format:

```bash
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw -f mkv
```

Available video formats: `mp4`, `mkv`, `webm`

### Quiet Mode

Suppress progress bars and non-error output (useful for scripts):

```bash
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw --silent
```

## Putting It All Together

Here's a real-world example combining multiple options:

```bash
ytdl download https://www.youtube.com/watch?v=jNQXAC9IVRw \
  -q 1080p \
  -f mp4 \
  -o ~/Downloads/YouTube
```

This command:
- Downloads the video in 1080p quality
- Saves it as an MP4 file
- Stores it in `~/Downloads/YouTube/`

## Quick Reference Cheat Sheet

```bash
# Download video (best quality)
ytdl download URL

# Download video (specific quality)
ytdl download URL -q 720p

# Extract audio (MP3)
ytdl audio URL

# Extract audio (FLAC)
ytdl audio URL -f flac

# Download playlist
ytdl playlist URL

# Playlist as audio only
ytdl playlist URL --audio-only

# View video info
ytdl info URL

# Specify output directory
ytdl download URL -o /path/to/folder

# Silent mode (no progress bars)
ytdl download URL --silent
```

## What's Next?

Now that you know the basics, explore more advanced features:

- **[Downloading Videos](./downloading-videos.md)** - Complete guide to video downloads with all quality and format options
- **[Extracting Audio](./extracting-audio.md)** - Detailed audio extraction guide with bitrate control
- **[Playlists](./playlists.md)** - Advanced playlist downloading techniques
- **[Configuration](./configuration.md)** - Set default preferences to avoid typing the same options repeatedly

## Getting Help

View help for any command:

```bash
# General help
ytdl --help

# Command-specific help
ytdl download --help
ytdl audio --help
ytdl playlist --help
ytdl config --help
```

Each help screen shows all available options and examples for that specific command.

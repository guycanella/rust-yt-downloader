# Extracting Audio

This guide covers everything you need to know about extracting audio from YouTube videos with `ytdl`. Learn about different audio formats, quality settings, and how to get the best audio for your needs.

## Basic Audio Extraction

The simplest way to extract audio from a YouTube video:

```bash
ytdl audio https://www.youtube.com/watch?v=VIDEO_ID
```

This extracts the audio and saves it as an MP3 file (320kbps) to your current directory.

### What Gets Downloaded

When you use the `audio` command, `ytdl`:
1. Downloads the best available audio stream from YouTube
2. Converts it to your chosen format using FFmpeg
3. Saves the file with the video title as the filename
4. Embeds metadata (title, artist, etc.) when available

## Audio Format Selection

`ytdl` supports five audio formats, each with different characteristics. Use the `-f` or `--format` flag to choose.

### Available Audio Formats

#### MP3 (Default - Most Compatible)

**Format**: MPEG Audio Layer III (lossy compression)

**Characteristics:**
- **Compatibility**: Excellent (works on all devices and players)
- **Quality**: Very good (transparent at high bitrates)
- **File Size**: Medium (320kbps ≈ 2.5 MB/minute)
- **Best For**: General use, portable devices, maximum compatibility

```bash
ytdl audio https://www.youtube.com/watch?v=jNQXAC9IVRw -f mp3
```

**When to Use:**
- Default choice for most users
- When you need broad device compatibility
- For music players, phones, or car stereos
- When file size and quality balance is important

---

#### FLAC (Lossless - Best Quality)

**Format**: Free Lossless Audio Codec

**Characteristics:**
- **Compatibility**: Good (supported by most modern players)
- **Quality**: Perfect (bit-perfect reproduction)
- **File Size**: Large (≈ 10-30 MB/minute)
- **Best For**: Archival, audiophiles, when quality is paramount

```bash
ytdl audio https://www.youtube.com/watch?v=jNQXAC9IVRw -f flac
```

**When to Use:**
- Archiving music collections
- When you have plenty of storage space
- For critical listening on high-end audio equipment
- When you might convert to other formats later
- Music production or editing

---

#### M4A/AAC (Modern Lossy)

**Format**: MPEG-4 Audio / Advanced Audio Coding

**Characteristics:**
- **Compatibility**: Excellent (Apple devices, modern players)
- **Quality**: Excellent (better than MP3 at same bitrate)
- **File Size**: Small to medium (≈ 1.5-2 MB/minute)
- **Best For**: Apple ecosystem, modern devices, efficiency

```bash
ytdl audio https://www.youtube.com/watch?v=jNQXAC9IVRw -f m4a
```

**When to Use:**
- For iPhone, iPad, or iTunes
- When you want better quality than MP3 at smaller file sizes
- For modern streaming or cloud storage
- When quality-to-size ratio is critical

---

#### WAV (Uncompressed)

**Format**: Waveform Audio File Format

**Characteristics:**
- **Compatibility**: Excellent (universal support)
- **Quality**: Perfect (uncompressed)
- **File Size**: Very large (≈ 50 MB/minute)
- **Best For**: Professional audio work, editing

```bash
ytdl audio https://www.youtube.com/watch?v=jNQXAC9IVRw -f wav
```

**When to Use:**
- Professional audio editing or production
- When compatibility is absolutely critical
- When you need uncompressed audio
- Short clips where file size doesn't matter

> **Note:** WAV files are typically 5-10x larger than FLAC while providing the same quality. FLAC is usually preferred for archival.

---

#### Opus (Modern Efficient)

**Format**: Opus Interactive Audio Codec

**Characteristics:**
- **Compatibility**: Good (modern browsers, newer players)
- **Quality**: Excellent (best quality-to-size ratio)
- **File Size**: Very small (≈ 0.8-1.5 MB/minute)
- **Best For**: Web streaming, voice content, minimal file size

```bash
ytdl audio https://www.youtube.com/watch?v=jNQXAC9IVRw -f opus
```

**When to Use:**
- For web applications or streaming
- When bandwidth or storage is limited
- For podcasts or spoken word content
- When you need the smallest files with good quality

---

## Quality and Bitrate Selection

The bitrate determines the audio quality for lossy formats (MP3, M4A, Opus). Use the `-b` or `--bitrate` flag to specify.

### Common Bitrate Settings

| Bitrate | Quality Level | Use Case |
|---------|---------------|----------|
| `128k` | Standard | Podcasts, speech, acceptable music |
| `192k` | Good | Casual listening, most music |
| `256k` | Very Good | High-quality music |
| `320k` | Excellent (default) | Maximum MP3 quality |

### Bitrate Examples

**High-quality MP3 (320kbps - default):**
```bash
ytdl audio https://www.youtube.com/watch?v=jNQXAC9IVRw -f mp3 -b 320k
```

**Standard quality MP3 (192kbps):**
```bash
ytdl audio https://www.youtube.com/watch?v=jNQXAC9IVRw -f mp3 -b 192k
```

**Smaller file for speech/podcasts (128kbps):**
```bash
ytdl audio https://www.youtube.com/watch?v=jNQXAC9IVRw -f mp3 -b 128k
```

> **Note:** Bitrate selection only applies to lossy formats (MP3, M4A, Opus). FLAC and WAV are lossless/uncompressed and ignore this setting.

### Bitrate vs. File Size

For a 4-minute song:

- **128k MP3**: ~4 MB
- **192k MP3**: ~6 MB
- **256k MP3**: ~8 MB
- **320k MP3**: ~10 MB
- **FLAC**: ~25-40 MB
- **WAV**: ~40-50 MB

## Output Directory

Specify where to save audio files using `-o` or `--output`:

```bash
ytdl audio https://www.youtube.com/watch?v=jNQXAC9IVRw -o ~/Music
```

### Organizing Your Audio Collection

**By genre:**
```bash
ytdl audio URL -o ~/Music/Rock
ytdl audio URL -o ~/Music/Classical
ytdl audio URL -o ~/Music/Jazz
```

**By format:**
```bash
ytdl audio URL -f flac -o ~/Music/Lossless
ytdl audio URL -f mp3 -o ~/Music/MP3
```

**By purpose:**
```bash
ytdl audio URL -o ~/Music/Podcasts
ytdl audio URL -o ~/Music/Soundtracks
ytdl audio URL -o ~/Music/Audiobooks
```

## Real-World Use Cases

### Use Case 1: Music Collection (Balanced)

Download music in high-quality MP3 for everyday listening:

```bash
ytdl audio https://www.youtube.com/watch?v=musicvideo \
  -f mp3 \
  -b 320k \
  -o ~/Music/Library
```

**Why this works:**
- MP3 plays on all devices
- 320k is the highest MP3 quality (transparent)
- Reasonable file sizes (~10 MB per song)

### Use Case 2: Archival Quality

Archive music in lossless quality for future use:

```bash
ytdl audio https://www.youtube.com/watch?v=musicvideo \
  -f flac \
  -o ~/Music/Archive
```

**Why this works:**
- FLAC preserves perfect quality
- Can convert to any lossy format later without quality loss
- Future-proofs your collection

### Use Case 3: Podcast Episodes

Download a podcast episode with small file size:

```bash
ytdl audio https://www.youtube.com/watch?v=podcastep \
  -f mp3 \
  -b 128k \
  -o ~/Podcasts
```

**Why this works:**
- 128k is sufficient for speech
- Smaller files are easier to transfer
- Maximum compatibility for podcast apps

### Use Case 4: Apple Device Optimization

Optimize for iPhone/iPad with M4A:

```bash
ytdl audio https://www.youtube.com/watch?v=musicvideo \
  -f m4a \
  -b 256k \
  -o ~/Music/iPhone
```

**Why this works:**
- M4A is native to Apple devices
- Better quality than MP3 at same file size
- Seamless integration with iTunes/Music app

### Use Case 5: Minimal File Size

Download with absolute minimum file size:

```bash
ytdl audio https://www.youtube.com/watch?v=musicvideo \
  -f opus \
  -b 128k \
  -o ~/Music/Mobile
```

**Why this works:**
- Opus provides best quality-to-size ratio
- Great for limited storage (old phones, limited data plans)
- Excellent for streaming or cloud storage

### Use Case 6: Professional Audio Work

Extract for audio editing:

```bash
ytdl audio https://www.youtube.com/watch?v=sourcevideo \
  -f wav \
  -o ~/AudioProjects/Sources
```

**Why this works:**
- WAV is universally compatible with audio software
- Uncompressed for maximum editing flexibility
- No compression artifacts

## Advanced Examples

### Silent Mode for Scripts

Extract audio without progress output:

```bash
ytdl audio https://www.youtube.com/watch?v=jNQXAC9IVRw \
  -f mp3 \
  --silent
```

### Combining All Options

Complete example with all options:

```bash
ytdl audio https://www.youtube.com/watch?v=jNQXAC9IVRw \
  -f mp3 \
  -b 320k \
  -o ~/Music/Downloads \
  --silent
```

This command:
- Extracts audio in MP3 format
- Uses maximum quality (320kbps)
- Saves to `~/Music/Downloads/`
- Runs silently

### Batch Audio Extraction

Extract audio from multiple videos:

```bash
#!/bin/bash
# extract-audio.sh

URLS=(
  "https://www.youtube.com/watch?v=example1"
  "https://www.youtube.com/watch?v=example2"
  "https://www.youtube.com/watch?v=example3"
)

for url in "${URLS[@]}"; do
  ytdl audio "$url" -f mp3 -b 320k -o ~/Music/Batch
done
```

## Format Comparison Chart

| Format | Type | Quality | Size (4min song) | Compatibility | Best For |
|--------|------|---------|------------------|---------------|----------|
| MP3 (320k) | Lossy | Very Good | ~10 MB | ★★★★★ | General use |
| FLAC | Lossless | Perfect | ~30 MB | ★★★★☆ | Archival |
| M4A (256k) | Lossy | Excellent | ~8 MB | ★★★★★ | Apple devices |
| WAV | Uncompressed | Perfect | ~40 MB | ★★★★★ | Pro audio |
| Opus (192k) | Lossy | Excellent | ~6 MB | ★★★☆☆ | Web/efficiency |

## Troubleshooting

### FFmpeg Not Found

```
Error: FFmpeg is required for audio extraction
```

**Solution:** Install FFmpeg. See the [Installation Guide](./installation.md#installing-ffmpeg).

### Low Audio Quality

If the extracted audio sounds poor:

1. **Check source quality** - Use `ytdl info URL` to see available audio streams
2. **Increase bitrate** - Try `-b 320k` for lossy formats
3. **Use lossless** - Switch to `-f flac` for best quality
4. **Source limitation** - YouTube audio is typically 128-256kbps AAC; extraction can't improve upon that

### File Size Too Large

If files are too large:

1. **Use lossy format** - Switch from FLAC/WAV to MP3/M4A/Opus
2. **Lower bitrate** - Try `-b 192k` or `-b 128k`
3. **Use Opus** - Best quality-to-size ratio
4. **Compress existing files** - Convert FLAC to MP3 after extraction

### Unsupported Format

If your player doesn't support the format:

- **FLAC not working?** Use MP3 instead (universal compatibility)
- **Opus not working?** Your device may be too old; use MP3
- **M4A not working?** Use MP3 for non-Apple devices

## Best Practices

1. **For most users**: Use MP3 at 320kbps - great balance of quality and compatibility
2. **For archival**: Use FLAC - perfect quality, can convert later without loss
3. **For Apple users**: Use M4A at 256kbps - better than MP3 at similar size
4. **For podcasts/speech**: Use MP3 at 128kbps - sufficient quality, small files
5. **For minimal size**: Use Opus - best quality per megabyte
6. **Test first**: Extract one file to verify format compatibility before batch operations

## Format Selection Decision Tree

```
Do you need lossless quality?
├── Yes → Use FLAC
└── No → Do you use primarily Apple devices?
    ├── Yes → Use M4A (256k)
    └── No → Is file size critical?
        ├── Yes → Use Opus (128-192k)
        └── No → Use MP3 (320k)
```

## Next Steps

- **[Playlists](./playlists.md)** - Extract audio from entire playlists
- **[Configuration](./configuration.md)** - Set default audio format and bitrate
- **[Downloading Videos](./downloading-videos.md)** - Download full videos instead of audio only

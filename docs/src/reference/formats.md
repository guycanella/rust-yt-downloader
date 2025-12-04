# Supported Formats Reference

Complete reference for all supported video containers, audio formats, codecs, and their technical specifications.

## Video Container Formats

### MP4 (MPEG-4 Part 14)

**Extension**: `.mp4`
**MIME type**: `video/mp4`
**Specification**: ISO/IEC 14496-14

**Supported codecs**:
- **Video**: H.264/AVC, H.265/HEVC
- **Audio**: AAC, MP3

**Features**:
- ✅ Universal compatibility (all devices/platforms)
- ✅ Hardware decoding support
- ✅ Fast seeking
- ✅ Streaming-optimized (with `faststart` flag)
- ✅ Chapter markers
- ❌ Limited codec support
- ❌ No FLAC or Opus audio
- ❌ No multiple video tracks

**Technical specifications**:
```
Max resolution: Unlimited (practical: 8K)
Max bitrate: Unlimited
Max audio channels: 48
Max subtitle tracks: Limited
Metadata support: ID3v2, iTunes-style
```

**Use cases**:
- General-purpose video distribution
- Mobile device playback
- Web streaming
- Maximum compatibility required

**Examples**:
```bash
# Download as MP4 (default)
ytdl download URL -f mp4

# High-quality MP4
ytdl download URL -q best -f mp4

# MP4 with metadata
ytdl download URL -f mp4  # Metadata embedded by default
```

---

### MKV (Matroska)

**Extension**: `.mkv`
**MIME type**: `video/x-matroska`
**Specification**: Matroska Media Container

**Supported codecs**:
- **Video**: H.264, H.265, VP8, VP9, AV1
- **Audio**: AAC, MP3, FLAC, Opus, Vorbis, DTS, AC3

**Features**:
- ✅ Supports all codecs
- ✅ Multiple audio tracks
- ✅ Multiple subtitle tracks (all formats)
- ✅ Chapter markers
- ✅ Attachments (fonts, thumbnails)
- ✅ Extensive metadata support
- ❌ Lower device compatibility (vs MP4)
- ❌ Larger overhead for small files

**Technical specifications**:
```
Max resolution: Unlimited
Max bitrate: Unlimited
Max audio channels: Unlimited
Max subtitle tracks: Unlimited
Metadata support: Full XML metadata
Attachment support: Yes (fonts, images)
```

**Use cases**:
- Archival storage
- High-quality encodes
- Multiple audio tracks (languages)
- Advanced subtitle features
- Open-source workflows

**Examples**:
```bash
# Download as MKV
ytdl download URL -f mkv

# MKV with all features
ytdl download URL -f mkv -q best

# MKV with embedded subtitles
ytdl config set video.embed_subtitles true
ytdl download URL -f mkv
```

---

### WebM

**Extension**: `.webm`
**MIME type**: `video/webm`
**Specification**: WebM Project (Google)

**Supported codecs**:
- **Video**: VP8, VP9, AV1
- **Audio**: Vorbis, Opus

**Features**:
- ✅ Open format (royalty-free)
- ✅ Optimized for web streaming
- ✅ Good compression (VP9, AV1)
- ✅ Native browser support
- ❌ Limited device compatibility
- ❌ Restricted codec support
- ❌ No H.264/AAC support

**Technical specifications**:
```
Max resolution: Unlimited (practical: 8K)
Max bitrate: Unlimited
Max audio channels: 8
Max subtitle tracks: Limited
Metadata support: Basic
```

**Use cases**:
- Web playback
- Open-source projects
- Efficient compression (VP9)
- Future-proof (AV1)

**Examples**:
```bash
# Download as WebM
ytdl download URL -f webm

# WebM with VP9 codec
ytdl download URL -f webm -q 1080p
```

---

## Video Codecs

### H.264 / AVC (Advanced Video Coding)

**Standard**: ITU-T H.264, ISO/IEC 14496-10
**Container support**: MP4, MKV, WebM (with transcoding)

**Characteristics**:
- **Compression**: Good (older standard)
- **Quality**: Very good at high bitrates
- **Decode performance**: Excellent (hardware support)
- **Encode performance**: Fast

**Profiles**:
- **Baseline**: Low complexity, mobile devices
- **Main**: Broadcast, streaming
- **High**: HD video, Blu-ray
- **High 10**: 10-bit color depth

**Bitrate recommendations** (1080p):
```
Low quality:  2-3 Mbps
Medium:       4-6 Mbps
High:         8-12 Mbps
Very high:    15-20 Mbps
```

**Use cases**:
- Universal compatibility
- Hardware encoding/decoding
- Live streaming
- Battery-efficient playback

---

### H.265 / HEVC (High Efficiency Video Coding)

**Standard**: ITU-T H.265, ISO/IEC 23008-2
**Container support**: MP4, MKV

**Characteristics**:
- **Compression**: Excellent (~50% better than H.264)
- **Quality**: Excellent at lower bitrates
- **Decode performance**: Good (modern hardware)
- **Encode performance**: Slow

**Profiles**:
- **Main**: 8-bit 4:2:0
- **Main 10**: 10-bit 4:2:0
- **Main Still Picture**: Single images

**Bitrate recommendations** (1080p):
```
Low quality:  1-2 Mbps
Medium:       2-4 Mbps
High:         4-6 Mbps
Very high:    8-12 Mbps
```

**Use cases**:
- 4K/8K video
- Bandwidth-limited scenarios
- Archival (better compression)

**Note**: Requires patent licensing for commercial use

---

### VP9

**Developer**: Google
**Container support**: WebM, MKV

**Characteristics**:
- **Compression**: Excellent (similar to H.265)
- **Quality**: Excellent
- **Decode performance**: Good (software)
- **Encode performance**: Very slow

**Profiles**:
- **Profile 0**: 8-bit 4:2:0
- **Profile 1**: 8-bit 4:2:0/4:2:2/4:4:4
- **Profile 2**: 10/12-bit 4:2:0
- **Profile 3**: 10/12-bit 4:2:0/4:2:2/4:4:4

**Bitrate recommendations** (1080p):
```
Low quality:  1-2 Mbps
Medium:       2-3 Mbps
High:         4-5 Mbps
Very high:    6-8 Mbps
```

**Use cases**:
- YouTube streaming (primary codec)
- Open-source projects
- Royalty-free distribution
- Web video

---

### AV1

**Developer**: Alliance for Open Media
**Container support**: WebM, MKV, MP4 (experimental)

**Characteristics**:
- **Compression**: Best-in-class (~30% better than VP9)
- **Quality**: Excellent at very low bitrates
- **Decode performance**: Poor (new, limited hardware)
- **Encode performance**: Extremely slow

**Bitrate recommendations** (1080p):
```
Low quality:  0.5-1 Mbps
Medium:       1-2 Mbps
High:         2-3 Mbps
Very high:    4-6 Mbps
```

**Use cases**:
- Future-proof archival
- Bandwidth-constrained streaming
- 8K video
- Open-source projects

**Note**: Limited availability on YouTube, cutting-edge codec

---

## Audio Formats

### MP3 (MPEG-1/2 Audio Layer 3)

**Extension**: `.mp3`
**MIME type**: `audio/mpeg`
**Type**: Lossy compression

**Specifications**:
- **Sample rates**: 8, 11.025, 12, 16, 22.05, 24, 32, 44.1, 48 kHz
- **Bitrates**: 8-320 kbps (CBR), VBR V0-V9
- **Channels**: Mono, stereo, joint stereo

**Quality tiers**:
```
128 kbps: Acceptable for speech/podcasts
192 kbps: Good for general music
256 kbps: Very good quality
320 kbps: Maximum quality (recommended)
```

**File size** (per minute at 320 kbps): ~2.5 MB

**Advantages**:
- ✅ Universal compatibility
- ✅ Small file sizes
- ✅ Wide software support

**Disadvantages**:
- ❌ Lossy compression
- ❌ Inferior to modern codecs (AAC, Opus)
- ❌ High-frequency artifacts

**Use cases**:
- Maximum compatibility
- Legacy device support
- General music listening

**Examples**:
```bash
# Extract as MP3 (default)
ytdl audio URL -f mp3

# High-quality MP3
ytdl config set audio.bitrate 320k
ytdl audio URL -f mp3
```

---

### M4A / AAC (Advanced Audio Coding)

**Extension**: `.m4a`, `.aac`
**MIME type**: `audio/mp4`, `audio/aac`
**Type**: Lossy compression

**Specifications**:
- **Sample rates**: 8-96 kHz
- **Bitrates**: 8-512 kbps
- **Channels**: Up to 48 channels
- **Profiles**: LC, HE-AAC, HE-AAC v2

**Quality tiers**:
```
128 kbps: Good (equivalent to MP3 192 kbps)
192 kbps: Very good
256 kbps: Excellent (equivalent to MP3 320 kbps)
320 kbps: Maximum quality
```

**File size** (per minute at 256 kbps): ~2 MB

**Advantages**:
- ✅ Better quality than MP3 at same bitrate
- ✅ Efficient encoding
- ✅ Apple ecosystem native
- ✅ Wide device support

**Disadvantages**:
- ❌ Patent-encumbered
- ❌ Lossy compression

**Use cases**:
- Apple devices (iTunes, iPhone)
- Quality-focused but compatibility needed
- Streaming services

**Examples**:
```bash
# Extract as M4A
ytdl audio URL -f m4a

# High-quality AAC
ytdl config set audio.bitrate 256k
ytdl audio URL -f m4a
```

---

### FLAC (Free Lossless Audio Codec)

**Extension**: `.flac`
**MIME type**: `audio/flac`
**Type**: Lossless compression

**Specifications**:
- **Sample rates**: 1 Hz - 655.35 kHz
- **Bit depth**: 4-32 bits per sample
- **Channels**: Up to 8 channels
- **Compression levels**: 0 (fastest) - 8 (smallest)

**Compression ratio**: Typically 40-60% of original size

**File size** (per minute, 16-bit/44.1kHz): ~4-5 MB

**Advantages**:
- ✅ Perfect audio reproduction (lossless)
- ✅ Open format (royalty-free)
- ✅ Metadata support
- ✅ Wide software support

**Disadvantages**:
- ❌ Larger file sizes than lossy
- ❌ Limited hardware device support

**Use cases**:
- Archival storage
- Audio production
- High-quality music libraries
- Audiophile listening

**Examples**:
```bash
# Extract as FLAC (lossless)
ytdl audio URL -f flac

# FLAC is always highest quality (no bitrate setting)
```

---

### Opus

**Extension**: `.opus`
**MIME type**: `audio/opus`
**Type**: Lossy compression

**Specifications**:
- **Sample rates**: 8, 12, 16, 24, 48 kHz (internally)
- **Bitrates**: 6-510 kbps
- **Channels**: Mono, stereo
- **Modes**: SILK (speech), CELT (music), Hybrid

**Quality tiers**:
```
64 kbps:  Good for speech
96 kbps:  Good for music
128 kbps: Very good
192 kbps: Excellent
256 kbps: Maximum practical quality
```

**File size** (per minute at 128 kbps): ~1 MB

**Advantages**:
- ✅ Best quality-to-bitrate ratio
- ✅ Low latency
- ✅ Excellent speech encoding
- ✅ Open format (royalty-free)

**Disadvantages**:
- ❌ Limited device compatibility (newer format)
- ❌ Not supported by Apple devices natively

**Use cases**:
- Modern streaming applications
- VoIP, gaming voice chat
- Efficient music storage
- Web audio

**Examples**:
```bash
# Extract as Opus
ytdl audio URL -f opus

# Opus with custom bitrate
ytdl config set audio.bitrate 192k
ytdl audio URL -f opus
```

---

### WAV (Waveform Audio File Format)

**Extension**: `.wav`
**MIME type**: `audio/wav`, `audio/x-wav`
**Type**: Uncompressed PCM

**Specifications**:
- **Sample rates**: Typically 44.1 or 48 kHz
- **Bit depth**: 8, 16, 24, 32 bits
- **Channels**: Mono, stereo, multi-channel
- **Format**: Linear PCM

**File size** (per minute, 16-bit/44.1kHz stereo): ~10 MB

**Advantages**:
- ✅ No compression (perfect quality)
- ✅ Universal compatibility
- ✅ Industry standard for production
- ✅ No encoding overhead

**Disadvantages**:
- ❌ Very large file sizes
- ❌ Limited metadata support
- ❌ Inefficient storage

**Use cases**:
- Audio editing (no generation loss)
- Professional production
- Temporary/intermediate files
- Archival (when storage not a concern)

**Examples**:
```bash
# Extract as WAV (uncompressed)
ytdl audio URL -f wav

# WAV always uncompressed (no bitrate setting)
```

---

## Audio Codec Comparison

### Quality Comparison (at equivalent bitrates)

**Lossy codecs ranked** (best to worst quality at same bitrate):
1. Opus
2. AAC (HE-AAC v2)
3. AAC (LC)
4. MP3

**Bitrate equivalency** (perceived quality):
```
Opus 96 kbps   ≈ AAC 128 kbps   ≈ MP3 192 kbps
Opus 128 kbps  ≈ AAC 192 kbps   ≈ MP3 256 kbps
Opus 160 kbps  ≈ AAC 256 kbps   ≈ MP3 320 kbps
```

### File Size Comparison (1-minute audio)

| Format | Bitrate | File Size | Quality |
|--------|---------|-----------|---------|
| MP3 | 128k | 1 MB | Acceptable |
| MP3 | 320k | 2.5 MB | High |
| AAC | 128k | 1 MB | Good |
| AAC | 256k | 2 MB | Excellent |
| Opus | 96k | 0.7 MB | Good |
| Opus | 192k | 1.5 MB | Excellent |
| FLAC | - | 4-5 MB | Perfect (lossless) |
| WAV | - | 10 MB | Perfect (uncompressed) |

### Compatibility Matrix

| Format | Windows | macOS | Linux | iOS | Android | Web |
|--------|---------|-------|-------|-----|---------|-----|
| MP3 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| AAC/M4A | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| FLAC | ✅ | ✅ | ✅ | ❌* | ✅ | ✅ |
| Opus | ✅ | ⚠️** | ✅ | ❌* | ✅ | ✅ |
| WAV | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

*Requires third-party apps
**Limited native support

---

## Format Selection Guide

### By Use Case

**Maximum compatibility**:
- Video: MP4 (H.264/AAC)
- Audio: MP3 (320 kbps)

**Best quality (lossless)**:
- Video: MKV (any codec)
- Audio: FLAC

**Smallest file size**:
- Video: WebM (VP9/Opus)
- Audio: Opus (128 kbps)

**Apple ecosystem**:
- Video: MP4 (H.264/AAC)
- Audio: M4A (AAC 256 kbps)

**Web streaming**:
- Video: WebM (VP9/Opus) or MP4 (H.264/AAC)
- Audio: Opus or AAC

**Archival/preservation**:
- Video: MKV (best available codec)
- Audio: FLAC

**Mobile offline**:
- Video: MP4 720p (H.264/AAC)
- Audio: AAC 128-192 kbps

### By Quality Priority

**Priority: Compatibility**
```bash
ytdl config set video.format mp4
ytdl config set audio.format mp3
ytdl config set audio.bitrate 320k
```

**Priority: Quality**
```bash
ytdl config set video.format mkv
ytdl config set audio.format flac
ytdl config set general.default_quality best
```

**Priority: File Size**
```bash
ytdl config set video.format webm
ytdl config set audio.format opus
ytdl config set audio.bitrate 128k
ytdl config set general.default_quality 720p
```

---

## Conversion Support Matrix

### Video Container Conversions

| From/To | MP4 | MKV | WebM |
|---------|-----|-----|------|
| **MP4** | - | Remux | Transcode |
| **MKV** | Remux* | - | Transcode |
| **WebM** | Transcode | Remux | - |

*Depends on codec compatibility

### Audio Format Conversions

| From/To | MP3 | AAC | FLAC | Opus | WAV |
|---------|-----|-----|------|------|-----|
| **Any** | Transcode | Transcode | Transcode | Transcode | Transcode |

**Note**: All audio conversions require transcoding via FFmpeg.

---

## Technical Specifications Summary

### Video Format Summary

| Format | Open | Patent-Free | Max Res | Subtitle Support | Metadata |
|--------|------|-------------|---------|------------------|----------|
| MP4 | ❌ | ❌ | Unlimited | Limited | Good |
| MKV | ✅ | ✅ | Unlimited | Excellent | Excellent |
| WebM | ✅ | ✅ | Unlimited | Good | Basic |

### Audio Format Summary

| Format | Type | Sample Rate | Bit Depth | Channels |
|--------|------|-------------|-----------|----------|
| MP3 | Lossy | Up to 48 kHz | - | Stereo |
| AAC | Lossy | Up to 96 kHz | - | Up to 48 |
| FLAC | Lossless | Up to 655 kHz | 4-32 bit | Up to 8 |
| Opus | Lossy | Up to 48 kHz | - | Stereo |
| WAV | Uncompressed | Any | 8-32 bit | Any |

---

## Related Documentation

- [Quality Selection](../advanced/quality-selection.md) - Video quality guide
- [Format Conversion](../advanced/format-conversion.md) - Converting between formats
- [FFmpeg Integration](../advanced/ffmpeg.md) - FFmpeg configuration
- [CLI Commands](./cli-commands.md) - Command-line usage

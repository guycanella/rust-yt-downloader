# Downloading Playlists

This comprehensive guide covers downloading multiple videos from YouTube playlists with `ytdl`. Learn how to download entire playlists, handle errors, and efficiently manage bulk downloads.

## Basic Playlist Download

Download all videos from a YouTube playlist:

```bash
ytdl playlist https://www.youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf
```

This downloads all videos in the playlist to your current directory using default settings (best quality, MP4 format).

### What Happens During Playlist Download

When you download a playlist, `ytdl`:
1. Fetches the playlist metadata and video list
2. Downloads each video sequentially
3. Shows progress for each individual video
4. Creates separate files for each video
5. Continues with remaining videos if one fails
6. Provides a summary when complete

### Example Output

```
Fetching playlist: "Python Tutorials for Beginners"
Found 15 videos

[1/15] Downloading: "Introduction to Python"
[████████████████████] 100% - 8.5 MB/s
✓ Complete: Introduction to Python.mp4

[2/15] Downloading: "Variables and Data Types"
[████████████████████] 100% - 9.2 MB/s
✓ Complete: Variables and Data Types.mp4

...

Download Summary:
✓ Successful: 14 videos
✗ Failed: 1 video (see errors above)
Total size: 2.3 GB
Total time: 15m 32s
```

## Downloading Multiple Playlists

You can download from multiple playlists at once by providing multiple URLs:

```bash
ytdl playlist \
  https://www.youtube.com/playlist?list=PLAYLIST_ID_1 \
  https://www.youtube.com/playlist?list=PLAYLIST_ID_2 \
  https://www.youtube.com/playlist?list=PLAYLIST_ID_3
```

Each playlist is processed sequentially, and all videos are downloaded to the same output directory (unless you specify different paths).

## Audio-Only from Playlists

Extract audio from all videos in a playlist using the `--audio-only` flag:

```bash
ytdl playlist https://www.youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf --audio-only
```

This extracts audio from each video and saves as MP3 files (default format and bitrate).

### Specifying Audio Format

Choose a specific audio format for playlist extraction:

```bash
# Extract as FLAC (lossless)
ytdl playlist https://www.youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf \
  --audio-only \
  -f flac

# Extract as M4A
ytdl playlist https://www.youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf \
  --audio-only \
  -f m4a

# Extract as Opus (smallest files)
ytdl playlist https://www.youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf \
  --audio-only \
  -f opus
```

### Specifying Audio Bitrate

Control the audio quality for lossy formats:

```bash
# High quality (320kbps)
ytdl playlist URL --audio-only -f mp3 -b 320k

# Standard quality (192kbps)
ytdl playlist URL --audio-only -f mp3 -b 192k

# Lower quality for speech/podcasts (128kbps)
ytdl playlist URL --audio-only -f mp3 -b 128k
```

## Quality and Format Options

### Video Quality for Playlists

Specify video quality for all videos in the playlist:

```bash
# Download all in 720p
ytdl playlist https://www.youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf -q 720p

# Download all in 1080p
ytdl playlist https://www.youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf -q 1080p

# Download all in best available quality
ytdl playlist https://www.youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf -q best

# Download all in lowest quality (save bandwidth)
ytdl playlist https://www.youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf -q worst
```

### Video Format for Playlists

Choose the container format for all videos:

```bash
# Download as MP4 (default, most compatible)
ytdl playlist URL -f mp4

# Download as MKV (better codec support)
ytdl playlist URL -f mkv

# Download as WebM
ytdl playlist URL -f webm
```

## Output Directory

Organize playlist downloads in dedicated folders:

```bash
ytdl playlist https://www.youtube.com/playlist?list=PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf \
  -o ~/Videos/Python_Tutorials
```

### Organizing Multiple Playlists

Download different playlists to different directories:

```bash
# Download playlist 1 to one folder
ytdl playlist https://www.youtube.com/playlist?list=PLAYLIST_1 \
  -o ~/Videos/Course_1

# Download playlist 2 to another folder
ytdl playlist https://www.youtube.com/playlist?list=PLAYLIST_2 \
  -o ~/Videos/Course_2
```

## Real-World Use Cases

### Use Case 1: Download a Music Album

Download a music playlist as high-quality MP3:

```bash
ytdl playlist https://www.youtube.com/playlist?list=PLmusicalbum \
  --audio-only \
  -f mp3 \
  -b 320k \
  -o ~/Music/Albums/ArtistName
```

**Why this works:**
- Audio-only saves space and time
- MP3 at 320k provides excellent quality
- Organized in a dedicated album folder

### Use Case 2: Educational Course

Download an online course for offline viewing:

```bash
ytdl playlist https://www.youtube.com/playlist?list=PLcourseid \
  -q 720p \
  -f mp4 \
  -o ~/Education/PythonCourse
```

**Why this works:**
- 720p balances quality and file size
- MP4 is universally compatible
- All videos in one course folder
- 720p is sufficient for screen recordings

### Use Case 3: Podcast Series

Download a podcast series:

```bash
ytdl playlist https://www.youtube.com/playlist?list=PLpodcastseries \
  --audio-only \
  -f mp3 \
  -b 128k \
  -o ~/Podcasts/SeriesName
```

**Why this works:**
- Audio-only (no need for video)
- 128k is sufficient for speech
- Smaller files for easier syncing
- Organized by series name

### Use Case 4: Conference Talks

Download conference presentations:

```bash
ytdl playlist https://www.youtube.com/playlist?list=PLconference \
  -q 1080p \
  -f mp4 \
  -o ~/Videos/Conferences/2024
```

**Why this works:**
- 1080p ensures slides are readable
- MP4 for broad compatibility
- Organized by year

### Use Case 5: Music Discovery Playlist

Download a large music playlist for archival:

```bash
ytdl playlist https://www.youtube.com/playlist?list=PLmusicdiscovery \
  --audio-only \
  -f flac \
  -o ~/Music/Discovery
```

**Why this works:**
- FLAC preserves maximum quality
- Can convert to other formats later
- Good for building a music library

### Use Case 6: Kids' Content

Download educational videos for kids (offline viewing):

```bash
ytdl playlist https://www.youtube.com/playlist?list=PLkidseducation \
  -q 480p \
  -f mp4 \
  -o ~/Videos/Kids
```

**Why this works:**
- 480p sufficient for young viewers
- Smaller files for tablets with limited storage
- MP4 works on all kids' devices

## Handling Errors in Playlists

### Continue on Error (Default Behavior)

By default, if one video fails, `ytdl` continues with the rest:

```bash
ytdl playlist URL
```

Example output:
```
[1/10] ✓ Video 1 downloaded
[2/10] ✗ Video 2 failed: Video unavailable
[3/10] ✓ Video 3 downloaded
...
Summary: 8 successful, 2 failed
```

### Common Playlist Errors

#### 1. Video Unavailable

**Error:** `Video unavailable (private or deleted)`

**What it means:** The video has been removed, made private, or deleted

**Action:** The video is skipped; download continues

#### 2. Age-Restricted Content

**Error:** `Video is age-restricted`

**What it means:** YouTube requires sign-in to view

**Action:** Video is skipped (authentication support coming in future updates)

#### 3. Geographic Restrictions

**Error:** `Video not available in your country`

**What it means:** Content is geo-blocked

**Action:** Video is skipped

#### 4. Copyright Claims

**Error:** `Video removed due to copyright claim`

**What it means:** Content was taken down by copyright holder

**Action:** Video is skipped

### Reviewing Failed Downloads

After a playlist download completes, check the summary:

```
Download Summary:
✓ Successful: 18 videos
✗ Failed: 2 videos
  - Video 5: Age-restricted
  - Video 12: Unavailable

Total size: 3.2 GB
Total time: 22m 15s
```

## Advanced Playlist Operations

### Silent Mode for Background Downloads

Download playlists without progress output:

```bash
ytdl playlist URL --silent
```

Useful for:
- Running in the background
- Scheduled downloads via cron
- Logging only errors

### Complete Example

Combining all options for a comprehensive download:

```bash
ytdl playlist https://www.youtube.com/playlist?list=PLexample123 \
  -q 1080p \
  -f mp4 \
  -o ~/Videos/Complete_Course \
  --silent
```

This command:
- Downloads entire playlist
- All videos in 1080p quality
- Saves as MP4 files
- Stores in `~/Videos/Complete_Course/`
- Runs silently (no progress bars)

### Batch Playlist Download Script

Download multiple playlists with a script:

```bash
#!/bin/bash
# download-playlists.sh

declare -A PLAYLISTS=(
  ["Python Course"]="https://www.youtube.com/playlist?list=PLpython"
  ["JavaScript Basics"]="https://www.youtube.com/playlist?list=PLjavascript"
  ["Web Development"]="https://www.youtube.com/playlist?list=PLwebdev"
)

for name in "${!PLAYLISTS[@]}"; do
  echo "Downloading: $name"
  ytdl playlist "${PLAYLISTS[$name]}" \
    -q 720p \
    -f mp4 \
    -o ~/Education/"$name" \
    --silent
  echo "✓ Completed: $name"
done

echo "All playlists downloaded!"
```

Make executable and run:
```bash
chmod +x download-playlists.sh
./download-playlists.sh
```

## Playlist Size Considerations

### Estimating Download Size and Time

Before downloading a large playlist, use `ytdl info` on a few videos to estimate:

```bash
# Check one video from the playlist
ytdl info https://www.youtube.com/watch?v=VIDEO_ID
```

**Quick estimation:**
- **720p MP4**: ~150-300 MB per 10-minute video
- **1080p MP4**: ~300-600 MB per 10-minute video
- **Audio MP3 (320k)**: ~2.5 MB per minute (~25 MB per 10 minutes)

**For a 30-video playlist:**
- **720p**: ~5-10 GB
- **1080p**: ~10-20 GB
- **Audio only**: ~750 MB - 1.5 GB

### Managing Large Playlists

For playlists with 50+ videos:

1. **Check disk space** before starting:
   ```bash
   df -h  # Linux/macOS
   ```

2. **Use lower quality** if space is limited:
   ```bash
   ytdl playlist URL -q 480p  # or 360p
   ```

3. **Extract audio only** to save space:
   ```bash
   ytdl playlist URL --audio-only -b 192k
   ```

4. **Download in batches** if you have limited bandwidth or time

## Troubleshooting

### Playlist Not Found

```
Error: Playlist not found or is private
```

**Solutions:**
- Verify the URL is correct
- Check if the playlist is public
- Ensure you copied the full playlist URL with `?list=` parameter

### Partial Downloads

If download is interrupted (network issue, power loss):

- **Resume:** Simply run the same command again
- **Existing files:** `ytdl` will skip already-downloaded videos
- **Check:** Look for partial `.part` files and remove them before retrying

### Out of Disk Space

```
Error: Insufficient disk space
```

**Solutions:**
1. Free up space
2. Use lower quality: `-q 480p` instead of `-q 1080p`
3. Extract audio only: `--audio-only`
4. Download to external drive: `-o /path/to/external/drive`

### Very Slow Downloads

**Causes and solutions:**

1. **Large playlist:**
   - Download in smaller batches
   - Use `--silent` to reduce terminal overhead

2. **Low bandwidth:**
   - Use lower quality settings
   - Download during off-peak hours

3. **YouTube throttling:**
   - Check network.retry_attempts in config
   - Add delay between videos (future feature)

## Best Practices

1. **Check playlist size first** - Use `info` command on sample videos to estimate
2. **Start with lower quality** - You can always re-download in higher quality later
3. **Use audio-only for music** - Save bandwidth and storage
4. **Organize by playlist name** - Use descriptive output directories
5. **Monitor first few downloads** - Ensure settings are correct before walking away
6. **Check summary** - Review failed downloads and reasons
7. **Clean up partial files** - Remove `.part` files from interrupted downloads
8. **Respect bandwidth** - Don't download huge playlists on metered connections

## Comparison: Video vs. Audio-Only Playlists

| Aspect | Video (1080p MP4) | Audio Only (MP3 320k) |
|--------|-------------------|------------------------|
| File size (30 videos) | ~15-20 GB | ~1-1.5 GB |
| Download time (10 Mbps) | ~4-5 hours | ~15-20 minutes |
| Use case | Tutorials, courses | Music, podcasts |
| Storage needed | High | Low |
| Bandwidth usage | High | Low |

## Next Steps

- **[Configuration](./configuration.md)** - Set default quality/format for all playlist downloads
- **[Downloading Videos](./downloading-videos.md)** - Learn about quality options for individual videos
- **[Extracting Audio](./extracting-audio.md)** - Detailed guide to audio formats and quality

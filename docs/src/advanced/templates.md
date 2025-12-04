# Filename Templates and Naming

This guide covers the filename templating system that allows you to customize how downloaded files are named, including metadata substitution, sanitization, and advanced naming patterns.

## Overview

The filename template system provides flexible control over output filenames using placeholder variables that get replaced with video metadata. This is essential for organizing large libraries and automating workflows.

## Template Syntax

### Basic Syntax

Templates use the `{variable}` syntax for placeholder substitution:

```
{title}.{ext}              → My Video.mp4
{channel} - {title}.{ext}  → AwesomeChannel - My Video.mp4
{upload_date}_{title}.{ext} → 20240315_My Video.mp4
```

### Available Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `{title}` | Video title | `My Awesome Video` |
| `{channel}` | Channel name | `TechChannel` |
| `{channel_id}` | Channel ID | `UC_x5XG1OV2P6uZZ5FSM9Ttw` |
| `{video_id}` | YouTube video ID | `dQw4w9WgXcQ` |
| `{ext}` | File extension | `mp4`, `mkv`, `mp3` |
| `{quality}` | Resolution quality | `1080p`, `720p` |
| `{upload_date}` | Upload date (YYYYMMDD) | `20240315` |
| `{duration}` | Video length (seconds) | `305` |
| `{duration_string}` | Formatted duration | `05:05` |
| `{view_count}` | Number of views | `1234567` |
| `{like_count}` | Number of likes | `50000` |
| `{playlist}` | Playlist name (if applicable) | `Best Videos 2024` |
| `{playlist_index}` | Position in playlist | `01`, `02`, `03` |
| `{playlist_id}` | Playlist ID | `PLrAXtmErZgOei...` |

## Template Examples

### Simple Templates

**Default template**:
```
{title}.{ext}
```

**Channel and title**:
```
{channel} - {title}.{ext}
→ TechReviews - iPhone 15 Review.mp4
```

**Date-based naming**:
```
{upload_date} - {title}.{ext}
→ 20240315 - How to Code in Rust.mp4
```

**ID-based (unique)**:
```
{video_id}.{ext}
→ dQw4w9WgXcQ.mp4
```

### Playlist Templates

**With index**:
```
{playlist_index}. {title}.{ext}
→ 01. Introduction.mp4
→ 02. Getting Started.mp4
→ 03. Advanced Topics.mp4
```

**Playlist folder structure**:
```
{playlist}/{playlist_index} - {title}.{ext}
→ Python Tutorial/01 - Introduction.mp4
→ Python Tutorial/02 - Variables.mp4
```

**Channel and playlist**:
```
{channel}/{playlist}/{title}.{ext}
→ CodingAcademy/Python Basics/Introduction.mp4
```

### Advanced Templates

**Complete metadata**:
```
[{upload_date}] {channel} - {title} [{quality}].{ext}
→ [20240315] TechReviews - iPhone 15 Review [1080p].mp4
```

**Archival naming**:
```
{upload_date}_{channel}_{video_id}.{ext}
→ 20240315_TechReviews_dQw4w9WgXcQ.mp4
```

**Library organization**:
```
{channel}/{upload_date} - {title}.{ext}
→ CodingTutorials/20240315 - Learn Rust.mp4
```

## Configuration

### Setting Default Templates

Configure templates in the configuration file:

```toml
[general]
filename_template = "{channel} - {title}.{ext}"
playlist_template = "{playlist}/{playlist_index}. {title}.{ext}"
```

### CLI Override

Override templates per download:

```bash
# Custom template via flag
ytdl download URL --template "{upload_date}_{title}.{ext}"

# Playlist with custom template
ytdl playlist URL --template "{playlist_index} - {title}.{ext}"
```

### Configuration Commands

```bash
# Set default template
ytdl config set general.filename_template "{channel} - {title}.{ext}"

# Set playlist template
ytdl config set general.playlist_template "{playlist}/{title}.{ext}"

# View current template
ytdl config get general.filename_template
```

## Filename Sanitization

### Automatic Sanitization

All filenames are automatically sanitized to ensure filesystem compatibility:

**Removed characters**:
- `/` (path separator)
- `\` (Windows path separator)
- `:` (Windows drive separator)
- `*` (wildcard)
- `?` (wildcard)
- `"` (quotes)
- `<` `>` (redirects)
- `|` (pipe)

**Replacement rules**:
```
My Video: The Best! → My Video - The Best!
Path/To/Video       → Path-To-Video
"Quoted Title"      → Quoted Title
```

### Sanitization Examples

| Original | Sanitized |
|----------|-----------|
| `Video: Part 1` | `Video - Part 1` |
| `Question?` | `Question` |
| `"The Movie"` | `The Movie` |
| `C:\Path\File` | `C-Path-File` |
| `Star*Wars` | `StarWars` |

### Custom Sanitization

Configure sanitization behavior:

```toml
[general]
# Replace instead of remove
sanitize_replace = "-"

# Allow certain characters
sanitize_allow = "[]()!@#$%"

# Truncate long filenames
max_filename_length = 200
```

## Directory Structure

### Nested Directories

Use `/` in templates to create directory structures:

```bash
# Create channel subdirectories
ytdl download URL --template "{channel}/{title}.{ext}"
→ TechChannel/My Video.mp4

# Date-based folders
ytdl download URL --template "{upload_date}/{title}.{ext}"
→ 20240315/My Video.mp4

# Multi-level structure
ytdl download URL --template "{channel}/{upload_date}/{title}.{ext}"
→ TechChannel/20240315/My Video.mp4
```

### Playlist Organization

Automatically organize playlists:

```bash
# Each playlist in its own folder
ytdl playlist URL --template "{playlist}/{playlist_index} - {title}.{ext}"

# Flat structure with playlist prefix
ytdl playlist URL --template "[{playlist}] {playlist_index} - {title}.{ext}"
```

## Advanced Features

### Conditional Templates

Use different templates based on context:

**Video vs. Audio**:
```toml
[video]
template = "{channel} - {title} [{quality}].{ext}"

[audio]
template = "{channel} - {title}.{ext}"
```

**Playlist vs. Single**:
- Single video: Uses `filename_template`
- Playlist: Uses `playlist_template`

### Date Formatting

The `{upload_date}` variable supports format specifiers:

```
{upload_date}           → 20240315
{upload_date:Y-m-d}     → 2024-03-15
{upload_date:Y/m/d}     → 2024/03/15
{upload_date:d-m-Y}     → 15-03-2024
{upload_date:B d, Y}    → March 15, 2024
```

**Format codes**:
- `Y` - Year (4 digits)
- `m` - Month (2 digits)
- `d` - Day (2 digits)
- `B` - Month name (full)
- `b` - Month name (abbreviated)

### Numeric Formatting

Format numeric values with padding:

```
{playlist_index}        → 5
{playlist_index:02}     → 05
{playlist_index:03}     → 005

{view_count}            → 1234567
{view_count:,}          → 1,234,567
```

### Text Transformations

Apply transformations to variables:

```
{title:upper}           → MY VIDEO TITLE
{title:lower}           → my video title
{title:title}           → My Video Title
{channel:snake}         → tech_reviews
{channel:kebab}         → tech-reviews
```

## Use Case Examples

### Personal Archive

Organize by channel and date:

```toml
[general]
filename_template = "{channel}/{upload_date:Y}/{upload_date:m}/{title}.{ext}"
```

Result:
```
TechReviews/2024/03/iPhone 15 Review.mp4
TechReviews/2024/03/Best Laptops 2024.mp4
CodingTutorials/2024/02/Learn Rust.mp4
```

### Course Materials

Number and organize educational content:

```toml
[general]
playlist_template = "{playlist}/{playlist_index:02} - {title}.{ext}"
```

Result:
```
Python Masterclass/01 - Introduction.mp4
Python Masterclass/02 - Variables.mp4
Python Masterclass/03 - Functions.mp4
```

### Music Collection

Organize music videos by artist:

```toml
[audio]
template = "{channel}/[{upload_date:Y}] {title}.{ext}"
```

Result:
```
The Beatles/[1965] Help!.mp3
The Beatles/[1969] Come Together.mp3
```

### Research Archive

Unique, sortable filenames:

```toml
[general]
filename_template = "{upload_date}_{video_id}_{channel}.{ext}"
```

Result:
```
20240315_dQw4w9WgXcQ_TechChannel.mp4
20240316_aBc123XyZ_ScienceDaily.mp4
```

### Backup System

Preserve all metadata:

```toml
[general]
filename_template = "[{upload_date}][{quality}] {channel} - {title} ({video_id}).{ext}"
```

Result:
```
[20240315][1080p] TechReviews - iPhone 15 (dQw4w9WgXcQ).mp4
```

## Best Practices

### 1. Keep Templates Simple

Start simple, add complexity as needed:

```bash
# Good: Simple and clear
{channel} - {title}.{ext}

# Overly complex: Hard to navigate
[{upload_date:Y-m-d}][{quality}][{duration_string}] {channel} ({channel_id}) - {title} (Views-{view_count:,}) [{video_id}].{ext}
```

### 2. Use Consistent Naming

Maintain consistency across your library:

```toml
[general]
filename_template = "{channel} - {title}.{ext}"
playlist_template = "{playlist}/{playlist_index:02} - {title}.{ext}"
```

### 3. Consider Filesystem Limits

**Filename length limits**:
- Windows: 255 characters
- Linux/macOS: 255 bytes

```toml
[general]
max_filename_length = 200  # Leave margin for extension
```

### 4. Plan for Sorting

Use zero-padded numbers for proper sorting:

```
{playlist_index:02}  → 01, 02, ..., 99
{playlist_index:03}  → 001, 002, ..., 999
```

### 5. Include Unique Identifiers

For archival, include `{video_id}` to ensure uniqueness:

```
{title} ({video_id}).{ext}
```

## Troubleshooting

### Template Not Applied

**Problem**: Downloads still use default naming

**Solution**:
```bash
# Verify config setting
ytdl config get general.filename_template

# Set if empty
ytdl config set general.filename_template "{channel} - {title}.{ext}"
```

### Invalid Characters in Filename

**Problem**: Filesystem rejects filename

**Check**:
- Are special characters being sanitized?
- Is filename too long?

**Solution**:
```bash
# Enable verbose sanitization logging
ytdl download URL --template "{title}.{ext}" -v
```

### Missing Variable Values

**Problem**: Some variables show as `{unknown}` or empty

**Cause**: Video metadata doesn't include that field

**Solution**:
```bash
# Check available metadata
ytdl info URL

# Use fallback in template
{playlist|Unknown}/{title}.{ext}
```

### Directory Creation Fails

**Problem**: Can't create nested directories

**Solution**:
```bash
# Ensure output directory exists
ytdl download URL -o ~/Videos --template "{channel}/{title}.{ext}"

# Check permissions
ls -la ~/Videos
```

## Related Documentation

- [Configuration Options](../reference/config-options.md) - All config settings
- [CLI Commands](../reference/cli-commands.md) - Command-line usage
- [Playlists Guide](../guide/playlists.md) - Playlist downloads

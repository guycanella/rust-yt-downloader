# Installation

This guide will walk you through installing `ytdl` and its dependencies on your system.

## Prerequisites

Before installing `ytdl`, you'll need:

- **Rust toolchain** (version 1.75.0 or later) - Required for building from source or installing from crates.io
- **FFmpeg** - Required for video/audio processing (installation covered below)

## Installing ytdl

### Option 1: Install from crates.io (Recommended)

The easiest way to install `ytdl` is from Rust's package registry:

```bash
cargo install ytdl
```

This will download, compile, and install the latest stable version of `ytdl`. The binary will be added to your `PATH` automatically (usually in `~/.cargo/bin/`).

> **Note:** The first installation may take a few minutes as Cargo compiles the binary and all dependencies.

### Option 2: Build from Source

If you want to build the latest development version or contribute to the project:

1. **Clone the repository:**

```bash
git clone https://github.com/yourusername/rust-yt-downloader.git
cd rust-yt-downloader
```

2. **Build the project:**

```bash
# Development build (faster compilation, slower runtime)
cargo build

# Release build (optimized, recommended for regular use)
cargo build --release
```

3. **Install the binary:**

```bash
cargo install --path .
```

This installs the `ytdl` binary to `~/.cargo/bin/`, making it available system-wide.

### Option 3: Run without Installing

You can also run `ytdl` directly without installing it:

```bash
cargo run -- download https://youtube.com/watch?v=abc123
```

> **Note:** Replace `ytdl` with `cargo run --` in all examples if using this method.

## Installing FFmpeg

FFmpeg is required for video and audio processing. Choose the installation method for your operating system:

### Linux (Debian/Ubuntu)

```bash
sudo apt update
sudo apt install ffmpeg
```

### Linux (Fedora/RHEL/CentOS)

```bash
sudo dnf install ffmpeg
```

### Linux (Arch)

```bash
sudo pacman -S ffmpeg
```

### macOS

Using Homebrew (recommended):

```bash
brew install ffmpeg
```

Using MacPorts:

```bash
sudo port install ffmpeg
```

### Windows

#### Option 1: Using Chocolatey (Recommended)

```powershell
choco install ffmpeg
```

#### Option 2: Using Scoop

```powershell
scoop install ffmpeg
```

#### Option 3: Manual Installation

1. Download FFmpeg from the [official website](https://ffmpeg.org/download.html)
2. Extract the archive to a folder (e.g., `C:\ffmpeg`)
3. Add the `bin` folder to your system PATH:
   - Open System Properties → Advanced → Environment Variables
   - Under "System variables", find and edit "Path"
   - Add `C:\ffmpeg\bin` (or your installation path)
   - Click OK and restart your terminal

## Verifying Installation

### Verify ytdl Installation

Check that `ytdl` is installed correctly:

```bash
ytdl --version
```

You should see output like:

```
ytdl 0.1.0
```

### Verify FFmpeg Installation

Check that FFmpeg is installed correctly:

```bash
ffmpeg -version
```

You should see version information for FFmpeg.

### Test a Download

Try downloading a video to ensure everything works:

```bash
ytdl info https://www.youtube.com/watch?v=jNQXAC9IVRw
```

This command displays information about the video without downloading it. If you see video details, your installation is working correctly!

## Updating ytdl

### If Installed from crates.io

```bash
cargo install ytdl --force
```

The `--force` flag reinstalls the package even if it's already installed, ensuring you get the latest version.

### If Built from Source

```bash
cd rust-yt-downloader
git pull
cargo install --path . --force
```

## Uninstalling

To remove `ytdl`:

```bash
cargo uninstall ytdl
```

This removes the binary but keeps your configuration file. To remove the configuration as well:

```bash
# Linux/macOS
rm -rf ~/.config/rust-yt-downloader/

# Windows (PowerShell)
Remove-Item -Recurse -Force "$env:APPDATA\rust-yt-downloader"
```

## Troubleshooting

### "Command not found: ytdl"

The `~/.cargo/bin` directory may not be in your PATH. Add it to your shell configuration:

**Bash** (`~/.bashrc` or `~/.bash_profile`):
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

**Zsh** (`~/.zshrc`):
```zsh
export PATH="$HOME/.cargo/bin:$PATH"
```

**Fish** (`~/.config/fish/config.fish`):
```fish
set -gx PATH $HOME/.cargo/bin $PATH
```

Then restart your terminal or run `source ~/.bashrc` (or appropriate config file).

### "FFmpeg not found"

If you get FFmpeg errors, verify it's in your PATH:

```bash
which ffmpeg  # Linux/macOS
where ffmpeg  # Windows
```

If not found, revisit the FFmpeg installation section above.

### Compilation Errors

If you encounter compilation errors:

1. **Update Rust:**
   ```bash
   rustup update
   ```

2. **Clean and rebuild:**
   ```bash
   cargo clean
   cargo build --release
   ```

3. **Check Rust version** (must be 1.75.0 or later):
   ```bash
   rustc --version
   ```

## Next Steps

Now that you have `ytdl` installed, check out the [Quick Start Guide](./quick-start.md) to learn how to use it!

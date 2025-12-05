//! Progress tracking and visual feedback using the indicatif crate.
//!
//! This module provides a comprehensive progress tracking system for downloads and long-running
//! operations. It uses the [`indicatif`](https://docs.rs/indicatif) crate to display progress bars
//! and spinners in the terminal with consistent styling across the application.
//!
//! # Overview
//!
//! The module consists of three main components:
//!
//! - **[`ProgressStyles`]** - Centralized progress bar styling with pre-configured templates
//! - **[`DownloadProgress`]** - Single progress bar for tracking individual downloads
//! - **[`MultiDownloadProgress`]** - Container for managing multiple simultaneous progress bars
//!
//! # Progress Bar Types
//!
//! ## Download Progress Bar
//!
//! The download style shows bytes transferred, transfer speed, and estimated time remaining:
//!
//! ```text
//! ⠁ [████████████████████░░░░░░░░] 42.5 MB/100 MB (2.1 MB/s, 27s)
//! ```
//!
//! ## Default Progress Bar
//!
//! The default style shows position, total, and percentage:
//!
//! ```text
//! ⠁ [━━━━━━━━━━━━━━━━━━━━╸─────────] 42/100 (42%)
//! ```
//!
//! ## Spinner
//!
//! Spinners indicate ongoing activity without a known total:
//!
//! ```text
//! ⠁ Fetching video metadata...
//! ```
//!
//! # Examples
//!
//! ## Single Download
//!
//! ```no_run
//! use rust_yt_downloader::progress::DownloadProgress;
//!
//! let progress = DownloadProgress::new(1024 * 1024); // 1 MB
//! progress.set_message("Downloading video.mp4");
//!
//! // Update progress as bytes are received
//! progress.inc(512 * 1024); // 512 KB
//! progress.inc(512 * 1024); // Another 512 KB
//!
//! progress.finish_with_message("Download complete!");
//! ```
//!
//! ## Multiple Parallel Downloads
//!
//! ```no_run
//! use rust_yt_downloader::progress::MultiDownloadProgress;
//!
//! let multi = MultiDownloadProgress::new();
//!
//! let video1 = multi.add_download(5_000_000); // 5 MB
//! let video2 = multi.add_download(3_000_000); // 3 MB
//! let spinner = multi.add_spinner("Processing metadata...");
//!
//! // Each progress bar updates independently
//! video1.inc(1_000_000);
//! video2.inc(500_000);
//!
//! spinner.finish_with_message("Metadata processed");
//! video1.finish_with_message("video1.mp4 ✓");
//! video2.finish_with_message("video2.mp4 ✓");
//! ```
//!
//! ## Using a Spinner for Indeterminate Operations
//!
//! ```no_run
//! use rust_yt_downloader::progress::DownloadProgress;
//!
//! let spinner = DownloadProgress::new_spinner("Analyzing video...");
//! // Perform operation...
//! spinner.finish_with_message("Analysis complete!");
//! ```
//!
//! # Messages Module
//!
//! The [`messages`] module provides consistent terminal output with colored icons:
//!
//! ```no_run
//! use rust_yt_downloader::progress::messages;
//!
//! messages::success("Video downloaded successfully");
//! messages::error("Failed to connect to server");
//! messages::warning("Low disk space");
//! messages::info("Starting download...");
//! messages::downloading("video.mp4");
//! ```

use crate::error::{AppError, AppResult};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;

/// Collection of pre-configured progress bar styles using indicatif templates.
///
/// This struct provides factory methods for creating consistent progress bar styles
/// throughout the application. All styles are stateless and created on-demand.
///
/// # Available Styles
///
/// - **[`download()`](Self::download)** - For file downloads with byte counts and speed
/// - **[`default()`](Self::default)** - For generic progress with counts and percentages
/// - **[`spinner()`](Self::spinner)** - For indeterminate operations
///
/// # Design
///
/// Each style uses the [`ProgressStyle`] template system from indicatif:
///
/// - `{spinner}` - Animated spinner character
/// - `{bar}` - The progress bar itself
/// - `{bytes}/{total_bytes}` - Byte counts with automatic unit conversion (KB, MB, GB)
/// - `{bytes_per_sec}` - Transfer speed
/// - `{eta}` - Estimated time to completion
/// - `{pos}/{len}` - Generic position/length counters
/// - `{percent}` - Percentage complete
/// - `{msg}` - Custom message text
///
/// # Examples
///
/// ```no_run
/// use indicatif::ProgressBar;
/// use rust_yt_downloader::progress::ProgressStyles;
///
/// let bar = ProgressBar::new(1000);
/// bar.set_style(ProgressStyles::download());
/// ```
pub struct ProgressStyles;

impl ProgressStyles {
    /// Creates a progress bar style optimized for file downloads.
    ///
    /// This style displays:
    /// - A green animated spinner
    /// - A 40-character cyan/blue progress bar with block characters (`█▓░`)
    /// - Bytes downloaded vs. total bytes (auto-formatted: B, KB, MB, GB)
    /// - Transfer speed (bytes per second, auto-formatted)
    /// - Estimated time to completion (ETA)
    ///
    /// # Visual Example
    ///
    /// ```text
    /// ⠁ [████████████████████▓░░░░░░░░░░] 42.5 MB/100 MB (2.1 MB/s, 27s)
    /// ```
    ///
    /// # Returns
    ///
    /// A [`ProgressStyle`] configured for download operations.
    ///
    /// # Panics
    ///
    /// This method uses `.unwrap()` on the template parsing. The template is hardcoded
    /// and guaranteed to be valid, so this should never panic in practice.
    pub fn download() -> ProgressStyle {
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .unwrap()
            .progress_chars("█▓░")
    }

    /// Creates a generic progress bar style for count-based operations.
    ///
    /// This style displays:
    /// - A green animated spinner
    /// - A 40-character white/gray progress bar with Unicode characters (`━╸─`)
    /// - Current position vs. total length
    /// - Percentage complete
    ///
    /// # Visual Example
    ///
    /// ```text
    /// ⠁ [━━━━━━━━━━━━━━━━━━━━╸────────────] 42/100 (42%)
    /// ```
    ///
    /// # Use Cases
    ///
    /// - Processing items in a playlist
    /// - Converting multiple files
    /// - Any operation with a known total count
    ///
    /// # Returns
    ///
    /// A [`ProgressStyle`] configured for generic progress tracking.
    ///
    /// # Panics
    ///
    /// This method uses `.unwrap()` on the template parsing. The template is hardcoded
    /// and guaranteed to be valid, so this should never panic in practice.
    pub fn default() -> ProgressStyle {
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.white/gray}] {pos}/{len} ({percent}%)")
            .unwrap()
            .progress_chars("━╸─")
    }

    /// Creates an indeterminate spinner style for operations without known progress.
    ///
    /// This style displays:
    /// - A cyan animated spinner
    /// - A custom message
    ///
    /// # Visual Example
    ///
    /// ```text
    /// ⠁ Fetching video metadata...
    /// ```
    ///
    /// # Use Cases
    ///
    /// - Fetching remote data
    /// - Waiting for external processes
    /// - Any operation where the total work is unknown
    ///
    /// # Returns
    ///
    /// A [`ProgressStyle`] configured for spinner display.
    ///
    /// # Panics
    ///
    /// This method uses `.unwrap()` on the template parsing. The template is hardcoded
    /// and guaranteed to be valid, so this should never panic in practice.
    pub fn spinner() -> ProgressStyle {
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
    }
}

/// A wrapper around indicatif's [`ProgressBar`] for tracking download progress.
///
/// This struct provides a simplified interface for common progress bar operations
/// and automatically applies the appropriate styling. It can be used for both
/// determinate progress (with a known total) and indeterminate progress (spinners).
///
/// # Thread Safety
///
/// The underlying [`ProgressBar`] from indicatif is thread-safe and can be cloned
/// and shared across threads. All update methods take `&self` rather than `&mut self`,
/// allowing concurrent updates.
///
/// # Examples
///
/// ## Determinate Progress (Known Total)
///
/// ```no_run
/// use rust_yt_downloader::progress::DownloadProgress;
///
/// let progress = DownloadProgress::new(1_000_000); // 1 MB total
/// progress.set_message("Downloading video.mp4");
///
/// // Simulate downloading in chunks
/// for _ in 0..10 {
///     progress.inc(100_000); // 100 KB per chunk
/// }
///
/// progress.finish_with_message("Download complete!");
/// ```
///
/// ## Indeterminate Progress (Spinner)
///
/// ```no_run
/// use rust_yt_downloader::progress::DownloadProgress;
///
/// let spinner = DownloadProgress::new_spinner("Fetching metadata...");
/// // Perform operation...
/// spinner.finish_with_message("Metadata fetched!");
/// ```
pub struct DownloadProgress {
    bar: ProgressBar,
}

impl DownloadProgress {
    /// Creates a new progress bar with a known total size.
    ///
    /// The progress bar is automatically styled with [`ProgressStyles::download()`],
    /// which displays bytes transferred, transfer speed, and ETA.
    ///
    /// # Parameters
    ///
    /// - `total_size`: The total number of bytes to download (or total units of work)
    ///
    /// # Returns
    ///
    /// A new [`DownloadProgress`] instance initialized at position 0.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::progress::DownloadProgress;
    ///
    /// let progress = DownloadProgress::new(5_000_000); // 5 MB
    /// ```
    pub fn new(total_size: u64) -> Self {
        let bar = ProgressBar::new(total_size);
        bar.set_style(ProgressStyles::download());

        Self { bar }
    }

    /// Creates a new indeterminate spinner with an initial message.
    ///
    /// Spinners are used when the total work is unknown. The spinner automatically
    /// ticks at 100ms intervals to provide visual feedback that work is ongoing.
    ///
    /// # Parameters
    ///
    /// - `message`: The initial message to display next to the spinner
    ///
    /// # Returns
    ///
    /// A new [`DownloadProgress`] instance configured as a spinner.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::progress::DownloadProgress;
    ///
    /// let spinner = DownloadProgress::new_spinner("Analyzing video...");
    /// spinner.set_message("Still analyzing...");
    /// spinner.finish_with_message("Analysis complete!");
    /// ```
    pub fn new_spinner(message: &str) -> Self {
        let bar = ProgressBar::new_spinner();
        bar.set_style(ProgressStyles::spinner());
        bar.set_message(message.to_string());
        bar.enable_steady_tick(Duration::from_millis(100));

        Self { bar }
    }

    /// Sets the progress bar to an absolute position.
    ///
    /// This method directly sets the position counter, rather than incrementing it.
    /// Useful when you know the exact current position (e.g., resuming a download).
    ///
    /// # Parameters
    ///
    /// - `pos`: The new absolute position (typically bytes downloaded)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::progress::DownloadProgress;
    ///
    /// let progress = DownloadProgress::new(1_000_000);
    /// progress.set_position(500_000); // Set to 50% complete
    /// ```
    pub fn set_position(&self, pos: u64) {
        self.bar.set_position(pos);
    }

    /// Increments the progress bar by a specified amount.
    ///
    /// This is the preferred method for updating progress during downloads, as it
    /// allows indicatif to calculate transfer speed and ETA accurately.
    ///
    /// # Parameters
    ///
    /// - `delta`: The amount to increment (typically bytes received in this chunk)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::progress::DownloadProgress;
    ///
    /// let progress = DownloadProgress::new(1_000_000);
    ///
    /// // Update as chunks are received
    /// progress.inc(10_000); // Received 10 KB
    /// progress.inc(15_000); // Received another 15 KB
    /// ```
    pub fn inc(&self, delta: u64) {
        self.bar.inc(delta);
    }

    /// Updates the message displayed with the progress bar.
    ///
    /// This can be called multiple times to show dynamic status updates
    /// (e.g., changing filenames during multi-file downloads).
    ///
    /// # Parameters
    ///
    /// - `msg`: The new message to display
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::progress::DownloadProgress;
    ///
    /// let progress = DownloadProgress::new(1_000_000);
    /// progress.set_message("Downloading video.mp4");
    /// // Later...
    /// progress.set_message("video.mp4 - 50% complete");
    /// ```
    pub fn set_message(&self, msg: &str) {
        self.bar.set_message(msg.to_string());
    }

    /// Finishes the progress bar, leaving it visible with its final state.
    ///
    /// The bar will show the final position and stop animating. The cursor
    /// will move to the next line for subsequent output.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::progress::DownloadProgress;
    ///
    /// let progress = DownloadProgress::new(1_000_000);
    /// progress.set_position(1_000_000);
    /// progress.finish(); // Shows completed bar
    /// ```
    pub fn finish(&self) {
        self.bar.finish();
    }

    /// Finishes the progress bar and displays a final message.
    ///
    /// This is the recommended way to complete a progress bar, as it provides
    /// clear feedback about the final status.
    ///
    /// # Parameters
    ///
    /// - `msg`: The final message to display (e.g., "Download complete!")
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::progress::DownloadProgress;
    ///
    /// let progress = DownloadProgress::new(1_000_000);
    /// progress.set_position(1_000_000);
    /// progress.finish_with_message("video.mp4 downloaded successfully ✓");
    /// ```
    pub fn finish_with_message(&self, msg: &str) {
        self.bar.finish_with_message(msg.to_string());
    }

    /// Finishes the progress bar and removes it from the terminal.
    ///
    /// Use this when you want to hide the progress bar after completion,
    /// rather than leaving it visible. Useful for temporary status indicators.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::progress::DownloadProgress;
    ///
    /// let spinner = DownloadProgress::new_spinner("Processing...");
    /// // Do work...
    /// spinner.finish_and_clear(); // Progress bar disappears
    /// ```
    pub fn finish_and_clear(&self) {
        self.bar.finish_and_clear();
    }

    /// Abandons the progress bar with an error message.
    ///
    /// This marks the progress as incomplete (failed) and displays an error message.
    /// The bar remains visible but stops updating. Typically used when an operation
    /// fails partway through.
    ///
    /// # Parameters
    ///
    /// - `msg`: The error or cancellation message to display
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::progress::DownloadProgress;
    ///
    /// let progress = DownloadProgress::new(1_000_000);
    /// progress.set_position(500_000);
    /// // Error occurs...
    /// progress.abandon_with_message("Download failed: connection lost");
    /// ```
    pub fn abandon_with_message(&self, msg: &str) {
        self.bar.abandon_with_message(msg.to_string());
    }

    /// Returns a reference to the underlying indicatif [`ProgressBar`].
    ///
    /// This provides access to advanced indicatif features not exposed by
    /// the simplified [`DownloadProgress`] API, such as:
    /// - Cloning for use across threads
    /// - Custom position/length manipulation
    /// - State queries (is_finished, position, length, etc.)
    ///
    /// # Returns
    ///
    /// A reference to the internal [`ProgressBar`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::progress::DownloadProgress;
    ///
    /// let progress = DownloadProgress::new(1_000_000);
    /// let bar = progress.inner();
    ///
    /// // Access advanced features
    /// println!("Current position: {}", bar.position());
    /// println!("Is finished: {}", bar.is_finished());
    /// ```
    pub fn inner(&self) -> &ProgressBar {
        &self.bar
    }
}

/// Container for managing multiple simultaneous progress bars.
///
/// This struct wraps indicatif's [`MultiProgress`] to coordinate multiple progress bars
/// that update concurrently. It's essential for displaying parallel downloads or batch
/// operations where each item has its own progress indicator.
///
/// # How It Works
///
/// [`MultiProgress`] acts as a container that:
/// 1. Manages the terminal rendering for multiple progress bars
/// 2. Ensures bars don't overwrite each other
/// 3. Keeps completed bars visible while active ones continue updating
/// 4. Handles terminal resizing and scrolling automatically
///
/// # Thread Safety
///
/// Like [`DownloadProgress`], this struct is thread-safe and can be shared across threads.
/// Each individual progress bar can be updated independently from different threads.
///
/// # Visual Example
///
/// ```text
/// ⠁ [████████████░░░░░░░░░░░░] 10 MB/20 MB (1.5 MB/s, 6s)   <- video1.mp4
/// ⠁ [██████░░░░░░░░░░░░░░░░░░] 5 MB/15 MB (800 KB/s, 12s)  <- video2.mp4
/// ⠁ Processing thumbnails...                                <- spinner
/// ```
///
/// # Examples
///
/// ## Parallel Downloads
///
/// ```no_run
/// use rust_yt_downloader::progress::MultiDownloadProgress;
/// use std::thread;
///
/// let multi = MultiDownloadProgress::new();
///
/// // Create multiple progress bars
/// let p1 = multi.add_download(5_000_000);
/// let p2 = multi.add_download(3_000_000);
/// let p3 = multi.add_download(8_000_000);
///
/// p1.set_message("video1.mp4");
/// p2.set_message("video2.mp4");
/// p3.set_message("video3.mp4");
///
/// // Each bar can be updated independently
/// p1.inc(1_000_000);
/// p2.inc(500_000);
/// p3.inc(2_000_000);
///
/// // Complete in any order
/// p2.finish_with_message("video2.mp4 ✓");
/// p1.finish_with_message("video1.mp4 ✓");
/// p3.finish_with_message("video3.mp4 ✓");
/// ```
///
/// ## Mixed Progress Types
///
/// ```no_run
/// use rust_yt_downloader::progress::MultiDownloadProgress;
///
/// let multi = MultiDownloadProgress::new();
///
/// let spinner = multi.add_spinner("Fetching playlist info...");
/// spinner.finish_with_message("Playlist info fetched ✓");
///
/// let download1 = multi.add_download(10_000_000);
/// let download2 = multi.add_download(15_000_000);
///
/// download1.set_message("video1.mp4");
/// download2.set_message("video2.mp4");
/// // ... update downloads
/// ```
pub struct MultiDownloadProgress {
    multi: MultiProgress,
}

impl MultiDownloadProgress {
    /// Creates a new container for multiple progress bars.
    ///
    /// The container starts empty. Progress bars are added using
    /// [`add_download()`](Self::add_download) or [`add_spinner()`](Self::add_spinner).
    ///
    /// # Returns
    ///
    /// A new [`MultiDownloadProgress`] instance ready to accept progress bars.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::progress::MultiDownloadProgress;
    ///
    /// let multi = MultiDownloadProgress::new();
    /// let p1 = multi.add_download(1_000_000);
    /// let p2 = multi.add_download(2_000_000);
    /// ```
    pub fn new() -> Self {
        Self {
            multi: MultiProgress::new(),
        }
    }

    /// Adds a new download progress bar to the container.
    ///
    /// The progress bar is automatically styled with the download template and
    /// added to the multi-progress display. It will appear below any existing
    /// progress bars.
    ///
    /// # Parameters
    ///
    /// - `total_size`: The total number of bytes for this download
    ///
    /// # Returns
    ///
    /// A new [`DownloadProgress`] instance that updates within this container.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::progress::MultiDownloadProgress;
    ///
    /// let multi = MultiDownloadProgress::new();
    /// let progress = multi.add_download(5_000_000);
    /// progress.set_message("video.mp4");
    /// progress.inc(1_000_000);
    /// ```
    pub fn add_download(&self, total_size: u64) -> DownloadProgress {
        let bar = ProgressBar::new(total_size);
        bar.set_style(ProgressStyles::download());
        let bar = self.multi.add(bar);

        DownloadProgress { bar }
    }

    /// Adds a new spinner to the container.
    ///
    /// Spinners are useful for showing indeterminate progress alongside
    /// determinate progress bars (e.g., "Processing metadata..." while
    /// downloads are in progress).
    ///
    /// # Parameters
    ///
    /// - `message`: The initial message to display with the spinner
    ///
    /// # Returns
    ///
    /// A new [`DownloadProgress`] instance configured as a spinner.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::progress::MultiDownloadProgress;
    ///
    /// let multi = MultiDownloadProgress::new();
    /// let spinner = multi.add_spinner("Analyzing playlist...");
    /// // ... perform analysis
    /// spinner.finish_with_message("Analysis complete ✓");
    /// ```
    pub fn add_spinner(&self, message: &str) -> DownloadProgress {
        let bar = ProgressBar::new_spinner();
        bar.set_style(ProgressStyles::spinner());
        bar.set_message(message.to_string());
        bar.enable_steady_tick(Duration::from_millis(100));
        let bar = self.multi.add(bar);

        DownloadProgress { bar }
    }
}

impl Default for MultiDownloadProgress {
    fn default() -> Self {
        Self::new()
    }
}

/// Consistent terminal messaging with colored icons.
///
/// This module provides utility functions for displaying user-facing messages
/// with consistent styling and visual indicators. Each message type has a
/// distinctive colored icon to help users quickly identify the message category.
///
/// # Message Types
///
/// - **[`success()`]** - Green checkmark (✓) for successful operations
/// - **[`error()`]** - Red cross (✗) for errors (printed to stderr)
/// - **[`warning()`]** - Yellow warning (⚠) for potential issues
/// - **[`info()`]** - Blue info (ℹ) for informational messages
/// - **[`downloading()`]** - Cyan arrow (↓) for download notifications
///
/// # Design Philosophy
///
/// These functions complement progress bars by providing discrete, one-line
/// status updates that don't interfere with ongoing progress displays. They're
/// particularly useful for:
/// - Initial status messages before starting a progress bar
/// - Final completion messages after finishing a progress bar
/// - Important notifications that need to stand out
/// - Error reporting during batch operations
///
/// # Examples
///
/// ## Basic Usage
///
/// ```no_run
/// use rust_yt_downloader::progress::messages;
///
/// messages::info("Starting download process...");
/// messages::downloading("video.mp4");
/// // ... download happens ...
/// messages::success("Download completed successfully");
/// ```
///
/// ## Error Handling
///
/// ```no_run
/// use rust_yt_downloader::progress::messages;
///
/// messages::warning("Disk space is running low");
///
/// if let Err(e) = download_video() {
///     messages::error(&format!("Download failed: {}", e));
/// }
/// # fn download_video() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
/// ```
///
/// ## Batch Operations
///
/// ```no_run
/// use rust_yt_downloader::progress::messages;
///
/// # struct Video { filename: String }
/// # let videos: Vec<Video> = vec![];
/// messages::info("Processing playlist with 10 videos");
///
/// for video in videos {
///     messages::downloading(&video.filename);
///     // ... download video ...
///     messages::success(&format!("{} downloaded", video.filename));
/// }
/// ```
pub mod messages {
    use colored::Colorize;

    /// Prints a success message with a green checkmark icon.
    ///
    /// Use this to confirm successful completion of operations. The message
    /// is printed to stdout with a bold green checkmark prefix.
    ///
    /// # Parameters
    ///
    /// - `msg`: The success message to display
    ///
    /// # Visual Output
    ///
    /// ```text
    /// ✓ Video downloaded successfully
    /// ```
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::progress::messages;
    ///
    /// messages::success("Configuration saved");
    /// messages::success("All 10 videos downloaded successfully");
    /// ```
    pub fn success(msg: &str) {
        println!("{} {}", "✓".green().bold(), msg);
    }

    /// Prints an error message with a red cross icon to stderr.
    ///
    /// Use this for error conditions and failures. The message is printed to
    /// stderr (not stdout) to allow proper error stream handling in scripts
    /// and pipelines.
    ///
    /// # Parameters
    ///
    /// - `msg`: The error message to display
    ///
    /// # Visual Output
    ///
    /// ```text
    /// ✗ Failed to download video: connection timeout
    /// ```
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::progress::messages;
    ///
    /// # let url = "invalid";
    /// messages::error("Network connection lost");
    /// messages::error(&format!("Invalid URL: {}", url));
    /// ```
    pub fn error(msg: &str) {
        eprintln!("{} {}", "✗".red().bold(), msg);
    }

    /// Prints a warning message with a yellow warning icon.
    ///
    /// Use this for non-critical issues that the user should be aware of
    /// but don't prevent the operation from completing. Printed to stdout.
    ///
    /// # Parameters
    ///
    /// - `msg`: The warning message to display
    ///
    /// # Visual Output
    ///
    /// ```text
    /// ⚠ Low disk space: less than 1GB remaining
    /// ```
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::progress::messages;
    ///
    /// messages::warning("Download quality limited by available formats");
    /// messages::warning("Configuration file not found, using defaults");
    /// ```
    pub fn warning(msg: &str) {
        println!("{} {}", "⚠".yellow().bold(), msg);
    }

    /// Prints an informational message with a blue info icon.
    ///
    /// Use this for general status updates and informational messages that
    /// help the user understand what's happening. Printed to stdout.
    ///
    /// # Parameters
    ///
    /// - `msg`: The informational message to display
    ///
    /// # Visual Output
    ///
    /// ```text
    /// ℹ Using configuration from ~/.config/rust-yt-downloader/config.toml
    /// ```
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::progress::messages;
    ///
    /// # let count = 5;
    /// messages::info("Fetching playlist metadata...");
    /// messages::info(&format!("Found {} videos in playlist", count));
    /// ```
    pub fn info(msg: &str) {
        println!("{} {}", "ℹ".blue().bold(), msg);
    }

    /// Prints a download notification with a cyan downward arrow icon.
    ///
    /// Use this to announce when a download is starting. The arrow icon
    /// visually indicates the download direction. Printed to stdout.
    ///
    /// # Parameters
    ///
    /// - `filename`: The name of the file being downloaded
    ///
    /// # Visual Output
    ///
    /// ```text
    /// ↓ video.mp4
    /// ```
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_yt_downloader::progress::messages;
    ///
    /// messages::downloading("my_video.mp4");
    /// messages::downloading("playlist_item_01.mp4");
    /// ```
    pub fn downloading(filename: &str) {
        println!("{} {}", "↓".cyan().bold(), filename);
    }
}

// ==================================================
//          UNITARY TESTS
// ==================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ============== ProgressStyles Tests ==============

    #[test]
    fn test_download_style_creation() {
        let style = ProgressStyles::download();
        assert!(true);
    }

    #[test]
    fn test_default_style_creation() {
        let style = ProgressStyles::default();
        assert!(true);
    }

    #[test]
    fn test_spinner_style_creation() {
        let style = ProgressStyles::spinner();
        assert!(true);
    }

    // ============== DownloadProgress Tests ==============

    #[test]
    fn test_download_progress_new() {
        let progress = DownloadProgress::new(1000);
        assert_eq!(progress.bar.length(), Some(1000));
    }

    #[test]
    fn test_download_progress_new_zero_size() {
        let progress = DownloadProgress::new(0);
        assert_eq!(progress.bar.length(), Some(0));
    }

    #[test]
    fn test_download_progress_new_large_size() {
        let progress = DownloadProgress::new(1024 * 1024 * 1024);
        assert_eq!(progress.bar.length(), Some(1024 * 1024 * 1024));
    }

    #[test]
    fn test_download_progress_new_spinner() {
        let progress = DownloadProgress::new_spinner("Loading...");
        assert!(progress.bar.length().is_none());
    }

    #[test]
    fn test_download_progress_set_position() {
        let progress = DownloadProgress::new(100);
        progress.set_position(50);
        assert_eq!(progress.bar.position(), 50);
    }

    #[test]
    fn test_download_progress_set_position_to_max() {
        let progress = DownloadProgress::new(100);
        progress.set_position(100);
        assert_eq!(progress.bar.position(), 100);
    }

    #[test]
    fn test_download_progress_set_position_beyond_max() {
        let progress = DownloadProgress::new(100);
        progress.set_position(150);
        assert_eq!(progress.bar.position(), 150);
    }

    #[test]
    fn test_download_progress_inc() {
        let progress = DownloadProgress::new(100);
        progress.inc(10);
        assert_eq!(progress.bar.position(), 10);
    }

    #[test]
    fn test_download_progress_inc_multiple() {
        let progress = DownloadProgress::new(100);
        progress.inc(10);
        progress.inc(20);
        progress.inc(30);
        assert_eq!(progress.bar.position(), 60);
    }

    #[test]
    fn test_download_progress_inc_zero() {
        let progress = DownloadProgress::new(100);
        progress.inc(0);
        assert_eq!(progress.bar.position(), 0);
    }

    #[test]
    fn test_download_progress_set_message() {
        let progress = DownloadProgress::new(100);
        progress.set_message("Downloading video.mp4");
        assert!(true);
    }

    #[test]
    fn test_download_progress_set_message_empty() {
        let progress = DownloadProgress::new(100);
        progress.set_message("");
        assert!(true);
    }

    #[test]
    fn test_download_progress_set_message_unicode() {
        let progress = DownloadProgress::new(100);
        progress.set_message("Baixando vídeo 🎬");
        assert!(true);
    }

    #[test]
    fn test_download_progress_finish() {
        let progress = DownloadProgress::new(100);
        progress.set_position(50);
        progress.finish();
        assert!(progress.bar.is_finished());
    }

    #[test]
    fn test_download_progress_finish_with_message() {
        let progress = DownloadProgress::new(100);
        progress.finish_with_message("Done!");
        assert!(progress.bar.is_finished());
    }

    #[test]
    fn test_download_progress_finish_and_clear() {
        let progress = DownloadProgress::new(100);
        progress.finish_and_clear();
        assert!(progress.bar.is_finished());
    }

    #[test]
    fn test_download_progress_abandon_with_message() {
        let progress = DownloadProgress::new(100);
        progress.abandon_with_message("Error occurred");
        assert!(progress.bar.is_finished());
    }

    #[test]
    fn test_download_progress_inner() {
        let progress = DownloadProgress::new(100);
        let inner = progress.inner();
        assert_eq!(inner.length(), Some(100));
    }

    #[test]
    fn test_download_progress_workflow() {
        let progress = DownloadProgress::new(100);

        progress.set_message("Starting download...");
        assert_eq!(progress.bar.position(), 0);

        progress.inc(25);
        assert_eq!(progress.bar.position(), 25);

        progress.inc(25);
        assert_eq!(progress.bar.position(), 50);

        progress.set_message("Halfway there...");

        progress.set_position(100);
        assert_eq!(progress.bar.position(), 100);

        progress.finish_with_message("Download complete!");
        assert!(progress.bar.is_finished());
    }

    // ============== MultiDownloadProgress Tests ==============

    #[test]
    fn test_multi_download_progress_new() {
        let multi = MultiDownloadProgress::new();
        assert!(true);
    }

    #[test]
    fn test_multi_download_progress_default() {
        let multi = MultiDownloadProgress::default();
        assert!(true);
    }

    #[test]
    fn test_multi_download_progress_add_download() {
        let multi = MultiDownloadProgress::new();
        let progress = multi.add_download(1000);
        assert_eq!(progress.bar.length(), Some(1000));
    }

    #[test]
    fn test_multi_download_progress_add_multiple_downloads() {
        let multi = MultiDownloadProgress::new();

        let p1 = multi.add_download(1000);
        let p2 = multi.add_download(2000);
        let p3 = multi.add_download(3000);

        assert_eq!(p1.bar.length(), Some(1000));
        assert_eq!(p2.bar.length(), Some(2000));
        assert_eq!(p3.bar.length(), Some(3000));
    }

    #[test]
    fn test_multi_download_progress_add_spinner() {
        let multi = MultiDownloadProgress::new();
        let spinner = multi.add_spinner("Processing...");
        assert!(spinner.bar.length().is_none());
    }

    #[test]
    fn test_multi_download_progress_mixed() {
        let multi = MultiDownloadProgress::new();

        let download = multi.add_download(5000);
        let spinner = multi.add_spinner("Analyzing...");

        assert_eq!(download.bar.length(), Some(5000));
        assert!(spinner.bar.length().is_none());
    }

    #[test]
    fn test_multi_download_progress_independent_updates() {
        let multi = MultiDownloadProgress::new();

        let p1 = multi.add_download(100);
        let p2 = multi.add_download(100);

        p1.set_position(50);
        p2.set_position(75);

        assert_eq!(p1.bar.position(), 50);
        assert_eq!(p2.bar.position(), 75);
    }

    #[test]
    fn test_multi_download_progress_independent_finish() {
        let multi = MultiDownloadProgress::new();

        let p1 = multi.add_download(100);
        let p2 = multi.add_download(100);

        p1.finish();

        assert!(p1.bar.is_finished());
        assert!(!p2.bar.is_finished());
    }

    #[test]
    fn test_multi_download_progress_workflow() {
        let multi = MultiDownloadProgress::new();

        let video1 = multi.add_download(1000);
        let video2 = multi.add_download(2000);
        let video3 = multi.add_download(1500);

        video1.inc(500);
        video2.inc(1000);
        video3.inc(750);

        assert_eq!(video1.bar.position(), 500);
        assert_eq!(video2.bar.position(), 1000);
        assert_eq!(video3.bar.position(), 750);

        video1.set_position(1000);
        video1.finish_with_message("video1.mp4 ✓");
        assert!(video1.bar.is_finished());

        video3.set_position(1500);
        video3.finish_with_message("video3.mp4 ✓");
        assert!(video3.bar.is_finished());

        video2.set_position(2000);
        video2.finish_with_message("video2.mp4 ✓");
        assert!(video2.bar.is_finished());
    }

    // ============== Messages Module Tests ==============

    #[test]
    fn test_messages_success() {
        messages::success("Operation completed");
        assert!(true);
    }

    #[test]
    fn test_messages_error() {
        messages::error("Something went wrong");
        assert!(true);
    }

    #[test]
    fn test_messages_warning() {
        messages::warning("This might be a problem");
        assert!(true);
    }

    #[test]
    fn test_messages_info() {
        messages::info("Here's some information");
        assert!(true);
    }

    #[test]
    fn test_messages_downloading() {
        messages::downloading("video.mp4");
        assert!(true);
    }

    #[test]
    fn test_messages_with_empty_string() {
        messages::success("");
        messages::error("");
        messages::warning("");
        messages::info("");
        messages::downloading("");
        assert!(true);
    }

    #[test]
    fn test_messages_with_unicode() {
        messages::success("Vídeo baixado com sucesso! 🎉");
        messages::error("Erro ao baixar 😢");
        messages::warning("Atenção: arquivo grande ⚠️");
        messages::info("Informação: 日本語テスト");
        messages::downloading("música_brasileira.mp3");
        assert!(true);
    }

    #[test]
    fn test_messages_with_special_characters() {
        messages::success("File: /path/to/file.mp4");
        messages::error("Error: \"file not found\"");
        messages::warning("Warning: 100% disk usage");
        messages::info("Info: <tag> & </tag>");
        assert!(true);
    }

    // ============== Edge Cases ==============

    #[test]
    fn test_progress_rapid_updates() {
        let progress = DownloadProgress::new(10000);

        for i in 0..10000 {
            progress.set_position(i);
        }

        assert_eq!(progress.bar.position(), 9999);
        progress.finish();
    }

    #[test]
    fn test_progress_large_increments() {
        let progress = DownloadProgress::new(1_000_000_000);

        progress.inc(500_000_000);
        assert_eq!(progress.bar.position(), 500_000_000);

        progress.inc(500_000_000);
        assert_eq!(progress.bar.position(), 1_000_000_000);
    }

    #[test]
    fn test_spinner_lifecycle() {
        let spinner = DownloadProgress::new_spinner("Loading...");

        spinner.set_message("Still loading...");
        spinner.set_message("Almost done...");
        spinner.finish_with_message("Complete!");

        assert!(spinner.bar.is_finished());
    }
}

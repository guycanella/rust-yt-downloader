use std::path::PathBuf;
use std::process::{Command, Output};
use tempfile::TempDir;

/// URL of a short video for testing (Creative Commons)
/// "video cortos 10 segundos" - 10 seconds
pub const TEST_VIDEO_SHORT: &str = "https://www.youtube.com/watch?v=E-DDmIhL4IM";

/// URL of an invalid video
pub const TEST_VIDEO_INVALID: &str = "https://www.youtube.com/watch?v=invalid12345";

/// Runs the ytdl binary with the provided arguments
pub fn run_ytdl(args: &[&str]) -> Output {
    let binary = get_binary_path();
    
    Command::new(binary)
        .args(args)
        .output()
        .expect("Failed to execute ytdl")
}

/// Runs ytdl and returns stdout as a string
pub fn run_ytdl_stdout(args: &[&str]) -> String {
    let output = run_ytdl(args);
    String::from_utf8_lossy(&output.stdout).to_string()
}

/// Runs ytdl and returns stderr as a string
pub fn run_ytdl_stderr(args: &[&str]) -> String {
    let output = run_ytdl(args);
    String::from_utf8_lossy(&output.stderr).to_string()
}

///  Returns the path to the compiled binary
pub fn get_binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("ytdl");
    path
}

/// Creates a temporary directory for tests
pub fn create_temp_dir() -> TempDir {
    TempDir::new().expect("Failed to create temp directory")
}

/// Checks if yt-dlp is available
pub fn ytdlp_available() -> bool {
    Command::new("yt-dlp")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Macro to skip test if yt-dlp is not available
#[macro_export]
macro_rules! skip_if_no_ytdlp {
    () => {
        if !common::ytdlp_available() {
            eprintln!("Skipping test: yt-dlp not available");
            return;
        }
    };
}

/// Macro to skip network tests in CI (optional)
#[macro_export]
macro_rules! skip_if_ci {
    () => {
        if std::env::var("CI").is_ok() {
            eprintln!("Skipping network test in CI");
            return;
        }
    };
}
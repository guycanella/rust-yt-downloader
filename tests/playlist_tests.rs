mod common;

use common::{run_ytdl, ytdlp_available, create_temp_dir};
use std::fs;

// ============== Helper ==============

fn skip_if_no_ytdlp() -> bool {
    if !ytdlp_available() {
        eprintln!("Skipping test: yt-dlp not available");
        true
    } else {
        false
    }
}

const TEST_PLAYLIST_SHORT: &str = "https://www.youtube.com/playlist?list=PLzMcBGfZo4-mP7qA9cagf68V06UM5z1ka";

// ============== Playlist Help Tests ==============

#[test]
fn test_playlist_help() {
    let output = run_ytdl(&["playlist", "--help"]);
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("playlist") || stdout.contains("URL"));
}

#[test]
fn test_playlist_missing_url() {
    let output = run_ytdl(&["playlist"]);
    
    assert!(!output.status.success());
}

// ============== Playlist Download Tests ==============

#[test]
#[ignore]
fn test_playlist_download_single_video() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&[
        "playlist",
        common::TEST_VIDEO_SHORT,
        "-o", &output_path,
        "-s"
    ]);

    assert!(output.status.success());

    let files: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    assert!(!files.is_empty(), "No files downloaded");
}

#[test]
#[ignore]
fn test_playlist_download_with_quality() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&[
        "playlist",
        common::TEST_VIDEO_SHORT,
        "-o", &output_path,
        "-q", "480p",
        "-s"
    ]);

    assert!(output.status.success());
}

#[test]
#[ignore]
fn test_playlist_audio_only() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&[
        "playlist",
        common::TEST_VIDEO_SHORT,
        "-o", &output_path,
        "--audio-only",
        "-s"
    ]);

    assert!(output.status.success());

    let files: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    assert!(!files.is_empty());

    let file = &files[0];
    let ext = file.path().extension().unwrap().to_string_lossy().to_string();
    assert!(
        ext == "mp3" || ext == "m4a" || ext == "opus" || ext == "wav" || ext == "flac",
        "Expected audio file, got: {}",
        ext
    );
}

#[test]
#[ignore]
fn test_playlist_multiple_urls() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&[
        "playlist",
        common::TEST_VIDEO_SHORT,
        "https://www.youtube.com/watch?v=jNQXAC9IVRw",
        "-o", &output_path,
        "-s"
    ]);

    assert!(output.status.success());

    let files: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    assert!(files.len() >= 2, "Expected at least 2 files, got {}", files.len());
}

#[test]
fn test_playlist_invalid_url() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&[
        "playlist",
        "https://invalid-url.com",
        "-o", &output_path
    ]);

    assert!(!output.status.success());
}

#[test]
fn test_playlist_creates_output_dir() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let nested_path = temp_dir.path().join("playlist").join("output");
    let output_path = nested_path.to_string_lossy().to_string();

    let output = run_ytdl(&[
        "playlist",
        common::TEST_VIDEO_SHORT,
        "-o", &output_path,
        "-s"
    ]);

    assert!(nested_path.exists() || output.status.success());
}
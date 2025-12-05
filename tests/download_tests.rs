mod common;

use common::{create_temp_dir, run_ytdl, ytdlp_available, TEST_VIDEO_SHORT};
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

// ============== Download Command Tests ==============

#[test]
fn test_download_video() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&["download", TEST_VIDEO_SHORT, "-o", &output_path, "-s"]);

    assert!(
        output.status.success(),
        "Download failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let files: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    assert!(!files.is_empty(), "No files downloaded");

    let file = &files[0];
    let ext = file
        .path()
        .extension()
        .unwrap()
        .to_string_lossy()
        .to_string();
    assert!(
        ext == "mp4" || ext == "mkv" || ext == "webm",
        "Unexpected file extension: {}",
        ext
    );
}

#[test]
fn test_download_with_quality_720p() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&[
        "download",
        TEST_VIDEO_SHORT,
        "-o",
        &output_path,
        "-q",
        "720p",
        "-s",
    ]);

    assert!(output.status.success());

    let files: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    assert!(!files.is_empty());
}

#[test]
fn test_download_with_format_mkv() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&[
        "download",
        TEST_VIDEO_SHORT,
        "-o",
        &output_path,
        "-f",
        "mkv",
        "-s",
    ]);

    assert!(output.status.success());

    let files: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "mkv")
                .unwrap_or(false)
        })
        .collect();

    assert!(!files.is_empty(), "No MKV file found");
}

#[test]
fn test_download_with_format_webm() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&[
        "download",
        TEST_VIDEO_SHORT,
        "-o",
        &output_path,
        "-f",
        "webm",
        "-s",
    ]);

    assert!(output.status.success());

    let files: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    assert!(!files.is_empty());
}

#[test]
fn test_download_creates_output_dir() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let nested_path = temp_dir.path().join("nested").join("output");
    let output_path = nested_path.to_string_lossy().to_string();

    let output = run_ytdl(&["download", TEST_VIDEO_SHORT, "-o", &output_path, "-s"]);

    assert!(output.status.success());
    assert!(nested_path.exists(), "Nested directory was not created");
}

#[test]
fn test_download_silence_mode() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&["download", TEST_VIDEO_SHORT, "-o", &output_path, "-s"]);

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("[download]"));
}

#[test]
fn test_download_invalid_url() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&["download", "https://invalid-url.com", "-o", &output_path]);

    assert!(!output.status.success());
}

#[test]
fn test_download_worst_quality() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&[
        "download",
        TEST_VIDEO_SHORT,
        "-o",
        &output_path,
        "-q",
        "worst",
        "-s",
    ]);

    assert!(output.status.success());

    let files: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    assert!(!files.is_empty());
}

#[test]
fn test_download_best_quality() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&[
        "download",
        TEST_VIDEO_SHORT,
        "-o",
        &output_path,
        "-q",
        "best",
        "-s",
    ]);

    assert!(output.status.success());

    let files: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    assert!(!files.is_empty());
}

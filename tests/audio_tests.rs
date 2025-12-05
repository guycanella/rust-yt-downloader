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

// ============== Audio Command Tests ==============

#[test]
fn test_audio_download_mp3() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&[
        "audio",
        TEST_VIDEO_SHORT,
        "-o",
        &output_path,
        "-f",
        "mp3",
        "-s",
    ]);

    assert!(
        output.status.success(),
        "Audio download failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let files: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "mp3")
                .unwrap_or(false)
        })
        .collect();

    assert!(!files.is_empty(), "No MP3 file found");
}

#[test]
fn test_audio_download_m4a() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&[
        "audio",
        TEST_VIDEO_SHORT,
        "-o",
        &output_path,
        "-f",
        "m4a",
        "-s",
    ]);

    assert!(output.status.success());

    let files: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "m4a")
                .unwrap_or(false)
        })
        .collect();

    assert!(!files.is_empty(), "No M4A file found");
}

#[test]
fn test_audio_download_opus() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&[
        "audio",
        TEST_VIDEO_SHORT,
        "-o",
        &output_path,
        "-f",
        "opus",
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
fn test_audio_download_wav() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&[
        "audio",
        TEST_VIDEO_SHORT,
        "-o",
        &output_path,
        "-f",
        "wav",
        "-s",
    ]);

    assert!(output.status.success());

    let files: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "wav")
                .unwrap_or(false)
        })
        .collect();

    assert!(!files.is_empty(), "No WAV file found");
}

#[test]
fn test_audio_download_flac() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&[
        "audio",
        TEST_VIDEO_SHORT,
        "-o",
        &output_path,
        "-f",
        "flac",
        "-s",
    ]);

    assert!(output.status.success());

    let files: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "flac")
                .unwrap_or(false)
        })
        .collect();

    assert!(!files.is_empty(), "No FLAC file found");
}

#[test]
fn test_audio_creates_output_dir() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let nested_path = temp_dir.path().join("audio").join("output");
    let output_path = nested_path.to_string_lossy().to_string();

    let output = run_ytdl(&[
        "audio",
        TEST_VIDEO_SHORT,
        "-o",
        &output_path,
        "-f",
        "mp3",
        "-s",
    ]);

    assert!(output.status.success());
    assert!(nested_path.exists(), "Nested directory was not created");
}

#[test]
fn test_audio_silence_mode() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&[
        "audio",
        TEST_VIDEO_SHORT,
        "-o",
        &output_path,
        "-f",
        "mp3",
        "-s",
    ]);

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("[download]"));
}

#[test]
fn test_audio_invalid_url() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&["audio", "https://invalid-url.com", "-o", &output_path]);

    assert!(!output.status.success());
}

#[test]
fn test_audio_default_format() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&["audio", TEST_VIDEO_SHORT, "-o", &output_path, "-s"]);

    assert!(output.status.success());

    let files: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "mp3")
                .unwrap_or(false)
        })
        .collect();

    assert!(!files.is_empty(), "No MP3 file found (default format)");
}

#[test]
fn test_audio_file_not_empty() {
    if skip_if_no_ytdlp() {
        return;
    }

    let temp_dir = create_temp_dir();
    let output_path = temp_dir.path().to_string_lossy().to_string();

    let output = run_ytdl(&[
        "audio",
        TEST_VIDEO_SHORT,
        "-o",
        &output_path,
        "-f",
        "mp3",
        "-s",
    ]);

    assert!(output.status.success());

    let files: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    assert!(!files.is_empty());

    let file_size = fs::metadata(files[0].path()).unwrap().len();
    assert!(file_size > 0, "Downloaded file is empty");
}

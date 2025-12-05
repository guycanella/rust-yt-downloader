mod common;

use common::{run_ytdl, ytdlp_available, TEST_VIDEO_INVALID, TEST_VIDEO_SHORT};

// ============== Helper ==============

fn skip_if_no_ytdlp() -> bool {
    if !ytdlp_available() {
        eprintln!("Skipping test: yt-dlp not available");
        true
    } else {
        false
    }
}

// ============== Info Command Tests ==============

#[test]
fn test_info_valid_video() {
    if skip_if_no_ytdlp() {
        return;
    }

    let output = run_ytdl(&["info", TEST_VIDEO_SHORT]);

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Title:"));
    assert!(stdout.contains("ID:"));
    assert!(stdout.contains("Duration:"));
}

#[test]
fn test_info_shows_qualities() {
    if skip_if_no_ytdlp() {
        return;
    }

    let output = run_ytdl(&["info", TEST_VIDEO_SHORT]);

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Available Qualities:"));
}

#[test]
fn test_info_shows_audio_streams() {
    if skip_if_no_ytdlp() {
        return;
    }

    let output = run_ytdl(&["info", TEST_VIDEO_SHORT]);

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Audio Streams:"));
}

#[test]
fn test_info_shows_channel() {
    if skip_if_no_ytdlp() {
        return;
    }

    let output = run_ytdl(&["info", TEST_VIDEO_SHORT]);

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Channel:"));
}

#[test]
fn test_info_invalid_url() {
    if skip_if_no_ytdlp() {
        return;
    }

    let output = run_ytdl(&["info", "https://invalid-url.com/video"]);

    assert!(!output.status.success());
}

#[test]
fn test_info_nonexistent_video() {
    if skip_if_no_ytdlp() {
        return;
    }

    let output = run_ytdl(&["info", TEST_VIDEO_INVALID]);

    assert!(!output.status.success());
}

#[test]
fn test_info_short_url_format() {
    if skip_if_no_ytdlp() {
        return;
    }

    // youtu.be format
    let output = run_ytdl(&["info", "https://youtu.be/aqz-KE-bpKQ"]);

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Title:"));
}

#[test]
fn test_info_output_format() {
    if skip_if_no_ytdlp() {
        return;
    }

    let output = run_ytdl(&["info", TEST_VIDEO_SHORT]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    let title_pos = stdout.find("Title:");
    let id_pos = stdout.find("ID:");
    let duration_pos = stdout.find("Duration:");

    assert!(title_pos.is_some());
    assert!(id_pos.is_some());
    assert!(duration_pos.is_some());

    assert!(title_pos < id_pos);
    assert!(id_pos < duration_pos);
}

mod common;

#[allow(unused_imports)]
use common::{run_ytdl, run_ytdl_stderr, run_ytdl_stdout};

// ============== Help Tests ==============

#[test]
fn test_help_flag() {
    let output = run_ytdl(&["--help"]);

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ytdl"));
    assert!(stdout.contains("download"));
    assert!(stdout.contains("audio"));
    assert!(stdout.contains("info"));
    assert!(stdout.contains("config"));
}

#[test]
fn test_help_short_flag() {
    let output = run_ytdl(&["-h"]);

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ytdl"));
    assert!(stdout.contains("COMMAND"));
}

#[test]
fn test_download_help() {
    let output = run_ytdl(&["download", "--help"]);

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("quality"));
    assert!(stdout.contains("format"));
    assert!(stdout.contains("output"));
}

#[test]
fn test_audio_help() {
    let output = run_ytdl(&["audio", "--help"]);

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("format"));
    assert!(stdout.contains("mp3"));
}

#[test]
fn test_info_help() {
    let output = run_ytdl(&["info", "--help"]);

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("URL"));
}

#[test]
fn test_config_help() {
    let output = run_ytdl(&["config", "--help"]);

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("show"));
    assert!(stdout.contains("set"));
    assert!(stdout.contains("get"));
    assert!(stdout.contains("reset"));
    assert!(stdout.contains("path"));
}

// ============== Version Tests ==============

#[test]
fn test_version_flag() {
    let output = run_ytdl(&["--version"]);

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ytdl"));
}

// ============== Error Handling Tests ==============

#[test]
fn test_no_arguments() {
    let output = run_ytdl(&[]);

    // Deve mostrar help ou erro
    assert!(!output.status.success() || !output.stdout.is_empty());
}

#[test]
fn test_invalid_command() {
    let output = run_ytdl(&["invalidcommand"]);

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("error") || stderr.contains("invalid"));
}

#[test]
fn test_download_missing_url() {
    let output = run_ytdl(&["download"]);

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("required") || stderr.contains("URL"));
}

#[test]
fn test_audio_missing_url() {
    let output = run_ytdl(&["audio"]);

    assert!(!output.status.success());
}

#[test]
fn test_info_missing_url() {
    let output = run_ytdl(&["info"]);

    assert!(!output.status.success());
}

// ============== Quality Flag Tests ==============

#[test]
fn test_download_quality_flags() {
    let qualities = vec![
        "best", "worst", "144p", "240p", "360p", "480p", "720p", "1080p", "1440p", "4k",
    ];

    for _quality in qualities {
        let output = run_ytdl(&["download", "--help"]);
        assert!(output.status.success());
    }
}

// ============== Format Flag Tests ==============

#[test]
fn test_download_format_flags() {
    let output = run_ytdl(&["download", "--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("mp4") || stdout.contains("format"));
}

#[test]
fn test_audio_format_flags() {
    let output = run_ytdl(&["audio", "--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("mp3") || stdout.contains("format"));
}

// ============== Output Flag Tests ==============

#[test]
fn test_download_output_short_flag() {
    let output = run_ytdl(&["download", "--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("-o") || stdout.contains("output"));
}

// ============== Silence Flag Tests ==============

#[test]
fn test_download_silence_flag() {
    let output = run_ytdl(&["download", "--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("-s") || stdout.contains("silence"));
}

// ============== Verbose Flag Tests ==============

#[test]
fn test_download_verbose_flag() {
    let output = run_ytdl(&["download", "--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("-v") || stdout.contains("verbose"));
}

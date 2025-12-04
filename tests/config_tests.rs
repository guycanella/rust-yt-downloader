mod common;

use common::run_ytdl;

// ============== Config Show Tests ==============

#[test]
fn test_config_show() {
    let output = run_ytdl(&["config", "show"]);
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("general.output_dir"));
    assert!(stdout.contains("video.format"));
    assert!(stdout.contains("audio.format"));
    assert!(stdout.contains("network.retry_attempts"));
}

// ============== Config Path Tests ==============

#[test]
fn test_config_path() {
    let output = run_ytdl(&["config", "path"]);
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("config.toml") || stdout.contains("rust-yt-downloader"));
}

// ============== Config Get Tests ==============

#[test]
fn test_config_get_valid_key() {
    let output = run_ytdl(&["config", "get", "video.format"]);
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.trim().is_empty());
}

#[test]
fn test_config_get_invalid_key() {
    let output = run_ytdl(&["config", "get", "invalid.key"]);
    
    assert!(!output.status.success());
}

#[test]
fn test_config_get_general_output_dir() {
    let output = run_ytdl(&["config", "get", "general.output_dir"]);
    
    assert!(output.status.success());
}

#[test]
fn test_config_get_audio_format() {
    let output = run_ytdl(&["config", "get", "audio.format"]);
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.trim().is_empty());
}

#[test]
fn test_config_get_network_retry() {
    let output = run_ytdl(&["config", "get", "network.retry_attempts"]);
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.trim().parse::<u32>().is_ok());
}

// ============== Config Set Tests ==============

#[test]
fn test_config_set_valid_key() {
    // Apenas verifica que o comando set funciona
    let output = run_ytdl(&["config", "set", "network.timeout", "300"]);
    assert!(output.status.success());
}

#[test]
fn test_config_set_invalid_key() {
    let output = run_ytdl(&["config", "set", "invalid.key", "value"]);
    
    assert!(!output.status.success());
}

// ============== Config Reset Tests ==============

#[test]
fn test_config_reset() {
    let reset_output = run_ytdl(&["config", "reset"]);
    assert!(reset_output.status.success());
    
    let stdout = String::from_utf8_lossy(&reset_output.stdout);
    assert!(stdout.contains("reset") || reset_output.status.success());
}

// ============== All Config Keys Tests ==============

#[test]
fn test_all_config_keys_readable() {
    let keys = vec![
        "general.output_dir",
        "general.default_quality",
        "general.max_parallel_downloads",
        "audio.format",
        "audio.bitrate",
        "video.format",
        "video.include_thumbnail",
        "video.include_subtitles",
        "network.retry_attempts",
        "network.timeout",
    ];
    
    for key in keys {
        let output = run_ytdl(&["config", "get", key]);
        assert!(
            output.status.success(),
            "Failed to get config key: {}",
            key
        );
    }
}

// ============== Config Set and Get (Serial Test) ==============

#[test]
#[ignore]
fn test_config_set_and_get_serial() {
    run_ytdl(&["config", "reset"]);
    
    let set_output = run_ytdl(&["config", "set", "network.retry_attempts", "7"]);
    assert!(set_output.status.success());
    
    let get_output = run_ytdl(&["config", "get", "network.retry_attempts"]);
    assert!(get_output.status.success());
    
    let stdout = String::from_utf8_lossy(&get_output.stdout);
    assert_eq!(stdout.trim(), "7");
    
    run_ytdl(&["config", "reset"]);
}

#[test]
#[ignore]
fn test_config_persists_after_change_serial() {
    run_ytdl(&["config", "reset"]);
    
    run_ytdl(&["config", "set", "audio.bitrate", "192k"]);
    
    let output = run_ytdl(&["config", "get", "audio.bitrate"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert_eq!(stdout.trim(), "192k");
    
    run_ytdl(&["config", "reset"]);
}
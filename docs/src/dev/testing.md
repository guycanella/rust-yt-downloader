# Testing Guide

This document provides comprehensive guidance on testing in the YouTube Downloader project, including test organization, running tests, writing new tests, and CI/CD considerations.

## Overview

The YouTube Downloader has extensive test coverage with:

- **1500+ lines of unit tests** across all modules
- **Inline tests** in each module (`#[cfg(test)] mod tests`)
- **Integration tests** using temporary files and HTTP mocking
- **Async tests** for async/await code using `tokio-test`
- **Organized test structure** with comment headers

## Test Coverage by Module

| Module | Lines of Tests | Coverage Focus |
|--------|----------------|----------------|
| `cli.rs` | 1000+ | Command parsing, enums, edge cases |
| `config.rs` | 500+ | Serialization, get/set, validation |
| `error.rs` | 300+ | Error display, constructors, retryability |
| `utils.rs` | 200+ | Filename sanitization, templating, formatting |
| `youtube/` | 150+ | URL validation, metadata parsing |
| `media/` | 100+ | FFmpeg integration, conversion |

**Total**: ~2250+ lines of tests

## Running Tests

### Basic Test Commands

```bash
# Run all tests
cargo test

# Run tests with output (show println! statements)
cargo test -- --nocapture

# Run tests with detailed output
cargo test -- --verbose

# Run tests in parallel (default)
cargo test

# Run tests sequentially (useful for debugging)
cargo test -- --test-threads=1
```

### Running Specific Tests

#### By Test Name

```bash
# Run all tests with "download" in the name
cargo test download

# Run all tests with "audio" in the name
cargo test audio

# Run a specific test by exact name
cargo test test_download_minimal_args

# Run all tests for VideoQuality enum
cargo test video_quality
```

#### By Module

```bash
# Run all tests in cli.rs
cargo test --lib cli

# Run all tests in config.rs
cargo test --lib config

# Run all tests in error.rs
cargo test --lib error
```

#### By Pattern

```bash
# Run all display/format tests
cargo test display

# Run all constructor tests
cargo test constructor

# Run all validation tests
cargo test validate
```

### Running Integration Tests

Integration tests are located in the `tests/` directory:

```bash
# Run all integration tests
cargo test --test '*'

# Run specific integration test file
cargo test --test integration
```

### Test Output Modes

```bash
# Quiet mode (only failures)
cargo test --quiet

# Show all test names
cargo test -- --list

# Run ignored tests
cargo test -- --ignored

# Run all tests including ignored
cargo test -- --include-ignored
```

## Test Organization

### Inline Test Modules

Each module has a `#[cfg(test)]` test module at the bottom:

```rust
// ============== Test Organization ==============

#[cfg(test)]
mod tests {
    use super::*;

    // ============== Download Command Tests ==============

    #[test]
    fn test_download_minimal_args() {
        // Test implementation
    }

    #[test]
    fn test_download_with_quality() {
        // Test implementation
    }

    // ============== Audio Command Tests ==============

    #[test]
    fn test_audio_format_mp3() {
        // Test implementation
    }

    // ============== VideoQuality Enum Tests ==============

    #[test]
    fn test_video_quality_default() {
        // Test implementation
    }
}
```

### Test Organization by Comment Headers

Tests are grouped with descriptive comment headers:

```rust
// ============== Display/Format Tests ==============
// ============== Helper Constructor Tests ==============
// ============== Config Path Tests ==============
// ============== Serialization Tests ==============
// ============== Validation Tests ==============
```

This makes it easy to navigate and understand test groups.

## Writing New Tests

### Unit Test Template

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name_behavior() {
        // Arrange: Set up test data
        let input = "test data";

        // Act: Execute the function under test
        let result = function_to_test(input);

        // Assert: Verify the result
        assert_eq!(result, expected_value);
    }
}
```

### Async Test Template

For testing async functions:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_function() {
        // Arrange
        let url = "https://youtube.com/watch?v=test123";

        // Act
        let result = async_function(url).await;

        // Assert
        assert!(result.is_ok());
    }
}
```

### Testing Error Cases

```rust
#[test]
fn test_invalid_input_returns_error() {
    let invalid_input = "invalid";

    let result = function_that_validates(invalid_input);

    assert!(result.is_err());
    match result {
        Err(AppError::InvalidArgument { argument, message }) => {
            assert_eq!(argument, "input");
            assert!(message.contains("invalid"));
        }
        _ => panic!("Expected InvalidArgument error"),
    }
}
```

### Testing with Temporary Files

Use the `tempfile` crate for filesystem tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_file_operations() {
        // Create temporary directory
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        // Test file operations
        write_file(&file_path, "test content").unwrap();
        let content = read_file(&file_path).unwrap();

        assert_eq!(content, "test content");

        // Temp directory is automatically cleaned up
    }
}
```

### Testing with HTTP Mocking

Use the `mockito` crate for HTTP tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};

    #[tokio::test]
    async fn test_http_request() {
        // Create mock server
        let _m = mock("GET", "/api/video")
            .with_status(200)
            .with_body(r#"{"title": "Test Video"}"#)
            .create();

        // Test HTTP request against mock server
        let url = format!("{}/api/video", server_url());
        let result = fetch_video_info(&url).await;

        assert!(result.is_ok());
    }
}
```

## Testing Patterns by Module

### cli.rs Tests

**Focus**: Command parsing, argument validation, enum variants

```rust
#[test]
fn test_download_command_parsing() {
    let args = vec!["ytdl", "download", "https://youtube.com/watch?v=abc123"];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        Commands::Download(download_args) => {
            assert_eq!(download_args.common.url, "https://youtube.com/watch?v=abc123");
        }
        _ => panic!("Expected Download command"),
    }
}

#[test]
fn test_video_quality_enum_values() {
    // Test all enum variants
    assert_eq!(VideoQuality::Best.to_string(), "best");
    assert_eq!(VideoQuality::Q1080p.to_string(), "1080p");
}

#[test]
fn test_invalid_command_parsing() {
    let args = vec!["ytdl", "invalid-command"];
    let result = Cli::try_parse_from(args);

    assert!(result.is_err());
}
```

### config.rs Tests

**Focus**: Serialization, deserialization, get/set operations

```rust
#[test]
fn test_config_default_values() {
    let config = Config::default();

    assert_eq!(config.general.default_quality, "best");
    assert_eq!(config.audio.format, "mp3");
    assert_eq!(config.video.format, "mp4");
}

#[test]
fn test_config_serialization() {
    let config = Config::default();
    let toml_string = toml::to_string(&config).unwrap();

    assert!(toml_string.contains("[general]"));
    assert!(toml_string.contains("default_quality = \"best\""));
}

#[test]
fn test_config_get_value() {
    let config = Config::default();
    let quality = config.get("general.default_quality");

    assert_eq!(quality, Some("best".to_string()));
}

#[test]
fn test_config_set_value() {
    let mut config = Config::default();
    config.set("general.default_quality", "1080p").unwrap();

    assert_eq!(config.general.default_quality, "1080p");
}
```

### error.rs Tests

**Focus**: Error display, constructors, retryability

```rust
#[test]
fn test_http_error_display() {
    let error = AppError::http(404, "Not Found");

    assert_eq!(
        error.to_string(),
        "HTTP request failed: Not Found (status code: 404)"
    );
}

#[test]
fn test_retryable_errors() {
    let timeout = AppError::Timeout { seconds: 30 };
    assert!(timeout.is_retryable());

    let invalid_url = AppError::InvalidUrl("bad".to_string());
    assert!(!invalid_url.is_retryable());
}

#[test]
fn test_error_source_chain() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
    let error = AppError::file_read("/test/path", io_error);

    assert!(error.source().is_some());
}
```

### utils.rs Tests

**Focus**: Utility functions, edge cases

```rust
#[test]
fn test_sanitize_filename() {
    assert_eq!(sanitize_filename("hello/world"), "hello_world");
    assert_eq!(sanitize_filename("test:file.txt"), "test_file.txt");
    assert_eq!(sanitize_filename("normal.txt"), "normal.txt");
}

#[test]
fn test_format_bytes() {
    assert_eq!(format_bytes(1024), "1.0 KB");
    assert_eq!(format_bytes(1048576), "1.0 MB");
    assert_eq!(format_bytes(1073741824), "1.0 GB");
}

#[test]
fn test_apply_template() {
    let metadata = VideoMetadata {
        title: "Test Video".to_string(),
        id: "abc123".to_string(),
        author: "Channel".to_string(),
        date: "2024-01-15".to_string(),
        quality: "1080p".to_string(),
    };

    let result = apply_template("{title}-{quality}", &metadata);
    assert_eq!(result, "Test_Video-1080p");
}
```

## Test-Driven Development (TDD)

### TDD Workflow

1. **Write the test first** (it should fail):
   ```rust
   #[test]
   fn test_new_feature() {
       let result = new_function("input");
       assert_eq!(result, "expected");
   }
   ```

2. **Run the test** to see it fail:
   ```bash
   cargo test test_new_feature
   ```

3. **Implement the minimum code** to make it pass:
   ```rust
   fn new_function(input: &str) -> String {
       "expected".to_string()
   }
   ```

4. **Run the test again** to see it pass:
   ```bash
   cargo test test_new_feature
   ```

5. **Refactor** while keeping tests green

### Example: Adding a New Feature

**Scenario**: Add support for custom output filenames

```rust
// Step 1: Write the test
#[test]
fn test_custom_filename() {
    let options = DownloadOptions {
        output_filename: Some("my-video.mp4".to_string()),
        ..Default::default()
    };

    let filename = generate_filename(&options);
    assert_eq!(filename, "my-video.mp4");
}

// Step 2: Implement the feature
fn generate_filename(options: &DownloadOptions) -> String {
    options.output_filename
        .clone()
        .unwrap_or_else(|| format!("{}.mp4", options.video_id))
}

// Step 3: Add more tests for edge cases
#[test]
fn test_filename_with_sanitization() {
    let options = DownloadOptions {
        output_filename: Some("video:name.mp4".to_string()),
        ..Default::default()
    };

    let filename = generate_filename(&options);
    assert_eq!(filename, "video_name.mp4");
}
```

## Continuous Integration (CI/CD)

### GitHub Actions Workflow

Create `.github/workflows/test.yml`:

```yaml
name: Tests

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true

    - name: Install FFmpeg
      run: sudo apt-get update && sudo apt-get install -y ffmpeg

    - name: Run tests
      run: cargo test --verbose

    - name: Run tests with --nocapture
      run: cargo test -- --nocapture

    - name: Check formatting
      run: cargo fmt -- --check

    - name: Run clippy
      run: cargo clippy -- -D warnings
```

### Pre-commit Hooks

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash

# Run tests before committing
cargo test --quiet

if [ $? -ne 0 ]; then
    echo "Tests failed. Commit aborted."
    exit 1
fi

# Check formatting
cargo fmt -- --check

if [ $? -ne 0 ]; then
    echo "Code formatting issues. Run 'cargo fmt' and try again."
    exit 1
fi

echo "All checks passed!"
```

Make it executable:
```bash
chmod +x .git/hooks/pre-commit
```

## Code Coverage

### Using `tarpaulin`

Install tarpaulin:
```bash
cargo install cargo-tarpaulin
```

Generate coverage report:
```bash
# Generate coverage and display in terminal
cargo tarpaulin --verbose

# Generate HTML report
cargo tarpaulin --out Html

# Generate lcov report (for CI integration)
cargo tarpaulin --out Lcov
```

### Coverage Goals

| Module | Target Coverage |
|--------|-----------------|
| `cli.rs` | 90%+ |
| `config.rs` | 95%+ |
| `error.rs` | 100% |
| `utils.rs` | 95%+ |
| Business logic | 80%+ |

## Testing Best Practices

### 1. One Assertion Per Test (When Possible)

❌ **Bad** - Multiple unrelated assertions:
```rust
#[test]
fn test_everything() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(subtract(5, 3), 2);
    assert_eq!(multiply(2, 3), 6);
}
```

✅ **Good** - Separate tests:
```rust
#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn test_subtract() {
    assert_eq!(subtract(5, 3), 2);
}
```

### 2. Test Edge Cases

```rust
#[test]
fn test_sanitize_filename_edge_cases() {
    // Empty string
    assert_eq!(sanitize_filename(""), "");

    // Only invalid characters
    assert_eq!(sanitize_filename("///"), "___");

    // Unicode characters
    assert_eq!(sanitize_filename("видео.mp4"), "видео.mp4");

    // Very long filename
    let long = "a".repeat(300);
    assert!(sanitize_filename(&long).len() <= 255);
}
```

### 3. Use Descriptive Test Names

❌ **Bad**:
```rust
#[test]
fn test1() { }

#[test]
fn test_config() { }
```

✅ **Good**:
```rust
#[test]
fn test_config_default_quality_is_best() { }

#[test]
fn test_config_invalid_quality_returns_error() { }
```

### 4. Arrange-Act-Assert Pattern

```rust
#[test]
fn test_download_options_merge() {
    // Arrange
    let config = Config::default();
    let args = DownloadArgs {
        quality: Some(VideoQuality::Q1080p),
        ..Default::default()
    };

    // Act
    let options = DownloadOptions::from_config(&config)
        .merge_from_cli_args(&args);

    // Assert
    assert_eq!(options.quality, "1080p");
}
```

### 5. Test Both Success and Failure Cases

```rust
#[test]
fn test_parse_bitrate_success() {
    assert_eq!(parse_bitrate("320k").unwrap(), 320);
}

#[test]
fn test_parse_bitrate_invalid_format() {
    let result = parse_bitrate("invalid");
    assert!(result.is_err());
}

#[test]
fn test_parse_bitrate_missing_k_suffix() {
    let result = parse_bitrate("320");
    assert!(result.is_err());
}
```

## Debugging Tests

### Show Test Output

```bash
# See println! statements
cargo test -- --nocapture

# See both stdout and stderr
cargo test -- --nocapture --show-output
```

### Run Single Test

```bash
cargo test test_specific_function -- --exact
```

### Debug with Print Statements

```rust
#[test]
fn test_with_debug_output() {
    let value = compute_value();
    println!("Computed value: {:?}", value);  // Use --nocapture to see this
    assert_eq!(value, expected);
}
```

### Use `dbg!` Macro

```rust
#[test]
fn test_with_dbg() {
    let value = dbg!(compute_value());  // Prints to stderr with file:line
    assert_eq!(value, expected);
}
```

## Summary

The YouTube Downloader has comprehensive test coverage with:

✅ **1500+ lines of unit tests** across all modules
✅ **Organized test structure** with comment headers
✅ **Multiple test running options** (by name, module, pattern)
✅ **Async testing support** with `tokio-test`
✅ **Integration testing** with `tempfile` and `mockito`
✅ **CI/CD ready** with GitHub Actions workflow
✅ **Code coverage** tracking with `tarpaulin`
✅ **Best practices** documented and followed

When adding new features:
1. Write tests first (TDD)
2. Test both success and error cases
3. Test edge cases
4. Use descriptive test names
5. Follow the Arrange-Act-Assert pattern
6. Run tests before committing

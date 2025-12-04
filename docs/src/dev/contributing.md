# Contributing Guidelines

Thank you for your interest in contributing to the YouTube Downloader! This document provides guidelines and best practices for contributing to the project.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Code Style](#code-style)
- [Pull Request Process](#pull-request-process)
- [Adding New Features](#adding-new-features)
- [Documentation Requirements](#documentation-requirements)
- [Testing Requirements](#testing-requirements)
- [Code Review Process](#code-review-process)
- [Release Process](#release-process)

## Getting Started

### Prerequisites

Before contributing, ensure you have:

1. **Rust** (1.75.0 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup update
   ```

2. **FFmpeg** (required for media processing)
   - Linux: `sudo apt-get install ffmpeg`
   - macOS: `brew install ffmpeg`
   - Windows: Download from [ffmpeg.org](https://ffmpeg.org)

3. **Git** for version control
   ```bash
   git --version
   ```

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/rust-yt-downloader.git
   cd rust-yt-downloader
   ```

3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/ORIGINAL_OWNER/rust-yt-downloader.git
   ```

### Build and Test

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run the binary
cargo run -- download https://youtube.com/watch?v=abc123
```

## Development Setup

### Recommended Tools

1. **IDE/Editor**:
   - VS Code with `rust-analyzer` extension
   - IntelliJ IDEA with Rust plugin
   - Vim/Neovim with `rust-analyzer` LSP

2. **Useful Cargo Commands**:
   ```bash
   # Auto-format code
   cargo fmt

   # Lint code
   cargo clippy

   # Check code without building
   cargo check

   # Build documentation
   cargo doc --open
   ```

3. **Development Dependencies**:
   ```bash
   # Install cargo-watch for auto-rebuilding
   cargo install cargo-watch

   # Install cargo-tarpaulin for coverage
   cargo install cargo-tarpaulin

   # Install cargo-edit for managing dependencies
   cargo install cargo-edit
   ```

### Development Workflow

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/my-new-feature
   ```

2. **Make changes** following the code style guidelines

3. **Run tests** frequently:
   ```bash
   cargo test
   ```

4. **Use cargo-watch** for continuous feedback:
   ```bash
   cargo watch -x test
   ```

5. **Format and lint** before committing:
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   ```

6. **Commit with descriptive messages**:
   ```bash
   git commit -m "feat: add support for 8K video quality"
   ```

## Code Style

### Rust Conventions

Follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/):

#### Naming Conventions

```rust
// Types: PascalCase
struct VideoInfo { }
enum AudioFormat { }

// Functions and variables: snake_case
fn download_video() { }
let output_path = PathBuf::new();

// Constants: SCREAMING_SNAKE_CASE
const MAX_RETRIES: u32 = 3;

// Type parameters: Single capital letter or PascalCase
fn generic_function<T>() { }
fn convert<InputFormat, OutputFormat>() { }
```

#### Formatting

Use `rustfmt` (automatically applied by `cargo fmt`):

```rust
// Maximum line length: 100 characters
// Indentation: 4 spaces (no tabs)
// Trailing commas in multi-line items

// Good:
let config = Config {
    output_dir: "~/Downloads".to_string(),
    quality: "1080p".to_string(),
    format: "mp4".to_string(),  // Trailing comma
};

// Use explicit types when clarity is needed
let quality: VideoQuality = VideoQuality::Q1080p;

// Prefer early returns
fn validate_url(url: &str) -> AppResult<()> {
    if url.is_empty() {
        return Err(AppError::InvalidUrl("URL cannot be empty".to_string()));
    }
    Ok(())
}
```

#### Documentation Comments

Use `///` for public items and `//!` for modules:

```rust
//! This module handles video downloads.
//!
//! It provides functions for downloading, retrying, and managing
//! video downloads from YouTube.

/// Downloads a video from YouTube.
///
/// # Arguments
///
/// * `url` - The YouTube video URL
/// * `quality` - The desired video quality
///
/// # Returns
///
/// Returns the path to the downloaded file on success.
///
/// # Errors
///
/// Returns an error if the video is not found or download fails.
///
/// # Examples
///
/// ```no_run
/// let path = download_video("https://youtube.com/watch?v=abc", "1080p").await?;
/// println!("Downloaded to: {:?}", path);
/// ```
pub async fn download_video(url: &str, quality: &str) -> AppResult<PathBuf> {
    // Implementation
}
```

### Module Organization

#### File Structure

Organize modules with clear separation of concerns:

```
src/
├── cli.rs              # Command-line interface
├── config.rs           # Configuration management
├── error.rs            # Error types
├── downloader.rs       # Download orchestration
├── progress.rs         # Progress tracking
├── utils.rs            # Utility functions
├── youtube/            # YouTube API module
│   ├── mod.rs          # Public exports
│   ├── client.rs       # YouTube client
│   ├── metadata.rs     # Metadata extraction
│   └── playlist.rs     # Playlist handling
├── media/              # Media processing module
│   ├── mod.rs          # Public exports
│   ├── ffmpeg.rs       # FFmpeg wrapper
│   ├── audio.rs        # Audio extraction
│   └── converter.rs    # Format conversion
└── main.rs             # Application entry point
```

#### Module Header Comments

Each module should start with documentation:

```rust
//! Short one-line description.
//!
//! Longer description explaining the module's purpose,
//! architecture, and key concepts.
//!
//! # Examples
//!
//! ```
//! // Example usage
//! ```
//!
//! # Module Organization
//!
//! - `SubModule1` - Description
//! - `SubModule2` - Description
```

#### Test Organization

Place tests at the bottom of each module:

```rust
// Module implementation above...

// ==================================================
//          UNIT TESTS
// ==================================================
#[cfg(test)]
mod tests {
    use super::*;

    // ============== Category 1 Tests ==============

    #[test]
    fn test_something() {
        // Test implementation
    }

    // ============== Category 2 Tests ==============

    #[test]
    fn test_something_else() {
        // Test implementation
    }
}
```

### Error Handling Style

#### Always Use AppResult

```rust
// Good:
fn load_config(path: &Path) -> AppResult<Config> {
    // Implementation
}

// Bad:
fn load_config(path: &Path) -> Result<Config, Box<dyn Error>> {
    // Don't use generic error types
}
```

#### Use Specific Error Variants

```rust
// Good: Specific error with context
if !is_valid_url(url) {
    return Err(AppError::InvalidUrl(url.to_string()));
}

// Bad: Generic error
if !is_valid_url(url) {
    return Err(AppError::Other("Invalid URL".to_string()));
}
```

#### Include Context in Errors

```rust
// Good: File path included
fs::write(path, data)
    .map_err(|e| AppError::file_write(path, e))?;

// Bad: No context
fs::write(path, data)?;
```

## Pull Request Process

### 1. Before Submitting

Ensure your changes meet these requirements:

- [ ] Code follows the style guidelines
- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] New features have tests
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated (if applicable)

### 2. Pull Request Template

Use this template for your PR description:

```markdown
## Description

Brief description of the changes made.

## Motivation

Why are these changes needed? What problem do they solve?

## Changes Made

- Change 1
- Change 2
- Change 3

## Testing

How were these changes tested?

- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed

## Checklist

- [ ] Code follows the style guidelines
- [ ] Tests pass locally
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] No breaking changes (or documented if unavoidable)

## Related Issues

Fixes #123
Relates to #456
```

### 3. Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples**:
```
feat(cli): add support for 8K video quality

Add Q8k variant to VideoQuality enum and update quality
filtering logic to support 8K resolution downloads.

Closes #123

---

fix(downloader): handle connection timeouts correctly

Previously, connection timeouts were not properly detected
and caused the download to hang indefinitely. This commit
adds explicit timeout handling with retry logic.

---

docs(readme): update installation instructions

Add instructions for installing on Windows and update
the FFmpeg installation section.
```

### 4. PR Review Process

1. **Automated Checks**: CI/CD pipeline runs automatically
   - All tests must pass
   - Code must be formatted
   - No clippy warnings

2. **Code Review**: Maintainers will review your code
   - Address review comments
   - Push additional commits if needed
   - Engage in constructive discussion

3. **Approval**: Once approved, a maintainer will merge

4. **Merge**: PRs are typically merged using "Squash and Merge"

## Adding New Features

### Feature Development Workflow

1. **Check Existing Issues**: See if the feature is already requested
2. **Create an Issue**: Discuss the feature before implementing
3. **Get Approval**: Wait for maintainer feedback
4. **Implement**: Follow the patterns below
5. **Submit PR**: Follow the PR process

### Adding a New CLI Command

When adding a new command:

1. **Update `cli.rs`**:
   ```rust
   #[derive(Subcommand, Debug)]
   pub enum Commands {
       // Existing commands...

       /// New command description
       NewCommand(NewCommandArgs),
   }

   #[derive(Args, Debug)]
   pub struct NewCommandArgs {
       /// Argument description
       pub arg: String,

       #[command(flatten)]
       pub common: CommonArgs,
   }
   ```

2. **Add Tests**:
   ```rust
   #[cfg(test)]
   mod tests {
       // ============== NewCommand Tests ==============

       #[test]
       fn test_new_command_parsing() {
           let args = vec!["ytdl", "new-command", "arg-value"];
           let cli = Cli::try_parse_from(args).unwrap();
           // Assertions...
       }
   }
   ```

3. **Update `main.rs`**:
   ```rust
   match cli.command {
       Commands::NewCommand(args) => {
           handle_new_command(args).await?;
       }
       // Other commands...
   }
   ```

4. **Update Documentation**: Add to relevant docs

### Adding a Configuration Option

1. **Update `config.rs`**:
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct SectionConfig {
       /// New option description
       #[serde(default = "SectionConfig::default_new_option")]
       pub new_option: String,
   }

   impl SectionConfig {
       fn default_new_option() -> String {
           "default_value".to_string()
       }
   }
   ```

2. **Add Tests**:
   ```rust
   #[test]
   fn test_new_option_default() {
       let config = Config::default();
       assert_eq!(config.section.new_option, "default_value");
   }

   #[test]
   fn test_new_option_from_toml() {
       let toml = r#"
           [section]
           new_option = "custom_value"
       "#;
       let config: Config = toml::from_str(toml).unwrap();
       assert_eq!(config.section.new_option, "custom_value");
   }
   ```

3. **Update `get()` and `set()` methods** in Config impl

4. **Update documentation** and example config file

### Adding an Error Type

1. **Add to `error.rs`**:
   ```rust
   #[derive(Error, Debug)]
   pub enum AppError {
       // Existing errors...

       /// New error description
       #[error("New error: {message}")]
       NewError { message: String },
   }
   ```

2. **Add Helper Constructor** (if needed):
   ```rust
   impl AppError {
       pub fn new_error(message: impl Into<String>) -> Self {
           Self::NewError {
               message: message.into(),
           }
       }
   }
   ```

3. **Add Tests**:
   ```rust
   #[test]
   fn test_new_error_display() {
       let error = AppError::new_error("test message");
       assert_eq!(error.to_string(), "New error: test message");
   }

   #[test]
   fn test_new_error_not_retryable() {
       let error = AppError::new_error("test");
       assert!(!error.is_retryable());
   }
   ```

4. **Update `is_retryable()`** if the error should be retryable

## Documentation Requirements

### Code Documentation

All public items must have documentation:

```rust
/// Public function - MUST have docs
pub fn public_function() { }

/// Public struct - MUST have docs
pub struct PublicStruct { }

// Private function - docs optional but recommended
fn private_function() { }
```

### Documentation Sections

Include these sections when relevant:

```rust
/// Brief one-line description.
///
/// More detailed description explaining what the function does,
/// how it works, and any important details.
///
/// # Arguments
///
/// * `arg1` - Description of argument 1
/// * `arg2` - Description of argument 2
///
/// # Returns
///
/// Description of return value.
///
/// # Errors
///
/// Describes when and why the function returns an error:
/// - `InvalidUrl` - If the URL is not a valid YouTube URL
/// - `VideoNotFound` - If the video doesn't exist
///
/// # Panics
///
/// Describes when the function might panic (if applicable).
///
/// # Safety
///
/// Explains safety invariants for unsafe functions (if applicable).
///
/// # Examples
///
/// ```
/// let result = function_name("arg1", "arg2")?;
/// assert_eq!(result, expected_value);
/// ```
pub fn function_name(arg1: &str, arg2: &str) -> AppResult<String> {
    // Implementation
}
```

### Updating User Documentation

When adding features, update:

1. **README.md**: If it affects usage
2. **docs/src/**: User-facing documentation
3. **CHANGELOG.md**: For version tracking
4. **CLAUDE.md**: If it affects architecture

## Testing Requirements

### Test Coverage Requirements

- **New features**: 80%+ coverage
- **Bug fixes**: Test that reproduces the bug
- **Refactoring**: Maintain existing coverage

### Required Tests

For each new feature, provide:

1. **Unit Tests**: Test individual functions
   ```rust
   #[test]
   fn test_feature_success_case() { }

   #[test]
   fn test_feature_error_case() { }

   #[test]
   fn test_feature_edge_cases() { }
   ```

2. **Integration Tests** (if applicable):
   ```rust
   #[tokio::test]
   async fn test_feature_end_to_end() { }
   ```

3. **Documentation Tests**:
   ```rust
   /// # Examples
   ///
   /// ```
   /// # use rust_yt_downloader::*;
   /// let result = new_feature()?;
   /// assert!(result.is_ok());
   /// # Ok::<(), AppError>(())
   /// ```
   ```

### Running Tests Locally

Before submitting a PR:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Check code coverage
cargo tarpaulin --verbose

# Ensure no clippy warnings
cargo clippy -- -D warnings
```

## Code Review Process

### What Reviewers Look For

1. **Code Quality**:
   - Follows style guidelines
   - Well-structured and readable
   - Appropriate error handling

2. **Testing**:
   - Adequate test coverage
   - Tests are meaningful
   - Edge cases covered

3. **Documentation**:
   - Public APIs documented
   - User-facing docs updated
   - Code comments where needed

4. **Architecture**:
   - Fits with existing patterns
   - Doesn't introduce unnecessary dependencies
   - Maintains separation of concerns

### Responding to Reviews

- **Be receptive**: Treat feedback as learning opportunities
- **Ask questions**: If you don't understand feedback, ask for clarification
- **Make changes**: Address all review comments
- **Explain decisions**: If you disagree, explain your reasoning
- **Be patient**: Reviews may take time

### Review Timeline

- **Initial review**: Usually within 3-5 days
- **Follow-up reviews**: 1-2 days after updates
- **Final approval**: Once all comments are addressed

## Release Process

### Version Numbering

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

Maintainers will:

1. Update `Cargo.toml` version
2. Update `CHANGELOG.md`
3. Create git tag: `git tag -a v1.2.3 -m "Version 1.2.3"`
4. Push tag: `git push origin v1.2.3`
5. Publish to crates.io: `cargo publish`
6. Create GitHub release with changelog

## Getting Help

- **Questions**: Open a [GitHub Discussion](https://github.com/owner/repo/discussions)
- **Bugs**: Open an [Issue](https://github.com/owner/repo/issues)
- **Chat**: Join our [Discord/Slack] (if applicable)

## Code of Conduct

Be respectful, inclusive, and constructive. We're all here to build great software together!

---

Thank you for contributing to the YouTube Downloader! 🎉

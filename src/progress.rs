use crate::error::{AppError, AppResult};
use indicatif::{ProgressBar, MultiProgress, ProgressStyle};
use std::time::Duration;

pub struct ProgressStyles;

impl ProgressStyles {
    pub fn download() -> ProgressStyle {
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .unwrap()
            .progress_chars("█▓░")
    }

    pub fn default() -> ProgressStyle {
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.white/gray}] {pos}/{len} ({percent}%)")
            .unwrap()
            .progress_chars("━╸─")
    }

    pub fn spinner() -> ProgressStyle {
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
    }
}

pub struct DownloadProgress {
    bar: ProgressBar,
}

impl DownloadProgress {
    pub fn new(total_size: u64) -> Self {
        let bar = ProgressBar::new(total_size);
        bar.set_style(ProgressStyles::download());

        Self { bar }
    }

    pub fn new_spinner(message: &str) -> Self {
        let bar = ProgressBar::new_spinner();
        bar.set_style(ProgressStyles::spinner());
        bar.set_message(message.to_string());
        bar.enable_steady_tick(Duration::from_millis(100));

        Self { bar }
    }

    pub fn set_position(&self, pos: u64) {
        self.bar.set_position(pos);
    }

    pub fn inc(&self, delta: u64) {
        self.bar.inc(delta);
    }

    pub fn set_message(&self, msg: &str) {
        self.bar.set_message(msg.to_string());
    }

    pub fn finish(&self) {
        self.bar.finish();
    }

    pub fn finish_with_message(&self, msg: &str) {
        self.bar.finish_with_message(msg.to_string());
    }

    pub fn finish_and_clear(&self) {
        self.bar.finish_and_clear();
    }

    pub fn abandon_with_message(&self, msg: &str) {
        self.bar.abandon_with_message(msg.to_string());
    }

    pub fn inner(&self) -> &ProgressBar {
        &self.bar
    }
}

pub struct MultiDownloadProgress {
    multi: MultiProgress,
}

impl MultiDownloadProgress {
    pub fn new() -> Self {
        Self {
            multi: MultiProgress::new(),
        }
    }

    pub fn add_download(&self, total_size: u64) -> DownloadProgress {
        let bar = ProgressBar::new(total_size);
        bar.set_style(ProgressStyles::download());
        let bar = self.multi.add(bar);

        DownloadProgress { bar }
    }

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

pub mod messages {
    use colored::Colorize;

    pub fn success(msg: &str) {
        println!("{} {}", "✓".green().bold(), msg);
    }

    pub fn error(msg: &str) {
        eprintln!("{} {}", "✗".red().bold(), msg);
    }

    pub fn warning(msg: &str) {
        println!("{} {}", "⚠".yellow().bold(), msg);
    }

    pub fn info(msg: &str) {
        println!("{} {}", "ℹ".blue().bold(), msg);
    }

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
        // Se não der panic, o estilo foi criado com sucesso
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
        let progress = DownloadProgress::new(1024 * 1024 * 1024); // 1 GB
        assert_eq!(progress.bar.length(), Some(1024 * 1024 * 1024));
    }

    #[test]
    fn test_download_progress_new_spinner() {
        let progress = DownloadProgress::new_spinner("Loading...");
        // Spinner não tem tamanho definido
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
        // Não há getter público para mensagem, mas não deve dar panic
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
        // Se criou sem panic, está ok
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

        // Simula download de 3 arquivos em paralelo
        let video1 = multi.add_download(1000);
        let video2 = multi.add_download(2000);
        let video3 = multi.add_download(1500);

        // Progresso parcial
        video1.inc(500);
        video2.inc(1000);
        video3.inc(750);

        assert_eq!(video1.bar.position(), 500);
        assert_eq!(video2.bar.position(), 1000);
        assert_eq!(video3.bar.position(), 750);

        // Video 1 termina primeiro
        video1.set_position(1000);
        video1.finish_with_message("video1.mp4 ✓");
        assert!(video1.bar.is_finished());

        // Video 3 termina
        video3.set_position(1500);
        video3.finish_with_message("video3.mp4 ✓");
        assert!(video3.bar.is_finished());

        // Video 2 termina por último
        video2.set_position(2000);
        video2.finish_with_message("video2.mp4 ✓");
        assert!(video2.bar.is_finished());
    }

    // ============== Messages Module Tests ==============

    // Nota: Os testes de mensagens apenas verificam que não há panic,
    // já que a saída vai para stdout/stderr

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
        let progress = DownloadProgress::new(1_000_000_000); // 1 bilhão

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
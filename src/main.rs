mod error;
mod cli;

use clap::Parser;
use cli::Cli;
use error::{AppError, AppResult};

fn main() {
    let cli = Cli::parse();

    println!("YouTube Downloader - Setup OK!");
    println!("Command parsed successfully: {:?}", cli.command);
}

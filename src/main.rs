mod cli;
mod config;
mod downloader;
mod error;
mod media;
mod progress;
mod utils;
mod youtube;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    println!("YouTube Downloader - Setup OK!");
    println!("Command parsed successfully: {:?}", cli.command);
}

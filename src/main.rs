mod clap_cli;
mod downloader;
mod video_utils;

use clap::Parser;
use clap_cli::cli::Cli;
use downloader::downloader::download;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    download(cli).await?;
    Ok(())
}

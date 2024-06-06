mod clap_cli;
mod video_utils;

use clap::Parser;
use clap_cli::clap::Cli;
use std::error::Error;
use video_utils::downloader::download;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match download(cli).await {
        Ok(_res) => println!("downloading"),
        Err(_e) => println!("something went wrong...please run the same command again!"),
    }
    Ok(())
}

mod clap_cli;
mod video_utils;

use clap::Parser;
use clap_cli::clap::Cli;
use video_utils::downloader::download;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match download(cli).await {
        Ok(_) => println!("Download completed successfully!"),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

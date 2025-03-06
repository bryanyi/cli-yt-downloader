use directories::UserDirs;
use std::{error::Error, path::PathBuf};
use crate::clap_cli::clap::Cli;
use crate::video_utils::general_utils::{expand_tilde, is_valid_link, sanitize_filename};
use crate::video_utils::yt_downloader::YoutubeDL;

pub async fn download(cli: Cli) -> Result<(), Box<dyn Error>> {
    let video_url = cli.url;

    if !is_valid_link(&video_url) {
        return Ok(());
    }

    // Determine the default "Downloads" output directory based on the OS, otherwise just download
    // to current directory
    let default_output_dir = UserDirs::new()
        .and_then(|user_dirs| user_dirs.download_dir().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."));

    let output_dir = expand_tilde(
        cli.output_dir
            .unwrap_or_else(|| default_output_dir.to_string_lossy().to_string()),
    )?;

    // Create YouTube downloader instance
    let mut yt = YoutubeDL::new();

    // Get video info first
    let video_info = yt.get_video_info(&video_url).await?;
    let video_title = sanitize_filename(&video_info.title);
    
    let file_extension = if cli.audio_only { "mp3" } else { "mp4" };
    let output_path = output_dir.join(format!("{}.{}", video_title, file_extension));

    if let Some(parent) = output_path.parent() {
        if let Err(err) = tokio::fs::create_dir_all(parent).await {
            eprintln!("Failed to create output directory: {}", err);
            return Err(err.into());
        }
    }

    // Download the video
    yt.download_video(&video_url, &output_path, cli.audio_only).await?;
    println!("Downloaded to {:?}", output_path);

    Ok(())
}

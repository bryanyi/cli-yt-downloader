use clap::ArgAction::SetTrue;
use clap::Parser;
use directories::UserDirs;
use rustube::{url::Url, Id, Stream, Video};
use std::{error::Error, path::PathBuf};
use tokio::fs::create_dir_all;

#[derive(Parser)]
struct Cli {
    url: String,
    #[arg(long, short = 'o')]
    output_dir: Option<String>,
    #[arg(long, short = 'a', action = SetTrue, default_value_t = false)]
    audio_only: bool,
}

// Function to expand the tilde to the home directory path
fn expand_tilde(path: String) -> Result<PathBuf, Box<dyn Error>> {
    if path.starts_with("~/") {
        if let Some(home_dir) = home::home_dir() {
            return Ok(home_dir.join(path.trim_start_matches("~/")));
        } else {
            return Err("Failed to retrieve home directory".into());
        }
    }
    Ok(PathBuf::from(path))
}

fn sanitize_filename(filename: &str) -> String {
    filename.replace(|c: char| !c.is_ascii_alphanumeric() && c != '.', "")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let video_url = cli.url;

    // Determine the default "Downloads" output directory based on the OS, otherwise just download
    // to current directory
    let default_output_dir = UserDirs::new()
        .and_then(|user_dirs| user_dirs.download_dir().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."));

    let output_dir = expand_tilde(
        cli.output_dir
            .unwrap_or_else(|| default_output_dir.to_string_lossy().to_string()),
    )?;

    let url = Url::parse(&video_url)?;
    //let id = Id::from_str(&video_url)?;
    let id = match Id::from_raw(url.as_str()) {
        Ok(id) => id,
        Err(err) => {
            eprintln!("Failed to parse video ID from URL: {}", err);
            return Ok(());
        }
    };

    let video: Video = Video::from_id(id.into_owned()).await?;
    let video_title = sanitize_filename(&video.video_details().title);

    let best_stream: &Stream = if cli.audio_only {
        video.best_audio().unwrap()
    } else {
        video.best_quality().unwrap()
    };

    let file_extension = if cli.audio_only { "mp3" } else { "mp4" };

    let output_path = output_dir.join(format!("{}.{}", video_title, file_extension));

    if let Some(parent) = output_path.parent() {
        create_dir_all(parent).await?;
    }

    println!("Downloading to {:?}", output_path);

    // let final_file = best_stream.download_to_dir(&output_path).await;

    match best_stream.download_to(&output_path).await {
        Ok(final_file) => println!("Downloaded video to {:?}, {:?}", output_path, final_file),
        Err(e) => {
            eprintln!("Download failed! Error message: {}", e);
            return Ok(());
        }
    }

    println!("Download complete!");

    Ok(())
}

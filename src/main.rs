mod video_utils;

use clap::ArgAction::SetTrue;
use clap::Parser;
use directories::UserDirs;
use indicatif::{ProgressBar, ProgressStyle};
use rustube::{url::Url, Callback, Id, Stream, Video};
use std::{
    error::Error,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::fs::create_dir_all;

use video_utils::general_utils::{expand_tilde, sanitize_filename};

#[derive(Parser)]
struct Cli {
    url: String,
    #[arg(long, short = 'o')]
    output_dir: Option<String>,
    #[arg(long, short = 'a', action = SetTrue, default_value_t = false)]
    audio_only: bool,
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

    let stream_content_length = best_stream.content_length().await?;

    let progress_bar = ProgressBar::new(stream_content_length);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .progress_chars("##-"),
    );

    let progress_bar = Arc::new(Mutex::new(progress_bar));

    let callback = Callback::new().connect_on_progress_closure({
        let progress_bar = Arc::clone(&progress_bar);

        move |callback_args| {
            let cur_chunk = callback_args.current_chunk;
            let progress_bar = progress_bar.lock().unwrap();
            progress_bar.set_position(cur_chunk as u64);
        }
    });

    let final_file = best_stream
        .download_to_with_callback(&output_path, callback)
        .await;

    match final_file {
        Ok(_) => {
            progress_bar
                .lock()
                .unwrap()
                .finish_with_message("Download complete!");
            println!("Downloaded video to {:?}", output_path);
        }
        Err(e) => {
            eprintln!("Download failed =( Error message: {}", e);
            return Ok(());
        }
    }

    Ok(())
}

use directories::UserDirs;
use indicatif::{ProgressBar, ProgressStyle};
use rustube::{url::Url, Callback, Id, Stream, Video};
use std::{
    error::Error,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::clap_cli::clap::Cli;
use crate::video_utils::general_utils::{expand_tilde, is_valid_link, sanitize_filename};

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
        let best_audio_quality = video
            .streams()
            .iter()
            .filter(|stream| {
                stream.includes_audio_track
                    // && !stream.includes_video_track
                    && stream.mime == "audio/mp4"
            })
            .max_by_key(|stream| stream.bitrate);

        match best_audio_quality {
            Some(stream) => stream,
            None => {
                println!(
                    "audio download stream failed...please retry the same command one more time!"
                );
                return Ok(());
            }
        }
    } else {
        let best_video = video
            .streams()
            .iter()
            .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
            .max_by_key(|stream| stream.quality_label);

        match best_video {
            Some(stream) => stream,
            None => {
                println!(
                    "video download stream failed...please retry the same command one more time!"
                );
                return Ok(());
            }
        }
    };

    let file_extension = if cli.audio_only { "mp3" } else { "mp4" };

    let output_path = output_dir.join(format!("{}.{}", video_title, file_extension));

    if let Some(parent) = output_path.parent() {
        if let Err(err) = tokio::fs::create_dir_all(parent).await {
            eprintln!("Failed to create output directory: {}", err);
            return Ok(());
        };
    }

    let stream_content_length = best_stream.content_length().await?;

    let progress_bar = ProgressBar::new(stream_content_length);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:25.cyan/blue}] [{bytes}/{total_bytes}] [ETA: {eta}]")
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

    // todo: final audio file does not play
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
        Err(_e) => {
            eprintln!("Download failed. Please retry the same command one more time!");
            return Ok(());
        }
    }

    Ok(())
}

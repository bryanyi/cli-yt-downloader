use std::{
    error::Error,
    process::Stdio,
    sync::{Arc, Mutex},
};
use tokio::{
    io::{BufReader, AsyncBufReadExt},
    process::Command as TokioCommand,
};

pub async fn run_yt_dlp(args: &[&str], progress_callback: Option<Arc<Mutex<Box<dyn FnMut(&str) + Send>>>>) -> Result<std::process::Output, Box<dyn Error>> {
    let mut command = TokioCommand::new("yt-dlp");
    
    // Add core arguments
    command
        .arg("-f")
        .arg("bv+ba/b")  // Best video + best audio, or best combined format as fallback
        .arg("-S")
        .arg("vcodec:h264,res,acodec:m4a")  // Sort formats to prefer h264 video and m4a audio
        .arg("--merge-output-format")
        .arg("mp4")  // Ensure final format is MP4
        .arg("--no-playlist")  // Don't download playlists
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Only add progress-related args if we have a progress callback
    if progress_callback.is_some() {
        command
            .arg("--progress")
            .arg("--newline")     // Ensure each progress update is on a new line
            .arg("--no-colors")   // Disable colors in the output
            .arg("--progress-template")
            .arg("[download] %(progress.downloaded_bytes)s/%(progress.total_bytes)s");
    }

    // Add user arguments
    command.args(args);

    match command.spawn() {
        Ok(mut child) => {
            if let Some(callback) = progress_callback {
                if let Some(stdout) = child.stdout.take() {
                    let mut reader = BufReader::new(stdout).lines();
                    
                    tokio::spawn(async move {
                        while let Ok(Some(line)) = reader.next_line().await {
                            if let Ok(mut callback) = callback.lock() {
                                callback(&line);
                            }
                        }
                    });
                }
            }

            // Wait for the process to complete and get output
            match child.wait_with_output().await {
                Ok(output) => Ok(output),
                Err(e) => Err(format!("Failed to get command output: {}", e).into()),
            }
        },
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                Err("yt-dlp is not installed. Please install it first using your package manager:\nmacOS: brew install yt-dlp\nLinux: sudo apt install yt-dlp\nWindows: choco install yt-dlp".into())
            } else {
                Err(format!("Failed to spawn yt-dlp: {}", e).into())
            }
        }
    }
} 
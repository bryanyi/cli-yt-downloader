use std::{error::Error, path::PathBuf, time::Duration, sync::{Arc, Mutex}};
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::Value;
use crate::video_utils::yt_dlp_binary;

#[derive(Debug)]
#[allow(dead_code)]
pub struct VideoInfo {
    pub title: String,
    pub formats: Vec<Format>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Format {
    pub format_id: String,
    pub ext: String,
    pub quality: String,
    pub filesize: Option<u64>,
    pub audio_only: bool,
}

#[derive(Debug)]
pub struct YoutubeDL {
    progress_bar: Option<ProgressBar>,
}

impl YoutubeDL {
    pub fn new() -> Self {
        Self {
            progress_bar: None,
        }
    }

    pub async fn get_video_info(&self, url: &str) -> Result<VideoInfo, Box<dyn Error>> {
        let output = yt_dlp_binary::run_yt_dlp(&["--dump-json", "--no-warnings", url], None).await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(format!(
                "Failed to get video info: {}\nOutput: {}",
                stderr, stdout
            ).into());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.trim().is_empty() {
            return Err("No video information received from yt-dlp".into());
        }

        let json: Value = serde_json::from_str(&stdout)?;
        
        let title = json["title"]
            .as_str()
            .ok_or("Could not get video title")?
            .to_string();

        let formats = json["formats"]
            .as_array()
            .ok_or("Could not get formats")?
            .iter()
            .filter_map(|format| {
                Some(Format {
                    format_id: format["format_id"].as_str()?.to_string(),
                    ext: format["ext"].as_str()?.to_string(),
                    quality: format["format_note"].as_str()?.to_string(),
                    filesize: format["filesize"].as_u64(),
                    audio_only: format["vcodec"].as_str().map_or(false, |v| v == "none"),
                })
            })
            .collect();

        Ok(VideoInfo { title, formats })
    }

    pub async fn download_video(
        &mut self,
        url: &str,
        output_path: &PathBuf,
        audio_only: bool,
    ) -> Result<(), Box<dyn Error>> {
        let mut args = vec![];

        if audio_only {
            args.extend_from_slice(&[
                "-x",                     // Extract audio
                "--audio-format", "mp3",
                "--audio-quality", "0",   // Best quality
            ]);
        } else {
            args.extend_from_slice(&[
                "--remux-video", "mp4",   // Ensure video is remuxed to MP4
            ]);
        }

        // Add output template
        args.extend_from_slice(&["-o", output_path.to_str().unwrap()]);
        
        // Add URL
        args.push(url);

        // Set up progress bar
        let progress_bar = ProgressBar::new(100);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% ({eta})")
                .unwrap_or_else(|_| ProgressStyle::default_bar())
                .progress_chars("█>-")
        );
        progress_bar.enable_steady_tick(Duration::from_millis(100));
        
        // Wrap progress bar in Arc<Mutex>
        let progress_bar = Arc::new(Mutex::new(progress_bar));
        self.progress_bar = Some(progress_bar.lock().unwrap().clone());

        // Create progress callback
        let progress_callback = {
            let progress_bar = Arc::clone(&progress_bar);
            Box::new(move |line: &str| {
                if line.starts_with("[download]") {
                    if let Some(percent) = parse_progress(line) {
                        if let Ok(pb) = progress_bar.lock() {
                            pb.set_position(percent);
                        }
                    }
                }
            }) as Box<dyn FnMut(&str) + Send>
        };

        // Wrap callback in Arc<Mutex>
        let progress_callback = Arc::new(Mutex::new(progress_callback));

        // Start the download
        println!("Starting download for: {}", url);
        let output = yt_dlp_binary::run_yt_dlp(&args, Some(progress_callback)).await?;

        // Check the output
        if !output.status.success() {
            if let Ok(pb) = progress_bar.lock() {
                pb.finish_and_clear();
            }
            let error_msg = String::from_utf8_lossy(&output.stderr);
            if error_msg.is_empty() {
                let stdout_msg = String::from_utf8_lossy(&output.stdout);
                return Err(format!(
                    "Download failed. Output: {}\nURL: {}",
                    stdout_msg, url
                ).into());
            }
            return Err(format!(
                "Download failed: {}\nURL: {}",
                error_msg, url
            ).into());
        }

        if let Ok(pb) = progress_bar.lock() {
            pb.finish_with_message("Download complete! ✨");
        }
        Ok(())
    }
}

fn parse_progress(line: &str) -> Option<u64> {
    // Parse line in format "[download] 45.2%"
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 2 {
        return None;
    }
    
    let percent_str = parts[1].trim_end_matches('%');
    percent_str.parse::<f64>().ok().map(|p| (p * 100.0) as u64)
}

impl Default for YoutubeDL {
    fn default() -> Self {
        Self::new()
    }
} 
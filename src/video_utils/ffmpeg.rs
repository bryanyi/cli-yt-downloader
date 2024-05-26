use std::error::Error;
use std::path::Path;
use std::process::{exit, Command};

pub fn extract_audio(input_video: &Path, output_audio: &Path) -> Result<(), Box<dyn Error>> {
    println!("extracting audio...");
    // Check if ffmpeg is installed
    if Command::new("ffmpeg").arg("-version").output().is_err() {
        eprintln!("ffmpeg is not installed on this system. Please install ffmpeg from https://ffmpeg.org/download.html");
        exit(1);
    }

    // Run the ffmpeg command to extract audio
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_video)
        .arg("-q:a")
        .arg("0")
        .arg("-map")
        .arg("a")
        .arg(output_audio)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err("Failed to extract audio with ffmpeg".into())
    }
}

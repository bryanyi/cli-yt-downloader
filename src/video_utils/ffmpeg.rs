use std::error::Error;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn convert_to_mp3<P: AsRef<Path>>(video_path: P) -> Result<(), Box<dyn Error>> {
    let video_path = video_path.as_ref();
    let mp3_path = video_path.with_extension("mp3");

    // Check if ffmpeg is installed
    let ffmpeg_check = Command::new("ffmpeg")
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    println!("ffmpeg status: {:?}", ffmpeg_check);

    match ffmpeg_check {
        Ok(status) if status.success() => {
            // ffmpeg is installed, proceed with conversion
            let status = Command::new("ffmpeg")
                .arg("-i")
                .arg(video_path.as_os_str())
                .arg("-vn") // No video
                .arg("-acodec")
                .arg("mp3")
                .arg(mp3_path.as_os_str())
                .status()?;

            if !status.success() {
                return Err("Failed to convert video to MP3".into());
            }

            Ok(())
        }
        _ => {
            // ffmpeg is not installed
            println!("ffmpeg is not installed on your system. Please download it from https://ffmpeg.org/download.html");
            Err("ffmpeg is not installed".into())
        }
    }
}

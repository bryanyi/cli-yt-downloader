use std::{
    error::Error,
    path::PathBuf,
};

use crate::video_utils::yt_dlp_binary;

#[tokio::test]
async fn test_download() -> Result<(), Box<dyn Error>> {
    // Set up fixed test parameters
    let url = "https://www.youtube.com/watch?v=xRBAsdx9Ve0";
    let output_path = PathBuf::from("test_download.mp4");

    // Set up download arguments
    let args = vec![
        "-o", output_path.to_str().unwrap(),
        url
    ];

    println!("Starting test download from: {}", url);
    let output = yt_dlp_binary::run_yt_dlp(&args, None).await?;

    // Check the output
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Download failed: {}", error_msg).into());
    }

    println!("Video downloaded to: {}", output_path.display());
    Ok(())
} 
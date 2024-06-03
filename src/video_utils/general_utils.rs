use std::{error::Error, path::PathBuf};
use url::Url;

pub fn expand_tilde(path: String) -> Result<PathBuf, Box<dyn Error>> {
    if path.starts_with("~/") {
        if let Some(home_dir) = home::home_dir() {
            return Ok(home_dir.join(path.trim_start_matches("~/")));
        } else {
            return Err("Failed to retrieve home directory".into());
        }
    }
    Ok(PathBuf::from(path))
}

pub fn sanitize_filename(filename: &str) -> String {
    filename.replace(|c: char| !c.is_ascii_alphanumeric() && c != '.', "")
}

fn is_valid_youtube_video_link(link: &str) -> bool {
    if let Ok(url) = Url::parse(link) {
        if url.host_str() == Some("www.youtube.com") || url.host_str() == Some("youtu.be") {
            if url.host_str() == Some("www.youtube.com") && url.path() == "/watch" {
                return url.query_pairs().any(|(key, _)| key == "v");
            } else if url.host_str() == Some("youtu.be") {
                return url
                    .path_segments()
                    .map_or(false, |segments| segments.count() == 1);
            }
        }
    }
    false
}

fn is_youtube_playlist(link: &str) -> bool {
    if let Ok(url) = Url::parse(link) {
        return url.query_pairs().any(|(key, _)| key == "list");
    }
    false
}

pub fn is_valid_link(link: &str) -> bool {
    if !is_valid_youtube_video_link(&link) {
        println!("#####################################");
        println!("Is not a valid youtube link or is a YouTube playlist link - please enter a valid URL to a single YouTube video.");
        println!("#####################################");
        return false;
    }

    if is_youtube_playlist(&link) {
        println!("#####################################");
        println!(
            "Youtube playlists are not supported at this time :( please enter a single video's url"
        );
        return false;
    }

    true
}

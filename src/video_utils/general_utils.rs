use std::{error::Error, path::PathBuf};

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

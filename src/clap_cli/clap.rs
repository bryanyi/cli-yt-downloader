use clap::ArgAction::SetTrue;
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    /// Full URL link to a single YouTube Video
    pub url: String,
    /// Optional - Full path to the output directory. Can use '~'
    #[arg(long, short = 'o')]
    pub output_dir: Option<String>,
    /// Optional - Download only the mp3 audio file of the YouTube video
    #[arg(long, short = 'a', action = SetTrue, default_value_t = false)]
    pub audio_only: bool,
}

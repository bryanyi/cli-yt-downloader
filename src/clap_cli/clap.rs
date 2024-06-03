use clap::ArgAction::SetTrue;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub url: String,
    #[arg(long, short = 'o')]
    pub output_dir: Option<String>,
    #[arg(long, short = 'a', action = SetTrue, default_value_t = false)]
    pub audio_only: bool,
}

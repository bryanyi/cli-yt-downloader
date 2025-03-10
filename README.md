# Cli YouTube Downloader

A simple YouTube downloader powered by [rustube](https://crates.io/crates/rustube) and [clap](https://crates.io/crates/clap)

[![Crates.io](https://img.shields.io/crates/v/cli-yt-downloader)](https://crates.io/crates/cli-yt-downloader)

## Prerequisites

Before installing this tool, you need to have yt-dlp installed on your system:

### macOS
```bash
brew install yt-dlp
```

### Linux
Using apt (Debian/Ubuntu):
```bash
sudo apt update
sudo apt install yt-dlp
```

Using dnf (Fedora):
```bash
sudo dnf install yt-dlp
```

### Windows
Using Chocolatey:
```bash
choco install yt-dlp
```

Using Scoop:
```bash
scoop install yt-dlp
```

## Installation

```bash
cargo install cli-yt-downloader
```

## Optional - Create alias to be 'yt'

Step 1 - create alias

For Bash/Zsh:
```bash
echo 'alias yt="cli-yt-downloader"' >> ~/.zshrc  # For Zsh
# OR
echo 'alias yt="cli-yt-downloader"' >> ~/.bashrc  # For Bash
```

For Windows (PowerShell):
```powershell
# Add to your PowerShell profile
Set-Alias -Name yt -Value cli-yt-downloader
```

Step 2 - apply changes

For Bash/Zsh:
```bash
source ~/.zshrc  # For Zsh
# OR
source ~/.bashrc  # For Bash
```

For Windows (PowerShell), restart your PowerShell session.

## Usage Examples

### Download a video

```bash
cli-yt-downloader -- "https://www.youtube.com/watch?v=FxQirt705gc"
```

### Download only audio

```bash
cli-yt-downloader -- "https://www.youtube.com/watch?v=FxQirt705gc" -a
```

### Download to a specific path

```bash
cli-yt-downloader -- "https://www.youtube.com/watch?v=FxQirt705gc" -o ~/some-path
```

## Default download location
The default download location is set to be your OS's download folder.

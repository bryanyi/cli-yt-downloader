# Cli YouTube Downloader

A simple YouTube downloader powered by [rustube](https://crates.io/crates/rustube) and [clap](https://crates.io/crates/clap)

## Installation

```
https://crates.io/crates/cli-yt-downloader
```

## Optional - Create alias to be 'yt'

Step 1 - create alias

```
echo 'alias yt="cli-yt-downloader"' >> ~/.zshrc
```

Step 2 - source the .zshrc

```
source ~/.zshrc
```

## Download a video

```
cli-yt-downloader -- "https://www.youtube.com/watch?v=xVuTFm1ckkI"
```

## Download only audio

```
cli-yt-downloader -- "https://www.youtube.com/watch?v=xVuTFm1ckkI" -a
```

## Download to a specific path

```
cli-yt-downloader -- "https://www.youtube.com/watch?v=xVuTFm1ckkI" -o ~/some-path
```

## Default download location
The default download location is set to be your OS's download folder.

# Cli YouTube Downloader

A simple YouTube downloader powered by [rusttube](https://crates.io/crates/rustube) and [clap](https://crates.io/crates/clap)

## Installation
`cargo install cli-yt-downloader`

## Download a video
`cli-yt-downloader "https://www.youtube.com/watch?v=xVuTFm1ckkI"`

## Download only audio
`cli-yt-downloader "https://www.youtube.com/watch?v=xVuTFm1ckkI" -a`

## Download to a specific path
`cli-yt-downloader "https://www.youtube.com/watch?v=xVuTFm1ckkI" -o ~/some-path`


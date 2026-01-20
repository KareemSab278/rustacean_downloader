
use yt_dlp::Youtube;
use std::path::PathBuf;
use yt_dlp::client::deps::Libraries;

#[tokio::main]
pub async fn download_audio(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let libraries_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from("output");
    std::fs::create_dir_all(&output_dir)?;

    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    let libraries = Libraries::new(youtube, ffmpeg);
    let fetcher = Youtube::new(libraries, output_dir).await?;

    fetcher.download_audio_stream_from_url(String::from(url), "audio.m4a").await?;
    Ok(())
}
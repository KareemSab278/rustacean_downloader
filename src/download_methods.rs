
use yt_dlp::Youtube;
use std::path::PathBuf;
use yt_dlp::client::deps::Libraries;

pub enum DownloadMethod { // you must set it as public to use outside of this file in main for example
    Video,
    Audio,
}

// #[tokio::main] // putting this above anything turns it into the main async function for the runtime.
pub async fn download(url: String, method: DownloadMethod) -> Result<(), Box<dyn std::error::Error>> {
    let libraries_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from("output");
    std::fs::create_dir_all(&output_dir)?;

    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    let libraries = Libraries::new(youtube, ffmpeg);
    let fetcher = Youtube::new(libraries, output_dir).await?;

    match method {
        DownloadMethod::Video => fetcher.download_video_stream_from_url(String::from(url), "video.mp4").await?,
        DownloadMethod::Audio => fetcher.download_audio_stream_from_url(String::from(url), "audio.m4a").await?,
    };

    Ok(())
}
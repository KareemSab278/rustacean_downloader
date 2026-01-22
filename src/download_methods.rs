use yt_dlp::Youtube;
use std::path::PathBuf;
use yt_dlp::client::deps::Libraries;
use std::process::Command;

fn download_with_yt_dlp(url: &str, output_dir: &str) {
    let status = Command::new("libs/yt-dlp.exe")
        .args([
            "-f", "bestvideo+bestaudio",
            "--merge-output-format", "mp4",
            "-o", &format!("{}/%(title)s.mp4", output_dir),
            url,
        ])
        .status()
        .expect("failed to execute yt-dlp");
    println!("yt-dlp exited with: {:?}", status);
}

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
    let fetcher = Youtube::new(libraries, output_dir.clone()).await?;

    match method {
        DownloadMethod::Video => {
            download_with_yt_dlp(&url, output_dir.to_str().unwrap());
        }
        DownloadMethod::Audio => {
            fetcher.download_audio_stream_from_url(url, "audio.m4a").await?;
        }
    };

    Ok(())
}
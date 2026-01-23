use yt_dlp::Youtube;
use std::path::PathBuf;
use yt_dlp::client::deps::Libraries;

// the ? operator cannot be applied to type () means the type is a function that returns nothing like void in other languages. needs to return a Result type!!!
// you can avoid this error by adding a return type to the function signature that matches the expected type,
// such as Result<(), Box<dyn std::error::Error>> for functions that may return an error but do not return any meaningful value on success. or even Ok(())
async fn download_with_yt_dlp(
    url: &str,
    output_path: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let libraries_dir = PathBuf::from("libs");

    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    let libraries = Libraries::new(youtube, ffmpeg);
    let fetcher = Youtube::new(libraries, output_path).await?;

    let video = fetcher.fetch_video_infos(url.to_string()).await?;

    let audio_format = video.best_audio_format().unwrap();
    let _audio_path = fetcher.download_format(&audio_format, "audio-stream.mp3").await?;

    let video_format = video.best_video_format().unwrap();
    let _video_path = fetcher.download_format(&video_format, "video-stream.mp4").await?;

    let title = video.title.replace(&['/', '\\', ':', '*', '?', '"', '<', '>', '|'][..], "_");

    let output_file = format!("{}.mp4", title);
    let output_path = fetcher.combine_audio_and_video(
        "audio-stream.mp3",
        "video-stream.mp4",
        &output_file
    ).await?;
    println!("Downloaded video to {:?}", output_path);
    Ok(()) // always end fn with Ok(()) if returning Result type like when using ? (try) operator
    // ? ok op returns the error and exits the function if there is an error!
}

pub enum DownloadMethod {
    Video,
    Audio,
}

pub async fn download(
    url: String,
    method: DownloadMethod
) -> Result<(), Box<dyn std::error::Error>> {
    let libraries_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from("output");
    std::fs::create_dir_all(&output_dir)?;

    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    let libraries = Libraries::new(youtube, ffmpeg);
    let fetcher = Youtube::new(libraries, output_dir.clone()).await?;

    match method {
        DownloadMethod::Video => {
            download_with_yt_dlp(&url, output_dir.to_str().unwrap()).await?;
        }
        DownloadMethod::Audio => {
            fetcher.download_audio_stream_from_url(url, "audio.m4a").await?;
        }
    }
    Ok(())
}

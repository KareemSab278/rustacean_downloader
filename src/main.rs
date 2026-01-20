mod download_methods;
use axum::{ routing::get, Router };

// im going to make a backend service for downloading vids and audios from youtube
// using yt-dlp and ffmpeg
// ill have the server get the url and method (vid or audio) from the frontend
// then call the appropriate download method based on what type the user selected
// payload something like {url: "some url", method: "video" or "audio"} simple json
// will probably use Axum

#[tokio::main]
async fn main() {
    // build our application with a single route
    // reading the axum docs for this part
    let app = Router::new().route(
        "/",
        get(|| async { "root works" })
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?; // curl http://localhost:3000/
    axum::serve(listener, app).await?; // using chaining gives the error automatically and is safe.

    // Use ? for error propagation in functions that return Result.

    match
        download_methods::download(
            String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
            download_methods::DownloadMethod::Audio
        ).await
    {
        Ok(_) => println!("Download succeeded"),
        Err(e) => eprintln!("Download failed: {}", e),
    }
}


// found bugs but got work tomorrow. spent an hour fixing compiliing bugs and god damh windows forcingme to download visual studuio as usual. cringe microslop.
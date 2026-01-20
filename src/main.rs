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
        get(|| async { "Hello, World!" })
    );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

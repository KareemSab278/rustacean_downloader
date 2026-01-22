mod download_methods;
use axum::{ extract::Query, routing::{ get, post }, Router };
use serde::Deserialize;

#[derive(Deserialize)]
struct DownloadParams {
    url: String,
    method: String,
}


// curl -X POST "http://localhost:3000/download?url=https://youtu.be/ThjvMReOXYM?si=zIzcoEut3NU0MpMd&method=video" // linux
// Invoke-WebRequest -Uri "http://localhost:3000/download?url=https://www.youtube.com/watch?v=M4TufsFlv_o&method=audio" -Method POST

async fn download_handler(Query(params): Query<DownloadParams>) {
    // get params directly
    let method = match params.method.as_str() {
        // find the method
        // if x then download as x
        "audio" => download_methods::DownloadMethod::Audio,
        "video" => download_methods::DownloadMethod::Video,
        _ => {
            eprintln!("Invalid method");
            return;
        }
    };

    match download_methods::download(params.url.clone(), method).await {
        // finally just call my download fn
        Ok(_) => println!("Download succeeded"),
        Err(e) => eprintln!("Download failed: {:?}", e),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/",
            get(|| async { "root works" })
        )
        .route("/download", post(download_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

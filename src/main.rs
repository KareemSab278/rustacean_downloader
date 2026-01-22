mod download_methods;
use axum::{ routing::get, Router };

// okay so important: if you want to use ? in main, main has to return a Result type.
#[tokio::main]
async fn main() {
    // returns a "future" that thus yeilds a Result enum which is in the std lib of rust.
    // build our application with a single route
    // reading the axum docs for this part
    let app = Router::new().route(
        "/", get(|| async { "root works" })
        // "/audio", post(|| async download(url));
    );

    // Invoke-WebRequest http://localhost:3000/ -UseBasicParsing
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await; // curl http://localhost:3000/
    let _ = axum::serve(listener.expect("reason?"), app).await; // using chaining gives the error automatically and is safe.
    // let _ means run expression but ginore result. use when you want to execute something but dont care about return val
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

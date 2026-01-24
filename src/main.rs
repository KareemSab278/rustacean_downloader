mod download_methods;
use axum::{ extract::Json, http::Method, response::IntoResponse, routing::{ get, post }, Router };
use serde::Deserialize;
use tower_http::cors::{ CorsLayer, Any };
use serde_json::json;

#[derive(Deserialize)]
struct DownloadParams {
    url: String,
    method: String,
}

// curl -X POST "http://localhost:3000/download?url=https://youtu.be/ThjvMReOXYM?si=zIzcoEut3NU0MpMd&method=video" // linux
// Invoke-WebRequest -Uri "http://localhost:3000/download?url=https://www.youtube.com/watch?v=ZnfUeVeMKjY&method=audio" -Method POST

async fn download_handler(Json(params): Json<DownloadParams>) -> impl IntoResponse {
    let method = match params.method.as_str() {
        "audio" => download_methods::DownloadMethod::Audio,
        "video" => download_methods::DownloadMethod::Video,
        _ => {
            eprintln!("Invalid method");
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid method"})),
            );
        }
    };

    match download_methods::get_download_info(params.url.clone(), method).await {
        Ok(info) => (
            axum::http::StatusCode::OK,
            Json(serde_json::to_value(info).unwrap()),
        ),
        Err(e) => {
            eprintln!("Download info failed: {:?}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to get download info"})),
            )
        }
    }
}

#[tokio::main]
async fn main() {
    // let cors_layer = CorsLayer::new().allow_origin(Any).allow_methods([Method::GET, Method::POST]);

    let app = Router::new()
        .route(
            "/",
            get(|| async { "root works" })
        )
        .route("/download", post(download_handler))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any)); // fuck it we ball

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

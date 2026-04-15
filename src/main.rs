use axum::{Router, http::Method, routing::get};
use tower_http::cors::CorsLayer;

mod error;
mod search;
mod tlds;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin([
            "https://shorter.dev".parse()?,
            "http://localhost:5173".parse()?,
        ]);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/search", get(search::get))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on http://{}", listener.local_addr()?);
    Ok(axum::serve(listener, app).await?)
}

use axum::{Router, routing::get};

mod error;
mod search;
mod tlds;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/search", get(search::get));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on http://{}", listener.local_addr()?);
    Ok(axum::serve(listener, app).await?)
}

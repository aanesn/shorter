use axum::{
    Router,
    extract::Request,
    http::{Method, header},
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use std::time::Duration;
use tower_http::cors::CorsLayer;

mod error;
mod search;
mod tlds;

const MAX_AGE: u64 = 300; // 5 min

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cors = CorsLayer::new()
        .allow_origin([
            "https://shorter.dev".parse()?,
            "http://localhost:5173".parse()?,
        ])
        .allow_methods([Method::GET])
        .max_age(Duration::from_secs(MAX_AGE));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/search", get(search::get))
        .layer(middleware::from_fn(cache_middleware))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on http://{}", listener.local_addr()?);
    Ok(axum::serve(listener, app).await?)
}

async fn cache_middleware(req: Request, next: Next) -> Response {
    let mut res = next.run(req).await;
    res.headers_mut().insert(
        header::CACHE_CONTROL,
        format!("public, max-age={MAX_AGE}").parse().unwrap(),
    );
    res
}

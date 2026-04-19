use axum::{
    Router,
    http::{HeaderValue, Method, header},
    routing::get,
};
use hickory_resolver::net::runtime::TokioRuntimeProvider;
use tower_http::{cors::CorsLayer, set_header::SetResponseHeaderLayer};

mod error;
mod lookup;
mod search;
mod tlds;

#[derive(Clone)]
struct Ctx {
    hickory: hickory_resolver::Resolver<TokioRuntimeProvider>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin([
            "https://shorter.dev".parse()?,
            "http://localhost:5173".parse()?,
        ]);

    let cache = SetResponseHeaderLayer::if_not_present(
        header::CACHE_CONTROL,
        HeaderValue::from_static("private, max-age=600"),
    );

    let ctx = Ctx {
        hickory: hickory_resolver::Resolver::builder_tokio()?.build()?,
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/search", get(search::get))
        .route("/lookup", get(lookup::get))
        .layer(cors)
        .layer(cache)
        .with_state(ctx);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on http://{}", listener.local_addr()?);
    Ok(axum::serve(listener, app).await?)
}

use crate::error::AppError;
use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;

const MAX_QUERY_LEN: usize = 255;

#[derive(Deserialize)]
pub struct SearchParams {
    q: String,
}

pub async fn get(
    Query(params): Query<SearchParams>,
) -> anyhow::Result<impl IntoResponse, AppError> {
    let q = params.q.trim().to_lowercase();
    check_query(&q)?;

    Ok(params.q.to_owned())
}

fn check_query(q: &str) -> anyhow::Result<()> {
    if q.is_empty() {
        anyhow::bail!("query cannot be empty")
    }
    if q.len() > MAX_QUERY_LEN {
        anyhow::bail!("query cannot be longer than {MAX_QUERY_LEN} characters")
    }
    Ok(())
}

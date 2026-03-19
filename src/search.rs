use crate::error::AppError;
use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchParams {
    q: String,
}

pub async fn get(
    Query(params): Query<SearchParams>,
) -> anyhow::Result<impl IntoResponse, AppError> {
    Ok(params.q.to_owned())
}

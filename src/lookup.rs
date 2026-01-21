use crate::{error::ApiError, tlds::TLDS};
use anyhow::Context;
use axum::{Json, extract::Query, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LookupParams {
    domain: String,
}

#[typeshare::typeshare]
#[derive(Serialize)]
pub struct LookupRes {
    available: bool,
}

pub async fn get(
    Query(params): Query<LookupParams>,
) -> anyhow::Result<impl IntoResponse, ApiError> {
    let tld = params
        .domain
        .rsplit('.')
        .next()
        .context("failed to get tld")?;
    Ok(Json(LookupRes {
        available: TLDS.get(tld).copied().unwrap_or(false),
    }))
}

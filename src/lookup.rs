use crate::{Ctx, error::AppError};
use anyhow::Context;
use axum::{
    Json,
    extract::{Query, State},
};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[derive(Deserialize)]
pub struct LookupParams {
    domain: String,
}

#[typeshare]
#[derive(Serialize)]
pub struct LookupRes {
    available: bool,
}

pub async fn get(
    Query(params): Query<LookupParams>,
    State(ctx): State<Ctx>,
) -> anyhow::Result<Json<LookupRes>, AppError> {
    let sld = params
        .domain
        .split('.')
        .next()
        .context("failed to get sld")?;
    if sld.len() <= 1 {
        return Ok(Json(LookupRes { available: false }));
    }
    let available = ctx
        .hickory
        .soa_lookup(format!("{}.", params.domain))
        .await
        .is_err();
    Ok(Json(LookupRes { available }))
}

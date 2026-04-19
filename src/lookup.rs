use crate::{Ctx, error::AppError};
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
    let nxdomain = ctx
        .hickory
        .soa_lookup(format!("{}.", params.domain))
        .await
        .is_err_and(|e| e.is_nx_domain());

    if nxdomain {
        return Ok(Json(LookupRes { available: true }));
    }

    Ok(Json(LookupRes { available: false }))
}

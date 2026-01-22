use crate::{Ctx, error::ApiError, tlds::TLDS};
use anyhow::Context;
use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
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

#[derive(Deserialize)]
struct DynadotData {
    available: String,
}

#[derive(Deserialize)]
struct DynadotRes {
    data: Option<DynadotData>,
}

pub async fn get(
    Query(params): Query<LookupParams>,
    State(ctx): State<Ctx>,
) -> anyhow::Result<impl IntoResponse, ApiError> {
    let tld = params
        .domain
        .rsplit('.')
        .next()
        .context("failed to get tld")?;
    if !TLDS.get(tld).copied().unwrap_or(true) {
        return Ok(Json(LookupRes { available: false }));
    }

    if ctx.hickory.lookup_ip(&params.domain).await.is_ok() {
        return Ok(Json(LookupRes { available: false }));
    }

    let url = format!(
        "https://api.dynadot.com/restful/v2/domains/{}/search",
        params.domain
    );
    let available = ctx
        .reqwest
        .get(&url)
        .bearer_auth(&ctx.dynadot_api_key)
        .header("Accept", "application/json")
        .send()
        .await
        .context("failed to get dynadot data")?
        .json::<DynadotRes>()
        .await
        .context("failed to parse dynadot response")?
        .data
        .map(|d| d.available == "Yes")
        .unwrap_or(false);
    Ok(Json(LookupRes { available }))
}

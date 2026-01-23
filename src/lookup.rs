use crate::{Ctx, error::ApiError, tlds::TLDS};
use anyhow::Context;
use axum::{Json, extract::State, response::IntoResponse};
use hickory_resolver::ResolveErrorKind;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LookupPayload {
    domains: Vec<String>,
}

#[typeshare::typeshare]
#[derive(Serialize)]
pub struct LookupRes {
    available: Vec<bool>,
}

#[derive(Deserialize)]
struct DynadotData {
    available: String,
}

#[derive(Deserialize)]
struct DynadotRes {
    data: Option<DynadotData>,
}

pub async fn post(
    State(ctx): State<Ctx>,
    Json(payload): Json<LookupPayload>,
) -> anyhow::Result<impl IntoResponse, ApiError> {
    let mut available = vec![];
    for domain in payload.domains {
        let (sld, tld) = domain
            .split_once('.')
            .context("failed to get sld and tld")?;
        if sld.len() == 1 || !TLDS.get(tld).copied().unwrap_or(true) {
            available.push(false);
            continue;
        }

        let is_nx_domain = match ctx.hickory.lookup_ip(&domain).await {
            Ok(_) => false,
            Err(e) => match e.kind() {
                ResolveErrorKind::Proto(proto_err) => proto_err.is_nx_domain(),
                _ => false,
            },
        };
        if is_nx_domain {
            available.push(true);
            continue;
        }

        let is_available = ctx
            .reqwest
            .get(format!(
                "https://api.dynadot.com/restful/v2/domains/{domain}/search"
            ))
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
        available.push(is_available);
    }
    Ok(Json(LookupRes { available }))
}

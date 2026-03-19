use crate::{error::AppError, tlds::TLDS};
use anyhow::Context;
use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;

const MAX_QUERY_LEN: usize = 255;
const DEFAULT_DOMAIN: &str = "com";
const MAX_SLD_LEN: usize = 63;

#[derive(Deserialize)]
pub struct SearchParams {
    q: String,
}

pub async fn get(
    Query(params): Query<SearchParams>,
) -> anyhow::Result<impl IntoResponse, AppError> {
    let q = params.q.trim().to_lowercase();
    check_query(&q)?;

    let domain = get_domain(&q);

    let sld = get_sld(&domain)?;
    check_sld(&sld)?;

    Ok(domain)
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

fn get_domain(q: &str) -> String {
    let parts: Vec<&str> = q.split('.').collect();
    parts
        .windows(2)
        .find(|w| TLDS.contains(w[1]))
        .map(|w| format!("{}.{}", w[0], w[1]))
        .unwrap_or_else(|| format!("{}.{}", parts[0], DEFAULT_DOMAIN))
}

fn get_sld(domain: &str) -> anyhow::Result<String> {
    domain
        .split('.')
        .next()
        .map(|sld| sld.chars().filter(|c| c.is_ascii_alphanumeric()).collect())
        .context("domain must contain dot")
}

fn check_sld(sld: &str) -> anyhow::Result<()> {
    if sld.is_empty() {
        anyhow::bail!("sld cannot be empty")
    }
    if sld.len() > MAX_SLD_LEN {
        anyhow::bail!("sld cannot be longer than {MAX_SLD_LEN} characters")
    }
    Ok(())
}

use crate::{error::AppError, tlds::TLDS};
use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;

const MAX_QUERY_LEN: usize = 255;
const DEFAULT_DOMAIN: &str = "com";

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

use crate::{error::ApiError, tlds::TLDS};
use axum::{Json, extract::Query, response::IntoResponse};
use serde::{Deserialize, Serialize};

// https://www.rfc-editor.org/rfc/rfc1035#section-2.3.4
const MAX_DOMAIN_LEN: usize = 253;
const MAX_SLD_LEN: usize = 63;
const DEFAULT_TLD: &str = "com";

#[derive(Deserialize)]
pub struct SearchParams {
    q: String,
}

#[typeshare::typeshare]
#[derive(Serialize)]
pub struct SearchRes {
    domains: Vec<String>,
}

pub async fn get(
    Query(params): Query<SearchParams>,
) -> anyhow::Result<impl IntoResponse, ApiError> {
    let q = params.q.trim().to_lowercase();
    check_query(&q)?;

    let domain = get_domain(&q);
    let mut domains = vec![domain.clone()];

    let sld = domain.split('.').next().unwrap().to_owned();
    check_sld(&sld)?;

    let mut curr_sld = sld;
    for i in (0..curr_sld.len()).rev() {
        if !matches!(curr_sld.as_bytes()[i], b'a' | b'e' | b'i' | b'o' | b'u') {
            continue;
        }
        for j in (1..curr_sld.len() - 1).rev() {
            let (new_sld, new_tld) = curr_sld.split_at(j);
            if TLDS.contains_key(new_tld) {
                domains.push(format!("{new_sld}.{new_tld}"));
            }
        }
        curr_sld.remove(i);
    }

    Ok(Json(SearchRes { domains }))
}

fn check_query(q: &str) -> anyhow::Result<()> {
    if q.is_empty() {
        anyhow::bail!("query must be at least 1 character");
    }
    if q.len() > MAX_DOMAIN_LEN {
        anyhow::bail!("query must be at most {} characters", MAX_DOMAIN_LEN);
    }
    Ok(())
}

fn get_domain(q: &str) -> String {
    let host = url::Url::parse(q)
        .ok()
        .and_then(|u| u.host_str().map(|h| h.to_owned()))
        .unwrap_or_else(|| q.to_owned());
    let cleaned = host
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '.')
        .collect::<String>();
    cleaned
        .split('.')
        .collect::<Vec<_>>()
        .windows(2)
        .find(|w| TLDS.contains_key(w[1]))
        .map(|w| format!("{}.{}", w[0], w[1]))
        .unwrap_or_else(|| format!("{}.{}", cleaned.split('.').next().unwrap(), DEFAULT_TLD))
}

fn check_sld(sld: &str) -> anyhow::Result<()> {
    if sld.is_empty() {
        anyhow::bail!("sld must be at least 1 character");
    }
    if sld.len() > MAX_SLD_LEN {
        anyhow::bail!("sld must be at most {} characters", MAX_SLD_LEN);
    }
    Ok(())
}

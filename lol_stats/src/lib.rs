use anyhow::Result;
use serde::de::DeserializeOwned;

pub mod model;
pub mod response;

pub(crate) const BASE_URL: &str = "https://ddragon.leagueoflegends.com/cdn/";
pub(crate) const LATEST_PATCH: &str = "14.3.1";

pub(crate) async fn get_endpoint<T: DeserializeOwned>(url: &str) -> Result<T> {
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let raw = client
        .get(format!("{}{}{}", BASE_URL, LATEST_PATCH, url))
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(serde_json::from_str(&raw)?)
}

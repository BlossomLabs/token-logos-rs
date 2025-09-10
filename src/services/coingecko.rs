use anyhow::Context;
use spin_sdk::http::{Request, Response};
use spin_sdk::http::send;
use serde::{Deserialize, Serialize};

// Define only the fields we care about from the JSON schema
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    #[serde(rename = "logoURI")]
    pub logo_uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenList {
    pub tokens: Vec<TokenInfo>,
}

/// Fetch the token list from CoinGecko for a given network id
pub async fn fetch_token_list(network_id: &str) -> anyhow::Result<TokenList> {
    let url = format!("https://tokens.coingecko.com/{}/all.json", network_id);
    println!("Fetching token list from {}", url);

    let req: Request = Request::builder()
        .method(spin_sdk::http::Method::Get)
        .uri(&url)
        .build();

    let resp: Response = send(req).await.context("Failed to send request")?;

    let status = resp.status();
    if !(200..=299).contains(status) {
        anyhow::bail!("Request failed: {}", status);
    }

    let body = resp.into_body();
    let list: TokenList =
        serde_json::from_slice(&body).context("Failed to parse token list JSON")?;

    Ok(list)
}



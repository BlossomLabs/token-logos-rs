use serde::Deserialize;

// Define only the fields we care about from the JSON schema
#[derive(Debug, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    #[serde(rename = "logoURI")]
    pub logo_uri: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenList {
    pub tokens: Vec<TokenInfo>,
}



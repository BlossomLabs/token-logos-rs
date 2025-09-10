use spin_sdk::key_value::Store;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::env;

use crate::services::coingecko::{TokenList};

#[derive(Serialize, Deserialize)]
struct Cache<T> {
    value: T,
    expires_at: u64,
}

impl<T: Serialize + DeserializeOwned> Cache<T> {
    fn is_expired(&self) -> bool {
        self.expires_at < Self::now()
    }
    fn cache_ttl() -> u64 {
        env::var("CACHE_TTL_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(3600)
    }
    fn now() -> u64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
    }
    fn new(value: T) -> Self {
        Self { value, expires_at: Self::cache_ttl() + Self::now() }
    }
    fn to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let bytes = serde_json::to_vec(self)?;
        Ok(bytes)
    }
    fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let cache = serde_json::from_slice(bytes)?;
        Ok(cache)
    }
    fn get_value(key: &str) -> anyhow::Result<Option<T>> {
        let store = Store::open("default")?;
        let bytes: Option<Vec<u8>> = store.get(key)?;
        match bytes {
            Some(bytes) => {
                let cache = Cache::from_bytes(&bytes)?;
                if cache.is_expired() {
                    return Ok(None);
                }
                return Ok(Some(cache.value));
            }
            None => Ok(None),
        }
    }
    fn set_value(key: &str, value: T) -> anyhow::Result<()> {
        let store = Store::open("default")?;
        let cache = Cache::new(value);
        let bytes = cache.to_bytes()?;
        store.set(key, &bytes)?;
        Ok(())
    }
}

pub fn get_url_from_cache(chain_id: &str, address: &str) -> anyhow::Result<Option<String>> {
    let value = Cache::<TokenList>::get_value(&chain_id)?;
    if let Some(value) = value {
        let url = value.tokens.iter().find(|token| token.address.to_lowercase() == address.to_lowercase()).and_then(|token| token.logo_uri.clone());
        return Ok(url);
    }
    Ok(None)
}

pub fn set_urls_in_cache(chain_id: &str, token_list: TokenList) -> anyhow::Result<()> {
    Cache::<TokenList>::set_value(chain_id, token_list)?;
    Ok(())
}

pub fn clear_cache() -> anyhow::Result<()> {
    let store = Store::open("default")?;
    for key in store.get_keys()? {
        store.delete(&key)?;
    }
    Ok(())
}
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
    fn now() -> u64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
    }
    fn new(value: T) -> Self {
        Self { value, expires_at: cache_ttl_secs() + Self::now() }
    }
    fn to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let bytes = serde_json::to_vec(self)?;
        Ok(bytes)
    }
    fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let cache = serde_json::from_slice(bytes)?;
        Ok(cache)
    }
    pub fn get_value(key: &str) -> anyhow::Result<Option<T>> {
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
    pub fn set_value(key: &str, value: T) -> anyhow::Result<()> {
        let store = Store::open("default")?;
        let cache = Cache::new(value);
        let bytes = cache.to_bytes()?;
        store.set(key, &bytes)?;
        Ok(())
    }
}

/**
 * Get the logo URL from the cache
 * @param chain_id - The chain ID
 * @param address - The address of the token
 * @return The logo URL. Empty string if not found. None if expired or not cached.
 */
pub fn get_url_from_cache(chain_id: &str, address: &str) -> anyhow::Result<Option<String>> {
    let key = format!("tokenlist:{}", chain_id);
    let value = Cache::<TokenList>::get_value(&key)?;
    // Tokenlist is cached and not expired
    if let Some(value) = value {
        let url = value.get_logo_url(address);
        println!("Cache hit for: {}/{} = {}", chain_id, address, url.as_ref().unwrap_or(&String::new()));
        match url {
            Some(url) => return Ok(Some(url)),
            // Return empty string if not found in tokenlist
            None => return Ok(Some(String::new())),
        }
    }
    // Tokenlist is expired or not cached, so return None
    println!("Cache miss for: {}/{}", chain_id, address);
    Ok(None)
}

/**
 * Set the tokenlist in the cache
 * @param chain_id - The chain ID
 * @param token_list - The tokenlist
 * @return The result of the operation
 */
pub fn set_urls_in_cache(chain_id: &str, token_list: TokenList) -> anyhow::Result<()> {
    let key = format!("tokenlist:{}", chain_id);
    Cache::<TokenList>::set_value(&key, token_list)?;
    println!("Cache set for: {}", chain_id);
    Ok(())
}

/**
 * Clear the cache
 * @return The result of the operation
 */
pub fn clear_cache() -> anyhow::Result<()> {
    let store = Store::open("default")?;
    for key in store.get_keys()? {
        store.delete(&key)?;
    }
    println!("Cache cleared");
    Ok(())
}

/**
 * Get the cache TTL from the environment variable
 * @return The cache TTL in seconds
 */
pub fn cache_ttl_secs() -> u64 {
    env::var("CACHE_TTL_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(3600)
}
use std::collections::HashMap;
use std::env;

/// Parse `SUPPORTED_CHAINS` env var into a map of chain_id -> network_id
pub fn get_supported_chains() -> HashMap<u32, String> {
    let mut chain_id_to_network: HashMap<u32, String> = HashMap::new();

    let raw = env::var("SUPPORTED_CHAINS").ok().unwrap_or_default();
    for entry in raw.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let mut parts = entry.split(':').map(|s| s.trim());
        if let (Some(network_id), Some(chain_id_str)) = (parts.next(), parts.next()) {
            if let Ok(chain_id) = chain_id_str.parse::<u32>() {
                chain_id_to_network.insert(chain_id, network_id.to_string());
            }
        }
    }

    chain_id_to_network
}

pub fn get_network_id(chain_id: &str) -> String {
    let chain_id_num = chain_id.parse::<u32>().unwrap_or_default();
    get_supported_chains()
        .get(&chain_id_num)
        .cloned()
        .unwrap_or_default()
}



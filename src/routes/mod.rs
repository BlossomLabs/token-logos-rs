use spin_sdk::http::{Request, Response};

use crate::config::get_network_id;
use crate::constants::{ETH_LOGO_URL, POL_LOGO_URL, ZERO_ADDRESS};
use crate::services::coingecko::fetch_token_list;
use crate::services::cache::{get_url_from_cache, set_urls_in_cache, clear_cache};

pub async fn route_request(req: Request) -> anyhow::Result<Response> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let path = req.path();
    let segments: Vec<&str> = path
        .trim_start_matches('/')
        .split('/')
        .collect();

    if segments.len() >= 3 && segments[0] == "token" {
        let chain_id = segments[1];
        let address = segments[2];
        return handle_token_route(chain_id, address).await;
    }

    if segments.len() == 1 && segments[0] == "clear-cache" {
        return clear_cache_route();
    }

    if segments.len() == 1 && segments[0] == "" {
        return main_route();
    }

    Ok(
        Response::builder()
            .status(404)
            .header("content-type", "text/plain")
            .body("Not Found")
            .build(),
    )
}

fn main_route() -> anyhow::Result<Response> {
    Ok(
        Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body("Usage: /token/{chain_id}/{address}")
            .build(),
    )
}

async fn handle_token_route(chain_id: &str, address: &str) -> anyhow::Result<Response> {
    if address == ZERO_ADDRESS {
        let logo_url = if chain_id != "137" { ETH_LOGO_URL } else { POL_LOGO_URL };
        return Ok(
            Response::builder()
                .status(302)
                .header("location", logo_url)
                .build(),
        );
    }

    let network_id = get_network_id(chain_id);
    if network_id.is_empty() {
        anyhow::bail!("Unsupported chain_id: {}", chain_id);
    }

    let mut logo_url = get_url_from_cache(chain_id, address)?;

    if let None = logo_url {
        let token_list = fetch_token_list(&network_id).await?;
        set_urls_in_cache(chain_id, token_list)?;
        logo_url = get_url_from_cache(chain_id, address)?;
    }

    if let Some(logo_url) = logo_url {
        if !logo_url.is_empty() {
            return Ok(
                Response::builder()
                    .status(302)
                    .header("location", logo_url)
                    .build(),
            );
        }
    }
    Ok(
        Response::builder()
            .status(404)
            .header("content-type", "text/plain")
            .body("Logo not found")
            .build(),
    )
}

fn clear_cache_route() -> anyhow::Result<Response> {
    clear_cache()?;
    Ok(
        Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body("Cache cleared")
            .build(),
    )
}

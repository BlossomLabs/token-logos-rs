use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

mod constants;
mod config;
mod services;
mod routes;

/// Spin HTTP component entrypoint
#[http_component]
async fn handle_token_logos_rs(req: Request) -> anyhow::Result<impl IntoResponse> {
    match routes::route_request(req).await {
        Ok(response) => Ok(response),
        Err(error) => {
            eprintln!("Internal error: {:?}", error);
            Ok(
                Response::builder()
                    .status(500)
                    .header("content-type", "text/plain")
                    .body("Internal Server Error")
                    .build(),
            )
        }
    }
}


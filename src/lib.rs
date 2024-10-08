use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use anyhow::Error;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use bb_lib_http_listener::Server;
use conf::config;

mod conf;

async fn root_handler() -> StatusCode {
    eprintln!("Root Handler HIT");
    StatusCode::OK
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SplunkWebhook {
    result: HashMap<String, Value>,
}

async fn webhook_handler(
    Json(payload): Json<SplunkWebhook>,
) -> Result<StatusCode, (StatusCode, String)> {
    eprintln!("Payload received -> \n {:#?}", payload);
    Ok(StatusCode::OK)
}

pub async fn run_server() -> Result<(), Error> {
    let config = config()?;
    let router = Router::new()
        .route("/", get(root_handler))
        .route("/", post(webhook_handler));

    let server = Server::new(&config.bind_addr);
    server.listen(Some(router)).await?;

    Ok(())
}

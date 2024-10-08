use anyhow::Error;
use axum::{http::StatusCode, routing::get, Router};
use bb_lib_http_listener::Server;
use conf::config;

mod conf;

async fn root_handler() -> StatusCode {
    eprintln!("Root Handler HIT");
    StatusCode::OK
}

pub async fn run_server() -> Result<(), Error> {
    let config = config()?;
    let router = Router::new().route("/", get(root_handler));

    let server = Server::new(&config.bind_addr);
    server.listen(Some(router)).await?;

    Ok(())
}

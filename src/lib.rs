use anyhow::Error;
use axum::{
    routing::{get, post},
    Router,
};
use bb_lib_http_listener::Server;
use bb_lib_surreal_client::{connect, setup, DbGuard};
use handlers::{root_handler, webhook_handler};

mod conf;
mod schema;
mod handlers;
mod statics;

pub async fn connect_to_db() -> Result<DbGuard, Error> {
    let db_cfg = setup();
    let db_guard = connect(&db_cfg).await?;
    Ok(db_guard)
}

pub async fn run_server() -> Result<(), Error> {
    let conf = &statics::CONF;
    let _db = connect_to_db().await?;

    let router = Router::new()
        .route("/", get(root_handler))
        .route("/", post(webhook_handler));

    let server = Server::new(&conf.bind_addr);
    eprintln!("Server listening on {}", &conf.bind_addr);
    server.listen(Some(router)).await?;
    Ok(())
}

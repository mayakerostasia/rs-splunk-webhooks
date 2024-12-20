use anyhow::Error;
use axum::{routing::post, Router};
use bb_lib_http_listener::Server;
use bb_lib_surreal_client::{connect, setup, DbGuard};
use handlers::webhook_handler;
use tower_http::trace::TraceLayer;
use tracing::debug;

mod conf;
mod handlers;
mod schema;
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
        .route("/", post(webhook_handler))
        .layer(TraceLayer::new_for_http());

    let server = Server::new(&conf.bind_addr);
    debug!("Server listening on {}", &conf.bind_addr);
    server.listen(Some(router), 2000, false).await?;
    Ok(())
}

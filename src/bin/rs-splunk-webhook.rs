use anyhow::Error;
use splunk_webhooks::run_server;
use tracing::debug;

#[tokio::main]
async fn main() -> Result<(), Error> {
    bb_lib_metrics::init_metrics().await?;
    let _otel = bb_lib_tracing::initialize()?;
    debug!("Starting Server");
    run_server().await?;
    Ok(())
}

use anyhow::Error;
use splunk_webhooks::run_server;
use tracing::debug;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _trace_guard = bb_lib_tracing::initialize()?;
    debug!("Starting Server");
    run_server().await?;
    Ok(())
}

use anyhow::Error;
use splunk_webhooks::run_server;

#[tokio::main]
async fn main() -> Result<(), Error> {
    eprintln!("Starting Server");
    run_server().await?;
    Ok(())
}

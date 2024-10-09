use anyhow::Error;
use config::Config;
use serde::Deserialize;

pub fn config() -> Result<SplunkWebhookSettings, Error> {
    let source =
        config::File::with_name(&std::env::var("BB_CONFIG").expect("Please Set BB_CONFIG ENV var"));
    let conf = Config::builder().add_source(source);
    let conf = conf.build()?;
    Ok(conf.try_deserialize::<SplunkWebhookSettings>()?)
}

#[derive(Debug, Clone, Deserialize)]
pub struct SplunkWebhookSettings {
    pub bind_addr: String,
    pub hash_table: String,
}

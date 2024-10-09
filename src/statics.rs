use lazy_static::lazy_static;
use crate::conf::{config, SplunkWebhookSettings};

lazy_static! {
    pub static ref CONF: SplunkWebhookSettings = config().expect("Config Failed");
}

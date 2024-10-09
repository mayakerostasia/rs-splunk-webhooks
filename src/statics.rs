use crate::conf::{config, SplunkWebhookSettings};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONF: SplunkWebhookSettings = config().expect("Config Failed");
}

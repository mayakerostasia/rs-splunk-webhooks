use crate::statics::CONF;
use bb_lib_surreal_client::{Record, Storable};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplunkWebhook {
    result: HashMap<String, Value>,
}

impl<'a> From<&'a SplunkWebhook> for Record<SplunkWebhook> {
    fn from(value: &'a SplunkWebhook) -> Self {
        value.clone().into()
    }
}

impl<'a> From<&'a mut SplunkWebhook> for Record<SplunkWebhook> {
    fn from(value: &'a mut SplunkWebhook) -> Self {
        value.clone().into()
    }
}

impl From<SplunkWebhook> for Record<SplunkWebhook> {
    fn from(value: SplunkWebhook) -> Self {
        Record::new(&CONF.db_table, None, Some(value))
    }
}

impl Storable<SplunkWebhook> for SplunkWebhook {}

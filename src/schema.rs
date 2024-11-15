use crate::statics::CONF;
use bb_lib_surreal_client::{prelude::RecordIdKey, Record, Storable};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplunkWebhook {
    sid: Option<String>,
    search_name: Option<String>,
    app: Option<String>,
    owner: Option<String>,
    results_link: Option<String>,
    #[serde(flatten)]
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
        let time = value
            .result
            .get("_time")
            .expect("No `_time` value")
            .to_string();
        Record::new(&CONF.db_table, Some(RecordIdKey::from(time)), Some(value))
    }
}

impl Storable<SplunkWebhook> for SplunkWebhook {}

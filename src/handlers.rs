use crate::schema::SplunkWebhook;
use axum::{
    http::StatusCode,
    Json
};
use bb_lib_surreal_client::Storable;

pub async fn root_handler() -> StatusCode {
    eprintln!("Root Handler HIT");
    StatusCode::OK
}

pub async fn webhook_handler(
    Json(payload): Json<SplunkWebhook>,
) -> Result<StatusCode, (StatusCode, String)> {
    eprintln!("Payload received -> \n {:#?}", payload);
    match payload.save().await {
        Ok(webhook) => {
            eprintln!("Webhook Stored! -> {:#?}", webhook);
            Ok(StatusCode::OK)
        },
        Err(e) => {
            eprintln!("Error saving webhook ->\n {:#?}", e);
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
    }
}

// use crate::schema::SplunkWebhook;
use axum::http::{HeaderMap, Method, StatusCode};
// use bb_lib_surreal_client::Storable;
use tracing::debug;

// pub async fn root_handler() -> StatusCode {
//     debug!("Root Handler HIT");
//     StatusCode::OK
// }

pub async fn webhook_handler(
    method: Method,
    headers: HeaderMap,
    payload: String,
    // Json(payload): Json<SplunkWebhook>,
) -> Result<StatusCode, (StatusCode, String)> {
    eprintln!("Method: {}", method);
    eprintln!("Headers: {:#?}", headers);

    debug!("Payload received -> \n {:#?}", payload);
    // match payload.save().await {
    //     Ok(webhook) => {
    //         debug!("Webhook Stored! -> {:#?}", webhook);
    //         Ok(StatusCode::OK)
    //     }
    //     Err(e) => {
    //         debug!("Error saving webhook ->\n {:#?}", e);
    //         Err((StatusCode::BAD_REQUEST, e.to_string()))
    //     }
    // }
    //
    Ok(StatusCode::OK)
}

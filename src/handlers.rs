// use crate::schema::SplunkWebhook;
use axum::body::Bytes;
use axum::http::{HeaderMap, Method, StatusCode};
// use bb_lib_surreal_client::Storable;
// use tracing::debug;

// pub async fn root_handler() -> StatusCode {
//     debug!("Root Handler HIT");
//     StatusCode::OK
// }

pub async fn webhook_handler(
    method: Method,
    headers: HeaderMap,
    payload: Bytes,
    // Json(payload): Json<SplunkWebhook>,
) -> Result<StatusCode, (StatusCode, String)> {
    eprintln!("Method: {}", method);
    eprintln!("Headers: {:#?}", headers);
    eprintln!("Payload received -> \n {:#?}", payload);
    eprintln!(
        "Bytes to String -> \n {:#?}",
        serde_json::to_string(&payload)
    );
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

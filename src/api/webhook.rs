
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::Value;

use crate::api::invoice::FindInvoiceResponse;
use crate::LNbitsRustClient;

#[derive(Debug, Clone)]
pub struct WebhookState {
   pub sender: tokio::sync::mpsc::Sender<String>,
}


impl LNbitsRustClient {
  pub async fn create_invoice_webhook_router(
    &self,
    webhook_endpoint: &str,
    sender: tokio::sync::mpsc::Sender<String>,
  ) -> {
    let state = WebhookState {
      sender
    };

   let router = Router::new().route(webhook_endpoint, post(handle_invoice)).with_state(state);

  }
}

async fn handle_invoice(
  State(state): State<WebhookState>,
  Json(payload): Json<Value>,
) -> Result<StatusCode, StatusCode> {
  let webhook_response: FindInvoiceResponse = serde_json::from_value(payload.clone()).map_err(|_err| {
      log::warn!("invalid payload on webhook recieved");
            log::debug!("Value: {}", payload);

             StatusCode::UNPROCESSABLE_ENTITY
  } )?;

  // log the correct payload
  log::debug!(
    "Received webhook update for: {}",
    webhook_response.checking_id
  );

  if let Err(err) = state.sender.send(webhook_response.checking_id).await {
    log::warn!("Unable to send on channel: {}", err);
  }
  Ok(StatusCode::Ok)
}
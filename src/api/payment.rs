use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::LnbitsEndpoints;

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDetails {
  pub status: String,
  pub pending: bool,
  pub checking_id: String,
  pub amount: u64,
  pub fee: u64,
  pub memo: String,
  pub time: u64,
  pub bolt11: String,
  pub preimage: String,
  pub payment_hash: String,
  pub expiry: u64,
  pub extra: serde_json::Value,
  pub webhook: Option<String>,
  pub webhook_status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
  pub paid: bool,
  pub preimage: String,
  pub payment_detail: PaymentDetails
}


impl crate::LNbitsRustClient {
  pub async fn get_payment_info (&self, payment_hash: &str) -> Result<Payment> {
    let body = self.make_get(
      LNBitsEndpoint::PaymentHash(payment_hash.to_string()),
      crate::api::LNBitsRequestKey::Admin,
    ).await?

    let payment_details: Payment = serde_json::from_str(&body)?;
    Ok(payment_details)
  }
}
use std::collections::HashMap;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use super::{LnbitsRequestKeyType, LnbitsEndpoints};


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvoiceResponse {
   /// Payment request (bolt11)
  pub payment_request: String,
  pub payment_hash: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentInvoiceResponse {
  pub payment_hash: String
}

// creare invoice request

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateInvoiceRequest {
  pub unit: String,

  pub amount: String,

   #[serde(skip_serializing_if = "Option::is_none")]
   pub memo: Option<String>,

  /// Expiry is in seconds
  #[serde(skip_serializing_if = "Option::is_none")]
  pub expiry: Option<u64>,

   /// Webhook url
  #[serde(skip_serializing_if = "Option::is_none")]
  pub webhook: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,

    pub out: bool,

}

/// response of Decoded invoice
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecodInvoiceResponse {
  pub payment_hash: String,
  pub amount_in_msat: u64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description_hash: Option<String>,
  pub description: String,

  pub date: i64,
  pub payee: String,

  pub expiry: f64,
  pub secret: String,

     /// Route hints
    pub route_hints: Vec<String>,
    /// Mint final cltx expiry
    pub min_final_cltv_expiry: i64,

}

/// find invoice response 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FindInvoiceResponse {
    pub checking_id: String,
    pub pending: bool,
    pub amount: i64,
    pub fee: i64,
    pub memo: String,
    pub time: u64,
    pub bolt11: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preimage: Option<String>,
    pub payment_hash: String,
    pub expiry: f64,
    pub extra: HashMap<String, serde_json::Value>,
    pub wallet_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_status: Option<String>,
}

impl crate::LNbitsRustClient {
  
  // create an invoice
  pub fn async create_invoice(
    &self,
    params: &CreateInvoiceRequest
  ) -> Result<CreateInvoiceResponse> {
    let response_body = self.post_request(
      LNBitsEndpoint::Payments,
      crate::api::LnbitsRequestKeyType::InvoiceRead,
      &serde_json::to_string(&params)?
    ).await?

    match serde_json::from_str(&response_body) {
      Ok(res) => Ok(res),
      Err(_) => {
        log::error!("Api error response on invoice creation"),
        log::error!("{}", response_body);
        bail!("Fail to create an invoice")
      }
    }
  }
  // pay an invoice (only admin can pay for an invoice)
  pub fn async pay_invoice(
    &self,
    bolt11:: &str
  ) -> Result<PaymentInvoiceResponse> {
    let response_body = self.make_post(
       LNBitsEndpoint::Payments,
       crate::api::LNBitsRequestKey::Admin,
      &serde_json::to_string(&serde_json::json!({ "out": true, "bolt11": bolt11 }))?,
    ).await?

      match serde_json::from_str(&response_body) {
      Ok(res) => Ok(res),
      Err(_) => {
        log::error!("Api error response on invoice creation"),
        log::error!("{}", response_body);
        bail!("Fail to create an invoice")
      }
    }

  }
}
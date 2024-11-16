use std::collections::HashMap;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use super::{LnbitsRequestKeyType, LnbitsEndpoints};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateLnurlResponse {
  details: String
}


#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateLnurlResquest {
  pub description_hash: String,
  pub callback: String,
  pub amount: u256,
  pub description: String,
  pub unit: String,
};



impl crate::LNbitsRustClient {
  pub fn create_pay_by_lnurl (
     &self,
     params: &CreateLnurlResquest
  ) -> Result<CreateLnurlResponse> {
    let response_body = self.post_request(
      LnbitsEndpoints::LNURL,
       crate::api::LnbitsRequestKeyType::InvoiceRead,
       &serde_json::to_string(&params)?
    ).await?

    match serde_json::from_str(&response_body) {
      Ok(res) => Ok(res),
      Err(_) => {
        log::error!("Api error response on lnurl payment"),
        log::error!("{}", response_body);
        bail!("Fail to make lnurl payment")
      }
    }
  }
}
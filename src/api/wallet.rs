use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use super::LnbitsEndpoints;


// wallet details
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletDetails {
  pub name: String, 
  pub balance: u64
}


impl crate::LNbitsRustClient {
  pub async fn get_wallet_details(&self) -> Result<WalletDetails> {
    let body = self.make_get(
      LnbitsEndpoints::Wallet,
      crate::api::LNBitsRequestKey::InvoiceRead,
    ).await?

     match serde_json::from_str(&body){
      Ok(res) => Ok(res),
      Err(_) => {
        log::error!("Api error response on get wallet details");
        log::error!("{}", body);
        bail!("Could not get wallet details")
      }
     }
  }
}
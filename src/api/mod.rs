
use std::fmt;


use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum LnbitRequestKeyType {
  InvoiceRead,
  Admin
}

/// Endpoints 
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum LnbitsEndpoints {
  Wallets,
  Payments,
  PaymentsDecode,
  Boltz,
  PaymentHash(String),
  PaymentsFindInvoice(String),
}

impl fmt::Display for LnbitsEndpoints {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      LnbitsEndpoints::Wallets => write!(f, "api/v1/wallet"),
      LnbitsEndpoints::Payments => write!(f, "api/v1/payments"),
      LnbitsEndpoints::PaymentsDecode => write!(f, "api/v1/payments/decode"),
      LnbitsEndpoints::Boltz => write!(f, "api/v1/beltz"),
      LnbitsEndpoints::PaymentHash(hash) => write!(f, "api/v1/payments/{}", hash),
      LnbitsEndpoints::PaymentsFindInvoice(checking_id) => {
        write!(f, "api/v1/payments?checking_id={}", checking_id)
      },
    }
  }
}


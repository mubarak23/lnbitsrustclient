use std::fmt;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum LnbitsRequestKeyType {
    InvoiceRead,
    Admin,
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
            }
        }
    }
}

impl crate::LNbitsRustClient {
    // GET REQUEST
    pub async fn get_request(
        &self,
        key: LnbitsRequestKeyType,
        endpoint: LnbitsEndpoints,
    ) -> Result<String> {
        let base_url = self.lnbits_url.join(&endpoint.to_string())?;
        let response = self
            .reqwest_client
            .get(base_url)
            .header("X-Api-Key", {
                match key {
                    LnbitsRequestKeyType::Admin => self.admin_key.clone(),
                    LnbitsRequestKeyType::InvoiceRead => self.invoice_read_key.clone(),
                }
            })
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            bail!("not found")
        }
        
        let response_body = response.text().await?;

        Ok(response_body)
    }

    pub async fn post_request(
        &self,
        key: LnbitsRequestKeyType,
        endpoint: LnbitsEndpoints,
        body: &str,
    ) -> Result<String> {
        let url = self.lnbits_url.join(&endpoint.to_string())?;
        let response = self
            .reqwest_client
            .post(url)
            .header("X-Api-Key", {
                match key {
                    LnbitsRequestKeyType::Admin => self.admin_key.clone(),
                    LnbitsRequestKeyType::InvoiceRead => self.invoice_read_key.clone(),
                }
            })
            .body(body.to_string())
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            bail!("Not Found")
        }

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            bail!("Unauthorized")
        }

        let body = response.text().await?;

        Ok(body)
    }
    pub async fn put_request(
        &self,
        key: LnbitsRequestKeyType,
        endpoint: LnbitsEndpoints,
        body: &str,
    ) -> Result<String> {
        let url = self.lnbits_url.join(&endpoint.to_string())?;
        let response = self
            .reqwest_client
            .patch(url)
            .header("X-Api-Key", {
                match key {
                    LnbitsRequestKeyType::Admin => self.admin_key.clone(),
                    LnbitsRequestKeyType::InvoiceRead => self.invoice_read_key.clone(),
                }
            })
            .body(body.to_string())
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            bail!("Not Found")
        }

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            bail!("Unauthorized")
        }

        let body = response.text().await?;

        Ok(body)
    }

    pub async fn delete_request(
        &self,
        key: LnbitsRequestKeyType,
        endpoint: LnbitsEndpoints,
    ) -> Result<String> {
        let url = self.lnbits_url.join(&endpoint.to_string())?;
        let response = self
            .reqwest_client
            .delete(url)
            .header("X-Api-Key", {
                match key {
                    LnbitsRequestKeyType::Admin => self.admin_key.clone(),
                    LnbitsRequestKeyType::InvoiceRead => self.invoice_read_key.clone(),
                }
            })
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            bail!("Not Found")
        }

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            bail!("Unauthorized")
        }

        let body = response.text().await?;

        Ok(body)
    }
}

use anyhow::Result;
use reqwest::Url;

pub mod api;

#[derive(Clone)]
pub struct LNbitsRustClient {
    lnbits_url: Url,
    admin_key: String,
    invoice_read_key: String,
    reqwest_client: reqwest::Client,
    wallet_id: String,
}

impl LNbitsRustClient {
    fn new(
        wallet_id: &str,
        lnbits_url: &str,
        invoice_read_key: &str,
        admin_key: &str,
        tor_socket: Option<&str>,
    ) -> Result<LNbitsRustClient> {
        let lnbits_url = Url::parse(lnbits_url)?;

        let client = {
            if let Some(tor_socket) = tor_socket {
                let proxy = reqwest::Proxy::all(tor_socket).expect("proxy tor should be provided");
                reqwest::Client::builder().proxy(proxy).build()?
            } else {
                reqwest::Client::builder().build()?
            }
        };

        Ok(LNbitsRustClient {
            lnbits_url: lnbits_url,
            admin_key: admin_key.to_string(),
            invoice_read_key: invoice_read_key.to_string(),
            reqwest_client: client,
            wallet_id: wallet_id.to_string(),
        })
    }
}

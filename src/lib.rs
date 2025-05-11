mod utils;

use anyhow::{Context, Result};
use hyperliquid_rust_sdk::{BaseUrl, InfoClient};
use std::collections::HashMap;
pub use utils::*;

pub struct CryptoClient {
    hl_client: InfoClient,
}

impl CryptoClient {
    pub async fn build() -> Result<Self> {
        let hl_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await?;
        Ok(Self { hl_client })
    }

    pub async fn fetch_perp_acct_value(&self, address: &str) -> Result<f64> {
        let res = self.hl_client.user_state(address.parse()?).await?;

        let acct_val = res.cross_margin_summary.account_value; // account value in USD
        let acct_val = acct_val.parse::<f64>()?;

        Ok(acct_val)
    }

    pub async fn fetch_spot_acct_value(&self, address: &str) -> Result<f64> {
        let (res, mids) = tokio::join!(
            self.hl_client.user_token_balances(address.parse()?),
            self.get_all_mids()
        );
        let mids = mids?;

        let mut acct_val = 0.0; // USD

        for token_bal in res?.balances {
            if token_bal.coin == "USDC" {
                acct_val += token_bal.total.parse::<f64>()?;
                continue;
            }
            let token_balance = token_bal.total.parse::<f64>()?;
            let token_price = mids
                .get(&token_bal.coin)
                .with_context(|| format!("Token {} not found in mids", token_bal.coin))
                .and_then(|p| {
                    p.parse::<f64>().context("Failed to parse token price into a f64")
                })?;
            acct_val += token_balance * token_price;
        }

        Ok(acct_val)
    }

    async fn get_all_mids(&self) -> Result<HashMap<String, String>> {
        let res = self.hl_client.all_mids().await?;
        Ok(res)
    }
}

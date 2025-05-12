mod utils;

use anyhow::{Context, Result};
use ethers::types::Address;
use hyperliquid_rust_sdk::{BaseUrl, InfoClient};
use serde_json::{Value, json};
use std::{collections::HashMap, ops::Neg as _};
pub use utils::*;

pub const USER_ADDRESS: &str = "0x53Dee653941345fC1241444F7b1E7dA3beC73Aab";
pub const INITIAL_BALANCE: f64 = 47_300.0;

pub struct CryptoClient {
    user_address: Address,
    hl_client: InfoClient,
}

impl CryptoClient {
    pub async fn build() -> Result<Self> {
        let hl_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await?;
        Ok(Self {
            user_address: USER_ADDRESS.parse()?,
            hl_client,
        })
    }

    pub async fn fetch_perp_acct_value(&self) -> Result<f64> {
        let res = self.hl_client.user_state(self.user_address).await?;

        let acct_val = res.cross_margin_summary.account_value;
        let acct_val = acct_val.parse::<f64>()?;

        Ok(acct_val)
    }

    pub async fn fetch_spot_acct_value(&self) -> Result<f64> {
        let (res, mids) = tokio::join!(
            self.hl_client.user_token_balances(self.user_address),
            self.get_all_mids()
        );
        let mids = mids?;

        let mut acct_val = 0.0;

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

    pub async fn calculate_fr_open_pos(&self) -> Result<f64> {
        let res = self
            .hl_client
            .http_client
            .post(
                "/info",
                json!({
                    "type": "clearinghouseState",
                    "user": self.user_address,
                })
                .to_string(),
            )
            .await?;
        let res: Value = serde_json::from_str(&res)?;

        let mut cum_funding = 0.0;

        if let Some(asset_positions) =
            res.get("assetPositions").and_then(|ap| ap.as_array())
        {
            for ap_value in asset_positions {
                if let Some(all_time_str) = ap_value
                    .get("position")
                    .and_then(|p| p.get("cumFunding"))
                    .and_then(|cf| cf.get("allTime"))
                    .and_then(|at| at.as_str())
                {
                    cum_funding += all_time_str.parse::<f64>().unwrap_or(0.0);
                }
            }
        }

        Ok(cum_funding.neg())
    }
}

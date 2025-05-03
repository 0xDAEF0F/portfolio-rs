#![allow(dead_code, unused)] // TODO: Remove

use anyhow::Result;
use chrono::{DateTime, Utc};
use coin::Coin;
use hyperliquid_rust_sdk::{BaseUrl, CandlesSnapshotResponse, InfoClient};

mod coin;

struct Client {
    hl_client: InfoClient,
}

impl Client {
    async fn build() -> Result<Self> {
        let hl_client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await?;
        Ok(Self { hl_client })
    }

    async fn fetch_candles_snapshot(&self, coin: Coin) -> Result<Vec<CandlesSnapshotResponse>> {
        let now = chrono::Utc::now().timestamp_millis() as u64;
        let yesterday = now - 24 * 60 * 60 * 1000;

        let candles = self
            .hl_client
            .candles_snapshot(coin.to_string(), "1m".to_string(), yesterday, now)
            .await?;

        Ok(candles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context as _, anyhow};
    use chrono::{DateTime, Utc};
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test() -> Result<()> {
        let client = Client::build().await?;

        let mids = client.hl_client.all_mids().await?;
        println!("{:#?}", mids);

        let candles = client.fetch_candles_snapshot(Coin::Btc).await?;

        let (first_candle, last_candle) = candles
            .first()
            .and_then(|fst| candles.last().map(|lst| (fst, lst)))
            .context("No candles found")?;

        // Convert timestamps to DateTime and print them
        let first_date = DateTime::<Utc>::from_timestamp_millis(first_candle.time_open as i64)
            .context("Invalid timestamp")?;
        let last_date = DateTime::<Utc>::from_timestamp_millis(last_candle.time_open as i64)
            .context("Invalid timestamp")?;

        println!(
            "First candle time: {} -- Price: {}",
            first_date.format("%Y-%m-%d %H:%M:%S UTC"),
            first_candle.open
        );
        println!(
            "Last candle time: {} -- Price: {}",
            last_date.format("%Y-%m-%d %H:%M:%S UTC"),
            last_candle.open
        );

        assert!(first_candle.time_open < last_candle.time_open);

        Ok(())
    }
}

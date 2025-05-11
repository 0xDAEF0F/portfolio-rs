use anyhow::Result;
use cryprice::{CryptoClient, USER_ADDRESS, print_table};
use log::LevelFilter;
use thin_logger::ThinLogger;

const INITIAL_BALANCE: f64 = 47_300.0;

#[tokio::main]
async fn main() -> Result<()> {
    ThinLogger::new(LevelFilter::Debug)
        .external_logs(LevelFilter::Info)
        .init()
        .ok();

    let client = CryptoClient::build().await?;

    let perp = client.fetch_perp_acct_value(USER_ADDRESS).await?;
    let spot = client.fetch_spot_acct_value(USER_ADDRESS).await?;

    let total = perp + spot;

    let pnl = total - INITIAL_BALANCE;
    let pnl_pct = pnl / INITIAL_BALANCE * 100.0;

    print_table(perp, spot, total, pnl, pnl_pct);

    Ok(())
}

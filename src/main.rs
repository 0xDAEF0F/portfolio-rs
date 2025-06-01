use anyhow::Result;
use cryprice::*;
use thin_logger::log::{self, LevelFilter};

#[tokio::main]
async fn main() -> Result<()> {
	thin_logger::build(Some(LevelFilter::Info)).init();

	let client = CryptoClient::build().await?;

	let (perp, spot, fr) = tokio::try_join!(
		client.fetch_perp_acct_value(),
		client.fetch_spot_acct_value(),
		client.calculate_fr_open_pos()
	)?;

	let total = perp + spot;

	let pnl = total - INITIAL_BALANCE;
	let pnl_pct = pnl / INITIAL_BALANCE * 100.0;

	print_table(perp, spot, total, pnl, pnl_pct);

	log::info!("fr: {}", fr);

	Ok(())
}

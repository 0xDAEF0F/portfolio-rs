use anyhow::Result;
use cryprice::coinmarketcap::fetch_top_memecoins;

#[tokio::main]
async fn main() -> Result<()> {
	let coins = fetch_top_memecoins(15).await?;

	println!("Top {} Meme Tokens by Market Cap:", coins.len());
	println!("{:-<80}", "");

	for (i, coin) in coins.iter().enumerate() {
		let usd = &coin.quote.usd;
		println!("{}. {} ({})", i + 1, coin.name, coin.symbol);
		if let Some(market_cap) = usd.market_cap {
			println!("   Market Cap: ${:.2}", market_cap);
		}
		if let Some(pct_1h) = usd.percent_change_1h {
			println!("   % Change 1h: {:.2}%", pct_1h);
		}
		if let Some(pct_24h) = usd.percent_change_24h {
			println!("   % Change 24h: {:.2}%", pct_24h);
		}
		if let Some(pct_7d) = usd.percent_change_7d {
			println!("   % Change 7d: {:.2}%", pct_7d);
		}
		println!("{:-<80}", "");
	}

	Ok(())
}

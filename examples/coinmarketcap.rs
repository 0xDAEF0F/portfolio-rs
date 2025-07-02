use anyhow::Result;
use cryprice::coinmarketcap::fetch_top_memecoins;

#[tokio::main]
async fn main() -> Result<()> {
	let coins = fetch_top_memecoins(10).await?;

	println!("Top {} Meme Tokens by Market Cap:", coins.len());
	println!("{:-<80}", "");

	for (i, coin) in coins.iter().enumerate() {
		let usd = &coin.quote.usd;
		println!("{}. {} ({})", i + 1, coin.name, coin.symbol);
		if let Some(market_cap) = usd.market_cap {
			let formatted = match market_cap {
				mc if mc >= 1_000_000_000.0 => format!("{:.2}b", mc / 1_000_000_000.0),
				mc if mc >= 1_000_000.0 => format!("{:.2}m", mc / 1_000_000.0),
				mc if mc >= 1_000.0 => format!("{:.2}k", mc / 1_000.0),
				mc => format!("{:.2}", mc),
			};
			println!("   Market Cap: ${}", formatted);
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

	// calculate averages
	let (sum_24h, count_24h): (f64, usize) = coins
		.iter()
		.filter_map(|coin| coin.quote.usd.percent_change_24h)
		.fold((0.0, 0), |(sum, count), pct| (sum + pct, count + 1));

	let (sum_7d, count_7d): (f64, usize) = coins
		.iter()
		.filter_map(|coin| coin.quote.usd.percent_change_7d)
		.fold((0.0, 0), |(sum, count), pct| (sum + pct, count + 1));

	println!("\nMarket Statistics:");
	println!("{:-<80}", "");
	if count_24h > 0 {
		println!("Average 24h Change: {:.2}%", sum_24h / count_24h as f64);
	}
	if count_7d > 0 {
		println!("Average 7d Change: {:.2}%", sum_7d / count_7d as f64);
	}

	Ok(())
}

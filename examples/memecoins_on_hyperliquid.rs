use anyhow::Result;
use cryprice::coinmarketcap::fetch_top_memecoins;
use hyperliquid_rust_sdk::{AssetMeta, BaseUrl, InfoClient};
use std::collections::{HashMap, HashSet};

async fn get_hyperliquid_perps() -> Result<Vec<AssetMeta>> {
	let client = InfoClient::new(None, Some(BaseUrl::Mainnet)).await?;
	let res = client.meta().await?;
	Ok(res.universe)
}

#[tokio::main]
async fn main() -> Result<()> {
	// fetch data from both sources
	let (memecoins, hl_perps) =
		tokio::try_join!(fetch_top_memecoins(50), get_hyperliquid_perps())?;

	// create hashset for faster lookup
	let hl_perps_set: HashSet<String> = hl_perps.into_iter().map(|p| p.name).collect();

	// manual mapping for k-prefixed tokens
	let k_token_mapping: HashMap<&str, &str> = [
		("SHIB", "kSHIB"),
		("PEPE", "kPEPE"),
		("BONK", "kBONK"),
		("FLOKI", "kFLOKI"),
		("DOGS", "kDOGS"),
		("NEIRO", "kNEIRO"),
		("LUNC", "kLUNC"),
	]
	.into();

	// debug: print all hyperliquid perps
	println!("DEBUG: All Hyperliquid perps:");
	let mut sorted_perps: Vec<_> = hl_perps_set.iter().collect();
	sorted_perps.sort();
	for perp in &sorted_perps {
		println!("  - {}", perp);
	}
	println!("\n");

	// debug: print all memecoin symbols
	println!("DEBUG: Top 50 memecoin symbols from CoinMarketCap:");
	for (i, coin) in memecoins.iter().enumerate() {
		println!("  {}. {} ({})", i + 1, coin.name, coin.symbol);
	}
	println!("\n");

	println!("Top Memecoins Trading on Hyperliquid:");
	println!("{:-<100}", "");

	let mut found_count = 0;

	for (i, coin) in memecoins.iter().enumerate() {
		// check if this memecoin trades on hyperliquid
		// check both direct match and k-prefixed version
		let hl_symbol = if hl_perps_set.contains(&coin.symbol) {
			Some(coin.symbol.clone())
		} else if let Some(&k_symbol) = k_token_mapping.get(coin.symbol.as_str()) {
			if hl_perps_set.contains(k_symbol) {
				Some(k_symbol.to_string())
			} else {
				None
			}
		} else {
			None
		};

		if let Some(hl_sym) = hl_symbol {
			found_count += 1;
			let usd = &coin.quote.usd;

			println!("{}. {} ({} → {}) ✓", i + 1, coin.name, coin.symbol, hl_sym);
			if let Some(market_cap) = usd.market_cap {
				println!("   Market Cap: ${:.2}", market_cap);
			}
			if let Some(pct_24h) = usd.percent_change_24h {
				println!("   24h Change: {:.2}%", pct_24h);
			}
			println!("{:-<100}", "");
		}
	}

	println!(
		"\nFound {} memecoins (out of top {}) trading on Hyperliquid",
		found_count,
		memecoins.len()
	);

	// also show which top memecoins are NOT on hyperliquid
	println!("\nTop Memecoins NOT on Hyperliquid:");
	println!("{:-<100}", "");

	for (i, coin) in memecoins.iter().enumerate().take(20) {
		// check both direct match and k-prefixed version
		let is_on_hl = hl_perps_set.contains(&coin.symbol)
			|| k_token_mapping
				.get(coin.symbol.as_str())
				.map(|&k_sym| hl_perps_set.contains(k_sym))
				.unwrap_or(false);

		if !is_on_hl {
			let usd = &coin.quote.usd;
			println!("{}. {} ({})", i + 1, coin.name, coin.symbol);
			if let Some(market_cap) = usd.market_cap {
				println!("   Market Cap: ${:.2}", market_cap);
			}
		}
	}

	Ok(())
}

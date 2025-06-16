use anyhow::Result;
use cryprice::coinmarketcap::{Coins, fetch_top_memecoins};
use hyperliquid_rust_sdk::{AssetMeta, BaseUrl, InfoClient};
use phf::phf_map;
use std::collections::HashSet;

// coinmarketcap => hyperliquid perps
static K_TOKEN_MAPPING: phf::Map<&'static str, &'static str> = phf_map! {
	"SHIB" => "kSHIB",
	"PEPE" => "kPEPE",
	"BONK" => "kBONK",
	"FLOKI" => "kFLOKI",
	"DOGS" => "kDOGS",
	"NEIRO" => "kNEIRO",
	"LUNC" => "kLUNC",
};

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

	// normalize symbols and filter by market cap
	let normalized_memecoins: Vec<_> = memecoins
		.into_iter()
		.filter_map(|coin| {
			coin.quote.usd.market_cap.and_then(|cap| {
				if cap > 100_000_000.0 {
					let hl_symbol = K_TOKEN_MAPPING
						.get(coin.symbol.as_str())
						.map(|&s| s.to_string())
						.unwrap_or_else(|| coin.symbol.clone());
					Some((coin, hl_symbol))
				} else {
					None
				}
			})
		})
		.skip(3)
		.collect();

	// partition into coins on HL and not on HL
	let (on_hl, _not_on_hl): (Vec<_>, Vec<_>) = normalized_memecoins
		.into_iter()
		.partition(|(_, hl_symbol)| hl_perps_set.contains(hl_symbol));

	// store coins for later use
	let on_hl_coins: Vec<_> = on_hl.iter().map(|(coin, _)| coin.clone()).collect();

	// print coins trading on hyperliquid
	println!("Top Memecoins Trading on Hyperliquid (Market Cap > $500M):");
	println!("{:-<100}", "");

	on_hl.iter().enumerate().for_each(|(i, (coin, hl_symbol))| {
		println!(
			"{}. {} ({} → {}) ✓",
			i + 1,
			coin.name,
			coin.symbol,
			hl_symbol
		);
		if let Some(market_cap) = coin.quote.usd.market_cap {
			println!("   Market Cap: ${:.2}M", market_cap / 1_000_000.0);
		}
		if let Some(pct_24h) = coin.quote.usd.percent_change_24h {
			println!("   24h Change: {:.2}%", pct_24h);
		}
		println!("{:-<100}", "");
	});

	println!(
		"\nFound {} memecoins (> $500M market cap) trading on Hyperliquid",
		on_hl.len()
	);

	// calculate market cap weighted short positions
	let short_positions = calculate_market_cap_weighted_shorts(on_hl_coins.clone());

	// print the short positions table
	println!("\n\nMarket Cap Weighted Short Positions:");
	println!("{:-<80}", "");
	println!(
		"{:<5} {:<10} {:<20} {:<15} {:<15}",
		"Rank", "Symbol", "Name", "Market Cap", "Short %"
	);
	println!("{:-<80}", "");

	short_positions
		.iter()
		.enumerate()
		.for_each(|(i, (name, pct))| {
			// find the coin to get its symbol and market cap
			if let Some(coin) = on_hl_coins.iter().find(|c| c.name == *name) {
				let market_cap_str = coin
					.quote
					.usd
					.market_cap
					.map(|cap| format!("${:.2}B", cap / 1_000_000_000.0))
					.unwrap_or_else(|| "N/A".to_string());

				println!(
					"{:<5} {:<10} {:<20} {:<15} {:<15.2}%",
					i + 1,
					coin.symbol,
					if name.len() > 20 { &name[..17] } else { name },
					market_cap_str,
					pct * 100.0
				);
			}
		});

	println!("{:-<80}", "");
	println!(
		"Total allocation: {:.2}%",
		short_positions.iter().map(|(_, pct)| pct).sum::<f64>() * 100.0
	);

	Ok(())
}

/// calculates market cap weighted short positions for a portfolio of coins
/// returns a vector of (coin_name, percentage) tuples where percentage is
/// the fraction of portfolio to allocate to shorting that coin
fn calculate_market_cap_weighted_shorts(coins: Vec<Coins>) -> Vec<(String, f64)> {
	// extract coins with valid market caps
	let coins_with_caps: Vec<_> = coins
		.into_iter()
		.filter_map(|coin| coin.quote.usd.market_cap.map(|cap| (coin.name, cap)))
		.collect();

	// calculate total market cap
	let total_market_cap: f64 = coins_with_caps.iter().map(|(_, cap)| cap).sum();

	// calculate weighted percentages
	let mut weighted_positions: Vec<_> = coins_with_caps
		.into_iter()
		.map(|(name, cap)| {
			let weight = cap / total_market_cap;
			(name, weight)
		})
		.collect();

	// sort by weight descending
	weighted_positions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

	weighted_positions
}

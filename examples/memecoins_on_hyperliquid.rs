use anyhow::Result;
use cryprice::coinmarketcap::fetch_top_memecoins;
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

	// normalize symbols and filter by market cap > 500M
	let normalized_memecoins: Vec<_> = memecoins
		.into_iter()
		.filter_map(|coin| {
			coin.quote.usd.market_cap.and_then(|cap| {
				if cap > 500_000_000.0 {
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
		.collect();

	// partition into coins on HL and not on HL
	let (on_hl, _not_on_hl): (Vec<_>, Vec<_>) = normalized_memecoins
		.into_iter()
		.partition(|(_, hl_symbol)| hl_perps_set.contains(hl_symbol));

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

	Ok(())
}

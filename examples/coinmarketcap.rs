use anyhow::Result;

nestruct::flatten! {
	#[derive(Debug, serde::Deserialize)]
	ApiResponse {
		data: {
			coins: [{
				name: String,
				symbol: String,
				quote: {
					#[serde(rename = "USD")]
					usd: {
						market_cap: f64?,
						percent_change_1h: f64?,
						percent_change_24h: f64?,
						percent_change_7d: f64?,
					}
				}
			}]
		}
	}
}

async fn fetch_top_memecoins(count: usize) -> Result<Vec<Coins>> {
	// load api key from .env
	let api_key = dotenvy::var("COINMARKETCAP_API_KEY")?;

	// fetch meme category tokens
	let client = reqwest::Client::new();
	let response = client
		.get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/category")
		.query(&[
			("id", "6051a82566fc1b42617d6dc6"),
			("limit", &count.to_string()),
		])
		.header("x-cmc_pro_api_key", api_key)
		.send()
		.await?;

	let api_response: ApiResponse = response.json().await?;

	// filter out coins without market cap and take requested count
	let coins_with_market_cap: Vec<_> = api_response
		.data
		.coins
		.into_iter()
		.filter(|coin| coin.quote.usd.market_cap.is_some())
		.collect();

	// they should already be sorted by market cap from the api
	Ok(coins_with_market_cap)
}

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

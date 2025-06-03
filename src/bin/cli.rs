use anyhow::Result;
use clap::{Parser, Subcommand};
use cryprice::CryptoClient;
use std::collections::HashMap;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	/// retrieves information about currencies
	Info {
		/// symbol(s) of the currencies, e.g., "BTC", "ETH", "SOL"
		#[arg(required = true, value_delimiter = ',')]
		symbol: Vec<String>,
		/// decimal precision. default is 2
		#[arg(long, short, value_delimiter = ',')]
		decimals: Vec<u8>,
	},
}

/// normalizes symbols and decimals vectors to ensure they have matching lengths
/// if fewer decimals than symbols: fills missing decimals with default value of 2
/// if more decimals than symbols: truncates extra decimals
fn normalize_symbols_and_decimals(
	symbols: Vec<String>,
	decimals: Vec<u8>,
) -> HashMap<String, u8> {
	let default_decimal = 2u8;
	symbols
		.into_iter()
		.enumerate()
		.map(|(i, symbol)| {
			let decimal = decimals.get(i).copied().unwrap_or(default_decimal);
			(symbol.to_ascii_uppercase(), decimal)
		})
		.collect()
}

#[tokio::main]
async fn main() -> Result<()> {
	let cli = Cli::parse();

	match cli.command {
		Commands::Info { symbol, decimals } => {
			let normalized_pairs = normalize_symbols_and_decimals(symbol, decimals);

			let client = CryptoClient::build().await?;
			let coins_info = client.fetch_currency_info(normalized_pairs).await?;

			let json = serde_json::to_string_pretty(&coins_info)?;

			println!("{json}");
		}
	}

	Ok(())
}

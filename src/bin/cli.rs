use anyhow::Result;
use clap::{Parser, Subcommand};
use cryprice::CryptoClient;

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
		#[arg(required = true)]
		symbols: Vec<String>,
	},
}

#[tokio::main]
async fn main() -> Result<()> {
	let cli = Cli::parse();

	match cli.command {
		Commands::Info { symbols } => {
			let client = CryptoClient::build().await?;

			let coins_info = client.fetch_currency_info(symbols).await?;

			let json = serde_json::to_string_pretty(&coins_info)?;

			println!("{json}");
		}
	}

	Ok(())
}

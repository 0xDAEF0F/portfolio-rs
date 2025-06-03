use super::USER_ADDRESS;
use colored::*;

pub fn round_to_n_decimals(number: f64, n: u8) -> f64 {
	let factor = 10f64.powi(n as i32);
	(number * factor).round() / factor
}

// TODO: Migrate this to "pretty_table" crate
pub fn print_table(perp: f64, spot: f64, total: f64, pnl: f64, pnl_pct: f64) {
	println!(
		"\n{}",
		"╔═══════════════════════════════════════════╗".bright_yellow()
	);
	println!(
		"{}",
		"║         CRYPTO PORTFOLIO SUMMARY          ║"
			.bright_yellow()
			.bold()
	);
	println!(
		"{}",
		"╚═══════════════════════════════════════════╝".bright_yellow()
	);

	println!("{}", format!("\nAddress: {USER_ADDRESS}").bright_yellow());

	// Create table with fixed widths for better alignment
	let metric_width = 20;
	let value_width = 15;

	// Table header
	println!(
		"\n┌{:─^width$}┬{:─^width2$}┐",
		"",
		"",
		width = metric_width,
		width2 = value_width
	);
	println!(
		"│ {:<width$} │ {:>width2$} │",
		"Metric".bright_cyan().bold(),
		"Value".bright_cyan().bold(),
		width = metric_width - 2,
		width2 = value_width - 2
	);
	println!(
		"├{:─^width$}┼{:─^width2$}┤",
		"",
		"",
		width = metric_width,
		width2 = value_width
	);

	// Perpetuals Value row
	println!(
		"│ {:<width$} │ {:>width2$} │",
		"Perpetuals Value".bright_blue(),
		format!("${:.2}", perp),
		width = metric_width - 2,
		width2 = value_width - 2
	);

	// Spot Value row
	println!(
		"│ {:<width$} │ {:>width2$} │",
		"Spot Value".bright_blue(),
		format!("${:.2}", spot),
		width = metric_width - 2,
		width2 = value_width - 2
	);

	// Total Value row
	println!(
		"│ {:<width$} │ {:>width2$} │",
		"Total Value".bright_green().bold(),
		format!("${total:.2}").bright_green().bold(),
		width = metric_width - 2,
		width2 = value_width - 2
	);

	// PnL row
	let pnl_metric = if pnl >= 0.0 {
		"PnL".green().bold()
	} else {
		"PnL".red().bold()
	};

	let pnl_value = if pnl >= 0.0 {
		format!("${pnl:.2}").green().bold()
	} else {
		format!("${pnl:.2}").red().bold()
	};

	println!(
		"│ {:<width$} │ {:>width2$} │",
		pnl_metric,
		pnl_value,
		width = metric_width - 2,
		width2 = value_width - 2
	);

	// PnL % row
	let pnl_pct_metric = if pnl_pct >= 0.0 {
		"PnL %".green().bold()
	} else {
		"PnL %".red().bold()
	};

	let pnl_pct_value = if pnl_pct >= 0.0 {
		format!("{pnl_pct:.2}%").green().bold()
	} else {
		format!("{pnl_pct:.2}%").red().bold()
	};

	println!(
		"│ {:<width$} │ {:>width2$} │",
		pnl_pct_metric,
		pnl_pct_value,
		width = metric_width - 2,
		width2 = value_width - 2
	);

	// Table footer
	println!(
		"└{:─^width$}┴{:─^width2$}┘",
		"",
		"",
		width = metric_width,
		width2 = value_width
	);
}

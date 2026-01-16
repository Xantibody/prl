mod config;
mod entry;
mod format;

use chrono::{Local, NaiveDate};
use clap::Parser;
use config::{Config, ConfigError};
use entry::DateEntry;
use format::format_remaining;

#[derive(Parser)]
#[command(name = "prl")]
#[command(about = "Display remaining days until configured dates")]
struct Cli {
    /// Entry name from config file
    name: Option<String>,

    /// Direct date (YYYY-MM-DD), bypasses config file
    #[arg(long)]
    custom_date: Option<String>,

    /// Output in days (default)
    #[arg(long)]
    days: bool,

    /// Output in hours
    #[arg(long)]
    hours: bool,

    /// Output in minutes
    #[arg(long)]
    minutes: bool,

    /// Custom output format (e.g., "%Y年%d日")
    #[arg(short, long)]
    format: Option<String>,

    /// Config file path
    #[arg(short, long)]
    config: Option<String>,
}

#[derive(Debug)]
enum OutputUnit {
    Days,
    Hours,
    Minutes,
    Custom(String),
}

impl OutputUnit {
    fn from_cli(cli: &Cli) -> Self {
        if let Some(fmt) = &cli.format {
            return OutputUnit::Custom(fmt.clone());
        }
        if cli.hours {
            return OutputUnit::Hours;
        }
        if cli.minutes {
            return OutputUnit::Minutes;
        }
        OutputUnit::Days
    }

    fn format(&self, entry: &DateEntry, today: NaiveDate) -> String {
        match self {
            OutputUnit::Days => entry.remaining_days(today).to_string(),
            OutputUnit::Hours => entry.remaining_hours(today).to_string(),
            OutputUnit::Minutes => entry.remaining_minutes(today).to_string(),
            OutputUnit::Custom(fmt) => format_remaining(entry.remaining_days(today), fmt),
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match run(&cli) {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run(cli: &Cli) -> Result<String, String> {
    let today = Local::now().date_naive();
    let unit = OutputUnit::from_cli(cli);

    // Direct date mode
    if let Some(date_str) = &cli.custom_date {
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|_| format!("Invalid date format: {}", date_str))?;
        let entry = DateEntry::new(date);
        return Ok(unit.format(&entry, today));
    }

    // Config entry mode
    if let Some(name) = &cli.name {
        let config = load_config(cli)?;
        let entry = config
            .get(name)
            .ok_or_else(|| format!("Entry '{}' not found in config", name))?;
        return Ok(unit.format(entry, today));
    }

    // No arguments: show sample config
    Ok(Config::sample().to_string())
}

fn load_config(cli: &Cli) -> Result<Config, String> {
    if let Some(path) = &cli.config {
        let content =
            std::fs::read_to_string(path).map_err(|_| format!("Cannot read config: {}", path))?;
        return Config::parse_str(&content).map_err(|e| e.to_string());
    }

    match Config::load() {
        Ok(config) => Ok(config),
        Err(ConfigError::NotFound) => Err(format!(
            "Config file not found. Create one at {:?}\n\nSample config:\n{}",
            Config::config_path().unwrap_or_default(),
            Config::sample()
        )),
        Err(e) => Err(e.to_string()),
    }
}

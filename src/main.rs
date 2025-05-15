mod config;
mod data;
use clap::{Parser, Subcommand};
use config::Config;
use std::fs;

#[derive(Parser)]
#[command(name = "guardian")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Store a file
    S { filename: String },
    /// Load a file
    L { filename: String, version: u64 },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("working directory is {:?}", std::env::current_dir());
    let cli = Cli::parse();
    let contents = fs::read_to_string("config.yaml")?;
    let config: Config = serde_yaml::from_str(&contents)?;
    println!("Loaded config: {:?}", config);
    let mut data_store = data::db::DataStore { backend: None };
    data_store.load_config(config);

    match cli.command {
        Commands::S { filename } => {
            return Ok(data_store.save_file(&filename));
        }
        Commands::L { filename, version } => {
            return Ok(data_store.load_file(&filename, version));
        }
        _ => return Ok(()),
    }
}

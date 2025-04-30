mod config;
mod data;

use clap::{Parser, Subcommand};
use config::Config;
use data::db::{DataStore, FileEntry};
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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("working directory is {:?}", std::env::current_dir());
    let contents = fs::read_to_string("config.yaml")?;
    let config: Config = serde_yaml::from_str(&contents)?;
    println!("Loaded config: {:?}", config);
    let mut data_store = data::db::DataStore { backend: None };
    data_store.load_config(config);
    data_store.save_file("test.txt");

    /*    match cli.command {
            Commands::S { filename } => {
                let file = FileEntry::from_path(&filename)?;
                data_store.save_file(&file);
                println!("File saved.");
            }
        }
    */
    Ok(())
}

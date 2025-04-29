mod data;
mod config;

use std::fs;
use clap::{Parser, Subcommand};
use data::db::{FileEntry, DataStore};
use config::Config;

#[derive(Parser)]
#[command(name = "guardian")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Store a file
    S {
        filename: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
   // let cli = Cli::parse();
    //let config = Config::from_file("C:\\Users\\steev\\RustroverProjects\\guardian\\src\\config.yaml")?;
    let contents = fs::read_to_string("C:\\Users\\steev\\RustroverProjects\\guardian\\src\\config.yaml")?;
    let config: Config = serde_yaml::from_str(&contents)?;
    println!("Loaded config: {:?}", config);
    let mut data_store = data::db::DataStore{backend: None };
    data_store.load_config(config);
    data_store.save_file("C:\\Users\\steev\\RustroverProjects\\guardian\\src\\test.txt");

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

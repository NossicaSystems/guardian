mod config;
mod data;
use clap::{Parser, Subcommand};
use config::Config;
use directories_next::ProjectDirs;
use std::fs;
use std::path::PathBuf;

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
    let config_dir: PathBuf = match ProjectDirs::from("com", "NossicaSystems", "guardian") {
        Some(proj_dirs) => {
            let dir = proj_dirs.config_dir().to_path_buf();
            fs::create_dir_all(&dir)?;
            dir
        }
        None => {
            eprintln!("Unable to determine a config directory");
            std::process::exit(1);
        }
    };

    println!("config directory is {:#}", config_dir.display());
    println!("working directory is {:?}", std::env::current_dir());
    let cli = Cli::parse();
    let contents = fs::read_to_string("config.yaml")?;
    let config: Config = serde_yaml::from_str(&contents)?;
    println!("Loaded config: {:?}", config);
    let mut data_store = data::db::DataStore { backend: None };
    data_store.load_config(config);

    match cli.command {
        Commands::S { filename } => {
            data_store.save_file(&filename);
            Ok(())
        }
        Commands::L { filename, version } => {
            data_store.load_file(&filename, version);
            Ok(())
        }
        _ => Ok(()),
    }
}

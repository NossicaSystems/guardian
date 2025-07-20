mod config;
mod data;
use clap::{Parser, Subcommand};
use config::Config;
/// I'm a firm believer in minimising the number of external crates, without much code you should
/// be able to replicate what you need from this library
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
    // If you want to make this faster you could memory map it, and then read the slice of bytes,
    // it would entail a single sys call on unix, and you would be sharing memory with the kernel
    // Read to string is slow because you are going from user space to kernel space doing a sys
    // call to read the file and then another to copy the kernel buffer to a user space buffer
    //
    // This is super pedantic by the way, probably way overkill for just reading a yaml file, but
    // if in this project you will be doing large file reads the system would benefit from doing it more
    // efficiently
    let contents = fs::read_to_string("config.yaml")?;
    let config: Config = serde_yaml::from_str(&contents)?;
    println!("Loaded config: {:?}", config);
    // Here I would load the backend from the config directly, i.e. Datastore::from_config(config);
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
        // No need for this, since you are already matching against the two commands
        _ => Ok(()),
    }
}

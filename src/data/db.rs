use std::fs;
use std::path::Path;
use crate::config::Config;
mod directory_backend;

pub trait StorageBackend {
    fn save_file(&self, file_name: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn load_file(&self, file_name: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}

pub struct FileEntry {
    pub filename: String,
    pub directory: String,
    pub version: i32,
    pub contents: Vec<u8>,
}

impl FileEntry {
    pub fn from_path(path: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read(path)?;
        let filename = Path::new(path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        Ok(FileEntry {
            filename,
            directory: "/".to_string(), // add directory parsing if needed
            version: 1,
            contents,
        })
    }
}

pub struct DataStore{
    pub backend: Option<Box<dyn StorageBackend>>,
}

impl DataStore {
    fn create_backend(config: Config) -> Box<dyn StorageBackend> {
        match config {
            Config::Directory { path } => {
                Box::new(directory_backend::DirectoryBackend { path })
            },
            Config::MySql { host, port, username, password, database } => {
                // Hypothetical future implementation
                Box::new(directory_backend::DirectoryBackend { path:host })
                //Box::new(MySqlBackend::new(host, port, username, password, database))
            },
        }
    }
    pub fn load_config(&mut self, config: Config) {
        //self.config = config;
        self.backend = Some(Self::create_backend(config));
    }

    pub fn save_file(&mut self,file_name: &str) {
        self.backend.as_ref().unwrap().save_file(file_name);
        let backend = self.backend.as_ref().unwrap();
        backend.save_file(file_name);
        println!("Saved file {file_name}");
    }

}

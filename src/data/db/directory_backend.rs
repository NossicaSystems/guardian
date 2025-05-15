use crate::data::db::StorageBackend;
use regex::Regex;
use std::fs;
use std::path::Path;

pub struct DirectoryBackend {
    pub path: String,
}

impl StorageBackend for DirectoryBackend {
    fn save_file(&self, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // path is the repo location
        let path = Path::new(file_name);
        let file_name_input = path.file_name().unwrap().to_str().unwrap();

        println!("repo path {} file to save {}", self.path, file_name_input);

        // check how many other versions of this file are present
        let escaped = regex::escape(file_name_input);
        let pattern = format!(r"^{}_([0-9]+)$", escaped);
        let re = Regex::new(&pattern).unwrap();

        let mut max_num = 0;

        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            if let Some(caps) = re.captures(&file_name_str) {
                if let Some(num_str) = caps.get(1) {
                    if let Ok(num) = num_str.as_str().parse::<u32>() {
                        max_num = max_num.max(num);
                    }
                }
            }
        }
        // we now know which revision number this is
        let full_file_name = format!("{}{}_{}", self.path, file_name_input, max_num + 1);
        println!("{} copying to {}", file_name, full_file_name);
        fs::copy(file_name, full_file_name);

        Ok(())
    }

    fn load_file(&self, file_name: &str, version: u64) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(file_name);
        let file_name_input = path.file_name().unwrap().to_str().unwrap();
        let working_folder = "/tmp/guardian_working_folder/";
        let destination_file = working_folder.to_string() + file_name_input;
        let target_file = format!("{file_name_input}_{version}");
        // go through the repo directory looking for the requested file with that version number
        for entry_result in fs::read_dir(self.path.clone())? {
            let entry = entry_result?; // Each item is a Result<DirEntry, Error>
            let path = entry.path();
            let file_name_str = path.file_name().unwrap();

            if *file_name_str == *target_file {
                fs::copy(path, destination_file)?;
                return Ok(());
            }
        }

        Ok(())
    }
}

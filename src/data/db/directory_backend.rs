use std::fs;
use std::fs::File;
use std::path::Path;
use regex::Regex;
use crate::data::db::StorageBackend;

pub struct DirectoryBackend {
    pub path: String,
}

impl StorageBackend for DirectoryBackend {
    fn save_file(&self, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let full_path = format!("{}/{}", self.path, file_name);
        println!("full path {}", full_path);

        let base_name = file_name.to_string() + "_";
        let re = Regex::new(r"^test\.txt_(\d+)$").unwrap();

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

        let new_file_name = format!("{}{}", base_name, max_num + 1);
        //let mut file = File::create(Path::new(&self.path).join(new_file_name))?;
        fs::copy(file_name, new_file_name)?;


        Ok(())
    }

    fn load_file(&self, file_name: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let full_path = format!("{}/{}", self.path, file_name);
        let contents = std::fs::read(full_path)?;
        Ok(contents)
    }
}

use crate::data::db::StorageBackend;

pub struct DirectoryBackend {
    pub path: String,
}

impl StorageBackend for DirectoryBackend {
    fn save_file(&self, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let full_path = format!("{}/{}", self.path, file_name);
        println!("full path {}", full_path);
        //std::fs::write(full_path, contents)?;
        Ok(())
    }

    fn load_file(&self, file_name: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let full_path = format!("{}/{}", self.path, file_name);
        let contents = std::fs::read(full_path)?;
        Ok(contents)
    }
}

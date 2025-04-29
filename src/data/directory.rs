struct Directory {
    location: String
}

impl Directory {

    fn store_file(_filename: &str) {
        println!("saving file in Directory");
    }
    fn load_file(_filename: &str, _version: u8) {
        println!("loading file from Directory")
    }
}
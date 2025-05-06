use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "storage_type")] // <- tells serde to match by this field
pub enum Config {
    Directory {
        path: String,
    },
    MySql {
        host: String,
        port: u16,
        username: String,
        password: String,
        database: String,
    },
}

use crate::serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    db_host: String,
    db_auth: String,
}

impl ConfigFile {
    pub fn from_file(filename: &str) -> Result<ConfigFile, Box<Error>> {
        let reader = File::open(filename)?;
        let config: ConfigFile = serde_yaml::from_reader(reader)?;

        Ok(config)
    }
}


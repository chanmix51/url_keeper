use crate::serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Config {
    db_host: String,
    db_auth: String,
}

impl Config {
    pub fn from_file(filename: &str) -> Result<Config, Box<Error>> {
        let reader = File::open(filename)?;
        let config: Config = serde_yaml::from_reader(reader)?;

        Ok(config)
    }
}


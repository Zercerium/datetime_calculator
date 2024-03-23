const DEFAULT_CONFIG_FILE: &str = include_str!("../config.default.yaml");

use std::collections::HashMap;

use config::{ConfigError, File};
use home::home_dir;

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub formats: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let home_dir = home_dir();

        let mut s = config::Config::builder();

        if let Some(mut home_dir) = home_dir {
            home_dir.push(".datetime_calculator/config");
            s = s.add_source(File::with_name(home_dir.to_str().unwrap()).required(true));
        }

        let s = s.build()?;
        s.try_deserialize()
    }

    pub fn create_default_config_file() -> Result<(), std::io::Error> {
        let home_dir = home_dir().unwrap();
        let mut config_dir = home_dir.clone();
        config_dir.push("./.datetime_calculator");
        std::fs::create_dir_all(config_dir)?;

        let mut config_file = home_dir;
        config_file.push("./.datetime_calculator/config.yaml");
        std::fs::write(config_file, DEFAULT_CONFIG_FILE)?;

        Ok(())
    }
}

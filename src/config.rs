use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    #[serde(default)]
    pub vcs_paths: Vec<PathBuf>,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = "config.toml";
    let config_str = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}

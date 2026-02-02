use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct FireflyConfig {
    pub base_url: String,
    pub token: String,
}

#[derive(Deserialize)]
pub struct MatrixConfig {
    pub homeserver: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub firefly: FireflyConfig,
    pub matrix: MatrixConfig,
}

pub fn load_config(config_path: &str) -> Config {
    let config_content = fs::read_to_string(config_path).expect("Failed to read config file");
    toml::from_str(&config_content).expect("Failed to parse config file")
}
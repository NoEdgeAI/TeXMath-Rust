use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub table_dir: String,
    pub server_port: u16,
    pub server_addr: String,
}

fn load_config(filename: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;
    let config: Config = toml::from_str(&content).
        expect("load_config: Could not parse toml");
    Ok(config)
}

lazy_static!{
    static ref CONFIG: Config = load_config("config.toml").unwrap();
}

pub fn get_config() -> &'static Config {
    &CONFIG
}
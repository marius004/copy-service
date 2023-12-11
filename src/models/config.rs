use serde::Deserialize;
use anyhow::Result;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub pid_file: String,
    pub working_directory: String,
    pub stdout_file: String,
    pub stderr_file: String,
    
    pub buffer_size: usize,
    pub max_threads: usize,

    pub testing: bool, 
    pub delay: f32,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let config_str = std::fs::read_to_string(path)?;
        toml::from_str(&config_str).map_err(Into::into)
    }
}

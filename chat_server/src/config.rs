use std::{env, fs::File};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub post: u16,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // read from ./app.yaml, or /etc/config/app.yaml, or from env CHAT_CONFIG
        let ret = match (
            File::open("app.yaml"),
            File::open("/etc/config/app.yaml"),
            env::var("CHAT_CONFIG"),
        ) {
            (Ok(reader), _, _) => serde_yaml::from_reader(reader),
            (_, Ok(reader), _) => serde_yaml::from_reader(reader),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            // return Err(anyhow::anyhow!()),
            _ => bail!("Config file not found"),
        };
        Ok(ret?)
    }
}

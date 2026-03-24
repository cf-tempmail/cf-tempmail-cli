use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Session {
    pub alias: String,
    pub signature: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub baseurl: String,
    pub session: Option<Session>,
}

fn config_path() -> PathBuf {
    let dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    dir.join("cf-tempmail").join("config.toml")
}

fn ensure_dir() -> Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

pub fn load() -> Result<Config> {
    let path = config_path();
    if !path.exists() {
        return Ok(Config {
            baseurl: "https://temp-email.mrwuliu.top".to_string(),
            session: None,
        });
    }

    let content = fs::read_to_string(&path)?;
    let cfg: Config = toml::from_str(&content)?;
    Ok(cfg)
}

pub fn save_config(cfg: &Config) -> Result<()> {
    ensure_dir()?;
    let content = toml::to_string_pretty(cfg)?;
    fs::write(config_path(), content)?;
    Ok(())
}

pub fn save_baseurl(baseurl: &str) -> Result<()> {
    let mut cfg = load().unwrap_or_default();
    cfg.baseurl = baseurl.to_string();
    save_config(&cfg)
}

pub fn save_session(alias: &str, signature: &str, email: &str) -> Result<()> {
    let mut cfg = load().unwrap_or_default();
    cfg.session = Some(Session {
        alias: alias.to_string(),
        signature: signature.to_string(),
        email: email.to_string(),
    });
    save_config(&cfg)
}

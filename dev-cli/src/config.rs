use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub gemini_api_key: Option<String>,
}

impl Config {
    pub async fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path).await?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config {
                gemini_api_key: None,
            })
        }
    }

    pub async fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content).await?;

        Ok(())
    }

    pub async fn save_api_key(api_key: &str) -> Result<()> {
        let mut config = Self::load().await?;
        config.gemini_api_key = Some(api_key.to_string());
        config.save().await?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        Ok(home.join(".dev-cli").join("config.toml"))
    }
}
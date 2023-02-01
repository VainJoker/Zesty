use anyhow::{Ok, Result};
use std::fs;
use std::path::PathBuf;

use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct ZestyConfig {
    // pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub config_file: PathBuf,
}
pub static ZESTY_CONFIG: OnceCell<ZestyConfig> = OnceCell::new();

impl ZestyConfig {
    pub fn new() -> Result<ZestyConfig> {
        let config_dir = dirs::home_dir()
            .ok_or(anyhow::anyhow!("With No Home Directory"))?
            .join(".config/zesty");
        let data_dir = dirs::home_dir()
            .ok_or(anyhow::anyhow!("With No Home Directory"))?
            .join(".local/share/zesty/plugins");
        // let config_dir = std::env::current_dir()?;
        // let data_dir = std::env::current_dir()?.join("plugins");

        let config_file = config_dir.join("zesty.zsh");

        if config_dir.try_exists()? {
            println!("Config directory already exists");
        } else {
            fs::create_dir(&config_dir)?;
            println!("Created config directory");
        };

        if data_dir.try_exists()? {
            println!("Data directory already exists");
        } else {
            fs::create_dir_all(&data_dir)?;
            println!("Created data directory");
        };
        Ok(ZestyConfig {
            // config_dir,
            data_dir,
            config_file,
        })
    }

    // pub fn get_config_dir() -> PathBuf {
    //     ZESTY_CONFIG.get().expect("ZestyConfig not
    // initialized").config_dir.to_path_buf() }
    pub fn get_data_dir() -> PathBuf {
        ZESTY_CONFIG
            .get()
            .expect("ZestyConfig not initialized")
            .data_dir
            .to_path_buf()
    }

    pub fn get_config_file() -> PathBuf {
        ZESTY_CONFIG
            .get()
            .expect("ZestyConfig not initialized")
            .config_file
            .to_path_buf()
    }
}
pub fn zesty_init() -> Result<()> {
    let zest = ZestyConfig::new()?;
    ZESTY_CONFIG
        .set(zest)
        .expect("Failed to Initialize ZestyConfig");
    Ok(())
}

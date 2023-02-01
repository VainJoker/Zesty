use anyhow::Result;
use std::{fs, io::Write, ops::Index};

use crate::{plugins::LAZY_PLUGIN_MAP, zesty::ZestyConfig};

pub fn zsh_file_exist() -> Result<()> {
    let file = ZestyConfig::get_config_file();
    if file.try_exists()? {
        fs::remove_file(&file)?;
        fs::File::create(&file)?;
    } else {
        fs::File::create(&file)?;
    };
    Ok(())
}

pub fn zsh_file_append(pluginname: &str) -> Result<()> {
    let config_file = ZestyConfig::get_config_file();

    let plugins = pluginname.split("/").collect::<Vec<&str>>();
    let tmp = plugins.index(plugins.len() - 2).to_string();

    let binding = config_file.join(pluginname);
    let source_str = binding.to_str().ok_or(anyhow::anyhow!("path error"))?;

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(&config_file)
        .expect("cannot open file");

    match LAZY_PLUGIN_MAP.get() {
        Some(val) => match val.lock().expect("LAZY_PLUGIN Error").get(&tmp) {
            Some(_) => {
                file.write(format!("zsh-defer source {}\n", source_str).as_bytes())?;
            }
            None => {
                file.write(format!("source {}\n", source_str).as_bytes())?;
            }
        },
        None => {
            file.write(format!("source {}\n", source_str).as_bytes())?;
        }
    }
    Ok(())
}

pub fn write_zsh_file() -> Result<()> {
    let mut plugins = Vec::new();
    let plugin_packages = fs::read_dir(ZestyConfig::get_data_dir())?;
    for plugin_package in plugin_packages {
        let path = plugin_package?.path();
        let plugin_name = path
            .file_name()
            .ok_or(anyhow::anyhow!("plugin name error"))?
            .to_str()
            .ok_or(anyhow::anyhow!("plugin name error"))?
            .to_string()
            .to_owned();
        let plugin_files = fs::read_dir(path)?;

        for plugin_file in plugin_files {
            let path = plugin_file?
                .path()
                .to_str()
                .ok_or(anyhow::anyhow!("plugin name error"))?
                .to_string();
            if path.ends_with(&(format!("{}.plugin.zsh", plugin_name))) {
                plugins.push(path);
            } else if path.ends_with(&(format!("{}.zsh-theme", plugin_name))) {
                plugins.push(path);
            }
        }
    }
    for plugin in plugins {
        if !plugin.ends_with("zsh-defer/zsh-defer.plugin.zsh") {
            zsh_file_append(&plugin)?;
        }
    }
    Ok(())
}

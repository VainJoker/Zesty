use anyhow::Result;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::{collections::HashMap, fs, sync::Mutex};

use crate::{
    utils::git_clone,
    zesty::ZestyConfig,
    zsh::{zsh_file_append, zsh_file_exist},
};


#[derive(Debug, Deserialize)]
struct Data {
    plugins: Vec<Plugin>,
}

#[derive(Debug, Deserialize)]
struct Plugin {
    git: String,
    lazy: i32,
}

pub static LAZY_PLUGIN_MAP: OnceCell<Mutex<HashMap<String, i32>>> = OnceCell::new();

// git clone https://github.com/romkatv/zsh-defer.git ~/zsh-defer
pub fn ensure_lazy() -> Result<()>{

    let path = ZestyConfig::get_data_dir().join(r#"zsh-defer"#).to_str().ok_or(anyhow::anyhow!("path error"))?.to_owned();
    // let path = binding.to_str().ok_or(anyhow::anyhow!("path error"))?;
    let append_str = format!(r#"{}/zsh-defer.plugin.zsh"#,path);
    git_clone("https://github.com/romkatv/zsh-defer",&path)?;

    zsh_file_exist()?;

    zsh_file_append(&append_str)?;

    Ok(())
}
/// TODO delete plugins

pub fn install_plugins() -> Result<()> {
    ensure_lazy()?;

    let data = fs::read_to_string("Plugins.toml").map(|s| toml::from_str::<Data>(&s))??;
    // TODO
    // async fn download_plugin(plugin: Plugin) {
    LAZY_PLUGIN_MAP.get_or_init(|| {
        let map = HashMap::new();
        Mutex::new(map)
    });

    for plugin in data.plugins {

        // 进度条 TODO
        let pluginname = {
            let this = plugin.git.split("/").last();
            match this {
                Some(val) => val,
                None => break,
            }
        };
        println!("Installing ... {}", pluginname);

        let binding = ZestyConfig::get_data_dir().join(pluginname);
        let path = binding.to_str().ok_or(anyhow::anyhow!("path error"))?;
        git_clone(&plugin.git,path)?;

        if plugin.lazy != 0 {
            LAZY_PLUGIN_MAP
                .get()
                .expect("lazy plugin map not init")
                .lock()
                .expect("lazy plugin map not init")
                .insert(pluginname.to_string(), plugin.lazy);
        }
    }

    Ok(())
}

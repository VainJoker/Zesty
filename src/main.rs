mod plugins;
mod utils;
mod zesty;
mod zsh;

use anyhow::Result;

use plugins::install_plugins;
use zesty::zesty_init;
use zsh::write_zsh_file;

fn main() -> Result<()> {
    zesty_init()?;
    install_plugins()?;
    write_zsh_file()?;
    Ok(())
}

use std::io::{self, Write};

use anyhow::Result;

pub fn git_clone(url: &str, path: &str) -> Result<()> {
    let output = std::process::Command::new("git")
        .arg("clone")
        .arg("--depth=1")
        .arg(url)
        .arg(path)
        .output()?;
    if output.status.success() {
        io::stdout().write_all(&output.stdout)?;
    } else {
        io::stderr().write_all(&output.stderr)?;
    }
    Ok(())
}

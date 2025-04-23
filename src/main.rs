use std::{fs, path::Path, process::Command};
use anyhow::{anyhow, Context, Result};
use chrono::Local;
use colored::Colorize;
use dirs::data_local_dir;

fn main() -> Result<()> {
    let base = data_local_dir()
        .ok_or_else(|| anyhow!("Could not find LOCALAPPDATA"))?
        .join("superstart");

    if base.is_dir() {
        let today = Local::now()
            .format("%A")
            .to_string()
            .to_lowercase();

        let mut has_shortcut = false;
        for dir in [&base.join("always"), &base.join(&today)] {
            if dir.exists() {
                for entry in fs::read_dir(dir)? {
                    let path = entry?.path();
                    if path.extension()
                        .and_then(|e| e.to_str())
                        .map(|e| e.eq_ignore_ascii_case("lnk"))
                        .unwrap_or(false)
                    {
                        has_shortcut = true;
                        println!("{} {}", "Starting".bold().green(), path.display());

                        let link = path.to_string_lossy().into_owned();
                        let mut cmd = Command::new("cmd");
                        cmd.args(&["/C", "start", "", &link]);

                        match cmd.spawn() {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("   {}: {}", "Error".bold().red(), e);
                            }
                        }
                    }
                }
            }
        }

        if has_shortcut {
            Ok(())
        } else {
            setup(&base)
        }
    } else {
        setup(&base)
    }
}

fn setup(base: &Path) -> Result<()> {
    let days = [
        "always",
        "monday", "tuesday", "wednesday",
        "thursday", "friday", "saturday", "sunday",
    ];

    for day in &days {
        let dir = base.join(day);
        fs::create_dir_all(&dir)
            .with_context(|| format!("Failed to create startup folder {:?}", dir))?;
    }

    Command::new("explorer")
        .arg(base)
        .spawn()
        .with_context(|| format!("Failed to open File Explorer at {:?}", base))?;

    Ok(())
}

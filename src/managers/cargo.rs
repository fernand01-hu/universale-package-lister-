use crate::manager::{GlobalBin, PackageManager};
use anyhow::Result;
use std::process::Command;
use which::which;

pub struct CargoManager;

impl PackageManager for CargoManager {
    fn name(&self) -> String {
        "Cargo".to_string()
    }

    fn is_installed(&self) -> bool {
        which("cargo").is_ok()
    }

    fn list_globals(&self) -> Result<Vec<GlobalBin>> {
        let output = Command::new("cargo").args(["install", "--list"]).output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut bins = Vec::new();

        // Simple parsing logic for cargo install --list output:
        // "package-name v1.2.3:"
        for line in stdout.lines() {
            if line.contains(":") && !line.starts_with(" ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    bins.push(GlobalBin {
                        name: parts[0].to_string(),
                        version: Some(parts[1].trim_matches(':').to_string()),
                        manager: self.name(),
                    });
                }
            }
        }
        Ok(bins)
    }

    fn update_all(&self) -> Result<()> {
        // Cargo doesn't have a built-in update-all, often requires cargo-update crate
        println!("Note: Cargo usually requires 'cargo-update' for bulk updates.");
        Ok(())
    }

    fn uninstall(&self, package_name: &str) -> Result<()> {
        Command::new("cargo").args(["uninstall", package_name]).status()?;
        Ok(())
    }
}

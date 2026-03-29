use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GlobalBin {
    pub name: String,
    pub version: Option<String>,
    pub manager: String,
}

pub trait PackageManager {
    /// The display name of the manager (e.g., "Cargo", "NPM")
    fn name(&self) -> String;
    
    /// Checks if the manager's binary (e.g., `npm`) exists on the system
    fn is_installed(&self) -> bool;
    
    /// Lists all globally installed packages/binaries
    fn list_globals(&self) -> Result<Vec<GlobalBin>>;
    
    /// Updates all global packages for this manager
    fn update_all(&self) -> Result<()>;
    
    /// Uninstalls a specific package
    fn uninstall(&self, package_name: &str) -> Result<()>;
}

pub mod cargo;
use crate::manager::PackageManager;
use crate::managers::cargo::CargoManager;

/// Get all supported and installed managers on this machine
pub fn get_active_managers() -> Vec<Box<dyn PackageManager>> {
    let mut managers: Vec<Box<dyn PackageManager>> = Vec::new();
    
    let cargo = CargoManager;
    if cargo.is_installed() {
        managers.push(Box::new(cargo));
    }
    
    // Future managers go here
    // let npm = NpmManager;
    // if npm.is_installed() { managers.push(Box::new(npm)); }

    managers
}

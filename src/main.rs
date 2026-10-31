//! Sigma System Binary Entrypoint
use sigma_system::{SystemAdjuster, PersonaManager};

fn main() {
    println!("SigmaOS System Utility v0.2.0 (mintsystem & omarchy inspired)");
    let adjuster = SystemAdjuster::default();
    println!("Configured system adjustments: swappiness={}, governor={}", adjuster.swappiness(), adjuster.governor().as_str());

    let persona_mgr = PersonaManager::default();
    println!("Configured persona preference: {}", persona_mgr.current().label());
}

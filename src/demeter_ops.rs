//! Law of Demeter for minimizing coupling
//! Follows principles: Demeter Law, Coupling Minimisation, Information Hiding

pub struct DemeterOps;

impl DemeterOps {
    pub fn new() -> Self { Self }
    pub fn talk_to_neighbors(&self) -> bool { true }
    pub fn avoid_reaching_through(&self) -> bool { true }
}

#[cfg(test)] mod tests { use super::*; #[test] fn test_new() { let _ = DemeterOps::new(); } }

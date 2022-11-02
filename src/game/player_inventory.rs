use serde::{Serialize, Deserialize};

use super::pc::PC;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInventory {
    cases: Vec<String>,
    mbs: Vec<String>,
    cpus: Vec<String>,
    cpu_coolers: Vec<String>,
    rams: Vec<String>,
    gpus: Vec<String>,
    storages: Vec<String>,
    fans: Vec<String>,
    psus: Vec<String>,
    pcs: Vec<PC>
}

impl PlayerInventory {
    pub fn new() -> PlayerInventory {
        PlayerInventory {
            cases: Vec::new(),
            mbs: Vec::new(),
            cpus: Vec::new(),
            cpu_coolers: Vec::new(),
            rams: Vec::new(),
            gpus: Vec::new(),
            storages: Vec::new(),
            fans: Vec::new(),
            psus: Vec::new(),
            pcs: Vec::new()
        }
    }
}

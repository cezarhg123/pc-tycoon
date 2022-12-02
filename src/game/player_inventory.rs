use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInventory {
    pub cases: Vec<String>,
    pub mbs: Vec<String>,
    pub cpus: Vec<String>,
    pub cpu_coolers: Vec<String>,
    pub rams: Vec<String>,
    pub gpus: Vec<String>,
    pub storages: Vec<String>,
    pub fans: Vec<String>,
    pub psus: Vec<String>
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
            psus: Vec::new()
        }
    }
}

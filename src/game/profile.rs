use std::io::{Error, self};
use crypt::{MagicCryptTrait, generic_array::typenum::U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ItemType {
    Case,
    MB,
    CPU,
    CPUCooler,
    RAM,
    GPU,
    Storage,
    Fan,
    PSU
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub money: i32,
    pub level: u32,
    pub points: u32,
    pub items: Vec<(String, ItemType)>
}

impl Profile {
    pub fn load_profile(name: impl ToString) -> Profile {
        let decryptkey = crypt::new_magic_crypt!("pctycoon", 256);

        let profile_str = std::fs::read_to_string(format!("saves/{}.save", name.to_string())).unwrap();
        let profile_str = decryptkey.decrypt_base64_to_string(profile_str.as_str()).unwrap();

        serde_json::from_str(&profile_str).unwrap()
    }

    pub fn save_profile(&self, name: &str) {
        let encryptkey = crypt::new_magic_crypt!("pctycoon", 256);

        let profile_str = serde_json::to_string(self).unwrap();
        let profile_str = encryptkey.encrypt_str_to_base64(profile_str.as_str());

        std::fs::write(format!("saves/{name}.save"), profile_str).unwrap()
    }
}

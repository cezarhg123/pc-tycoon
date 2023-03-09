use std::io::{Error, self};
use crypt::{MagicCryptTrait, generic_array::typenum::U256, MagicCrypt256};
use serde::{Deserialize, Serialize};

use crate::log::{log, save_log};

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
    pub goal: u32,
    pub items: Vec<(String, ItemType)>
}

impl Profile {
    pub fn load_profile_path(name: impl ToString) -> Profile {
        let profile_str = std::fs::read_to_string(format!("saves/{}.save", name.to_string())).unwrap_or_else(|e| {
            log(format!("NH Err: {}", e.to_string()));
            save_log();
            panic!();
        });
        let profile_str = get_key().decrypt_base64_to_string(profile_str.as_str()).unwrap_or_else(|e| {
            log(format!("NH Err: {}", e.to_string()));
            save_log();
            panic!();
        });

        serde_json::from_str(&profile_str).unwrap_or_else(|e| {
            log(format!("NH Err: {}", e.to_string()));
            save_log();
            panic!();
        })
    }

    pub fn load_profile_json(json: impl ToString) -> Profile {
        serde_json::from_str(json.to_string().as_str()).unwrap_or_else(|e| {
            log(format!("NH Err: {}", e.to_string()));
            save_log();
            panic!();
        })
    }

    pub fn save_profile(&self, name: &str) {
        let profile_str = serde_json::to_string(self).unwrap_or_else(|e| {
            log(format!("NH Err: {}", e.to_string()));
            save_log();
            panic!();
        });
        let profile_str = get_key().encrypt_str_to_base64(profile_str.as_str());

        std::fs::write(format!("saves/{name}.save"), profile_str).unwrap_or_else(|e| {
            log(format!("NH Err: {}", e.to_string()));
            save_log();
            panic!();
        })
    }
}

static mut ENCRYPTION_KEY: Option<MagicCrypt256> = None;

pub fn get_key() -> &'static MagicCrypt256 {
    unsafe {
        ENCRYPTION_KEY.as_ref().unwrap()
    }
}

pub fn create_encryption_key() {
    unsafe {
        ENCRYPTION_KEY = Some(crypt::new_magic_crypt!("pctycoon", 256));
    }
}

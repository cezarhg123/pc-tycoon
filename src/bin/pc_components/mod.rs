use std::str::FromStr;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MBFormFactor {
    ATX,
    MicroATX
}

impl FromStr for MBFormFactor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "atx" => {
                Ok(Self::ATX)
            }
            "micro atx" => {
                Ok(Self::MicroATX)
            }
            _ => {
                Err("input is not 'atx' or 'micro atx'".to_string())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SocketType {
    AM4,
    LGA1151,
    LGA1200
}

impl FromStr for SocketType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "am4" => {
                Ok(Self::AM4)
            }
            "lga1151" => {
                Ok(Self::LGA1151)
            }
            "lga1200" => {
                Ok(Self::LGA1200)
            }
            _ => {
                Err("input is not 'am4', 'lga1151' or 'lga1200'".to_string())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CaseFormFactor {
    FullTower,
    MidTower
}

impl FromStr for CaseFormFactor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "full tower" => {
                Ok(Self::FullTower)
            }
            "mid tower" => {
                Ok(Self::MidTower)
            }
            _ => {
                Err("input not 'full tower' or 'mid tower'".to_string())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RamType {
    DDR3,
    DDR4
}

impl FromStr for RamType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "ddr3" => {
                Ok(Self::DDR3)
            }
            "ddr4" => {
                Ok(Self::DDR4)
            }
            _ => {
                Err("input not 'ddr3' or 'ddr4'".to_string())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StorageType {
    M2,
    SSD,
    HDD
}

impl FromStr for StorageType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "m2" => {
                Ok(Self::M2)
            }
            "ssd" => {
                Ok(Self::SSD)
            }
            "hdd" => {
                Ok(Self::HDD)
            }
            _ => {
                Err("input not 'm2', 'ssd' or 'hdd'".to_string())
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Case {
    pub name: String,
    pub price: u32,
    pub case_form_factor: CaseFormFactor,
    pub mb_form_factor: MBFormFactor,
    pub max_fans: u32,
    pub max_ssd: u32,
    pub max_hdd: u32,
    pub max_cpu_cooler_height: u32,
    pub max_gpu_length: u32,
    pub max_gpu_width: u32,
    pub max_power_supply_length: u32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MB {
    pub name: String,
    pub price: u32,
    pub power_usage: u32,
    pub mb_form_factor: MBFormFactor,
    pub socket_type: SocketType,
    pub ram_type: RamType,
    pub ram_slots: u32,
    pub m2_slots: u32,
    pub sata_slots: u32,
    pub max_cpu_speed: u32,
    pub max_ram_speed: u32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPU {
    pub name: String,
    pub price: u32,
    pub power_usage: u32,
    pub socket_type: SocketType,
    pub base: f32, // use base to change score based on architecture. e.g zen1-zen2
    pub cores: u32,
    pub threads: u32,
    pub speed: u32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPUCooler {
    pub name: String,
    pub price: u32,
    pub power_usage: u32,
    pub socket_type: SocketType,
    pub height: u32,
    pub cooling: f32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAM {
    pub name: String,
    pub price: u32,
    pub power_usage: u32,
    pub ram_type: RamType,
    pub size: u32,
    pub speed: u32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPU {
    pub name: String,
    pub price: u32,
    pub power_usage: u32,
    pub length: u32,
    pub width: u32,
    pub cores: u32,
    pub rt_cores: u32,
    pub speed: u32,
    pub vram: u32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storage {
    pub name: String,
    pub price: u32,
    pub power_usage: u32,
    pub storage_type: StorageType,
    pub size: u32,
    pub speed: u32 // MB/S
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fan {
    pub name: String,
    pub price: u32,
    pub power_usage: u32,
    pub large: bool,
    pub cooling: f32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PSU {
    pub name: String,
    pub price: u32,
    pub wattage: u32,
    pub length: u32
}

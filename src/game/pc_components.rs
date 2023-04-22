use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MBFormFactor {
    ATX,
    MicroATX
}

impl ToString for MBFormFactor {
    fn to_string(&self) -> String {
        match self {
            MBFormFactor::ATX => {"ATX".to_string()}
            MBFormFactor::MicroATX => {"Micro ATX".to_string()}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SocketType {
    AM4,
    AM5,
    LGA1151,
    LGA1200
}

impl ToString for SocketType {
    fn to_string(&self) -> String {
        match self {
            SocketType::AM4 => {"AM4".to_string()}
            SocketType::AM5 => {"AM5".to_string()}
            SocketType::LGA1151 => {"LGA1151".to_string()}
            SocketType::LGA1200 => {"LGA1200".to_string()}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CaseFormFactor {
    FullTower,
    MidTower
}

impl ToString for CaseFormFactor {
    fn to_string(&self) -> String {
        match self {
            CaseFormFactor::FullTower => {"Full Tower".to_string()}
            CaseFormFactor::MidTower => {"Mid Tower".to_string()}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RamType {
    DDR3,
    DDR4,
    DDR5
}

impl ToString for RamType {
    fn to_string(&self) -> String {
        match self {
            RamType::DDR3 => {"DDR3".to_string()}
            RamType::DDR4 => {"DDR4".to_string()}
            RamType::DDR5 => {"DDR5".to_string()}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum StorageType {
    M2,
    SSD,
    HDD
}

impl ToString for StorageType {
    fn to_string(&self) -> String {
        match self {
            StorageType::M2 => {"M2".to_string()}
            StorageType::SSD => {"SSD".to_string()}
            StorageType::HDD => {"HDD".to_string()}
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
    pub flow: f32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PSU {
    pub name: String,
    pub price: u32,
    pub wattage: u32,
    pub length: u32
}

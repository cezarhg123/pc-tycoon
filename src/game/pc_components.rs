use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MBFormFactor {
    ATX,
    MicroATX
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SocketType {
    AM4,
    LGA1151,
    LGA1200
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CaseFormFactor {
    FullTower,
    MidTower
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RamType {
    DDR3,
    DDR4
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StorageType {
    M2,
    SSD,
    HDD
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Case {
    pub name: String,
    pub alias: String,
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
    pub alias: String,
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
    pub alias: String,
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
    pub alias: String,
    pub price: u32,
    pub power_usage: u32,
    pub socket_type: SocketType,
    pub height: u32,
    pub cooling: f32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAM {
    pub name: String,
    pub alias: String,
    pub price: u32,
    pub power_usage: u32,
    pub ram_type: RamType,
    pub size: u32,
    pub speed: u32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPU {
    pub name: String,
    pub alias: String,
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
    pub alias: String,
    pub price: u32,
    pub power_usage: u32,
    pub storage_type: StorageType,
    pub size: u32,
    pub speed: u32 // MB/S
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fan {
    pub name: String,
    pub alias: String,
    pub price: u32,
    pub power_usage: u32,
    pub large: bool,
    pub cooling: f32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PSU {
    pub name: String,
    pub alias: String,
    pub price: u32,
    pub wattage: u32,
    pub length: u32
}

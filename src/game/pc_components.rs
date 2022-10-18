pub mod case {
    use serde::{Serialize, Deserialize};
    use super::power_supply::PowerSupplyFormFactor;

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub enum Size {
        Atx,
        MircroAtx
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Case {
        pub name: String,
        pub alias: String,
        pub size: Size,
        pub max_fans: u32,
        pub max_cpu_cooler_height: u32,
        pub max_gpu_length: u32,
        pub max_gpu_width: u32,
        pub max_power_supply_length: u32,
        pub power_supply_form_factor: PowerSupplyFormFactor,
        pub max_drives_2_5: u32,
        pub max_drives_3_5: u32,
        pub price: u32
    }
}

pub mod motherboard {
    use serde::{Serialize, Deserialize};
    use super::case::Size;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum SocketType {
        Am4,
        Lga
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct MotherBoard {
        pub name: String,
        pub alias: String,
        pub size: Size,
        pub socket_type: SocketType,
        pub max_cpu_speed: u32,
        pub max_ram_speed: u32,
        pub ram_slots: u32,
        pub m2_slots: u32,
        pub max_storage_devices: u32,
        pub price: u32
    }
}

pub mod cpu {
    use serde::{Deserialize, Serialize};
    use super::motherboard::SocketType;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Cpu {
        pub name: String,
        pub alias: String,
        pub socket_type: SocketType,
        pub base_multiplier: f32,
        pub cores: u32,
        pub threads: u32,
        pub speed: u32,
        pub power_usage: u32,
        pub price: u32
    }
}

pub mod cpu_cooler {
    use serde::{Serialize, Deserialize};
    use super::motherboard::SocketType;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CpuCooler {
        pub name: String,
        pub alias: String,
        pub socket_type: SocketType,
        pub height: u32,
        pub base: f32,
        pub power_usage: u32,
        pub price: u32
    }
}

pub mod ram {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Ram {
        pub name: String,
        pub alias: String,
        pub size: u32,
        pub speed: u32,
        pub price: u32
    }
}

pub mod gpu {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Gpu {
        pub name: String,
        pub alias: String,
        pub cores: u32,
        pub rt_cores: u32,
        pub speed: u32,
        pub vram: u32,
        pub length: u32,
        pub width: u32,
        pub cooling: f32,
        pub power_usage: u32,
        pub price: u32
    }
}

pub mod storage_device {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum StorageDeviceType {
        M2,
        Ssd,
        Hdd
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct StorageDevice {
        pub name: String,
        pub alias: String,
        pub storage_device_type: StorageDeviceType,
        pub size: u32, //in GB
        pub price: u32
    }
}

pub mod fan {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Fan {
        pub name: String,
        pub alias: String,
        pub large: bool,
        pub effectiveness: f32,
        pub price: u32
    }
}

pub mod power_supply {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum PowerSupplyFormFactor {
        Atx
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PowerSupply {
        pub name: String,
        pub alias: String,
        pub form_factor: PowerSupplyFormFactor,
        pub length: u32,
        pub wattage: u32,
        pub price: u32
    }
}

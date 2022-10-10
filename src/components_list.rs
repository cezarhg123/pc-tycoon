use std::collections::HashMap;

use crate::game::pc_components::{case::Case, motherboard::MotherBoard, cpu::Cpu, cpu_cooler::CpuCooler, ram::Ram, gpu::Gpu, storage_device::StorageDevice, fan::Fan, power_supply::PowerSupply};

static mut case_list: Option<Vec<Case>> = None;
static mut motherboard_list: Option<Vec<MotherBoard>> = None;
static mut cpu_list: Option<Vec<Cpu>> = None;
static mut cpu_cooler_list: Option<Vec<CpuCooler>> = None;
static mut ram_list: Option<Vec<Ram>> = None;
static mut gpu_list: Option<Vec<Gpu>> = None;
static mut storage_list: Option<Vec<StorageDevice>> = None;
static mut fan_list: Option<Vec<Fan>> = None;
static mut power_supply_list: Option<Vec<PowerSupply>> = None;

pub fn load_all_components() {
    unsafe {
        case_list = Some(Vec::new());
        motherboard_list = Some(Vec::new());
        cpu_list = Some(Vec::new());
        cpu_cooler_list = Some(Vec::new());
        ram_list = Some(Vec::new());
        gpu_list = Some(Vec::new());
        storage_list = Some(Vec::new());
        fan_list = Some(Vec::new());
        power_supply_list = Some(Vec::new());
    }

    let case_entrys = std::fs::read_dir("parts/case/").unwrap();
    for entry in case_entrys {
        let json = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let mut case: Case = serde_json::from_str(json.as_str()).unwrap();
        case.name.push('\0');
        unsafe {
            case_list.as_mut().unwrap().push(case);
        }
    }

    let motherboard_entrys = std::fs::read_dir("parts/motherboard/").unwrap();
    for entry in motherboard_entrys {
        let json = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let mut motherboard: MotherBoard = serde_json::from_str(json.as_str()).unwrap();
        motherboard.name.push('\0');
        unsafe {
            motherboard_list.as_mut().unwrap().push(motherboard);
        }
    }

    let cpu_entrys = std::fs::read_dir("parts/cpu/").unwrap();
    for entry in cpu_entrys {
        let json = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let mut cpu: Cpu = serde_json::from_str(json.as_str()).unwrap();
        cpu.name.push('\0');
        unsafe {
            cpu_list.as_mut().unwrap().push(cpu);
        }
    }

    let cpu_cooler_entrys = std::fs::read_dir("parts/cpu_cooler/").unwrap();
    for entry in cpu_cooler_entrys {
        let json = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let mut cpu_cooler: CpuCooler = serde_json::from_str(json.as_str()).unwrap();
        cpu_cooler.name.push('\0');
        unsafe {
            cpu_cooler_list.as_mut().unwrap().push(cpu_cooler);
        }
    }

    let ram_entrys = std::fs::read_dir("parts/ram/").unwrap();
    for entry in ram_entrys {
        let json = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let mut ram: Ram = serde_json::from_str(json.as_str()).unwrap();
        ram.name.push('\0');
        unsafe {
            ram_list.as_mut().unwrap().push(ram);
        }
    }

    let gpu_entrys = std::fs::read_dir("parts/gpu/").unwrap();
    for entry in gpu_entrys {
        let json = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let mut gpu: Gpu = serde_json::from_str(json.as_str()).unwrap();
        gpu.name.push('\0');
        unsafe {
            gpu_list.as_mut().unwrap().push(gpu);
        }
    }

    let storage_entrys = std::fs::read_dir("parts/storage/").unwrap();
    for entry in storage_entrys {
        let json = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let mut storage: StorageDevice = serde_json::from_str(json.as_str()).unwrap();
        storage.name.push('\0');
        unsafe {
            storage_list.as_mut().unwrap().push(storage);
        }
    }

    let fan_entrys = std::fs::read_dir("parts/fan/").unwrap();
    for entry in fan_entrys {
        let json = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let mut fan: Fan = serde_json::from_str(json.as_str()).unwrap();
        fan.name.push('\0');
        unsafe {
            fan_list.as_mut().unwrap().push(fan);
        }
    }

    let power_supply_entrys = std::fs::read_dir("parts/power_supply/").unwrap();
    for entry in power_supply_entrys {
        let json = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let mut power_supply: PowerSupply = serde_json::from_str(json.as_str()).unwrap();
        power_supply.name.push('\0');
        unsafe {
            power_supply_list.as_mut().unwrap().push(power_supply);
        }
    }
}

pub fn get_case_list<'a>() -> &'a Vec<Case> {
    unsafe {
        case_list.as_ref().unwrap()
    }
}

pub fn get_motherboard_list<'a>() -> &'a Vec<MotherBoard> {
    unsafe {
        motherboard_list.as_ref().unwrap()
    }
}

pub fn get_cpu_list<'a>() -> &'a Vec<Cpu> {
    unsafe {
        cpu_list.as_ref().unwrap()
    }
}

pub fn get_cpu_cooler_list<'a>() -> &'a Vec<CpuCooler> {
    unsafe {
        cpu_cooler_list.as_ref().unwrap()
    }
}

pub fn get_ram_list<'a>() -> &'a Vec<Ram> {
    unsafe {
        ram_list.as_ref().unwrap()
    }
}

pub fn get_gpu_list<'a>() -> &'a Vec<Gpu> {
    unsafe {
        gpu_list.as_ref().unwrap()
    }
}

pub fn get_storage_list<'a>() -> &'a Vec<StorageDevice> {
    unsafe {
        storage_list.as_ref().unwrap()
    }
}

pub fn get_fan_list<'a>() -> &'a Vec<Fan> {
    unsafe {
        fan_list.as_ref().unwrap()
    }
}

pub fn get_power_supply_list<'a>() -> &'a Vec<PowerSupply> {
    unsafe {
        power_supply_list.as_ref().unwrap()
    }
}

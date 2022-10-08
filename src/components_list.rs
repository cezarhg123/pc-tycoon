use std::collections::HashMap;

use crate::game::pc_components::{case::Case, motherboard::MotherBoard, cpu::Cpu, cpu_cooler::CpuCooler, ram::Ram, gpu::Gpu, storage_device::StorageDevice, fan::Fan, power_supply::PowerSupply};

static mut case_list: Option<HashMap<String, Case>> = None;
static mut motherboard_list: Option<HashMap<String, MotherBoard>> = None;
static mut cpu_list: Option<HashMap<String, Cpu>> = None;
static mut cpu_cooler_list: Option<HashMap<String, CpuCooler>> = None;
static mut ram_list: Option<HashMap<String, Ram>> = None;
static mut gpu_list: Option<HashMap<String, Gpu>> = None;
static mut storage_list: Option<HashMap<String, StorageDevice>> = None;
static mut fan_list: Option<HashMap<String, Fan>> = None;
static mut power_supply_list: Option<HashMap<String, PowerSupply>> = None;

pub fn load_all_components() {
    unsafe {
        case_list = Some(HashMap::new());
        motherboard_list = Some(HashMap::new());
        cpu_list = Some(HashMap::new());
        cpu_cooler_list = Some(HashMap::new());
        ram_list = Some(HashMap::new());
        gpu_list = Some(HashMap::new());
        storage_list = Some(HashMap::new());
        fan_list = Some(HashMap::new());
        power_supply_list = Some(HashMap::new());
    }

    let case_entrys = std::fs::read_dir("parts/case/").unwrap();
    for entry in case_entrys {
        let json = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let case: Case = serde_json::from_str(json.as_str()).unwrap();
        unsafe {
            case_list.as_mut().unwrap().insert(case.name.clone(), case);
        }
    }

    let motherboard_entrys = std::fs::read_dir("parts/motherboard/").unwrap();
    for entry in motherboard_entrys {
        let json = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let motherboard: MotherBoard = serde_json::from_str(json.as_str()).unwrap();
        unsafe {
            motherboard_list.as_mut().unwrap().insert(motherboard.name.clone(), motherboard);
        }
    }

    let cpu_entrys = std::fs::read_dir("parts/cpu/").unwrap();
    for entry in cpu_entrys {
        let json = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let cpu: Cpu = serde_json::from_str(json.as_str()).unwrap();
        unsafe {
            cpu_list.as_mut().unwrap().insert(cpu.name.clone(), cpu);
        }
    }

    let cpu_cooler_entrys = std::fs::read_dir("parts/cpu_cooler/").unwrap();
    for entry in cpu_cooler_entrys {
        let json = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let cpu_cooler: CpuCooler = serde_json::from_str(json.as_str()).unwrap();
        unsafe {
            cpu_cooler_list.as_mut().unwrap().insert(cpu_cooler.name.clone(), cpu_cooler);
        }
    }

    let ram_entrys = std::fs::read_dir("parts/ram/").unwrap();
    for entry in ram_entrys {
        let json = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let ram: Ram = serde_json::from_str(json.as_str()).unwrap();
        unsafe {
            ram_list.as_mut().unwrap().insert(ram.name.clone(), ram);
        }
    }

    let gpu_entrys = std::fs::read_dir("parts/gpu/").unwrap();
    for entry in gpu_entrys {
        let json = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let gpu: Gpu = serde_json::from_str(json.as_str()).unwrap();
        unsafe {
            gpu_list.as_mut().unwrap().insert(gpu.name.clone(), gpu);
        }
    }

    let storage_entrys = std::fs::read_dir("parts/storage/").unwrap();
    for entry in storage_entrys {
        let json = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let storage: StorageDevice = serde_json::from_str(json.as_str()).unwrap();
        unsafe {
            storage_list.as_mut().unwrap().insert(storage.name.clone(), storage);
        }
    }

    let fan_entrys = std::fs::read_dir("parts/fan/").unwrap();
    for entry in fan_entrys {
        let json = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let fan: Fan = serde_json::from_str(json.as_str()).unwrap();
        unsafe {
            fan_list.as_mut().unwrap().insert(fan.name.clone(), fan);
        }
    }

    let power_supply_entrys = std::fs::read_dir("parts/power_supply/").unwrap();
    for entry in power_supply_entrys {
        let json = std::fs::read_to_string(entry.unwrap().path()).unwrap();
        let power_supply: PowerSupply = serde_json::from_str(json.as_str()).unwrap();
        unsafe {
            power_supply_list.as_mut().unwrap().insert(power_supply.name.clone(), power_supply);
        }
    }
}
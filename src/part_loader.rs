use std::collections::HashMap;
use crate::game::pc_components::*;

#[derive(Debug, Clone)]
struct PartList<T> {
    part_names: Vec<String>,
    part_map: HashMap<String, T>
}

static mut unsafe_case_list: Option<PartList<Case>> = None;
static mut unsafe_mb_list: Option<PartList<MB>> = None;
static mut unsafe_cpu_list: Option<PartList<CPU>> = None;
static mut unsafe_cpu_cooler_list: Option<PartList<CPUCooler>> = None;
static mut unsafe_ram_list: Option<PartList<RAM>> = None;
static mut unsafe_gpu_list: Option<PartList<GPU>> = None;
static mut unsafe_storage_list: Option<PartList<Storage>> = None;
static mut unsafe_fan_list: Option<PartList<Fan>> = None;
static mut unsafe_psu_list: Option<PartList<PSU>> = None;

pub fn load_parts() {
    let mut case_list = PartList::<Case> {
        part_names: Vec::new(),
        part_map: HashMap::new()
    };

    let case_dir = std::fs::read_dir("parts/cases/").unwrap();
    for entry in case_dir {
        match entry {
            Ok(entry) => {
                let case: Case = serde_json::from_slice(
                    std::fs::read(entry.path()).unwrap().as_slice()
                ).unwrap();

                case_list.part_names.push(case.name.clone());
                case_list.part_map.insert(case.name.clone(), case);
            }
            Err(_) => {}
        }
    }

    unsafe {
        unsafe_case_list = Some(case_list);
    }

    let mut mb_list = PartList::<MB> {
        part_names: Vec::new(),
        part_map: HashMap::new()
    };

    let mb_dir = std::fs::read_dir("parts/motherboards/").unwrap();
    for entry in mb_dir {
        match entry {
            Ok(entry) => {
                let mb: MB = serde_json::from_slice(
                    std::fs::read(entry.path()).unwrap().as_slice()
                ).unwrap();

                mb_list.part_names.push(mb.name.clone());
                mb_list.part_map.insert(mb.name.clone(), mb);
            }
            Err(_) => {}
        }
    }

    unsafe {
        unsafe_mb_list = Some(mb_list);
    }

    let mut cpu_list = PartList::<CPU> {
        part_names: Vec::new(),
        part_map: HashMap::new()
    };

    let cpu_dir = std::fs::read_dir("parts/cpus/").unwrap();
    for entry in cpu_dir {
        match entry {
            Ok(entry) => {
                let cpu: CPU = serde_json::from_slice(
                    std::fs::read(entry.path()).unwrap().as_slice()
                ).unwrap();

                cpu_list.part_names.push(cpu.name.clone());
                cpu_list.part_map.insert(cpu.name.clone(), cpu);
            }
            Err(_) => {}
        }
    }

    unsafe {
        unsafe_cpu_list = Some(cpu_list);
    }

    let mut cpu_cooler_list = PartList::<CPUCooler> {
        part_names: Vec::new(),
        part_map: HashMap::new()
    };

    let cpu_cooler_dir = std::fs::read_dir("parts/cpu coolers/").unwrap();
    for entry in cpu_cooler_dir {
        match entry {
            Ok(entry) => {
                let cpu_cooler: CPUCooler = serde_json::from_slice(
                    std::fs::read(entry.path()).unwrap().as_slice()
                ).unwrap();

                cpu_cooler_list.part_names.push(cpu_cooler.name.clone());
                cpu_cooler_list.part_map.insert(cpu_cooler.name.clone(), cpu_cooler);
            }
            Err(_) => {}
        }
    }

    unsafe {
        unsafe_cpu_cooler_list = Some(cpu_cooler_list);
    }

    let mut ram_list = PartList::<RAM> {
        part_names: Vec::new(),
        part_map: HashMap::new()
    };

    let ram_dir = std::fs::read_dir("parts/rams/").unwrap();
    for entry in ram_dir {
        match entry {
            Ok(entry) => {
                let ram: RAM = serde_json::from_slice(
                    std::fs::read(entry.path()).unwrap().as_slice()
                ).unwrap();

                ram_list.part_names.push(ram.name.clone());
                ram_list.part_map.insert(ram.name.clone(), ram);
            }
            Err(_) => {}
        }
    }

    unsafe {
        unsafe_ram_list = Some(ram_list);
    }

    let mut gpu_list = PartList::<GPU> {
        part_names: Vec::new(),
        part_map: HashMap::new()
    };

    let gpu_dir = std::fs::read_dir("parts/gpus/").unwrap();
    for entry in gpu_dir {
        match entry {
            Ok(entry) => {
                let gpu: GPU = serde_json::from_slice(
                    std::fs::read(entry.path()).unwrap().as_slice()
                ).unwrap();

                gpu_list.part_names.push(gpu.name.clone());
                gpu_list.part_map.insert(gpu.name.clone(), gpu);
            }
            Err(_) => {}
        }
    }

    unsafe {
        unsafe_gpu_list = Some(gpu_list);
    }

    let mut storage_list = PartList::<Storage> {
        part_names: Vec::new(),
        part_map: HashMap::new()
    };

    let storage_dir = std::fs::read_dir("parts/storages/").unwrap();
    for entry in storage_dir {
        match entry {
            Ok(entry) => {
                let storage: Storage = serde_json::from_slice(
                    std::fs::read(entry.path()).unwrap().as_slice()
                ).unwrap();

                storage_list.part_names.push(storage.name.clone());
                storage_list.part_map.insert(storage.name.clone(), storage);
            }
            Err(_) => {}
        }
    }

    unsafe {
        unsafe_storage_list = Some(storage_list);
    }

    let mut fan_list = PartList::<Fan> {
        part_names: Vec::new(),
        part_map: HashMap::new()
    };

    let fan_dir = std::fs::read_dir("parts/fans/").unwrap();
    for entry in fan_dir {
        match entry {
            Ok(entry) => {
                let fan: Fan = serde_json::from_slice(
                    std::fs::read(entry.path()).unwrap().as_slice()
                ).unwrap();

                fan_list.part_names.push(fan.name.clone());
                fan_list.part_map.insert(fan.name.clone(), fan);
            }
            Err(_) => {}
        }
    }

    unsafe {
        unsafe_fan_list = Some(fan_list);
    }

    let mut power_supply_list = PartList::<PSU> {
        part_names: Vec::new(),
        part_map: HashMap::new()
    };

    let power_supply_dir = std::fs::read_dir("parts/power supplies/").unwrap();
    for entry in power_supply_dir {
        match entry {
            Ok(entry) => {
                let power_supply: PSU = serde_json::from_slice(
                    std::fs::read(entry.path()).unwrap().as_slice()
                ).unwrap();

                power_supply_list.part_names.push(power_supply.name.clone());
                power_supply_list.part_map.insert(power_supply.name.clone(), power_supply);
            }
            Err(_) => {}
        }
    }

    unsafe {
        unsafe_psu_list = Some(power_supply_list);
    }

    #[cfg(debug_assertions)]
    {
        let case_num;
        let mb_num;
        let cpu_num;
        let cpu_cooler_num;
        let ram_num;
        let gpu_num;
        let storage_num;
        let fan_num;
        let psu_num;
        
        unsafe {
            case_num = unsafe_case_list.as_ref().unwrap().part_names.len();
            mb_num = unsafe_mb_list.as_ref().unwrap().part_names.len();
            cpu_num = unsafe_cpu_list.as_ref().unwrap().part_names.len();
            cpu_cooler_num = unsafe_cpu_cooler_list.as_ref().unwrap().part_names.len();
            ram_num = unsafe_ram_list.as_ref().unwrap().part_names.len();
            gpu_num = unsafe_gpu_list.as_ref().unwrap().part_names.len();
            storage_num = unsafe_storage_list.as_ref().unwrap().part_names.len();
            fan_num = unsafe_fan_list.as_ref().unwrap().part_names.len();
            psu_num = unsafe_psu_list.as_ref().unwrap().part_names.len();
        }
        
        let total = case_num + mb_num + cpu_num + cpu_cooler_num + ram_num + gpu_num + storage_num + fan_num + psu_num;
        println!("info: Loaded {} Cases", case_num);
        println!("info: Loaded {} Motherboards", mb_num);
        println!("info: Loaded {} CPUs", cpu_num);
        println!("info: Loaded {} CPU Coolers", cpu_cooler_num);
        println!("info: Loaded {} RAMs", ram_num);
        println!("info: Loaded {} GPUs", gpu_num);
        println!("info: Loaded {} Storages", storage_num);
        println!("info: Loaded {} Fans", fan_num);
        println!("info: Loaded {} PSUs", psu_num);
        println!("info: Loaded {} Total Parts", total);
    }
}

pub fn get_case_names<'a>() -> &'a [String] {
    unsafe {
        unsafe_case_list.as_ref().unwrap().part_names.as_slice()
    }
}

pub fn get_case(name: &str) -> Case {
    unsafe {
        unsafe_case_list.as_ref().unwrap().part_map.get(name).unwrap().clone()
    }
}

pub fn get_mb_names<'a>() -> &'a [String] {
    unsafe {
        unsafe_mb_list.as_ref().unwrap().part_names.as_slice()
    }
}

pub fn get_mb(name: &str) -> MB {
    unsafe {
        unsafe_mb_list.as_ref().unwrap().part_map.get(name).unwrap().clone()
    }
}

pub fn get_cpu_names<'a>() -> &'a [String] {
    unsafe {
        unsafe_cpu_list.as_ref().unwrap().part_names.as_slice()
    }
}

pub fn get_cpu(name: &str) -> CPU {
    unsafe {
        unsafe_cpu_list.as_ref().unwrap().part_map.get(name).unwrap().clone()
    }
}

pub fn get_cpu_cooler_names<'a>() -> &'a [String] {
    unsafe {
        unsafe_cpu_cooler_list.as_ref().unwrap().part_names.as_slice()
    }
}

pub fn get_cpu_cooler(name: &str) -> CPUCooler {
    unsafe {
        unsafe_cpu_cooler_list.as_ref().unwrap().part_map.get(name).unwrap().clone()
    }
}

pub fn get_ram_names<'a>() -> &'a [String] {
    unsafe {
        unsafe_ram_list.as_ref().unwrap().part_names.as_slice()
    }
}

pub fn get_ram(name: &str) -> RAM {
    unsafe {
        unsafe_ram_list.as_ref().unwrap().part_map.get(name).unwrap().clone()
    }
}

pub fn get_gpu_names<'a>() -> &'a [String] {
    unsafe {
        unsafe_gpu_list.as_ref().unwrap().part_names.as_slice()
    }
}

pub fn get_gpu(name: &str) -> GPU {
    unsafe {
        unsafe_gpu_list.as_ref().unwrap().part_map.get(name).unwrap().clone()
    }
}

pub fn get_storage_names<'a>() -> &'a [String] {
    unsafe {
        unsafe_storage_list.as_ref().unwrap().part_names.as_slice()
    }
}

pub fn get_storage(name: &str) -> Storage {
    unsafe {
        unsafe_storage_list.as_ref().unwrap().part_map.get(name).unwrap().clone()
    }
}

pub fn get_fan_names<'a>() -> &'a [String] {
    unsafe {
        unsafe_fan_list.as_ref().unwrap().part_names.as_slice()
    }
}

pub fn get_fan(name: &str) -> Fan {
    unsafe {
        unsafe_fan_list.as_ref().unwrap().part_map.get(name).unwrap().clone()
    }
}

pub fn get_psu_names<'a>() -> &'a [String] {
    unsafe {
        unsafe_psu_list.as_ref().unwrap().part_names.as_slice()
    }
}

pub fn get_psu(name: &str) -> PSU {
    unsafe {
        unsafe_psu_list.as_ref().unwrap().part_map.get(name).unwrap().clone()
    }
}

mod pc_components;

use std::{io::Write, str::FromStr};
use pc_components::*;

fn main() {
    println!("Part Maker for PC Tycoon");
    println!("Choose What Part to Create with 1-9");
    println!("1. Case");
    println!("2. Motherboard");
    println!("3. CPU");
    println!("4. CPU Cooler");
    println!("5. RAM");
    println!("6. GPU");
    println!("7. Storage");
    println!("8. Fan");
    println!("9. Power Supply");

    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer).unwrap();
    let answer: u32 = answer.trim().parse().unwrap();
    
    let name: String = get_input("Name: ").unwrap();
    let price: u32 = get_input("Price: ").unwrap();
    
    match answer {
        1 => {
            let case_form_factor: CaseFormFactor = get_input("Case Form Factor: ").unwrap();
            let mb_form_factor: MBFormFactor = get_input("Motherboard Form Factor: ").unwrap();
            let max_fans: u32 = get_input("Max fans: ").unwrap();
            let max_ssd: u32 = get_input("Max SSD: ").unwrap();
            let max_hdd: u32 = get_input("Max HDD: ").unwrap();
            let max_cpu_cooler_height: u32 = get_input("Max CPU Cooler Height: ").unwrap();
            let max_gpu_length: u32 = get_input("Max GPU Length: ").unwrap();
            let max_gpu_width: u32 = get_input("Max GPU Width: ").unwrap();
            let max_power_supply_length: u32 = get_input("Max Power Supply Length: ").unwrap();

            let case = Case {
                name: name.clone(),
                price,
                case_form_factor,
                mb_form_factor,
                max_fans,
                max_ssd,
                max_hdd,
                max_cpu_cooler_height,
                max_gpu_length,
                max_gpu_width,
                max_power_supply_length
            };

            std::fs::write(format!("parts/cases/{}.json", name.clone()), serde_json::to_vec_pretty(&case).unwrap()).unwrap();
        }
        2 => {
            let power_usage: u32 = get_input("Power Usage: ").unwrap();
            let mb_form_factor: MBFormFactor = get_input("Motherboard Form Factor: ").unwrap();
            let socket_type: SocketType = get_input("Socket Type: ").unwrap();
            let ram_type: RamType = get_input("Ram Type: ").unwrap();
            let ram_slots: u32 = get_input("Ram Slots: ").unwrap();
            let m2_slots: u32 = get_input("M.2 Slots: ").unwrap();
            let sata_slots: u32 = get_input("Sata Slots: ").unwrap();
            let max_cpu_speed: u32 = get_input("Max CPU Speed: ").unwrap();
            let max_ram_speed: u32 = get_input("Max RAM Speed: ").unwrap();

            let mb = MB {
                name: name.clone(),
                price,
                power_usage,
                mb_form_factor,
                socket_type,
                ram_type,
                ram_slots,
                m2_slots,
                sata_slots,
                max_cpu_speed,
                max_ram_speed
            };

            std::fs::write(format!("parts/motherboards/{}.json", name.clone()), serde_json::to_vec_pretty(&mb).unwrap().as_slice()).unwrap();
        }
        3 => {
            let power_usage: u32 = get_input("Power Usage: ").unwrap();
            let socket_type: SocketType = get_input("Socket Type: ").unwrap();
            let base: f32 = get_input("Base Multiplier: ").unwrap();
            let cores: u32 = get_input("Cores: ").unwrap();
            let threads: u32 = get_input("Threads: ").unwrap();
            let speed: u32 = get_input("Speed: ").unwrap();

            let cpu = CPU {
                name: name.clone(),
                price,
                power_usage,
                socket_type,
                base,
                cores,
                threads,
                speed
            };

            std::fs::write(format!("parts/cpus/{}.json", name.clone()), serde_json::to_vec_pretty(&cpu).unwrap().as_slice()).unwrap();
        }
        4 => {
            let power_usage: u32 = get_input("Power Usage: ").unwrap();
            let socket_type: SocketType = get_input("Socket Type: ").unwrap();
            let height: u32 = get_input("Height: ").unwrap();
            let cooling: f32 = get_input("Cooling: ").unwrap();

            let cpu_cooler = CPUCooler {
                name: name.clone(),
                price,
                power_usage,
                socket_type,
                height,
                cooling
            };

            std::fs::write(format!("parts/cpu coolers/{}.json", name.clone()), serde_json::to_vec_pretty(&cpu_cooler).unwrap().as_slice()).unwrap();
        }
        5 => {
            let power_usage: u32 = get_input("Power Usage: ").unwrap();
            let ram_type: RamType = get_input("Ram Type: ").unwrap();
            let size: u32 = get_input("Size: ").unwrap();
            let speed: u32 = get_input("Speed: ").unwrap();

            let ram = RAM {
                name: name.clone(),
                price,
                power_usage,
                ram_type,
                size,
                speed
            };
            
            std::fs::write(format!("parts/rams/{}.json", name.clone()), serde_json::to_vec_pretty(&ram).unwrap().as_slice()).unwrap();
        }
        6 => {
            let power_usage: u32 = get_input("Power Usage: ").unwrap();
            let length: u32 = get_input("Length: ").unwrap();
            let width: u32 = get_input("Width: ").unwrap();
            let cores: u32 = get_input("Cores: ").unwrap();
            let rt_cores: u32 = get_input("RT Cores: ").unwrap();
            let speed: u32 = get_input("Speed: ").unwrap();
            let vram: u32 = get_input("VRAM: ").unwrap();

            let gpu = GPU {
                name: name.clone(),
                price,
                power_usage,
                length,
                width,
                cores,
                rt_cores,
                speed,
                vram
            };

            std::fs::write(format!("parts/gpus/{}.json", name.clone()), serde_json::to_vec_pretty(&gpu).unwrap().as_slice()).unwrap();
        }
        7 => {
            let power_usage: u32 = get_input("Power Usage: ").unwrap();
            let storage_type: StorageType = get_input("Storage Type: ").unwrap();
            let size: u32 = get_input("Size: ").unwrap();
            let speed: u32 = get_input("Speed: ").unwrap();

            let storage = Storage {
                name: name.clone(),
                price,
                power_usage,
                storage_type,
                size,
                speed
            };
            
            std::fs::write(format!("parts/storages/{}.json", name.clone()), serde_json::to_vec_pretty(&storage).unwrap().as_slice()).unwrap();
        }
        8 => {
            let power_usage: u32 = get_input("Power Usage: ").unwrap();
            let large: bool = get_input("Large: ").unwrap();
            let cooling: f32 = get_input("Cooling: ").unwrap();

            let fan = Fan {
                name: name.clone(),
                price,
                power_usage,
                large,
                cooling
            };
            
            std::fs::write(format!("parts/fans/{}.json", name.clone()), serde_json::to_vec_pretty(&fan).unwrap().as_slice()).unwrap();
        }
        9 => {
            let wattage: u32 = get_input("Wattage: ").unwrap();
            let length: u32 = get_input("Length: ").unwrap();

            let psu = PSU {
                name: name.clone(),
                price,
                wattage,
                length
            };
            
            std::fs::write(format!("parts/power supplies/{}.json", name.clone()), serde_json::to_vec_pretty(&psu).unwrap().as_slice()).unwrap();
        }
        _ => {}
    }
}

fn get_input<T: FromStr>(question: &str) -> Option<T> {
    let mut input = String::new();
    std::io::stdout().write(question.as_bytes()).unwrap();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse() {
        Ok(i) => {
            Some(i)
        }
        Err(_) => {
            None
        }
    }
}

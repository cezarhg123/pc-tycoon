mod pc_components;
use std::{io::Write, fs::File};
use crate::pc_components::{case::{Size, Case}, power_supply::{PowerSupplyFormFactor, PowerSupply}, motherboard::{SocketType, MotherBoard}, cpu::Cpu, cpu_cooler::{self, CpuCooler}, gpu::Gpu, storage_device::{StorageDeviceType, StorageDevice}, ram::Ram, fan::Fan};

fn main() {
    println!("what part to make?");
    println!("1. Case");
    println!("2. Motherboard");
    println!("3. CPU");
    println!("4. CPU Cooler");
    println!("5. RAM");
    println!("6. GPU");
    println!("7. Storage Device");
    println!("8. Fan");
    println!("9. Power Supply");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num: usize = input.trim().parse().unwrap();
    let mut component_type = String::new();

    match num {
        1 => {
            component_type = String::from("case");
            print!("name: ");
            std::io::stdout().flush().unwrap();
            let mut name = String::new();
            std::io::stdin().read_line(&mut name).unwrap();
            let name = name.trim();
            let name = name.to_owned();

            let mut size = Size::Atx;
            loop {
                print!("size: ");
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.to_lowercase();
                if input.contains("atx") {
                    size = Size::Atx;
                    break;
                } else if input.contains("micro atx") {
                    size = Size::MircroAtx;
                    break;
                } else {
                    println!("fucking atx or micro atx");
                }
            }

            print!("max fans: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let max_fans: u32 = input.trim().parse().unwrap();
            
            print!("max cpu cooler height: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let max_cpu_cooler_height: u32 = input.trim().parse().unwrap();

            print!("max gpu length: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let max_gpu_length: u32 = input.trim().parse().unwrap();

            print!("max gpu width: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let max_gpu_width: u32 = input.trim().parse().unwrap();

            print!("max power supply length: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let max_power_supply_length: u32 = input.trim().parse().unwrap();
            
            let mut power_supply_form_factor = PowerSupplyFormFactor::Atx;
            loop {
                print!("power supply form factor: ");
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.to_lowercase();
                if input.contains("atx") {
                    power_supply_form_factor = PowerSupplyFormFactor::Atx;
                    break;
                } else {
                    println!("atx dumbshit");
                }
            }

            print!("max 2.5 drives: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let max_drives_2_5: u32 = input.trim().parse().unwrap();

            print!("max 3.5 drives: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let max_drives_3_5: u32 = input.trim().parse().unwrap();

            print!("price: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let price: u32 = input.trim().parse().unwrap();

            let case = Case {
                name: name.clone(),
                alias: change_name_to_alias(&name),
                size,
                max_fans,
                max_cpu_cooler_height,
                max_gpu_length,
                max_gpu_width,
                max_power_supply_length,
                power_supply_form_factor,
                max_drives_2_5,
                max_drives_3_5,
                price
            };

            let json = serde_json::to_string_pretty(&case).unwrap();
            let mut file = std::fs::File::create(format!("parts/{}/{}.json", component_type.trim(), case.name.trim())).unwrap();
            file.write_all(json.as_bytes()).unwrap();
        }
        2 => {
            component_type = String::from("motherboard");

            print!("name: ");
            std::io::stdout().flush().unwrap();
            let mut name = String::new();
            std::io::stdin().read_line(&mut name).unwrap();
            let name = name.trim();
            let name = name.to_owned();

            let mut size = Size::Atx;
            loop {
                print!("size: ");
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.to_lowercase();
                if input.contains("atx") {
                    size = Size::Atx;
                    break;
                } else if input.contains("micro atx") {
                    size = Size::MircroAtx;
                    break;
                } else {
                    println!("fucking atx or micro atx");
                }
            }

            let socket_type;
            loop {
                print!("socket type: ");
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.to_lowercase();
                if input.contains("am4") {
                    socket_type = SocketType::Am4;
                    break;
                } else if input.contains("lga") {
                    socket_type = SocketType::Lga;
                    break;
                } else {
                    println!("am4 or lga dipshit");
                }
            }

            print!("max cpu speed: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let max_cpu_speed: u32 = input.trim().parse().unwrap();

            print!("max ram speed: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let max_ram_speed: u32 = input.trim().parse().unwrap();

            print!("ram slots: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let ram_slots: u32 = input.trim().parse().unwrap();

            print!("M.2 slots: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let m2_slots: u32 = input.trim().parse().unwrap();

            print!("Max storage devices: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let max_storage_devices: u32 = input.trim().parse().unwrap();
            
            print!("price: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let price: u32 = input.trim().parse().unwrap();

            let motherboard = MotherBoard {
                name: name.clone(),
                alias: change_name_to_alias(&name),
                size,
                socket_type,
                max_cpu_speed,
                max_ram_speed,
                ram_slots,
                m2_slots,
                max_storage_devices,
                price
            };

            let json = serde_json::to_string_pretty(&motherboard).unwrap();
            let mut file = std::fs::File::create(format!("parts/{}/{}.json", component_type.trim(), motherboard.name.trim())).unwrap();
            file.write_all(json.as_bytes()).unwrap();
        }
        3 => {
            component_type = String::from("cpu");

            print!("name: ");
            std::io::stdout().flush().unwrap();
            let mut name = String::new();
            std::io::stdin().read_line(&mut name).unwrap();
            let name = name.trim();
            let name = name.to_owned();

            let mut socket_type = SocketType::Am4;
            loop {
                print!("socket type: ");
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.to_lowercase();
                if input.contains("am4") {
                    socket_type = SocketType::Am4;
                    break;
                } else if input.contains("lga") {
                    socket_type = SocketType::Lga;
                    break;
                } else {
                    println!("am4 or lga, dickballs");
                }
            }

            print!("base multipler: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let base_multiplier: f32 = input.trim().parse().unwrap();
            
            print!("cores: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let cores: u32 = input.trim().parse().unwrap();
        
            print!("threads: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let threads: u32 = input.trim().parse().unwrap();
            
            print!("speed: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let speed: u32 = input.trim().parse().unwrap();
            
            print!("power usage: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let power_usage: u32 = input.trim().parse().unwrap();
            
            print!("price: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let price: u32 = input.trim().parse().unwrap();

            let cpu = Cpu {
                name: name.clone(),
                alias: change_name_to_alias(&name),
                socket_type,
                base_multiplier,
                cores,
                threads,
                speed,
                power_usage,
                price
            };

            let json = serde_json::to_string_pretty(&cpu).unwrap();
            let mut file = std::fs::File::create(format!("parts/{}/{}.json", component_type.trim(), cpu.name.trim())).unwrap();
            file.write_all(json.as_bytes()).unwrap();
        }
        4 => {
            component_type = String::from("cpu_cooler");
            print!("name: ");
            std::io::stdout().flush().unwrap();
            let mut name = String::new();
            std::io::stdin().read_line(&mut name).unwrap();
            let name = name.trim();
            let name = name.to_owned();

            let mut socket_type = SocketType::Am4;
            loop {
                print!("socket type: ");
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.to_lowercase();
                if input.contains("am4") {
                    socket_type = SocketType::Am4;
                    break;
                } else if input.contains("lga") {
                    socket_type = SocketType::Lga;
                    break;
                } else {
                    println!("am4 or lga dickass");
                }
            }

            print!("height: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let height: u32 = input.trim().parse().unwrap();
            
            print!("base: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let base: f32 = input.trim().parse().unwrap();

            print!("power usage: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let power_usage: u32 = input.trim().parse().unwrap();
            
            print!("price: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let price: u32 = input.trim().parse().unwrap();

            let cpu_cooler = CpuCooler {
                name: name.clone(),
                alias: change_name_to_alias(&name),
                socket_type,
                height,
                base,
                power_usage,
                price
            };

            let json = serde_json::to_string_pretty(&cpu_cooler).unwrap();
            let mut file = std::fs::File::create(format!("parts/{}/{}.json", component_type.trim(), cpu_cooler.name.trim())).unwrap();
            file.write_all(json.as_bytes()).unwrap();
        }
        5 => {
            component_type = String::from("ram");

            print!("name: ");
            std::io::stdout().flush().unwrap();
            let mut name = String::new();
            std::io::stdin().read_line(&mut name).unwrap();
            let name = name.trim();
            let name = name.to_owned();

            print!("size: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let size: u32 = input.trim().parse().unwrap();
            
            print!("speed: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let speed: u32 = input.trim().parse().unwrap();
            
            print!("price: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let price: u32 = input.trim().parse().unwrap();

            let ram = Ram {
                name: name.clone(),
                alias: change_name_to_alias(&name),
                size,
                speed,
                price
            };

            let json = serde_json::to_string_pretty(&ram).unwrap();
            let mut file = std::fs::File::create(format!("parts/{}/{}.json", component_type.trim(), ram.name.trim())).unwrap();
            file.write_all(json.as_bytes()).unwrap();
        }
        6 => {
            component_type = String::from("gpu");

            print!("name: ");
            std::io::stdout().flush().unwrap();
            let mut name = String::new();
            std::io::stdin().read_line(&mut name).unwrap();
            let name = name.trim();
            let name = name.to_owned();

            print!("cores: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let cores: u32 = input.trim().parse().unwrap();
            
            print!("rt cores: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let rt_cores: u32 = input.trim().parse().unwrap();
            
            print!("speed: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let speed: u32 = input.trim().parse().unwrap();
            
            print!("vram: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let vram: u32 = input.trim().parse().unwrap();
            
            print!("length: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let length: u32 = input.trim().parse().unwrap();
            
            print!("width: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let width: u32 = input.trim().parse().unwrap();
            
            print!("cooling: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let cooling: f32 = input.trim().parse().unwrap();
            
            print!("power usage: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let power_usage: u32 = input.trim().parse().unwrap();
            
            print!("price: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let price: u32 = input.trim().parse().unwrap();

            let gpu = Gpu {
                name: name.clone(),
                alias: change_name_to_alias(&name),
                cores,
                rt_cores,
                speed,
                vram,
                length,
                width,
                cooling,
                power_usage,
                price
            };

            let json = serde_json::to_string_pretty(&gpu).unwrap();
            let mut file = std::fs::File::create(format!("parts/{}/{}.json", component_type.trim(), gpu.name.trim())).unwrap();
            file.write_all(json.as_bytes()).unwrap();
        }
        7 => {
            component_type = String::from("storage");

            print!("name: ");
            std::io::stdout().flush().unwrap();
            let mut name = String::new();
            std::io::stdin().read_line(&mut name).unwrap();
            let name = name.trim();
            let name = name.to_owned();

            let mut storage_device_type = StorageDeviceType::Hdd;
            loop {
                print!("storage device type: ");
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.to_lowercase();
                if input.contains("m.2") {
                    storage_device_type = StorageDeviceType::M2;
                    break;
                } else if input.contains("ssd") {
                    storage_device_type = StorageDeviceType::Ssd;
                    break;
                } else if input.contains("hdd") {
                    storage_device_type = StorageDeviceType::Hdd;
                    break;
                } else {
                    println!("m.2, ssd or hdd, dumb pussy fart faggot");
                }
            }

            print!("size: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let size: u32 = input.trim().parse().unwrap();

            print!("price: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let price: u32 = input.trim().parse().unwrap();

            let storage = StorageDevice {
                name: name.clone(),
                alias: change_name_to_alias(&name),
                storage_device_type,
                size,
                price
            };

            let json = serde_json::to_string_pretty(&storage).unwrap();
            let mut file = std::fs::File::create(format!("parts/{}/{}.json", component_type.trim(), storage.name.trim())).unwrap();
            file.write_all(json.as_bytes()).unwrap();
        }
        8 => {
            component_type = String::from("fan");

            print!("name: ");
            std::io::stdout().flush().unwrap();
            let mut name = String::new();
            std::io::stdin().read_line(&mut name).unwrap();
            let name = name.trim();
            let name = name.to_owned();

            let mut large = false;
            loop {
                print!("size: ");
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if input.contains("120") {
                    large = false;
                    break;
                } else if input.contains("140") {
                    large = true;
                    break;
                } else {
                    println!("120 or 140");
                }
            }

            print!("effectiveness: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let effectiveness: f32 = input.trim().parse().unwrap();

            print!("price: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let price: u32 = input.trim().parse().unwrap();

            let fan = Fan {
                name: name.clone(),
                alias: change_name_to_alias(&name),
                large,
                effectiveness,
                price
            };

            let json = serde_json::to_string_pretty(&fan).unwrap();
            let mut file = std::fs::File::create(format!("parts/{}/{}.json", component_type.trim(), fan.name.trim())).unwrap();
            file.write_all(json.as_bytes()).unwrap();
        }
        9 => {
            component_type = String::from("power_supply");

            print!("name: ");
            std::io::stdout().flush().unwrap();
            let mut name = String::new();
            std::io::stdin().read_line(&mut name).unwrap();
            let name = name.trim();
            let name = name.to_owned();

            let form_factor = PowerSupplyFormFactor::Atx;

            print!("length: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let length: u32 = input.trim().parse().unwrap();
            
            print!("wattage: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let wattage: u32 = input.trim().parse().unwrap();
            
            print!("price: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let price: u32 = input.trim().parse().unwrap();

            let power_supply = PowerSupply {
                name: name.clone(),
                alias: change_name_to_alias(&name),
                form_factor,
                length,
                wattage,
                price
            };

            let json = serde_json::to_string_pretty(&power_supply).unwrap();
            let mut file = std::fs::File::create(format!("parts/{}/{}.json", component_type.trim(), power_supply.name.trim())).unwrap();
            file.write_all(json.as_bytes()).unwrap();
        }
        _ => {}
    }
}

fn change_name_to_alias(name: &String) -> String {
    let name = name.clone();
    let name = name.trim().to_ascii_lowercase();
    let name = name.replace(" ", "_");
    name
}

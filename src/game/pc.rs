use super::pc_components::{case::Case, motherboard::MotherBoard, cpu::Cpu, ram::Ram, cpu_cooler::CpuCooler, gpu::Gpu, storage_device::{StorageDevice, StorageDeviceType}, fan::Fan, power_supply::PowerSupply};

pub struct Pc {
    pub case: Case,
    pub motherboard: MotherBoard,
    pub cpu: Cpu,
    pub cpu_cooler: CpuCooler,
    pub ram: Vec<Ram>,
    pub gpu: Gpu,
    pub storage: Vec<StorageDevice>,
    pub fans: Vec<Fan>,
    pub power_supply: PowerSupply,
    pub computing_score: u32,
    pub graphics_score: u32,
    pub total_score: u32
}

impl Pc {
    pub fn check_compatability(&self) -> Result<String, String> {
        let mut errors = String::new();
        
        //checking case parameters
        if self.case.size != self.motherboard.size {
        errors.push_str("motherboard size is not the same as case size\n");
        }

        if self.fans.len() as u32 > self.case.max_fans {
            errors.push_str(format!("more fans than available space: {} fans but only {} spaces\n", self.fans.len(), self.case.max_fans).as_str());
        }

        if self.cpu_cooler.height > self.case.max_cpu_cooler_height {
            errors.push_str(format!("cpu cooler cant fit: cooler height - {}, max height - {}\n", self.cpu_cooler.height, self.case.max_cpu_cooler_height).as_str());
        }

        if self.gpu.length > self.case.max_gpu_length {
            errors.push_str(format!("GPU can't fit: gpu length - {}, max gpu length - {}\n", self.gpu.length, self.case.max_gpu_length).as_str());
        }

        if self.gpu.width > self.case.max_gpu_width {
            errors.push_str(format!("GPU can't fit: gpu width - {}, max gpu width - {}\n", self.gpu.width, self.case.max_gpu_width).as_str());
        }

        if self.power_supply.length > self.case.max_power_supply_length {
            errors.push_str(format!("Power supply too long: length - {}, max length - {}\n", self.power_supply.length, self.case.max_power_supply_length).as_str());
        }

        if self.power_supply.form_factor != self.case.power_supply_form_factor {
            errors.push_str("Mismatched power supply form factor\n");
        }

        let small_drives_num = 
            self.storage.iter()
                .clone()
                .filter(|s| s.storage_device_type == StorageDeviceType::Ssd)
                .count() as u32;
        
        let big_drives_num = 
            self.storage.iter()
                .clone()
                .filter(|s| s.storage_device_type == StorageDeviceType::Ssd)
                .count() as u32;
        
        if small_drives_num > self.case.max_drives_2_5 {
            errors.push_str(format!("more 2.5 drives than there is space in case: {} drives, max drives - {}\n", small_drives_num, self.case.max_drives_2_5).as_str());
        }

        if big_drives_num > self.case.max_drives_3_5 {
            errors.push_str(format!("more 3.5 drives than there is space in case: {} drives, max drives - {}\n", big_drives_num, self.case.max_drives_3_5).as_str());
        }
        //CASE

        //MOTHERBOARD
        if self.cpu.socket_type != self.motherboard.socket_type {
            errors.push_str("cpu socket type isn't the same as motherboard\n");
        }

        if self.cpu.speed > self.motherboard.max_cpu_speed {
            errors.push_str("cpu too fast for motherboard\n");
        }

        let m2_num = 
        self.storage.iter()
        .clone()
        .filter(|s| s.storage_device_type == StorageDeviceType::M2)
        .count() as u32;
        
        if m2_num > self.motherboard.m2_slots {
            errors.push_str("too many m2 drives\n");
        }
        
        if self.storage.len() as u32 > self.motherboard.max_storage_devices {
            errors.push_str("too many storage devices\n");
        }
        //MOTHERBOARD

        //CPU COOLER
        if self.cpu_cooler.socket_type != self.cpu.socket_type {
            errors.push_str("cpu cooler does not support the cpu socket type\n");
        }
        //CPU COOLER
        
        //RAM
        if self.ram.len() == 0 {
            errors.push_str("what the fuck do you think you're gonna do with no RAM?\n");
        } else {
            if self.ram[0].speed > self.motherboard.max_ram_speed {
                errors.push_str("ram too fast for motherboard\n");
            }
            let first = self.ram.iter().next().unwrap();
            if !self.ram.iter().all(|r| r.name == first.name) {
                errors.push_str("all ram arn't the same type\n");
            }
        }
        //RAM

        //STORAGE
        if self.storage.len() == 0 {
            errors.push_str("no storage\n");
        }
        //STORAGE

        //POWER SUPPLY
        let mut power_usage = 
            self.cpu.power_usage +
            self.cpu_cooler.power_usage +
            self.gpu.power_usage +
            50 /*MOTHERBOARD*/;
        power_usage += 5 * self.ram.len() as u32;
        power_usage += 10 * self.storage.len() as u32;
        power_usage += 10 * self.fans.len() as u32;

        if power_usage > self.power_supply.wattage {
            errors.push_str(format!("power supply too weak: {}W usage, max {}W\n", power_usage, self.power_supply.wattage).as_str());
        }
        //POWER SUPPLY

        if errors.len() == 0 {
            Ok("pc is perfectly fine".to_string())
        } else {
            Err(errors)
        }
    }

    pub fn calculate_score(&mut self) {
        let mut case_cooling = 1.0;
        for fan in &self.fans {
            case_cooling += fan.effectiveness * 0.1;
        }

        let mut ram_score: u32 = 0;
        for ram in &self.ram {
            ram_score += ram.speed / 2;
        }

        let cpu_cooling = case_cooling + self.cpu_cooler.base;
        let cpu_score = ((self.cpu.threads + self.cpu.cores) as f32 * 0.2) * self.cpu.speed as f32 * self.cpu.base_multiplier;
        let cpu_score = (cpu_score * cpu_cooling) as u32;
        let cpu_score = cpu_score + ram_score;

        let gpu_cooling = case_cooling + self.gpu.cooling;
        let gpu_score = (self.gpu.cores + self.gpu.rt_cores * 4) as f32 * (self.gpu.speed as f32 * 0.2);
        let gpu_score = gpu_score + (self.gpu.vram * 10) as f32;
        let gpu_score = gpu_score * 0.2;
        let gpu_score = (gpu_score * gpu_cooling) as u32;

        let mut other_score = 0;
        for ram in &self.ram {
            other_score += ram.size * 10;
        }

        for storage in &self.storage {
            match &storage.storage_device_type {
                StorageDeviceType::M2 => {
                    other_score += 1000;
                    other_score += storage.size * 10;
                }
                StorageDeviceType::Ssd => {
                    other_score += 300;
                    other_score += storage.size * 2;
                }
                StorageDeviceType::Hdd => {
                    other_score += 50;
                    other_score += storage.size;
                }
            }
        }

        self.computing_score = cpu_score;
        self.graphics_score = gpu_score;
        self.total_score = cpu_score + gpu_score + other_score;
    }
}

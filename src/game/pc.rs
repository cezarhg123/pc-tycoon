use serde::{Serialize, Deserialize};

use super::pc_components::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PC {
    case: Option<Case>,
    mb: Option<MB>,
    cpu: Option<CPU>,
    cpu_cooler: Option<CPUCooler>,
    ram: Vec<RAM>,
    gpu: Option<GPU>,
    storage: Vec<Storage>,
    fans: Vec<Fan>,
    psu: Option<PSU>
}

impl PC {
    pub fn new() -> PC {
        PC {
            case: None,
            mb: None,
            cpu: None,
            cpu_cooler: None,
            ram: Vec::new(),
            gpu: None,
            storage: Vec::new(),
            fans: Vec::new(),
            psu: None
        }
    }

    pub fn get_price(&self) -> u32 {
        let mut price = 0;

        match &self.case {
            Some(case) => {
                price += case.price;
            }
            None => {}
        }

        match &self.mb {
            Some(mb) => {
                price += mb.price;
            }
            None => {}
        }

        match &self.cpu {
            Some(cpu) => {
                price += cpu.price;
            }
            None => {}
        }

        match &self.cpu_cooler {
            Some(cpu_cooler) => {
                price += cpu_cooler.price;
            }
            None => {}
        }

        for ram in &self.ram {
            price += ram.price;
        }

        match &self.gpu {
            Some(gpu) => {
                price += gpu.price;
            }
            None => {}
        }

        for storage in &self.storage {
            price += storage.price;
        }

        for fan in &self.fans {
            price += fan.price;
        }

        match &self.psu {
            Some(psu) => {
                price += psu.price;
            }
            None => {}
        }

        price
    }

    #[allow(unused_assignments)]
    pub fn works(&self) -> bool {
        let mut out;

        out = self.case.is_some();
        out = self.mb.is_some();
        out = self.cpu.is_some();
        out = self.cpu_cooler.is_some();
        out = self.ram.len() > 0;
        out = self.gpu.is_some();
        out = self.storage.len() > 0;
        out = self.psu.is_some();

        out
    }

    pub fn get_compute_score(&self) -> u32 {
        let mut global_cooling = 0.0;
        for fan in &self.fans {
            global_cooling += fan.cooling;
        }

        let cpu = self.cpu.as_ref().unwrap();
        let cpu_cooler = self.cpu_cooler.as_ref().unwrap();
        let mut ram_total = 0;
        let ram_speed = self.ram[0].speed;
        for ram in &self.ram {
            ram_total += ram.size;
        }

        let score = (cpu.threads / cpu.cores) * cpu.speed;
        let score = score + (cpu_cooler.cooling + (global_cooling / 10.0)) as u32;
        let score = score + (ram_total * (ram_speed / 10));

        score
    }

    pub fn get_graphic_score(&self) -> u32 {
        let mut global_cooling = 0.0;
        for fan in &self.fans {
            global_cooling += fan.cooling;
        }

        let gpu = self.gpu.as_ref().unwrap();

        let score = ((gpu.cores + (gpu.rt_cores * 20)) * (gpu.speed / 10)) + (gpu.vram * 10);
        let score = score + (global_cooling / 5.0) as u32;

        score
    }

    pub fn get_total_score(&self) -> u32 {
        self.get_compute_score() + self.get_graphic_score()
    }

    pub fn set_case(&mut self, case: Case) {        
        self.case = Some(case);
        self.mb = None;
        self.cpu = None;
        self.cpu_cooler = None;
        self.ram.clear();
        self.gpu = None;
        self.storage.clear();
        self.fans.clear();
        self.psu = None;
    }

    pub fn can_set_mb(&self, mb: &MB) -> bool {
        if mb.mb_form_factor == self.case.as_ref().unwrap().mb_form_factor {
            true
        } else {
            false
        }
    }

    pub fn set_mb(&mut self, mb: MB) {
        self.mb = Some(mb);
        self.cpu = None;
        self.cpu_cooler = None;
        self.ram.clear();
        self.gpu = None;
        self.storage.clear();
        self.psu = None;
    }

    pub fn can_set_cpu(&self, cpu: &CPU) -> bool {
        if cpu.socket_type == self.mb.as_ref().unwrap().socket_type && cpu.speed <= self.mb.as_ref().unwrap().max_cpu_speed {
            true
        } else {
            false
        }
    }

    pub fn set_cpu(&mut self, cpu: CPU) {
        self.cpu = Some(cpu);
        self.cpu_cooler = None;
        self.psu = None;
    }

    pub fn can_set_cpu_cooler(&self, cpu_cooler: &CPUCooler) -> bool {
        if cpu_cooler.socket_type == self.cpu.as_ref().unwrap().socket_type && cpu_cooler.height <= self.case.as_ref().unwrap().max_cpu_cooler_height {
            true
        } else {
            false
        }
    }

    pub fn set_cpu_cooler(&mut self, cpu_cooler: CPUCooler) {
        self.cpu_cooler = Some(cpu_cooler);
        self.psu = None;
    }

    pub fn can_set_ram(&self, ram: &RAM) -> bool {
        if ram.ram_type == self.mb.as_ref().unwrap().ram_type && ram.speed <= self.mb.as_ref().unwrap().max_ram_speed && self.ram.len() as u32 <= self.mb.as_ref().unwrap().ram_slots {
            if self.ram.len() > 0 {
                for installed_ram in &self.ram {
                    if installed_ram.name == ram.name {
                        return true;
                    }
                }
            } else {
                return true
            }

            //return false if ram is compatible with motherboard but isn't the same ram model as the other RAMs already installed
            false
        } else {
            false
        }
    }

    pub fn set_ram(&mut self, ram: RAM) {
        self.ram.push(ram);
        self.psu = None;
    }

    pub fn can_set_gpu(&self, gpu: &GPU) -> bool {
        if gpu.length <= self.case.as_ref().unwrap().max_gpu_length && gpu.width <= self.case.as_ref().unwrap().max_gpu_width {
            true
        } else {
            false
        }
    }

    pub fn set_gpu(&mut self, gpu: GPU) {
        self.gpu = Some(gpu);
        self.psu = None;
    }

    pub fn can_set_storage(&self, storage: &Storage) -> bool {
        if storage.storage_type == StorageType::M2 {
            let mut m2s_drives = 0;
            for storage in &self.storage {
                if storage.storage_type == StorageType::M2 {
                    m2s_drives += 1;
                }
            }

            m2s_drives + 1 <= self.mb.as_ref().unwrap().m2_slots
        } else if storage.storage_type == StorageType::SSD {
            let mut available_sata = self.mb.as_ref().unwrap().sata_slots;
            for storage in &self.storage {
                if storage.storage_type != StorageType::M2 {
                    available_sata -= 1;
                }
            }
            
            let mut available_space = self.case.as_ref().unwrap().max_ssd;
            for storage in &self.storage {
                if storage.storage_type == StorageType::SSD {
                    available_space -= 1;
                }
            }

            available_sata > 0 && available_space > 0
        } else {
            let mut available_sata = self.mb.as_ref().unwrap().sata_slots;
            for storage in &self.storage {
                if storage.storage_type != StorageType::M2 {
                    available_sata -= 1;
                }
            }

            let mut available_space = self.case.as_ref().unwrap().max_hdd;
            for storage in &self.storage {
                if storage.storage_type == StorageType::HDD {
                    available_space -= 1;
                }
            }

            available_sata > 0 && available_space > 0
        }
    }

    pub fn set_storage(&mut self, storage: Storage) {
        self.storage.push(storage);
        self.psu = None;
    }

    pub fn can_set_fan(&self, fan: &Fan) -> bool {
        (self.fans.len() + 1) as u32 <= self.case.as_ref().unwrap().max_fans
    }

    pub fn set_fan(&mut self, fan: Fan) {
        self.fans.push(fan);
        self.psu = None;
    }

    pub fn can_set_psu(&self, psu: &PSU) -> bool {
        let mut power_usage = 0;
        power_usage += self.mb.as_ref().unwrap().power_usage;
        power_usage += self.cpu.as_ref().unwrap().power_usage;
        power_usage += self.cpu_cooler.as_ref().unwrap().power_usage;
        for ram in &self.ram {
            power_usage += ram.power_usage;
        }
        power_usage += self.gpu.as_ref().unwrap().power_usage;
        for storage in &self.storage {
            power_usage += storage.power_usage;
        }
        for fan in &self.fans {
            power_usage += fan.power_usage;
        }

        psu.wattage >= power_usage
    }

    pub fn set_psu(&mut self, psu: PSU) {
        self.psu = Some(psu);
    }

    pub fn case(&self) -> Option<&Case> {
        self.case.as_ref()
    }

    pub fn mb(&self) -> Option<&MB> {
        self.mb.as_ref()
    }

    pub fn cpu(&self) -> Option<&CPU> {
        self.cpu.as_ref()
    }

    pub fn cpu_cooler(&self) -> Option<&CPUCooler> {
        self.cpu_cooler.as_ref()
    }

    pub fn ram(&self) -> &[RAM] {
        self.ram.as_ref()
    }

    pub fn gpu(&self) -> Option<&GPU> {
        self.gpu.as_ref()
    }

    pub fn storage(&self) -> &[Storage] {
        self.storage.as_ref()
    }

    pub fn fans(&self) -> &[Fan] {
        self.fans.as_ref()
    }

    pub fn psu(&self) -> Option<&PSU> {
        self.psu.as_ref()
    }
}

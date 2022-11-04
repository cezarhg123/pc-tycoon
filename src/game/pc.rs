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
            for installed_ram in &self.ram {
                if installed_ram.name == ram.name {
                    return true;
                }
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

    pub fn can_set_storage(&self, storage: Storage) -> bool {
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
}

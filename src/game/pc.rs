use super::pc_components::*;

pub enum MissingPart {
    Case,
    MB,
    CPU,
    CPUCooler,
    RAM,
    GPU,
    Storage,
    Fan,
    PSU
}

pub enum PCBuildError {
    MissingPart(MissingPart),
    IncorrectMBFormFactor {
        case: MBFormFactor,
        mb: MBFormFactor
    },
    PartAlreadyExists,
    IncorrectSocketTypeCPU {
        mb: SocketType,
        cpu: SocketType
    },
    IncorrectSocketTypeCooler {
        mb: SocketType,
        cpu_cooler: SocketType
    },
    IncorrectRamType {
        mb: RamType,
        ram: RamType
    },
    MismatchedRAM,
    GPUTooWide {
        max_width: u32,
        gpu_width: u32
    },
    GPUTooLong {
        max_length: u32,
        gpu_length: u32
    },
    M2SlotsFull,
    SataSlotsFull,
    SSDMountsFull,
    HDDMountsFull,
    CPUCoolerTooTall {
        max_height: u32,
        cooler_height: u32
    },
    FanMountsFull,
    PSUTooLong {
        max_length: u32,
        psu_length: u32
    }
}

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

    pub fn add_case(&mut self, case: Case) {
        self.case = Some(case);
    }

    pub fn get_case(&self) -> Option<Case> {
        self.case.clone()
    }

    pub fn remove_case(&mut self) {
        self.case = None;
        self.mb = None;
        self.cpu = None;
        self.cpu_cooler = None;
        self.ram.clear();
        self.gpu = None;
        self.storage.clear();
        self.fans.clear();
        self.psu = None;
    }

    pub fn add_mb(&mut self, mb: MB) -> Result<(), PCBuildError> {
        let case = match self.case {
            Some(ref case) => {case}
            None => {return Err(PCBuildError::MissingPart(MissingPart::Case))}
        };
        
        if self.mb.is_some() {
            return Err(PCBuildError::PartAlreadyExists);
        }

        if case.mb_form_factor != mb.mb_form_factor {
            return Err(PCBuildError::IncorrectMBFormFactor {
                case: case.mb_form_factor,
                mb: mb.mb_form_factor
            });
        }


        self.mb = Some(mb);
        
        Ok(())
    }

    pub fn get_mb(&self) -> Option<MB> {
        self.mb.clone()
    }

    pub fn remove_mb(&mut self) {
        self.mb = None;
        self.cpu = None;
        self.cpu_cooler = None;
        self.ram.clear();
        self.gpu = None;

        let mut m2_indexes = Vec::new();
        for storage in self.storage.iter().enumerate() {
            if storage.1.storage_type == StorageType::M2 {
                m2_indexes.push(storage.0)
            }
        }

        for i in m2_indexes.into_iter().enumerate() {
            self.storage.remove(i.1 - i.0);
        }
    }

    pub fn add_cpu(&mut self, cpu: CPU) -> Result<(), PCBuildError> {
        let mb = match self.mb {
            Some(ref mb) => {mb}
            None => {return Err(PCBuildError::MissingPart(MissingPart::MB))}
        };

        if self.cpu.is_some() {
            return Err(PCBuildError::PartAlreadyExists)
        }

        if mb.socket_type != cpu.socket_type {
            return Err(PCBuildError::IncorrectSocketTypeCPU {
                mb: mb.socket_type,
                cpu: cpu.socket_type
            });
        }

        self.cpu = Some(cpu);
        
        Ok(())
    }

    pub fn get_cpu(&self) -> Option<CPU> {
        self.cpu.clone()
    }

    pub fn remove_cpu(&mut self) {
        self.cpu = None;
        self.cpu_cooler = None;
    }

    pub fn add_cpu_cooler(&mut self, cpu_cooler: CPUCooler) -> Result<(), PCBuildError> {
        let case = match self.case {
            Some(ref case) => {case}
            None => {return Err(PCBuildError::MissingPart(MissingPart::Case))}
        };
        
        let mb = match self.mb {
            Some(ref mb) => {mb}
            None => {return Err(PCBuildError::MissingPart(MissingPart::MB))}
        };

        if mb.socket_type != mb.socket_type {
            return Err(PCBuildError::IncorrectSocketTypeCooler {
                mb: mb.socket_type,
                cpu_cooler: cpu_cooler.socket_type
            });
        }

        if cpu_cooler.height > case.max_cpu_cooler_height {
            return Err(PCBuildError::CPUCoolerTooTall {
                max_height: case.max_cpu_cooler_height, cooler_height: cpu_cooler.height
            });
        }

        self.cpu_cooler = Some(cpu_cooler);

        Ok(())
    }

    pub fn get_cpu_cooler(&self) -> Option<CPUCooler> {
        self.cpu_cooler.clone()
    }

    pub fn remove_cpu_cooler(&mut self) {
        self.cpu_cooler = None;
    }

    pub fn add_ram(&mut self, ram: RAM) -> Result<(), PCBuildError> {
        let mb = match self.mb {
            Some(ref mb) => {mb}
            None => {return Err(PCBuildError::MissingPart(MissingPart::MB))}
        };

        if mb.ram_type != ram.ram_type {
            return Err(PCBuildError::IncorrectRamType {
                mb: mb.ram_type,
                ram: ram.ram_type
            });
        }

        if let Some(first_ram) = self.ram.first() {
            if first_ram.name != ram.name {
                return Err(PCBuildError::MismatchedRAM);
            }
        }

        self.ram.push(ram);

        Ok(())
    }

    pub fn get_ram(&self) -> Vec<RAM> {
        self.ram.clone()
    }

    pub fn remove_ram(&mut self, index: usize) {
        self.ram.remove(index);
    }

    pub fn add_gpu(&mut self, gpu: GPU) -> Result<(), PCBuildError> {
        let case = match self.case {
            Some(ref case) => {case}
            None => {return Err(PCBuildError::MissingPart(MissingPart::Case))}
        };

        if self.gpu.is_some() {
            return Err(PCBuildError::PartAlreadyExists);
        }

        if gpu.width > case.max_gpu_width {
            return Err(PCBuildError::GPUTooWide {
                max_width: case.max_gpu_width,
                gpu_width: gpu.width
            })
        }

        if gpu.length > case.max_gpu_length {
            return Err(PCBuildError::GPUTooLong {
                max_length: case.max_gpu_length, gpu_length: gpu.length
            });
        }

        self.gpu = Some(gpu);

        Ok(())
    }

    pub fn get_gpu(&self) -> Option<GPU> {
        self.gpu.clone()
    }

    pub fn remove_gpu(&mut self) {
        self.gpu = None;
    }

    pub fn add_storage(&mut self, storage: Storage) -> Result<(), PCBuildError> {
        let case = match self.case {
            Some(ref case) => {case}
            None => {return Err(PCBuildError::MissingPart(MissingPart::Case))}
        };

        let mb = match self.mb {
            Some(ref mb) => {mb}
            None => {return Err(PCBuildError::MissingPart(MissingPart::MB))}
        };
        
        let ssd_count = self.storage.iter().filter(|storage| storage.storage_type == StorageType::SSD).count();
        let hdd_count = self.storage.iter().filter(|storage| storage.storage_type == StorageType::HDD).count();
        let sata_count = ssd_count + hdd_count;

        if sata_count as u32 == mb.sata_slots {
            return Err(PCBuildError::SataSlotsFull);
        }

        match storage.storage_type {
            StorageType::M2 => {
                let m2_count = self.storage.iter().filter(|storage| storage.storage_type == StorageType::M2).count();
                
                if m2_count as u32 == mb.m2_slots {
                    return Err(PCBuildError::M2SlotsFull);
                }

                self.storage.push(storage);
            }
            StorageType::SSD => {
                if ssd_count as u32 == case.max_ssd {
                    return Err(PCBuildError::SSDMountsFull);
                }

                self.storage.push(storage);
            }
            StorageType::HDD => {
                if hdd_count as u32 == case.max_hdd {
                    return Err(PCBuildError::HDDMountsFull);
                }

                self.storage.push(storage);
            }
        }

        Ok(())
    }

    pub fn get_storage(&self) -> Vec<Storage> {
        self.storage.clone()
    }

    pub fn remove_storage(&mut self, index: usize) {
        self.storage.remove(index);
    }

    pub fn add_fan(&mut self, fan: Fan) -> Result<(), PCBuildError> {
        let case = match self.case {
            Some(ref case) => {case}
            None => {return Err(PCBuildError::MissingPart(MissingPart::Case))}
        };

        if self.fans.len() == case.max_fans as usize {
            return Err(PCBuildError::FanMountsFull);
        }

        self.fans.push(fan);

        Ok(())
    }

    pub fn get_fans(&self) -> Vec<Fan> {
        self.fans.clone()
    }

    pub fn remove_fan(&mut self, index: usize) {
        self.fans.remove(index);
    }

    pub fn add_psu(&mut self, psu: PSU) -> Result<(), PCBuildError> {
        let case = match self.case {
            Some(ref case) => {case}
            None => {return Err(PCBuildError::MissingPart(MissingPart::Case))}
        };

        if self.psu.is_some() {
            return Err(PCBuildError::PartAlreadyExists)
        }

        if psu.length > case.max_power_supply_length {
            return Err(PCBuildError::PSUTooLong {
                max_length: case.max_power_supply_length,
                psu_length: psu.length
            });
        }

        self.psu = Some(psu);

        Ok(())
    }

    pub fn get_psu(&self) -> Option<PSU> {
        self.psu.clone()
    }

    pub fn remove_psu(&mut self) {
        self.psu = None;
    }

    pub fn power_usage(&self) -> u32 {
        let mut power_usage = 0;

        if let Some(mb) = &self.mb {
            power_usage += mb.power_usage;
        }

        if let Some(cpu) = &self.cpu {
            power_usage += cpu.power_usage;
        }

        if let Some(cpu_cooler) = &self.cpu_cooler {
            power_usage += cpu_cooler.power_usage;
        }

        for ram in self.ram.iter() {
            power_usage += ram.power_usage;
        }

        if let Some(gpu) = &self.gpu {
            power_usage += gpu.power_usage;
        }

        for storage in self.storage.iter() {
            power_usage += storage.power_usage;
        }

        for fan in self.fans.iter() {
            power_usage += fan.power_usage;
        }

        power_usage
    }

    pub fn total_price(&self) -> u32 {
        let mut total_price = 0;

        if let Some(case) = &self.case {
            total_price += case.price;
        }

        if let Some(mb) = &self.mb {
            total_price += mb.price;
        }

        if let Some(cpu) = &self.cpu {
            total_price += cpu.price;
        }

        if let Some(cpu_cooler) = &self.cpu_cooler {
            total_price += cpu_cooler.price;
        }

        for ram in self.ram.iter() {
            total_price += ram.price;
        }

        if let Some(gpu) = &self.gpu {
            total_price += gpu.price;
        }

        for storage in self.storage.iter() {
            total_price += storage.price;
        }

        for fan in self.fans.iter() {
            total_price += fan.price;
        }

        if let Some(psu) = &self.psu {
            total_price += psu.price;
        }

        total_price
    }
}

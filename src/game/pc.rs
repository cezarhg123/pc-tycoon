use super::pc_components::*;

#[derive(Debug, Clone)]
pub struct PC {
    case: Option<Case>,
    mb: Option<MB>,
    cpu: Option<CPU>,
    cpu_cooler: Option<CPUCooler>,
    ram: Vec<RAM>,
    gpu: Option<GPU>,
    storage: Vec<Storage>,
    fan: Vec<Fan>,
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
            fan: Vec::new(),
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
        self.fan.clear();
        self.psu = None;
    }

    pub fn set_mb(&mut self, mb: MB) -> bool {
        if mb.mb_form_factor == self.case.as_ref().unwrap().mb_form_factor {
            self.mb = Some(mb);
            self.cpu = None;
            self.cpu_cooler = None;
            self.ram.clear();
            false
        } else {
            true
        }
    }
}

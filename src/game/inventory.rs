use glfw::Window;

use crate::{gfx::{image_rect::ImageRect, color_rect::ColorRect, texture::Texture, vectors::{vec2::vec2, vec3::vec3}, text::Text}, ui::{Ui, button::Button, listbox::ListBox, info_popup::InfoPopup}, WINDOW_WIDTH, WINDOW_HEIGHT, part_loader::{get_case, get_mb, get_cpu, get_cpu_cooler, get_ram, get_gpu, get_storage, get_fan, get_psu}};

use super::{save::Save, Scroll};

#[derive(Debug, Clone)]
pub struct Inventory<'a> {
    background: ImageRect,
    back_button: Button<'a>,
    popups: Vec<InfoPopup<'a>>,
    case_list: ListBox<'a>,
    mb_list: ListBox<'a>,
    cpu_list: ListBox<'a>,
    cpu_cooler_list: ListBox<'a>,
    ram_list: ListBox<'a>,
    gpu_list: ListBox<'a>,
    storage_list: ListBox<'a>,
    fan_list: ListBox<'a>,
    psu_list: ListBox<'a>,
    label_texts: Vec<Text<'a>>
}

impl<'a> Inventory<'a> {
    pub fn new(save: &Save, ui: &'a Ui) -> Inventory<'a> {
        // y + 220

        let case_list = if save.inventory.cases.as_slice().len() > 0 {
            ListBox::new(vec2(20.0, 140.0), vec2(600.0, 140.0), save.inventory.cases.as_slice(), 24.0, ui)
        } else {
            ListBox::new(vec2(20.0, 140.0), vec2(600.0, 140.0), ["None".to_string()].as_slice(), 24.0, ui)
        };

        let mut case_label = ui.text("Cases", 30.0, vec3(0, 0, 0), Some(vec2(40.0, 100.0)));
        case_label.set_center(case_list.get_center());
        case_label.set_top(case_list.get_top() - 30.0);

        let mb_list = if save.inventory.mbs.as_slice().len() > 0 {
            ListBox::new(vec2(660.0, 140.0), vec2(600.0, 140.0), save.inventory.mbs.as_slice(), 24.0, ui)
        } else {
            ListBox::new(vec2(660.0, 140.0), vec2(600.0, 140.0), ["None".to_string()].as_slice(), 24.0, ui)
        };

        let mut mb_label = ui.text("Motherboards", 30.0, vec3(0, 0, 0), Some(vec2(40.0, 100.0)));
        mb_label.set_center(mb_list.get_center());
        mb_label.set_top(mb_list.get_top() - 30.0);

        let cpu_list = if save.inventory.cpus.as_slice().len() > 0 {
            ListBox::new(vec2(1300.0, 140.0), vec2(600.0, 140.0), save.inventory.cpus.as_slice(), 24.0, ui)
        } else {
            ListBox::new(vec2(1300.0, 140.0), vec2(600.0, 140.0), ["None".to_string()].as_slice(), 24.0, ui)
        };

        let mut cpu_label = ui.text("CPUs", 30.0, vec3(0, 0, 0), Some(vec2(40.0, 100.0)));
        cpu_label.set_center(cpu_list.get_center());
        cpu_label.set_top(cpu_list.get_top() - 30.0);

        let cpu_cooler_list = if save.inventory.cpu_coolers.as_slice().len() > 0 {
            ListBox::new(vec2(20.0, 360.0), vec2(600.0, 140.0), save.inventory.cpu_coolers.as_slice(), 24.0, ui)
        } else {
            ListBox::new(vec2(20.0, 360.0), vec2(600.0, 140.0), ["None".to_string()].as_slice(), 24.0, ui)
        };

        let mut cpu_cooler_label = ui.text("CPU Coolers", 30.0, vec3(0, 0, 0), Some(vec2(40.0, 100.0)));
        cpu_cooler_label.set_center(cpu_cooler_list.get_center());
        cpu_cooler_label.set_top(cpu_cooler_list.get_top() - 30.0);

        let ram_list = if save.inventory.rams.as_slice().len() > 0 {
            ListBox::new(vec2(660.0, 360.0), vec2(600.0, 140.0), save.inventory.rams.as_slice(), 24.0, ui)
        } else {
            ListBox::new(vec2(660.0, 360.0), vec2(600.0, 140.0), ["None".to_string()].as_slice(), 24.0, ui)
        };

        let mut ram_label = ui.text("RAMs", 30.0, vec3(0, 0, 0), Some(vec2(40.0, 100.0)));
        ram_label.set_center(ram_list.get_center());
        ram_label.set_top(ram_list.get_top() - 30.0);

        let gpu_list = if save.inventory.gpus.as_slice().len() > 0 {
            ListBox::new(vec2(1300.0, 360.0), vec2(600.0, 140.0), save.inventory.gpus.as_slice(), 24.0, ui)
        } else {
            ListBox::new(vec2(1300.0, 360.0), vec2(600.0, 140.0), ["None".to_string()].as_slice(), 24.0, ui)
        };

        let mut gpu_label = ui.text("GPUs", 30.0, vec3(0, 0, 0), Some(vec2(40.0, 100.0)));
        gpu_label.set_center(gpu_list.get_center());
        gpu_label.set_top(gpu_list.get_top() - 30.0);

        let storage_list = if save.inventory.storages.as_slice().len() > 0 {
            ListBox::new(vec2(20.0, 580.0), vec2(600.0, 140.0), save.inventory.storages.as_slice(), 24.0, ui)
        } else {
            ListBox::new(vec2(20.0, 580.0), vec2(600.0, 140.0), ["None".to_string()].as_slice(), 24.0, ui)
        };

        let mut storage_label = ui.text("Storages", 30.0, vec3(0, 0, 0), Some(vec2(40.0, 100.0)));
        storage_label.set_center(storage_list.get_center());
        storage_label.set_top(storage_list.get_top() - 30.0);

        let fan_list = if save.inventory.fans.as_slice().len() > 0 {
            ListBox::new(vec2(660.0, 580.0), vec2(600.0, 140.0), save.inventory.fans.as_slice(), 24.0, ui)
        } else {
            ListBox::new(vec2(660.0, 580.0), vec2(600.0, 140.0), ["None".to_string()].as_slice(), 24.0, ui)
        };

        let mut fan_label = ui.text("Fans", 30.0, vec3(0, 0, 0), Some(vec2(40.0, 100.0)));
        fan_label.set_center(fan_list.get_center());
        fan_label.set_top(fan_list.get_top() - 30.0);

        let psu_list = if save.inventory.psus.as_slice().len() > 0 {
            ListBox::new(vec2(1300.0, 580.0), vec2(600.0, 140.0), save.inventory.psus.as_slice(), 24.0, ui)
        } else {
            ListBox::new(vec2(1300.0, 580.0), vec2(600.0, 140.0), ["None".to_string()].as_slice(), 24.0, ui)
        };

        let mut psu_label = ui.text("Power Supplies", 30.0, vec3(0, 0, 0), Some(vec2(40.0, 100.0)));
        psu_label.set_center(psu_list.get_center());
        psu_label.set_top(psu_list.get_top() - 30.0);

        Inventory {
            background: ImageRect::new(Texture::from_path("textures/inventory-background.png"), 0.0, 0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
            back_button: ui.button("a", vec2(1750.0, 0.0), vec2(170.0, 85.0)),
            popups: Vec::new(),
            case_list,
            mb_list,
            cpu_list,
            cpu_cooler_list,
            ram_list,
            gpu_list,
            storage_list,
            fan_list,
            psu_list,
            label_texts: vec![
                case_label,
                mb_label,
                cpu_label,
                cpu_cooler_label,
                ram_label,
                gpu_label,
                storage_label,
                fan_label,
                psu_label
            ]
        }
    }

    pub fn run(&mut self, window: &Window, scrolls: &mut Vec<Scroll>, ui: &'a Ui) -> bool {
        self.case_list.run(window, scrolls);
        self.mb_list.run(window, scrolls);

        let cursor_pos = window.get_cursor_pos().try_into().unwrap();
        let mut info = Vec::new();
        let mut info_type = String::new();

        // CASE INFO POPUP
        for text in &self.case_list.texts {
            if info_type.len() != 0 {
                break;
            }

            if text.contains(cursor_pos) {
                info_type = "case info".to_string();

                if text.get_str() == "None".to_string() {
                    info.push("None".to_string());
                } else {
                    let case = get_case(text.get_str().as_str());
                    info.append(&mut vec![
                        case.name,
                        format!("Price - {}", case.price),
                        format!("Case Form Factor - {:#?}", case.case_form_factor),
                        format!("Motherboard Form Factor - {:#?}", case.mb_form_factor),
                        format!("Max Fans - {}", case.max_fans),
                        format!("Max SSD - {}", case.max_ssd),
                        format!("Max HDD - {}", case.max_hdd),
                        format!("Max CPU Cooler Height - {}", case.max_cpu_cooler_height),
                        format!("Max GPU Length - {}", case.max_gpu_length),
                        format!("Max GPU Width - {}", case.max_gpu_width),
                        format!("Max Power Supply Length - {}", case.max_power_supply_length)    
                    ]);
                }

                break;
            } else {
                info_type.clear();
                let out = self.popups.iter().position(|s| s.id == info_type);
                match out {
                    Some(i) => {
                        self.popups.remove(i);
                    }
                    None => {}
                }
            }
        }

        // MB INFO POPUP
        for text in &self.mb_list.texts {
            if info_type.len() != 0 {
                break;
            }

            if text.contains(cursor_pos) {
                info_type = "mb info".to_string();

                if text.get_str() == "None".to_string() {
                    info.push("None".to_string());
                } else {
                    let mb = get_mb(text.get_str().as_str());
                    info.append(&mut vec![
                        mb.name,
                        format!("Price - {}", mb.price),
                        format!("Power Usage - {}", mb.power_usage),
                        format!("Motherboard Form Factor - {:#?}", mb.mb_form_factor),
                        format!("Socket Type - {:#?}", mb.socket_type),
                        format!("Ram Type - {:#?}", mb.ram_type),
                        format!("Ram Slots - {}", mb.ram_slots),
                        format!("M.2 Slots - {}", mb.m2_slots),
                        format!("Sata Slots - {}", mb.sata_slots),
                        format!("Max CPU Speed - {}MHz", mb.max_cpu_speed),
                        format!("Max RAM Speed - {}MHz", mb.max_ram_speed)
                    ]);
                }

                break;
            } else {
                info_type.clear();
                let out = self.popups.iter().position(|s| s.id == info_type);
                match out {
                    Some(i) => {
                        self.popups.remove(i);
                    }
                    None => {}
                }
            }
        }

        // CPU INFO POPUP
        for text in &self.cpu_list.texts {
            if info_type.len() != 0 {
                break;
            }

            if text.contains(cursor_pos) {
                info_type = "cpu info".to_string();

                if text.get_str() == "None".to_string() {
                    info.push("None".to_string());
                } else {
                    let cpu = get_cpu(text.get_str().as_str());
                    info.append(&mut vec![
                        cpu.name,
                        format!("Price - {}", cpu.price),
                        format!("Power Usage - {}", cpu.power_usage),
                        format!("Socket Type - {:#?}", cpu.socket_type),
                        format!("Cores - {}", cpu.cores),
                        format!("Threads - {}", cpu.threads),
                        format!("Speed - {}MHz", cpu.speed)
                    ]);
                }

                break;
            } else {
                info_type.clear();
                let out = self.popups.iter().position(|s| s.id == info_type);
                match out {
                    Some(i) => {
                        self.popups.remove(i);
                    }
                    None => {}
                }
            }
        }

        // CPU COOLER INFO POPUP
        for text in &self.cpu_cooler_list.texts {
            if info_type.len() != 0 {
                break;
            }

            if text.contains(cursor_pos) {
                info_type = "cpu cooler info".to_string();

                if text.get_str() == "None".to_string() {
                    info.push("None".to_string());
                } else {
                    let cpu_cooler = get_cpu_cooler(text.get_str().as_str());
                    info.append(&mut vec![
                        cpu_cooler.name,
                        format!("Price - {}", cpu_cooler.price),
                        format!("Power Usage - {}", cpu_cooler.power_usage),
                        format!("Socket Type - {:#?}", cpu_cooler.socket_type),
                        format!("Height - {}", cpu_cooler.height),
                        format!("Cooling - {}", cpu_cooler.cooling)
                    ]);
                }

                break;
            } else {
                info_type.clear();
                let out = self.popups.iter().position(|s| s.id == info_type);
                match out {
                    Some(i) => {
                        self.popups.remove(i);
                    }
                    None => {}
                }
            }
        }

        // RAM INFO POPUP
        for text in &self.ram_list.texts {
            if info_type.len() != 0 {
                break;
            }

            if text.contains(cursor_pos) {
                info_type = "ram info".to_string();

                if text.get_str() == "None".to_string() {
                    info.push("None".to_string());
                } else {
                    let ram = get_ram(text.get_str().as_str());
                    info.append(&mut vec![
                        ram.name,
                        format!("Price - {}", ram.price),
                        format!("Power Usage - {}", ram.power_usage),
                        format!("Ram Type - {:#?}", ram.ram_type),
                        format!("Size - {}GB", ram.size),
                        format!("Speed - {}MHz", ram.speed)
                    ]);
                }

                break;
            } else {
                info_type.clear();
                let out = self.popups.iter().position(|s| s.id == info_type);
                match out {
                    Some(i) => {
                        self.popups.remove(i);
                    }
                    None => {}
                }
            }
        }

        // GPU INFO POPUP
        for text in &self.gpu_list.texts {
            if info_type.len() != 0 {
                break;
            }

            if text.contains(cursor_pos) {
                info_type = "gpu info".to_string();

                if text.get_str() == "None".to_string() {
                    info.push("None".to_string());
                } else {
                    let gpu = get_gpu(text.get_str().as_str());
                    info.append(&mut vec![
                        gpu.name,
                        format!("Price - {}", gpu.price),
                        format!("Power Usage - {}", gpu.power_usage),
                        format!("Length - {}", gpu.length),
                        format!("Width - {}", gpu.width),
                        format!("Cores - {}", gpu.cores),
                        format!("Ray Tracing Cores - {}", gpu.rt_cores),
                        format!("Speed - {}MHz", gpu.speed),
                        format!("VRAM - {}GB", gpu.vram)
                    ]);
                }

                break;
            } else {
                info_type.clear();
                let out = self.popups.iter().position(|s| s.id == info_type);
                match out {
                    Some(i) => {
                        self.popups.remove(i);
                    }
                    None => {}
                }
            }
        }

        // STORAGE INFO POPUP
        for text in &self.storage_list.texts {
            if info_type.len() != 0 {
                break;
            }

            if text.contains(cursor_pos) {
                info_type = "storage info".to_string();

                if text.get_str() == "None".to_string() {
                    info.push("None".to_string());
                } else {
                    let storage = get_storage(text.get_str().as_str());
                    info.append(&mut vec![
                        storage.name,
                        format!("Price - {}", storage.price),
                        format!("Power Usage - {}", storage.power_usage),
                        format!("Storage Type - {:#?}", storage.storage_type),
                        format!("Size - {}GB", storage.size),
                        format!("Speed - {}MB/S", storage.speed)
                    ]);
                }

                break;
            } else {
                info_type.clear();
                let out = self.popups.iter().position(|s| s.id == info_type);
                match out {
                    Some(i) => {
                        self.popups.remove(i);
                    }
                    None => {}
                }
            }
        }

        // FAN INFO POPUP
        for text in &self.fan_list.texts {
            if info_type.len() != 0 {
                break;
            }

            if text.contains(cursor_pos) {
                info_type = "fan info".to_string();

                if text.get_str() == "None".to_string() {
                    info.push("None".to_string());
                } else {
                    let fan = get_fan(text.get_str().as_str());
                    info.append(&mut vec![
                        fan.name,
                        format!("Price - {}", fan.price),
                        format!("Power Usage - {}", fan.power_usage),
                        if fan.large {
                            format!("Size - 140MM")
                        } else {
                            format!("Size - 120MM4")
                        },
                        format!("Cooling - {}", fan.cooling)
                    ]);
                }

                break;
            } else {
                info_type.clear();
                let out = self.popups.iter().position(|s| s.id == info_type);
                match out {
                    Some(i) => {
                        self.popups.remove(i);
                    }
                    None => {}
                }
            }
        }

        // PSU INFO POPUP
        for text in &self.psu_list.texts {
            if info_type.len() != 0 {
                break;
            }

            if text.contains(cursor_pos) {
                info_type = "psu info".to_string();

                if text.get_str() == "None".to_string() {
                    info.push("None".to_string());
                } else {
                    let psu = get_psu(text.get_str().as_str());
                    info.append(&mut vec![
                        psu.name,
                        format!("Price - {}", psu.price),
                        format!("Wattage - {}W", psu.wattage),
                        format!("Length - {}MM", psu.length)
                    ]);
                }

                break;
            } else {
                info_type.clear();
                let out = self.popups.iter().position(|s| s.id == info_type);
                match out {
                    Some(i) => {
                        self.popups.remove(i);
                    }
                    None => {}
                }
            }
        }
        
        // using 'info_type' to check if any popups should be created
        // if not then just pop from the 'popups' vector
        if info_type.len() > 0 {
            let popup = ui.info_popup(info_type.as_str(), info.as_slice(), cursor_pos, 24.0);
            
            // just push the current popup if there are no popups
            if self.popups.len() > 0 {
                for i in 0..self.popups.len() {
                    // if the current popup already exists then update it in the vector and break
                    // if not then just push the popup to the list and break
                    if self.popups[i].id == info_type.as_str() {
                        self.popups[i] = popup;
                        break;
                    } else {
                        self.popups.push(popup);
                        break;
                    }
                }
            } else {
                self.popups.push(popup);
            }
        } else {
            self.popups.pop();
        }
            
        self.back_button.clicked(window)
    }

    pub fn draw(&self) {
        self.background.draw();

        for text in &self.label_texts {
            text.draw();
        }

        self.case_list.draw();
        self.mb_list.draw();
        self.cpu_list.draw();
        self.cpu_cooler_list.draw();
        self.ram_list.draw();
        self.gpu_list.draw();
        self.storage_list.draw();
        self.fan_list.draw();
        self.psu_list.draw();

        for popup in &self.popups {
            popup.draw();
        }
    }
}

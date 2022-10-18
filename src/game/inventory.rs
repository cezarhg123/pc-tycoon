use imgui_glfw_rs::glfw::Action;
use imgui_glfw_rs::glfw::Window;
use imgui_glfw_rs::imgui::Ui;
use serde::{Serialize, Deserialize};
use crate::components_list::*;
use crate::str_to_imstr;
use super::GameState;



///INVENTORY STORES ITEM ALIASES
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Inventory {
    pub cases: Vec<String>,
    pub motherboards: Vec<String>,
    pub cpus: Vec<String>,
    pub cpu_coolers: Vec<String>,
    pub rams: Vec<String>,
    pub gpus: Vec<String>,
    pub storages: Vec<String>,
    pub fans: Vec<String>,
    pub power_supplys: Vec<String>
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            cases: Vec::new(),
            motherboards: Vec::new(),
            cpus: Vec::new(),
            cpu_coolers: Vec::new(),
            rams: Vec::new(),
            gpus: Vec::new(),
            storages: Vec::new(),
            fans: Vec::new(),
            power_supplys: Vec::new(),
        }
    }
}

pub fn show_inventory(inventory: &Inventory, window: &Window, ui: &Ui) -> bool {
    let mut show_inventory = true;

    ui.window(str_to_imstr("Inventory\0"))
    .collapsible(false)
    .build(|| {
        if ui.collapsing_header(str_to_imstr("Cases\0")).build() {
            for case in &inventory.cases {
                ui.indent();
                let mut case = get_case_list().iter().find(|f| f.alias == *case).unwrap().clone();
                case.name.pop();
                                
                ui.text(&case.name);
                let hovered = ui.is_item_hovered();
                let clicked = window.get_mouse_button(imgui_glfw_rs::glfw::MouseButton::Button2) == Action::Press;
                ui.popup(str_to_imstr("Case Info\0"), || {
                    ui.text(format!("Size - {:#?}", case.size));
                    ui.text(format!("Max fans - {}", case.max_fans));
                    ui.text(format!("Max CPU Cooler Height - {}", case.max_cpu_cooler_height));
                    ui.text(format!("Max GPU Length - {}", case.max_gpu_length));
                    ui.text(format!("Max GPU Width - {}", case.max_gpu_width));
                    ui.text(format!("Max PSU Length - {}", case.max_power_supply_length));
                    ui.text(format!("Max 2.5 Drives - {}", case.max_drives_2_5));
                    ui.text(format!("Max 3.5 Drives - {}", case.max_drives_3_5));
                    ui.text(format!("Price - {}", case.price));
                });
                
                if hovered && clicked {
                    ui.open_popup(str_to_imstr("Case Info\0"));
                }

                ui.unindent();
            }
        }

        if ui.collapsing_header(str_to_imstr("Motherboards\0")).build() {
            for motherboard in &inventory.motherboards {
                ui.indent();
                let mut motherboard = get_motherboard_list().iter().find(|f| f.alias == *motherboard).unwrap().clone();
                motherboard.name.pop();
                                
                ui.text(&motherboard.name);
                let hovered = ui.is_item_hovered();
                let clicked = window.get_mouse_button(imgui_glfw_rs::glfw::MouseButton::Button2) == Action::Press;
                ui.popup(str_to_imstr("Motherboard Info\0"), || {
                    ui.text(format!("Size - {:#?}", motherboard.size));
                    ui.text(format!("Socket Type - {:#?}", motherboard.socket_type));
                    ui.text(format!("Max CPU Speed - {}", motherboard.max_cpu_speed));
                    ui.text(format!("Max RAM Speed - {}", motherboard.max_ram_speed));
                    ui.text(format!("Ram Slots - {}", motherboard.ram_slots));
                    ui.text(format!("M.2 Slots - {}", motherboard.m2_slots));
                    ui.text(format!("Max Sata Slots - {} (doesn't matter for now)", motherboard.max_storage_devices));
                    ui.text(format!("Price - {}", motherboard.price));
                });
                
                if hovered && clicked {
                    ui.open_popup(str_to_imstr("Motherboard Info\0"));
                }

                ui.unindent();
            }
        }

        if ui.collapsing_header(str_to_imstr("CPUs\0")).build() {
            for cpu in &inventory.cpus {
                ui.indent();
                let mut cpu = get_cpu_list().iter().find(|f| f.alias == *cpu).unwrap().clone();
                cpu.name.pop();
                                
                ui.text(&cpu.name);
                let hovered = ui.is_item_hovered();
                let clicked = window.get_mouse_button(imgui_glfw_rs::glfw::MouseButton::Button2) == Action::Press;
                ui.popup(str_to_imstr("CPU Info\0"), || {
                    ui.text(format!("Socket Type - {:#?}", cpu.socket_type));
                    ui.text(format!("Cores - {}", cpu.cores));
                    ui.text(format!("Threads - {}", cpu.threads));
                    ui.text(format!("Speed - {}MHz", cpu.speed));
                    ui.text(format!("{}W Usage", cpu.power_usage));
                    ui.text(format!("Price - {}", cpu.price));
                });
                
                if hovered && clicked {
                    ui.open_popup(str_to_imstr("CPU Info\0"));
                }

                ui.unindent();
            }
        }

        if ui.collapsing_header(str_to_imstr("CPU Coolers\0")).build() {
            for cpu_cooler in &inventory.cpu_coolers {
                ui.indent();
                let mut cpu_cooler = get_cpu_cooler_list().iter().find(|f| f.alias == *cpu_cooler).unwrap().clone();
                cpu_cooler.name.pop();
                                
                ui.text(&cpu_cooler.name);
                let hovered = ui.is_item_hovered();
                let clicked = window.get_mouse_button(imgui_glfw_rs::glfw::MouseButton::Button2) == Action::Press;
                ui.popup(str_to_imstr("CPU Cooler Info\0"), || {
                    ui.text(format!("Socket Type - {:#?}", cpu_cooler.socket_type));
                    ui.text(format!("Height - {}mm", cpu_cooler.height));
                    ui.text(format!("Effectiveness - {}", cpu_cooler.base));
                    ui.text(format!("{}W Usage", cpu_cooler.power_usage));
                    ui.text(format!("Price - {}", cpu_cooler.price));
                });
                
                if hovered && clicked {
                    ui.open_popup(str_to_imstr("CPU Cooler Info\0"));
                }

                ui.unindent();
            }
        }

        if ui.collapsing_header(str_to_imstr("RAMs\0")).build() {
            for ram in &inventory.rams {
                ui.indent();
                let mut ram = get_ram_list().iter().find(|f| f.alias == *ram).unwrap().clone();
                ram.name.pop();
                                
                ui.text(&ram.name);
                let hovered = ui.is_item_hovered();
                let clicked = window.get_mouse_button(imgui_glfw_rs::glfw::MouseButton::Button2) == Action::Press;
                ui.popup(str_to_imstr("RAM Info\0"), || {
                    ui.text(format!("{}GB", ram.size));
                    ui.text(format!("{}MHz", ram.speed));
                    ui.text(format!("Price - {}", ram.price));
                });
                
                if hovered && clicked {
                    ui.open_popup(str_to_imstr("RAM Info\0"));
                }

                ui.unindent();
            }
        }

        if ui.collapsing_header(str_to_imstr("GPUs\0")).build() {
            for gpu in &inventory.gpus {
                ui.indent();
                let mut gpu = get_gpu_list().iter().find(|f| f.alias == *gpu).unwrap().clone();
                gpu.name.pop();
                                
                ui.text(&gpu.name);
                let hovered = ui.is_item_hovered();
                let clicked = window.get_mouse_button(imgui_glfw_rs::glfw::MouseButton::Button2) == Action::Press;
                ui.popup(str_to_imstr("GPU Info\0"), || {
                    ui.text(format!("{} Cores", gpu.cores));
                    ui.text(format!("{} Ray Tracing Cores", gpu.rt_cores));
                    ui.text(format!("{}MHz GPU Clock", gpu.speed));
                    ui.text(format!("{}GB Vram", gpu.vram));
                    ui.text(format!("{}mm Length", gpu.length));
                    ui.text(format!("{}mm Width", gpu.width));
                    ui.text(format!("Cooling - {}", gpu.cooling));
                    ui.text(format!("{}W Usage", gpu.power_usage));
                    ui.text(format!("Price - {}", gpu.price));
                });
                
                if hovered && clicked {
                    ui.open_popup(str_to_imstr("GPU Info\0"));
                }

                ui.unindent();
            }
        }

        if ui.collapsing_header(str_to_imstr("Storage Devices\0")).build() {
            for storage in &inventory.storages {
                ui.indent();
                let mut storage = get_storage_list().iter().find(|f| f.alias == *storage).unwrap().clone();
                storage.name.pop();
                                
                ui.text(&storage.name);
                let hovered = ui.is_item_hovered();
                let clicked = window.get_mouse_button(imgui_glfw_rs::glfw::MouseButton::Button2) == Action::Press;
                ui.popup(str_to_imstr("Storage Device Info\0"), || {
                    ui.text(format!("Type - {:#?}", storage.storage_device_type));
                    ui.text(format!("{}GB", storage.size));
                    ui.text(format!("Price - {}", storage.price));
                });
                
                if hovered && clicked {
                    ui.open_popup(str_to_imstr("Storage Device Info\0"));
                }

                ui.unindent();
            }
        }

        if ui.collapsing_header(str_to_imstr("Fans\0")).build() {
            for fan in &inventory.fans {
                ui.indent();
                let mut fan = get_fan_list().iter().find(|f| f.alias == *fan).unwrap().clone();
                fan.name.pop();
                                
                ui.text(&fan.name);
                let hovered = ui.is_item_hovered();
                let clicked = window.get_mouse_button(imgui_glfw_rs::glfw::MouseButton::Button2) == Action::Press;
                ui.popup(str_to_imstr("Fan Info\0"), || {
                    if fan.large {
                        ui.text("140mm");
                    } else {
                        ui.text("120mm");
                    }
                    ui.text(format!("Cooling - {}", fan.effectiveness));
                    ui.text(format!("Price - {}", fan.price));
                });
                
                if hovered && clicked {
                    ui.open_popup(str_to_imstr("Fan Info\0"));
                }

                ui.unindent();
            }
        }

        if ui.collapsing_header(str_to_imstr("Power Supplys\0")).build() {
            for power_supply in &inventory.power_supplys {
                ui.indent();
                let mut power_supply = get_power_supply_list().iter().find(|f| f.alias == *power_supply).unwrap().clone();
                power_supply.name.pop();
                                
                ui.text(&power_supply.name);
                let hovered = ui.is_item_hovered();
                let clicked = window.get_mouse_button(imgui_glfw_rs::glfw::MouseButton::Button2) == Action::Press;
                ui.popup(str_to_imstr("Power Supply Info\0"), || {
                    ui.text(format!("Form Factor - {:#?}", power_supply.form_factor));
                    ui.text(format!("{}mm Length", power_supply.length));
                    ui.text(format!("{}W", power_supply.wattage));
                    ui.text(format!("Price - {}", power_supply.price));
                });
                
                if hovered && clicked {
                    ui.open_popup(str_to_imstr("Power Supply Info\0"));
                }

                ui.unindent();
            }
        }

        if ui.button(str_to_imstr("Exit\0"), [60.0, 30.0]) {
            show_inventory = false;
        }
    });

    show_inventory
}

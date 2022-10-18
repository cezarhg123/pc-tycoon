use imgui_glfw_rs::glfw::Action;
use imgui_glfw_rs::{imgui::Ui, glfw::Window};
use crate::str_to_imstr;
use crate::components_list::*;

use super::Save;

pub fn market(active_save: &mut Save, ui: &Ui, window: &Window) -> bool {
    let mut out = true;

    ui.window(str_to_imstr("Market\0"))
    .build(|| {
        if ui.collapsing_header(str_to_imstr("Cases\0")).build() {
            for case in get_case_list() {
                ui.indent();
                let mut case = case.clone();
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
                    
                    if ui.button(str_to_imstr("Buy\0"), [40.0, 20.0]) {
                        active_save.inventory.cases.push(case.alias.clone());
                        active_save.money -= case.price as i32;
                        ui.close_current_popup();
                    }
                });
                
                if hovered && clicked {
                    ui.open_popup(str_to_imstr("Case Info\0"));
                }

                ui.unindent();
            }
        }

        if ui.collapsing_header(str_to_imstr("Motherboards\0")).build() {
            for motherboard in get_motherboard_list() {
                ui.indent();
                let mut motherboard = motherboard.clone();
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
                    if ui.button(str_to_imstr("Buy\0"), [40.0, 20.0]) {
                        active_save.inventory.motherboards.push(motherboard.alias.clone());
                        active_save.money -= motherboard.price as i32;
                        ui.close_current_popup();
                    }
                });

                
                if hovered && clicked {
                    ui.open_popup(str_to_imstr("Motherboard Info\0"));
                }

                ui.unindent();
            }
        }

        if ui.collapsing_header(str_to_imstr("CPUs\0")).build() {
            for cpu in get_cpu_list() {
                ui.indent();
                let mut cpu = cpu.clone();
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
                    if ui.button(str_to_imstr("Buy\0"), [40.0, 20.0]) {
                        active_save.inventory.cpus.push(cpu.alias.clone());
                        active_save.money -= cpu.price as i32;
                        ui.close_current_popup();
                    }
                });
                

                if hovered && clicked {
                    ui.open_popup(str_to_imstr("CPU Info\0"));
                }

                ui.unindent();
            }
        }

        if ui.collapsing_header(str_to_imstr("CPU Coolers\0")).build() {
            for cpu_cooler in get_cpu_cooler_list() {
                ui.indent();
                let mut cpu_cooler = cpu_cooler.clone();
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
                    if ui.button(str_to_imstr("Buy\0"), [40.0, 20.0]) {
                        active_save.inventory.cpu_coolers.push(cpu_cooler.alias.clone());
                        active_save.money -= cpu_cooler.price as i32;
                        ui.close_current_popup();
                    }
                });
                
                
                if hovered && clicked {
                    ui.open_popup(str_to_imstr("CPU Cooler Info\0"));
                }

                ui.unindent();
            }
        }

        if ui.collapsing_header(str_to_imstr("RAMs\0")).build() {
            for ram in get_ram_list() {
                ui.indent();
                let mut ram = ram.clone();
                ram.name.pop();
                                
                ui.text(&ram.name);
                let hovered = ui.is_item_hovered();
                let clicked = window.get_mouse_button(imgui_glfw_rs::glfw::MouseButton::Button2) == Action::Press;
                ui.popup(str_to_imstr("RAM Info\0"), || {
                    ui.text(format!("{}GB", ram.size));
                    ui.text(format!("{}MHz", ram.speed));
                    ui.text(format!("Price - {}", ram.price));
                    if ui.button(str_to_imstr("Buy\0"), [40.0, 20.0]) {
                        active_save.inventory.rams.push(ram.alias.clone());
                        active_save.money -= ram.price as i32;
                        ui.close_current_popup();
                    }
                });

                
                if hovered && clicked {
                    ui.open_popup(str_to_imstr("RAM Info\0"));
                }

                ui.unindent();
            }
        }

        if ui.collapsing_header(str_to_imstr("GPUs\0")).build() {
            for gpu in get_gpu_list() {
                ui.indent();
                let mut gpu = gpu.clone();
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
                    if ui.button(str_to_imstr("Buy\0"), [40.0, 20.0]) {
                        active_save.inventory.gpus.push(gpu.alias.clone());
                        active_save.money -= gpu.price as i32;
                        ui.close_current_popup();
                    }
                });

                
                if hovered && clicked {
                    ui.open_popup(str_to_imstr("GPU Info\0"));
                }

                ui.unindent();
            }
        }

        if ui.collapsing_header(str_to_imstr("Storage Devices\0")).build() {
            for storage in get_storage_list() {
                ui.indent();
                let mut storage = storage.clone();
                storage.name.pop();
                                
                ui.text(&storage.name);
                let hovered = ui.is_item_hovered();
                let clicked = window.get_mouse_button(imgui_glfw_rs::glfw::MouseButton::Button2) == Action::Press;
                ui.popup(str_to_imstr("Storage Device Info\0"), || {
                    ui.text(format!("Type - {:#?}", storage.storage_device_type));
                    ui.text(format!("{}GB", storage.size));
                    ui.text(format!("Price - {}", storage.price));
                    if ui.button(str_to_imstr("Buy\0"), [40.0, 20.0]) {
                        active_save.inventory.storages.push(storage.alias.clone());
                        active_save.money -= storage.price as i32;
                        ui.close_current_popup();
                    }
                });

                
                if hovered && clicked {
                    ui.open_popup(str_to_imstr("Storage Device Info\0"));
                }

                ui.unindent();
            }
        }

        if ui.collapsing_header(str_to_imstr("Fans\0")).build() {
            for fan in get_fan_list() {
                ui.indent();
                let mut fan = fan.clone();
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
                    if ui.button(str_to_imstr("Buy\0"), [40.0, 20.0]) {
                        active_save.inventory.fans.push(fan.alias.clone());
                        active_save.money -= fan.price as i32;
                        ui.close_current_popup();
                    }
                });

                
                if hovered && clicked {
                    ui.open_popup(str_to_imstr("Fan Info\0"));
                }

                ui.unindent();
            }
        }

        if ui.collapsing_header(str_to_imstr("Power Supplys\0")).build() {
            for power_supply in get_power_supply_list() {
                ui.indent();
                let mut power_supply = power_supply.clone();
                power_supply.name.pop();
                                
                ui.text(&power_supply.name);
                let hovered = ui.is_item_hovered();
                let clicked = window.get_mouse_button(imgui_glfw_rs::glfw::MouseButton::Button2) == Action::Press;
                ui.popup(str_to_imstr("Power Supply Info\0"), || {
                    ui.text(format!("Form Factor - {:#?}", power_supply.form_factor));
                    ui.text(format!("{}mm Length", power_supply.length));
                    ui.text(format!("{}W", power_supply.wattage));
                    ui.text(format!("Price - {}", power_supply.price));
                    
                    if ui.button(str_to_imstr("Buy\0"), [40.0, 20.0]) {
                        active_save.inventory.power_supplys.push(power_supply.alias.clone());
                        active_save.money -= power_supply.price as i32;
                        ui.close_current_popup();
                    }
                });

                
                if hovered && clicked {
                    ui.open_popup(str_to_imstr("Power Supply Info\0"));
                }

                ui.unindent();
            }
        }

        if ui.button(str_to_imstr("Exit\0"), [60.0, 40.0]) {
            out = false;
        }
    });

    out
}
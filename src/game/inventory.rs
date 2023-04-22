use std::io::Cursor;

use glium::{Display, Frame};

use crate::{gfx::rect::{Rect, RectBuilder}, math::{vec2::vec2, vec4::vec4, vec3::vec3}, get_window_width, get_window_height, get_ui_mut, ui::{button::{ButtonBuilder, ButtonFace}, uielement::{UiOutput, UiElement}, textline::TextLineBuilder, listbox::ListboxBuilder, multitextline::{MultiTextLine, MultiTextLineBuilder, TextLayout}}, get_ui, part_loader::{get_case, get_mb, get_cpu, get_cpu_cooler, get_ram, get_gpu, get_storage, get_fan, get_psu}};

use super::profile::{Profile, ItemType};

pub struct Inventory {
    background: Rect,
    /// 0 = back rect for outline
    /// 1 = black rect
    /// 2 = info text
    popup: Option<(Rect, Rect, MultiTextLine)>
}

impl Inventory {
    pub fn new(display: &Display, profile: &Profile) -> Inventory {
        let background = RectBuilder {
            position: vec2(get_window_width() as f32 / 2.0, get_window_height() as f32 / 2.0),
            size: vec2(get_window_width() as f32, get_window_height() as f32),
            color: vec4(1.0, 1.0, 1.0, 1.0),
            texture: Some(image::load(
                Cursor::new(std::fs::read("textures/inventory-background.png").unwrap()),
                image::ImageFormat::Png
            ).unwrap())
        }.build(display);

        let close_button = get_ui_mut().add_element(ButtonBuilder {
            id: "close_button".to_string(),
            custom_data: Vec::new(),
            position: vec2(1842.0, 1038.0),
            size: vec2(155.0, 84.0),
            text: None,
            normal_face: ButtonFace::Color(vec4(1.0, 1.0, 1.0, 0.6)),
            hovered_face: None,
            clicked_face: None
        }.build(display));

        let case_names: Vec<String> = profile.items.iter().filter(|item| item.1 == ItemType::Case).map(|item| item.0.clone()).collect();
        let mb_names: Vec<String> = profile.items.iter().filter(|item| item.1 == ItemType::MB).map(|item| item.0.clone()).collect();
        let cpu_names: Vec<String> = profile.items.iter().filter(|item| item.1 == ItemType::CPU).map(|item| item.0.clone()).collect();
        let cpu_cooler_names: Vec<String> = profile.items.iter().filter(|item| item.1 == ItemType::CPUCooler).map(|item| item.0.clone()).collect();
        let ram_names: Vec<String> = profile.items.iter().filter(|item| item.1 == ItemType::RAM).map(|item| item.0.clone()).collect();
        let gpu_names: Vec<String> = profile.items.iter().filter(|item| item.1 == ItemType::GPU).map(|item| item.0.clone()).collect();
        let storage_names: Vec<String> = profile.items.iter().filter(|item| item.1 == ItemType::Storage).map(|item| item.0.clone()).collect();
        let fan_names: Vec<String> = profile.items.iter().filter(|item| item.1 == ItemType::Fan).map(|item| item.0.clone()).collect();
        let psu_names: Vec<String> = profile.items.iter().filter(|item| item.1 == ItemType::PSU).map(|item| item.0.clone()).collect();
        
        let cases_listbox = get_ui_mut().add_element(ListboxBuilder {
            id: "cases_listbox".to_string(),
            custom_data: Vec::new(),
            elements: if case_names.len() > 0 {
                case_names.iter().enumerate().map(|case| {
                    get_ui_mut().add_element(TextLineBuilder {
                        id: format!("{}{}", case.1, case.0),
                        custom_data: Vec::new(),
                        text: case.1.clone(),
                        font_size: 36.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display))
                }).collect()
            } else {
                vec![
                    get_ui_mut().add_element(TextLineBuilder {
                        id: "No Cases".to_string(),
                        custom_data: Vec::new(),
                        text: "No Cases".to_string(),
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display))
                ]
            },
            bar_width: 10.0,
            position: vec2(180.0, 748.0),
            size: vec2(237.0, 300.0)
        }.build(display));

        let mbs_listbox = get_ui_mut().add_element(ListboxBuilder {
            id: "mbs_listbox".to_string(),
            custom_data: Vec::new(),
            elements: if mb_names.len() > 0 {
                mb_names.iter().enumerate().map(|mb| {
                    get_ui_mut().add_element(TextLineBuilder {
                        id: format!("{}{}", mb.1, mb.0),
                        custom_data: Vec::new(),
                        text: mb.1.clone(),
                        font_size: 36.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display))
                }).collect()
            } else {
                vec![
                    get_ui_mut().add_element(TextLineBuilder {
                        id: "No Motherboard".to_string(),
                        custom_data: Vec::new(),
                        text: "No Motherboard".to_string(),
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display))
                ]
            },
            bar_width: 10.0,
            position: vec2(554.0, 748.0),
            size: vec2(237.0, 300.0)
        }.build(display));

        let cpus_listbox = get_ui_mut().add_element(ListboxBuilder {
            id: "cpus_listbox".to_string(),
            custom_data: Vec::new(),
            elements: if cpu_names.len() > 0 {
                cpu_names.iter().enumerate().map(|cpu| {
                    get_ui_mut().add_element(TextLineBuilder {
                        id: format!("{}{}", cpu.1, cpu.0),
                        custom_data: Vec::new(),
                        text: cpu.1.clone(),
                        font_size: 36.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display))
                }).collect()
            } else {
                vec![
                    get_ui_mut().add_element(TextLineBuilder {
                        id: "No CPU".to_string(),
                        custom_data: Vec::new(),
                        text: "No CPU".to_string(),
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display))
                ]
            },
            bar_width: 10.0,
            position: vec2(919.0, 748.0),
            size: vec2(237.0, 300.0)
        }.build(display));

        let cpu_coolers_listbox = get_ui_mut().add_element(ListboxBuilder {
            id: "cpu_coolers_listbox".to_string(),
            custom_data: Vec::new(),
            elements: if cpu_cooler_names.len() > 0 {
                cpu_cooler_names.iter().enumerate().map(|cpu_cooler| {
                    get_ui_mut().add_element(TextLineBuilder {
                        id: format!("{}{}", cpu_cooler.1, cpu_cooler.0),
                        custom_data: Vec::new(),
                        text: cpu_cooler.1.clone(),
                        font_size: 36.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display))
                }).collect()
            } else {
                vec![
                    get_ui_mut().add_element(TextLineBuilder {
                        id: "No CPU Cooler".to_string(),
                        custom_data: Vec::new(),
                        text: "No CPU Cooler".to_string(),
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display))
                ]
            },
            bar_width: 10.0,
            position: vec2(1314.0, 748.0),
            size: vec2(237.0, 300.0)
        }.build(display));

        let rams_listbox = get_ui_mut().add_element(ListboxBuilder {
            id: "rams_listbox".to_string(),
            custom_data: Vec::new(),
            elements: if ram_names.len() > 0 {
                ram_names.iter().enumerate().map(|ram| {
                    get_ui_mut().add_element(TextLineBuilder {
                        id: format!("{}{}", ram.1, ram.0),
                        custom_data: Vec::new(),
                        text: ram.1.clone(),
                        font_size: 36.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display))
                }).collect()
            } else {
                vec![
                    get_ui_mut().add_element(TextLineBuilder {
                        id: "No RAM".to_string(),
                        custom_data: Vec::new(),
                        text: "No RAM".to_string(),
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display))
                ]
            },
            bar_width: 10.0,
            position: vec2(1681.0, 748.0),
            size: vec2(237.0, 300.0)
        }.build(display));

        let gpus_listbox = get_ui_mut().add_element(ListboxBuilder {
            id: "gpus_listbox".to_string(),
            custom_data: Vec::new(),
            elements: if gpu_names.len() > 0 {
                gpu_names.iter().enumerate().map(|gpu| {
                    get_ui_mut().add_element(TextLineBuilder {
                        id: format!("{}{}", gpu.1, gpu.0),
                        custom_data: Vec::new(),
                        text: gpu.1.clone(),
                        font_size: 36.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display))
                }).collect()
            } else {
                vec![
                    get_ui_mut().add_element(TextLineBuilder {
                        id: "No GPU".to_string(),
                        custom_data: Vec::new(),
                        text: "No GPU".to_string(),
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display))
                ]
            },
            bar_width: 10.0,
            position: vec2(180.0, 248.0),
            size: vec2(237.0, 300.0)
        }.build(display));

        let storages_listbox = get_ui_mut().add_element(ListboxBuilder {
            id: "storages_listbox".to_string(),
            custom_data: Vec::new(),
            elements: if storage_names.len() > 0 {
                storage_names.iter().enumerate().map(|storage| {
                    get_ui_mut().add_element(TextLineBuilder {
                        id: format!("{}{}", storage.1, storage.0),
                        custom_data: Vec::new(),
                        text: storage.1.clone(),
                        font_size: 36.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display))
                }).collect()
            } else {
                vec![
                    get_ui_mut().add_element(TextLineBuilder {
                        id: "No Storage".to_string(),
                        custom_data: Vec::new(),
                        text: "No Storage".to_string(),
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display))
                ]
            },
            bar_width: 10.0,
            position: vec2(697.0, 248.0),
            size: vec2(237.0, 300.0)
        }.build(display));

        let fans_listbox = get_ui_mut().add_element(ListboxBuilder {
            id: "fans_listbox".to_string(),
            custom_data: Vec::new(),
            elements: if fan_names.len() > 0 {
                fan_names.iter().enumerate().map(|fan| {
                    get_ui_mut().add_element(TextLineBuilder {
                        id: format!("{}{}", fan.1, fan.0),
                        custom_data: Vec::new(),
                        text: fan.1.clone(),
                        font_size: 36.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display))
                }).collect()
            } else {
                vec![
                    get_ui_mut().add_element(TextLineBuilder {
                        id: "No Fan".to_string(),
                        custom_data: Vec::new(),
                        text: "No Fan".to_string(),
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display))
                ]
            },
            bar_width: 10.0,
            position: vec2(1213.0, 248.0),
            size: vec2(237.0, 300.0)
        }.build(display));

        let psus_listbox = get_ui_mut().add_element(ListboxBuilder {
            id: "psus_listbox".to_string(),
            custom_data: Vec::new(),
            elements: if psu_names.len() > 0 {
                psu_names.iter().enumerate().map(|psu| {
                    get_ui_mut().add_element(TextLineBuilder {
                        id: format!("{}{}", psu.1, psu.0),
                        custom_data: Vec::new(),
                        text: psu.1.clone(),
                        font_size: 36.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display))
                }).collect()
            } else {
                vec![
                    get_ui_mut().add_element(TextLineBuilder {
                        id: "No PSU".to_string(),
                        custom_data: Vec::new(),
                        text: "No PSU".to_string(),
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display))
                ]
            },
            bar_width: 10.0,
            position: vec2(1700.0, 248.0),
            size: vec2(237.0, 300.0)
        }.build(display));
        
        Inventory {
            background,
            popup: None
        }
    }

    pub fn run(&mut self, profile: &Profile, display: &Display) -> bool {
        // go back to ingame menu if the top right button is pressed
        if get_ui().get_element("close_button").unwrap().output() == UiOutput::LeftClicked {
            return true;
        }

        let mut items_checked: Vec<&(String, ItemType)> = Vec::new();
        for item in profile.items.iter() {
            let item_name = item.0.clone();
            let item_type = item.1.clone();

            let id = format!("{}{}", item.0, items_checked.iter().filter(|i| i.0 == item.0).count());

            if let Some(item) = get_ui().get_element(id.as_str()) {
                if item.output() == UiOutput::Hovered {
                    let info = match item_type {
                        ItemType::Case => {
                            let case = get_case(&item_name);

                            format!(r#"{}
Price: ${}
Case Form Factor: {}
Motherboard
Form Factor: {}
Max Fans: {}
Max SSD: {}
Max HDD: {}
Max CPU Cooler
Height: {}mm,
Max GPU Length: {}mm
Max GPU Width: {}mm
Max Power Supply
Length: {}mm"#, case.name.clone(),
                                case.price,
                                case.case_form_factor.to_string(),
                                case.mb_form_factor.to_string(),
                                case.max_fans,
                                case.max_ssd,
                                case.max_hdd,
                                case.max_cpu_cooler_height,
                                case.max_gpu_length,
                                case.max_gpu_width,
                                case.max_power_supply_length
                            )
                        }
                        ItemType::MB => {
                            let mb = get_mb(&item_name);

                            format!(r#"{}
Price: ${}
Power Usage: {}W
Motherboard Form
Factor: {}
Socket Type: {}
RAM Type: {}
RAM Slots: {}
M.2 Slots: {}
Sata Slots: {}
Max CPU Speed: {}MHz
Max RAM Speed: {}MHz"#, mb.name.clone(),
                                mb.price,
                                mb.power_usage,
                                mb.mb_form_factor.to_string(),
                                mb.socket_type.to_string(),
                                mb.ram_type.to_string(),
                                mb.ram_slots,
                                mb.m2_slots,
                                mb.sata_slots,
                                mb.max_cpu_speed,
                                mb.max_ram_speed
                            )
                        }
                        ItemType::CPU => {
                            let cpu = get_cpu(&item_name);

                            format!(r#"{}
Price: ${}
Power Usage: {}W
Socket Type: {}
Cores: {}
Threads: {}
Speed: {}MHz"#, cpu.name.clone(),
                                cpu.price,
                                cpu.power_usage,
                                cpu.socket_type.to_string(),
                                cpu.cores,
                                cpu.threads,
                                cpu.speed
                            )
                        }
                        ItemType::CPUCooler => {
                            let cpu_cooler = get_cpu_cooler(&item_name);

                            format!(r#"{}
Price: ${}
Power Usage: {}W
Socket Type: {}
Height: {}mm"#, cpu_cooler.name.clone(),
                                cpu_cooler.price,
                                cpu_cooler.power_usage,
                                cpu_cooler.socket_type.to_string(),
                                cpu_cooler.height
                            )
                        }
                        ItemType::RAM => {
                            let ram = get_ram(&item_name);

                            format!(r#"{}
Price: ${}
Power Usage: {}W
Ram Type: {}
Size: {}GB
Speed: {}MHz"#, ram.name.clone(),
                                ram.price,
                                ram.power_usage,
                                ram.ram_type.to_string(),
                                ram.size,
                                ram.speed
                            )
                        }
                        ItemType::GPU => {
                            let gpu = get_gpu(&item_name);

                            format!(r#"{}
Price: ${}
Power Usage: {}W
Length: {}mm
Width: {}mm
Cores: {}
Ray Tracing
Cores: {}
Speed: {}MHz
VRAM: {}GB"#, gpu.name.clone(),
                                gpu.price,
                                gpu.power_usage,
                                gpu.length,
                                gpu.width,
                                gpu.cores,
                                gpu.rt_cores,
                                gpu.speed,
                                gpu.vram
                            )
                        }
                        ItemType::Storage => {
                            let storage = get_storage(&item_name);

                            format!(r#"{}
Price: ${}
Power Usage: {}W
Storage Type: {}
Size: {}
Speed: {}MB/s"#, storage.name.clone(),
                                storage.price,
                                storage.power_usage,
                                storage.storage_type.to_string(),
                                storage.size,
                                storage.speed
                            )
                        }
                        ItemType::Fan => {
                            let fan = get_fan(&item_name);

                            format!(r#"{}
Price: ${}
Power Usage: {}W
Size: {}mm
Flow: {}CM/s"#, fan.name.clone(),
                                fan.price,
                                fan.power_usage,
                                if fan.large {
                                    "140"
                                } else {
                                    "120"
                                },
                                fan.flow
                            )
                        }
                        ItemType::PSU => {
                            let psu = get_psu(&item_name);
                            format!(r#"{}
Price: ${}
Wattage: {}W
Length: {}mm"#, psu.name.clone(),
                                psu.price,
                                psu.wattage,
                                psu.length
                            )
                        }
                    };

                    let mut info = MultiTextLineBuilder {
                        id: "info".to_string(),
                        custom_data: Vec::new(),
                        text: info,
                        layout: TextLayout::Left,
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display);

                    let mut back_rect = RectBuilder {
                        position: vec2(0.0, 0.0),
                        size: vec2(info.width() + 2.0, info.height() + 2.0),
                        color: vec4(1.0, 1.0, 1.0, 1.0),
                        texture: None
                    }.build(display);

                    let mut black_rect = RectBuilder {
                        position: vec2(0.0, 0.0),
                        size: vec2(info.width(), info.height()),
                        color: vec4(0.0, 0.0, 0.0, 1.0),
                        texture: None
                    }.build(display);

                    let cursor_pos = get_ui().get_cursor_pos();
                    back_rect.set_left(cursor_pos.x);
                    back_rect.set_top(cursor_pos.y);

                    black_rect.set_centre(back_rect.centre());
                    info.set_centre(black_rect.centre());

                    self.popup = Some((back_rect, black_rect, info));
                    break;
                } else {
                    self.popup = None;
                }
            }

            items_checked.push(item);
        }

        false
    }

    pub fn draw(&self, target: &mut Frame) {
        self.background.draw(target);

        get_ui().get_element("cases_listbox").unwrap().draw(target);
        get_ui().get_element("mbs_listbox").unwrap().draw(target);
        get_ui().get_element("cpus_listbox").unwrap().draw(target);
        get_ui().get_element("cpu_coolers_listbox").unwrap().draw(target);
        get_ui().get_element("rams_listbox").unwrap().draw(target);
        get_ui().get_element("gpus_listbox").unwrap().draw(target);
        get_ui().get_element("storages_listbox").unwrap().draw(target);
        get_ui().get_element("fans_listbox").unwrap().draw(target);
        get_ui().get_element("psus_listbox").unwrap().draw(target);

        if let Some(popup) = &self.popup {
            popup.0.draw(target);
            popup.1.draw(target);
            popup.2.draw(target);
        }
    }
}

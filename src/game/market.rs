use std::io::Cursor;
use crypt::generic_array::typenum::private::PrivateCmp;
use glium::{Display, Frame};
use crate::{gfx::rect::{Rect, RectBuilder}, math::{vec2::vec2, vec4::vec4, vec3::vec3}, get_window_width, get_window_height, log::{log, save_log}, get_ui_mut, ui::{button::{ButtonBuilder, ButtonFace, ButtonTextType}, uielement::{UiOutput, UiElement}, listbox::ListboxBuilder, textline::{TextLineBuilder, TextLine}, multitextline::{MultiTextLine, MultiTextLineBuilder, TextLayout}}, get_ui, part_loader::*, ptrcell::PtrCell};

use super::profile::{Profile, ItemType};

pub struct Market {
    background: Rect,
    /// 0 = back rect for white outline
    /// 1 = black rect
    /// 2 = multitextline for part info. ID contains name and part type
    /// 3 = buy button ptrcell
    /// 4 = close button ptrcell
    buy_popup: Option<(Rect, Rect, MultiTextLine, PtrCell<dyn UiElement>, PtrCell<dyn UiElement>)>
}

impl Market {
    pub fn new(display: &Display, profile: &mut Profile) -> Market {
        let background = RectBuilder {
            position: vec2(get_window_width() as f32 / 2.0, get_window_height() as f32 / 2.0),
            size: vec2(get_window_width() as f32, get_window_height() as f32),
            color: vec4(1.0, 1.0, 1.0, 1.0),
            texture: Some(image::load(
                Cursor::new(std::fs::read("textures/market-background.png").unwrap_or_else(|e| {
                    log(format!("NH Err: {}", e.to_string()));
                    save_log();
                    panic!();
                })),
                image::ImageFormat::Png
            ).unwrap_or_else(|e| {
                log(format!("NH Err: {}", e.to_string()));
                save_log();
                panic!();
            }))
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

        let cases_listbox = get_ui_mut().add_element(ListboxBuilder {
            id: "cases_listbox".to_string(),
            custom_data: Vec::new(),
            elements: get_case_names().iter().map(|case| {
                get_ui_mut().add_element(TextLineBuilder {
                    id: case.clone(),
                    custom_data: Vec::new(),
                    text: case.clone(),
                    font_size: 36.0,
                    color: vec3(1.0, 1.0, 1.0),
                    bold: false,
                    position: vec2(0.0, 0.0)
                }.build(display))
            }).collect(),
            bar_width: 10.0,
            position: vec2(180.0, 748.0),
            size: vec2(237.0, 300.0)
        }.build(display));

        let mbs_listbox = get_ui_mut().add_element(ListboxBuilder {
            id: "mbs_listbox".to_string(),
            custom_data: Vec::new(),
            elements: get_mb_names().iter().map(|mb| {
                get_ui_mut().add_element(TextLineBuilder {
                    id: mb.clone(),
                    custom_data: Vec::new(),
                    text: mb.clone(),
                    font_size: 36.0,
                    color: vec3(1.0, 1.0, 1.0),
                    bold: false,
                    position: vec2(0.0, 0.0)
                }.build(display))
            }).collect(),
            bar_width: 10.0,
            position: vec2(554.0, 748.0),
            size: vec2(237.0, 300.0)
        }.build(display));

        let cpus_listbox = get_ui_mut().add_element(ListboxBuilder {
            id: "cpus_listbox".to_string(),
            custom_data: Vec::new(),
            elements: get_cpu_names().iter().map(|cpu| {
                get_ui_mut().add_element(TextLineBuilder {
                    id: cpu.clone(),
                    custom_data: Vec::new(),
                    text: cpu.clone(),
                    font_size: 36.0,
                    color: vec3(1.0, 1.0, 1.0),
                    bold: false,
                    position: vec2(0.0, 0.0)
                }.build(display))
            }).collect(),
            bar_width: 10.0,
            position: vec2(919.0, 748.0),
            size: vec2(237.0, 300.0)
        }.build(display));

        let cpu_coolers_listbox = get_ui_mut().add_element(ListboxBuilder {
            id: "cpu_coolers_listbox".to_string(),
            custom_data: Vec::new(),
            elements: get_cpu_cooler_names().iter().map(|cpu_cooler| {
                get_ui_mut().add_element(TextLineBuilder {
                    id: cpu_cooler.clone(),
                    custom_data: Vec::new(),
                    text: cpu_cooler.clone(),
                    font_size: 36.0,
                    color: vec3(1.0, 1.0, 1.0),
                    bold: false,
                    position: vec2(0.0, 0.0)
                }.build(display))
            }).collect(),
            bar_width: 10.0,
            position: vec2(1314.0, 748.0),
            size: vec2(237.0, 300.0)
        }.build(display));

        let rams_listbox = get_ui_mut().add_element(ListboxBuilder {
            id: "rams_listbox".to_string(),
            custom_data: Vec::new(),
            elements: get_ram_names().iter().map(|ram| {
                get_ui_mut().add_element(TextLineBuilder {
                    id: ram.clone(),
                    custom_data: Vec::new(),
                    text: ram.clone(),
                    font_size: 36.0,
                    color: vec3(1.0, 1.0, 1.0),
                    bold: false,
                    position: vec2(0.0, 0.0)
                }.build(display))
            }).collect(),
            bar_width: 10.0,
            position: vec2(1681.0, 748.0),
            size: vec2(237.0, 300.0)
        }.build(display));

        let gpus_listbox = get_ui_mut().add_element(ListboxBuilder {
            id: "gpus_listbox".to_string(),
            custom_data: Vec::new(),
            elements: get_gpu_names().iter().map(|gpu| {
                get_ui_mut().add_element(TextLineBuilder {
                    id: gpu.clone(),
                    custom_data: Vec::new(),
                    text: gpu.clone(),
                    font_size: 36.0,
                    color: vec3(1.0, 1.0, 1.0),
                    bold: false,
                    position: vec2(0.0, 0.0)
                }.build(display))
            }).collect(),
            bar_width: 10.0,
            position: vec2(180.0, 248.0),
            size: vec2(237.0, 300.0)
        }.build(display));

        let storages_listbox = get_ui_mut().add_element(ListboxBuilder {
            id: "storages_listbox".to_string(),
            custom_data: Vec::new(),
            elements: get_storage_names().iter().map(|storage| {
                get_ui_mut().add_element(TextLineBuilder {
                    id: storage.clone(),
                    custom_data: Vec::new(),
                    text: storage.clone(),
                    font_size: 36.0,
                    color: vec3(1.0, 1.0, 1.0),
                    bold: false,
                    position: vec2(0.0, 0.0)
                }.build(display))
            }).collect(),
            bar_width: 10.0,
            position: vec2(697.0, 248.0),
            size: vec2(237.0, 300.0)
        }.build(display));

        let fans_listbox = get_ui_mut().add_element(ListboxBuilder {
            id: "fans_listbox".to_string(),
            custom_data: Vec::new(),
            elements: get_fan_names().iter().map(|fan| {
                get_ui_mut().add_element(TextLineBuilder {
                    id: fan.clone(),
                    custom_data: Vec::new(),
                    text: fan.clone(),
                    font_size: 36.0,
                    color: vec3(1.0, 1.0, 1.0),
                    bold: false,
                    position: vec2(0.0, 0.0)
                }.build(display))
            }).collect(),
            bar_width: 10.0,
            position: vec2(1213.0, 248.0),
            size: vec2(237.0, 300.0)
        }.build(display));

        let psus_listbox = get_ui_mut().add_element(ListboxBuilder {
            id: "psus_listbox".to_string(),
            custom_data: Vec::new(),
            elements: get_psu_names().iter().map(|psu| {
                get_ui_mut().add_element(TextLineBuilder {
                    id: psu.clone(),
                    custom_data: Vec::new(),
                    text: psu.clone(),
                    font_size: 36.0,
                    color: vec3(1.0, 1.0, 1.0),
                    bold: false,
                    position: vec2(0.0, 0.0)
                }.build(display))
            }).collect(),
            bar_width: 10.0,
            position: vec2(1700.0, 248.0),
            size: vec2(237.0, 300.0)
        }.build(display));

        let mut money_textline = get_ui_mut().add_element(TextLineBuilder {
            id: "money_textline".to_string(),
            custom_data: Vec::new(),
            text: format!("${}", profile.money),
            font_size: 68.0,
            color: vec3(0.0, 0.0, 0.0),
            bold: false,
            position: vec2(502.0, 1035.0)
        }.build(display));

        Market {
            background,
            buy_popup: None
        }
    }

    pub fn run(&mut self, display: &Display, profile: &mut Profile) -> bool {
        // function so that i dont have to repeat shit
        fn create_popup_shit(multitextline: &mut MultiTextLine, display: &Display, listbox: PtrCell<dyn UiElement>, flip_side: bool) -> (Rect, Rect, PtrCell<dyn UiElement>, PtrCell<dyn UiElement>) {
            let mut back_rect = RectBuilder {
                position: vec2(0.0, 0.0),
                size: vec2(multitextline.width() + 10.0, multitextline.height() + 40.0),
                color: vec4(1.0, 1.0, 1.0, 1.0),
                texture: None
            }.build(display);
            if flip_side {
                back_rect.set_right(listbox.left());
            } else {
                back_rect.set_left(listbox.right());
            }
            back_rect.set_top(listbox.top());

            let mut black_rect = RectBuilder {
                position: vec2(0.0, 0.0),
                size: vec2(back_rect.width() - 2.0, back_rect.height() - 2.0),
                color: vec4(0.0, 0.0, 0.0, 1.0),
                texture: None
            }.build(display);
            black_rect.set_centre(back_rect.centre());

            multitextline.set_centre(back_rect.centre() + vec2(0.0, 20.0));

            let mut buy_button = get_ui_mut().add_element(ButtonBuilder {
                id: "buy_button".to_string(),
                custom_data: Vec::new(),
                position: vec2(0.0, 0.0),
                size: vec2(black_rect.width() / 2.2, 30.0),
                text: Some(ButtonTextType::Single(TextLineBuilder {
                    id: "buy_button_text".to_string(),
                    custom_data: Vec::new(),
                    text: "Buy".to_string(),
                    font_size: 26.0,
                    color: vec3(1.0, 1.0, 1.0),
                    bold: false,
                    position: vec2(0.0, 0.0)
                }.build(display))),
                normal_face: ButtonFace::Color(vec4(0.0, 0.6, 0.0, 1.0)),
                hovered_face: None,
                clicked_face: None
            }.build(display));
            buy_button.set_left(black_rect.left() + 2.0);
            buy_button.set_bottom(black_rect.bottom() + 2.0);

            let mut close_button = get_ui_mut().add_element(ButtonBuilder {
                id: "pop_up_close_button".to_string(),
                custom_data: Vec::new(),
                position: vec2(0.0, 0.0),
                size: vec2(black_rect.width() / 2.2, 30.0),
                text: Some(ButtonTextType::Single(TextLineBuilder {
                    id: "close_button_text".to_string(),
                    custom_data: Vec::new(),
                    text: "Close".to_string(),
                    font_size: 26.0,
                    color: vec3(1.0, 1.0, 1.0),
                    bold: false,
                    position: vec2(0.0, 0.0)
                }.build(display))),
                normal_face: ButtonFace::Color(vec4(0.6, 0.0, 0.0, 1.0)),
                hovered_face: None,
                clicked_face: None
            }.build(display));
            close_button.set_right(black_rect.right() - 2.0);
            close_button.set_bottom(black_rect.bottom() + 2.0);

            (back_rect, black_rect, buy_button, close_button)
        }

        // go back to ingame menu if the top right button is pressed
        if get_ui().get_element("close_button").unwrap().output() == UiOutput::LeftClicked {
            return true;
        }

        get_ui_mut().remove_element("money_textline");
        get_ui_mut().add_element(TextLineBuilder {
            id: "money_textline".to_string(),
            custom_data: Vec::new(),
            text: format!("${}", profile.money),
            font_size: 68.0,
            color: vec3(0.0, 0.0, 0.0),
            bold: false,
            position: vec2(502.0, 1035.0)
        }.build(display));

        // good luck reading the next few hundred lines

        // cases
        for case_name in get_case_names() {
            if let Some(element) = get_ui_mut().get_element(&case_name) {
                if element.output() == UiOutput::LeftClicked {
                    let case = get_case(&case_name);

                    let mut info_multitextline = MultiTextLineBuilder {
                        id: case.name.clone() + "/case",
                        custom_data: Vec::new(),
                        text: format!(r#"{}
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
                        ),
                        layout: TextLayout::Left,
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display);

                    let popup = create_popup_shit(&mut info_multitextline, display, get_ui().get_element("cases_listbox").unwrap(), false);

                    self.buy_popup = Some((popup.0, popup.1, info_multitextline, popup.2, popup.3));
                    break;
                }
            }
        }

        // mbs
        for mb_name in get_mb_names() {
            if let Some(element) = get_ui_mut().get_element(&mb_name) {
                if element.output() == UiOutput::LeftClicked {
                    let mb = get_mb(&mb_name);

                    let mut info_multitextline = MultiTextLineBuilder {
                        id: mb.name.clone() + "/mb",
                        custom_data: Vec::new(),
                        text: format!(r#"{}
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
                        ),
                        layout: TextLayout::Left,
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display);

                    let popup = create_popup_shit(&mut info_multitextline, display, get_ui().get_element("mbs_listbox").unwrap(), false);

                    self.buy_popup = Some((popup.0, popup.1, info_multitextline, popup.2, popup.3));
                    break;
                }
            }
        }

        // cpus
        for cpu_name in get_cpu_names() {
            if let Some(element) = get_ui_mut().get_element(&cpu_name) {
                if element.output() == UiOutput::LeftClicked {
                    let cpu = get_cpu(&cpu_name);

                    let mut info_multitextline = MultiTextLineBuilder {
                        id: cpu.name.clone() + "/cpu",
                        custom_data: Vec::new(),
                        text: format!(r#"{}
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
                        ),
                        layout: TextLayout::Left,
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display);

                    let popup = create_popup_shit(&mut info_multitextline, display, get_ui().get_element("cpus_listbox").unwrap(), false);

                    self.buy_popup = Some((popup.0, popup.1, info_multitextline, popup.2, popup.3));
                    break;
                }
            }
        }

        for cpu_cooler_name in get_cpu_cooler_names() {
            if let Some(element) = get_ui_mut().get_element(&cpu_cooler_name) {
                if element.output() == UiOutput::LeftClicked {
                    let cpu_cooler = get_cpu_cooler(&cpu_cooler_name);

                    let mut info_multitextline = MultiTextLineBuilder {
                        id: cpu_cooler.name.clone() + "/cpu_cooler",
                        custom_data: Vec::new(),
                        text: format!(r#"{}
Price: ${}
Power Usage: {}W
Socket Type: {}
Height: {}mm"#, cpu_cooler.name.clone(),
                        cpu_cooler.price,
                        cpu_cooler.power_usage,
                        cpu_cooler.socket_type.to_string(),
                        cpu_cooler.height
                        ),
                        layout: TextLayout::Left,
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display);

                    let popup = create_popup_shit(&mut info_multitextline, display, get_ui().get_element("cpu_coolers_listbox").unwrap(), false);

                    self.buy_popup = Some((popup.0, popup.1, info_multitextline, popup.2, popup.3));
                    break;
                }
            }
        }

        for ram_name in get_ram_names() {
            if let Some(element) = get_ui_mut().get_element(&ram_name) {
                if element.output() == UiOutput::LeftClicked {
                    let ram = get_ram(&ram_name);

                    let mut info_multitextline = MultiTextLineBuilder {
                        id: ram.name.clone() + "/ram",
                        custom_data: Vec::new(),
                        text: format!(r#"{}
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
                        ),
                        layout: TextLayout::Left,
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display);

                    let popup = create_popup_shit(&mut info_multitextline, display, get_ui().get_element("rams_listbox").unwrap(), true);

                    self.buy_popup = Some((popup.0, popup.1, info_multitextline, popup.2, popup.3));
                    break;
                }
            }
        }

        for gpu_name in get_gpu_names() {
            if let Some(element) = get_ui_mut().get_element(&gpu_name) {
                if element.output() == UiOutput::LeftClicked {
                    let gpu = get_gpu(&gpu_name);

                    let mut info_multitextline = MultiTextLineBuilder {
                        id: gpu.name.clone() + "/gpu",
                        custom_data: Vec::new(),
                        text: format!(r#"{}
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
                        ),
                        layout: TextLayout::Left,
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display);

                    let popup = create_popup_shit(&mut info_multitextline, display, get_ui().get_element("gpus_listbox").unwrap(), false);

                    self.buy_popup = Some((popup.0, popup.1, info_multitextline, popup.2, popup.3));
                    break;
                }
            }
        }

        for storage_name in get_storage_names() {
            if let Some(element) = get_ui_mut().get_element(&storage_name) {
                if element.output() == UiOutput::LeftClicked {
                    let storage = get_storage(&storage_name);

                    let mut info_multitextline = MultiTextLineBuilder {
                        id: storage.name.clone() + "/storage",
                        custom_data: Vec::new(),
                        text: format!(r#"{}
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
                        ),
                        layout: TextLayout::Left,
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display);

                    let popup = create_popup_shit(&mut info_multitextline, display, get_ui().get_element("storages_listbox").unwrap(), false);

                    self.buy_popup = Some((popup.0, popup.1, info_multitextline, popup.2, popup.3));
                    break;
                }
            }
        }

        for fan_name in get_fan_names() {
            if let Some(element) = get_ui_mut().get_element(&fan_name) {
                if element.output() == UiOutput::LeftClicked {
                    let fan = get_fan(&fan_name);

                    let mut info_multitextline = MultiTextLineBuilder {
                        id: fan.name.clone() + "/fan",
                        custom_data: Vec::new(),
                        text: format!(r#"{}
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
                        ),
                        layout: TextLayout::Left,
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display);

                    let popup = create_popup_shit(&mut info_multitextline, display, get_ui().get_element("fans_listbox").unwrap(), false);

                    self.buy_popup = Some((popup.0, popup.1, info_multitextline, popup.2, popup.3));
                    break;
                }
            }
        }

        for psu_name in get_psu_names() {
            if let Some(element) = get_ui_mut().get_element(&psu_name) {
                if element.output() == UiOutput::LeftClicked {
                    let psu = get_psu(&psu_name);

                    let mut info_multitextline = MultiTextLineBuilder {
                        id: psu.name.clone() + "/psu",
                        custom_data: Vec::new(),
                        text: format!(r#"{}
Price: ${}
Wattage: {}W
Length: {}mm"#, psu.name.clone(),
                        psu.price,
                        psu.wattage,
                        psu.length
                        ),
                        layout: TextLayout::Left,
                        font_size: 26.0,
                        color: vec3(1.0, 1.0, 1.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(display);

                    let popup = create_popup_shit(&mut info_multitextline, display, get_ui().get_element("psus_listbox").unwrap(), true);

                    self.buy_popup = Some((popup.0, popup.1, info_multitextline, popup.2, popup.3));
                    break;
                }
            }
        }

        let mut bought = false;
        if let Some(buy_popup) = &self.buy_popup {
            if buy_popup.3.output() == UiOutput::LeftClicked {
                let (part_name, part_type) = buy_popup.2.id().split_once('/').unwrap();
                
                match part_type {
                    "case" => {
                        let case = get_case(part_name);
                        if profile.money >= case.price as i32 {
                            profile.items.push((part_name.to_string(), ItemType::Case));
                            profile.money -= case.price as i32;
                            bought = true;
                        }
                    }
                    "mb" => {
                        let mb = get_mb(part_name);
                        if profile.money >= mb.price as i32 {
                            profile.items.push((part_name.to_string(), ItemType::MB));
                            profile.money -= mb.price as i32;
                            bought = true;
                        }
                    }
                    "cpu" => {
                        let cpu = get_cpu(part_name);
                        if profile.money >= cpu.price as i32 {
                            profile.items.push((part_name.to_string(), ItemType::CPU));
                            profile.money -= cpu.price as i32;
                            bought = true;
                        }
                    }
                    "cpu_cooler" => {
                        let cpu_cooler = get_cpu_cooler(part_name);
                        if profile.money >= cpu_cooler.price as i32 {
                            profile.items.push((part_name.to_string(), ItemType::CPUCooler));
                            profile.money -= cpu_cooler.price as i32;
                            bought = true;
                        }
                    }
                    "ram" => {
                        let ram = get_ram(part_name);
                        if profile.money >= ram.price as i32 {
                            profile.items.push((part_name.to_string(), ItemType::RAM));
                            profile.money -= ram.price as i32;
                            bought = true;
                        }
                    }
                    "gpu" => {
                        let gpu = get_gpu(part_name);
                        if profile.money >= gpu.price as i32 {
                            profile.items.push((part_name.to_string(), ItemType::GPU));
                            profile.money -= gpu.price as i32;
                            bought = true;
                        }
                    }
                    "storage" => {
                        let storage = get_storage(part_name);
                        if profile.money >= storage.price as i32 {
                            profile.items.push((part_name.to_string(), ItemType::Storage));
                            profile.money -= storage.price as i32;
                            bought = true;
                        }
                    }
                    "fan" => {
                        let fan = get_fan(part_name);
                        if profile.money >= fan.price as i32 {
                            profile.items.push((part_name.to_string(), ItemType::Fan));
                            profile.money -= fan.price as i32;
                            bought = true;
                        }
                    }
                    "psu" => {
                        let psu = get_psu(part_name);
                        if profile.money >= psu.price as i32 {
                            profile.items.push((part_name.to_string(), ItemType::PSU));
                            profile.money -= psu.price as i32;
                            bought = true;
                        }
                    }
                    _ => {}
                }
            }

            if buy_popup.4.output() == UiOutput::LeftClicked {
                self.buy_popup = None;
            }
        }

        if bought {
            self.buy_popup = None;
            get_ui_mut().remove_element("pop_up_close_button");
            get_ui_mut().remove_element("buy_button");
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
        get_ui().get_element("money_textline").unwrap().draw(target);

        if let Some(buy_popup) = &self.buy_popup {
            buy_popup.0.draw(target);
            buy_popup.1.draw(target);
            buy_popup.2.draw(target);
            buy_popup.3.draw(target);
            buy_popup.4.draw(target);
        }
    }
}

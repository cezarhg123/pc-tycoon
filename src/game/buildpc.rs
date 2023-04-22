use std::io::Cursor;
use glium::{Display, Frame, glutin::event::VirtualKeyCode};
use crate::{gfx::rect::{Rect, RectBuilder}, math::{vec2::{vec2, Vec2}, vec4::vec4, vec3::vec3}, get_window_width, get_window_height, log::{log, save_log}, get_ui, ui::{uielement::{UiOutput, UiElement}, button::{ButtonBuilder, ButtonFace, ButtonTextType, Button}, textline::TextLineBuilder, uirect::UiRect}, get_ui_mut, ptrcell::PtrCell};
use super::profile::{Profile, ItemType};

enum State {
    Buttons, // first menu, shows all component types buttons
    Items // second menu, shows all items in inventory of that certain component type
}

pub struct BuildPC {
    background: Rect,
    state: State,
    hitboxes: (
        Option<PtrCell<dyn UiElement>>,
        Option<PtrCell<dyn UiElement>>,
        Option<PtrCell<dyn UiElement>>,
        Option<PtrCell<dyn UiElement>>,
        [Option<PtrCell<dyn UiElement>>; 4],
        Option<PtrCell<dyn UiElement>>,
        [Option<PtrCell<dyn UiElement>>; 6],
        [Option<PtrCell<dyn UiElement>>; 5],
        Option<PtrCell<dyn UiElement>>
    ),
    pc_rect: (Option<Rect>, Option<Rect>, Option<Rect>, Option<Rect>, Vec<Rect>, Option<Rect>, Vec<Rect>, Vec<Rect>, Option<Rect>),
}

impl BuildPC {
    pub fn new(display: &Display) -> BuildPC {
        let background = RectBuilder {
            position: vec2(get_window_width() as f32 / 2.0, get_window_height() as f32 / 2.0),
            size: vec2(get_window_width() as f32, get_window_height() as f32),
            color: vec4(1.0, 1.0, 1.0, 1.0),
            texture: Some(image::load(
                Cursor::new(std::fs::read("textures/pc-builder.png").unwrap_or_else(|e| {
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

        let pc_cost_textline = get_ui_mut().add_element(TextLineBuilder {
            id: "pc_cost_textline".to_string(),
            custom_data: Vec::new(),
            text: "Value: $0".to_string(),
            font_size: 54.0,
            color: vec3(0.0, 0.0, 0.0),
            bold: true,
            position: vec2(1631.0, 1040.0)
        }.build(display));

        create_buttons(display);

        BuildPC {
            background,
            state: State::Buttons,
            hitboxes: (
                Some(
                    get_ui_mut().add_element(UiRect::new("insert_case_hitbox", RectBuilder {
                        position: vec2(get_window_width() as f32 / 2.0, get_window_height() as f32 / 2.0),
                        size: vec2(100.0, 100.0),
                        color: vec4(0.0, 0.0, 0.0, 0.0),
                        texture: None
                    }.build(display), Vec::new()))
                ),
                None,
                None,
                None,
                [None, None, None, None],
                None,
                [None, None, None, None, None, None],
                [None, None, None, None, None],
                None
            ),
            pc_rect: (None, None, None, None, Vec::new(), None, Vec::new(), Vec::new(), None),
        }
    }

    pub fn run(&mut self, profile: &mut Profile, display: &Display) -> bool {
        fn remove_buttons(
            cases_button: PtrCell<dyn UiElement>,
            mbs_button: PtrCell<dyn UiElement>,
            cpus_button: PtrCell<dyn UiElement>,
            cpu_coolers_button: PtrCell<dyn UiElement>,
            rams_button: PtrCell<dyn UiElement>,
            gpus_button: PtrCell<dyn UiElement>,
            storages_button: PtrCell<dyn UiElement>,
            fans_button: PtrCell<dyn UiElement>,
            psus_button: PtrCell<dyn UiElement>
        ) {
            get_ui_mut().remove_element(cases_button.id());
            get_ui_mut().remove_element(mbs_button.id());
            get_ui_mut().remove_element(cpus_button.id());
            get_ui_mut().remove_element(cpu_coolers_button.id());
            get_ui_mut().remove_element(rams_button.id());
            get_ui_mut().remove_element(gpus_button.id());
            get_ui_mut().remove_element(storages_button.id());
            get_ui_mut().remove_element(fans_button.id());
            get_ui_mut().remove_element(psus_button.id());
        }

        if get_ui().get_element("close_button").unwrap().output() == UiOutput::LeftClicked {
            return true;
        }
        
        match &self.state {
            State::Buttons => {
                let cases_button = get_ui().get_element("cases_button").unwrap();
                let mbs_button = get_ui().get_element("mbs_button").unwrap();
                let cpus_button = get_ui().get_element("cpus_button").unwrap();
                let cpu_coolers_button = get_ui().get_element("cpu_coolers_button").unwrap();
                let rams_button = get_ui().get_element("rams_button").unwrap();
                let gpus_button = get_ui().get_element("gpus_button").unwrap();
                let storages_button = get_ui().get_element("storages_button").unwrap();
                let fans_button = get_ui().get_element("fans_button").unwrap();
                let psus_button = get_ui().get_element("psus_button").unwrap();

                if cases_button.output() == UiOutput::LeftClicked {
                    remove_buttons(cases_button, mbs_button, cpus_button, cpu_coolers_button, rams_button, gpus_button, storages_button, fans_button, psus_button);
                    create_items(&profile, display, "textures/case.png");
                    self.state = State::Items;
                } else if mbs_button.output() == UiOutput::LeftClicked {
                    remove_buttons(cases_button, mbs_button, cpus_button, cpu_coolers_button, rams_button, gpus_button, storages_button, fans_button, psus_button);
                    create_items(&profile, display, "textures/mb.png");
                    self.state = State::Items;
                } else if cpus_button.output() == UiOutput::LeftClicked {
                    remove_buttons(cases_button, mbs_button, cpus_button, cpu_coolers_button, rams_button, gpus_button, storages_button, fans_button, psus_button);
                    create_items(&profile, display, "textures/cpu.png");
                    self.state = State::Items;
                } else if cpu_coolers_button.output() == UiOutput::LeftClicked {
                    remove_buttons(cases_button, mbs_button, cpus_button, cpu_coolers_button, rams_button, gpus_button, storages_button, fans_button, psus_button);
                    create_items(&profile, display, "textures/cpu_cooler.png");
                    self.state = State::Items;
                } else if rams_button.output() == UiOutput::LeftClicked {
                    remove_buttons(cases_button, mbs_button, cpus_button, cpu_coolers_button, rams_button, gpus_button, storages_button, fans_button, psus_button);
                    create_items(&profile, display, "textures/ram-icon.png");
                    self.state = State::Items;
                } else if gpus_button.output() == UiOutput::LeftClicked {
                    remove_buttons(cases_button, mbs_button, cpus_button, cpu_coolers_button, rams_button, gpus_button, storages_button, fans_button, psus_button);
                    create_items(&profile, display, "textures/gpu.png");
                    self.state = State::Items;
                } else if storages_button.output() == UiOutput::LeftClicked {
                    remove_buttons(cases_button, mbs_button, cpus_button, cpu_coolers_button, rams_button, gpus_button, storages_button, fans_button, psus_button);
                    create_items(&profile, display, "textures/storage-icon.png");
                    self.state = State::Items;
                } else if fans_button.output() == UiOutput::LeftClicked {
                    remove_buttons(cases_button, mbs_button, cpus_button, cpu_coolers_button, rams_button, gpus_button, storages_button, fans_button, psus_button);
                    create_items(&profile, display, "textures/fan-icon.png");
                    self.state = State::Items;
                } else if psus_button.output() == UiOutput::LeftClicked {
                    remove_buttons(cases_button, mbs_button, cpus_button, cpu_coolers_button, rams_button, gpus_button, storages_button, fans_button, psus_button);
                    create_items(&profile, display, "textures/psu.png");
                    self.state = State::Items;
                }
            }
            State::Items => {
                if get_ui().get_key_pressed(VirtualKeyCode::Escape) {
                    let mut i = 1;

                    loop {
                        match get_ui().get_element(format!("item{i}").as_str()) {
                            Some(_) => {
                                get_ui_mut().remove_element(format!("item{i}").as_str());
                            }
                            None => {break}
                        }

                        i += 1;
                    }

                    create_buttons(display);
                    self.state = State::Buttons;
                }
            }
        }
        
        false
    }

    pub fn draw(&self, target: &mut Frame) {
        self.background.draw(target);

        get_ui().get_element("pc_cost_textline").unwrap().draw(target);
        if let Some(button) = get_ui().get_element("cases_button") {
            button.draw(target);
        }
        if let Some(button) = get_ui().get_element("mbs_button") {
            button.draw(target);
        }
        if let Some(button) = get_ui().get_element("cpus_button") {
            button.draw(target);
        }
        if let Some(button) = get_ui().get_element("cpu_coolers_button") {
            button.draw(target);
        }
        if let Some(button) = get_ui().get_element("rams_button") {
            button.draw(target);
        }
        if let Some(button) = get_ui().get_element("gpus_button") {
            button.draw(target);
        }
        if let Some(button) = get_ui().get_element("storages_button") {
            button.draw(target);
        }
        if let Some(button) = get_ui().get_element("fans_button") {
            button.draw(target);
        }
        if let Some(button) = get_ui().get_element("psus_button") {
            button.draw(target);
        }

        let mut i = 1;
        loop {
            if let Some(item) = get_ui().get_element(format!("item{i}").as_str()) {
                item.draw(target);
            } else {
                break;
            }
            i += 1;
        }

        if let Some(pc_case) = &self.pc_rect.0 {
            pc_case.draw(target);
        }

        if let Some(pc_mb) = &self.pc_rect.1 {
            pc_mb.draw(target);
        }

        if let Some(pc_cpu) = &self.pc_rect.2 {
            pc_cpu.draw(target);
        }

        if let Some(pc_cpu_cooler) = &self.pc_rect.3 {
            pc_cpu_cooler.draw(target);
        }

        for pc_ram in self.pc_rect.4.iter() {
            pc_ram.draw(target);
        }

        if let Some(pc_gpu) = &self.pc_rect.5 {
            pc_gpu.draw(target);
        }

        for pc_storage in self.pc_rect.6.iter() {
            pc_storage.draw(target);
        }

        for pc_fan in self.pc_rect.7.iter() {
            pc_fan.draw(target);
        }

        if let Some(pc_psu) = &self.pc_rect.8 {
            pc_psu.draw(target);
        }
    }
}

fn create_buttons(display: &Display) -> [PtrCell<dyn UiElement>; 9] {
    let cases_button = get_ui_mut().add_element(ButtonBuilder {
        id: "cases_button".to_string(),
        custom_data: Vec::new(),
        position: vec2(204.0, 928.0),
        size: vec2(400.0, 100.0),
        text: Some(ButtonTextType::Single(TextLineBuilder {
            id: "Cases".to_string(),
            custom_data: Vec::new(),
            text: "Cases".to_string(),
            font_size: 48.0,
            color: vec3(1.0, 1.0, 1.0),
            bold: false,
            position: vec2(0.0, 0.0)
        }.build(display))),
        normal_face: ButtonFace::Color(vec4(0.78, 0.24, 0.0, 1.0)),
        hovered_face: None,
        clicked_face: None
    }.build(display));

    let mbs_button = get_ui_mut().add_element(ButtonBuilder {
        id: "mbs_button".to_string(),
        custom_data: Vec::new(),
        position: vec2(204.0, 821.0),
        size: vec2(400.0, 100.0),
        text: Some(ButtonTextType::Single(TextLineBuilder {
            id: "Motherboards".to_string(),
            custom_data: Vec::new(),
            text: "Motherboards".to_string(),
            font_size: 48.0,
            color: vec3(1.0, 1.0, 1.0),
            bold: false,
            position: vec2(0.0, 0.0)
        }.build(display))),
        normal_face: ButtonFace::Color(vec4(0.78, 0.24, 0.0, 1.0)),
        hovered_face: None,
        clicked_face: None
    }.build(display));

    let cpus_button = get_ui_mut().add_element(ButtonBuilder {
        id: "cpus_button".to_string(),
        custom_data: Vec::new(),
        position: vec2(204.0, 714.0),
        size: vec2(400.0, 100.0),
        text: Some(ButtonTextType::Single(TextLineBuilder {
            id: "CPUs".to_string(),
            custom_data: Vec::new(),
            text: "CPUs".to_string(),
            font_size: 48.0,
            color: vec3(1.0, 1.0, 1.0),
            bold: false,
            position: vec2(0.0, 0.0)
        }.build(display))),
        normal_face: ButtonFace::Color(vec4(0.78, 0.24, 0.0, 1.0)),
        hovered_face: None,
        clicked_face: None
    }.build(display));

    let cpu_coolers_button = get_ui_mut().add_element(ButtonBuilder {
        id: "cpu_coolers_button".to_string(),
        custom_data: Vec::new(),
        position: vec2(204.0, 607.0),
        size: vec2(400.0, 100.0),
        text: Some(ButtonTextType::Single(TextLineBuilder {
            id: "CPU Coolers".to_string(),
            custom_data: Vec::new(),
            text: "CPU Coolers".to_string(),
            font_size: 48.0,
            color: vec3(1.0, 1.0, 1.0),
            bold: false,
            position: vec2(0.0, 0.0)
        }.build(display))),
        normal_face: ButtonFace::Color(vec4(0.78, 0.24, 0.0, 1.0)),
        hovered_face: None,
        clicked_face: None
    }.build(display));

    let rams_button = get_ui_mut().add_element(ButtonBuilder {
        id: "rams_button".to_string(),
        custom_data: Vec::new(),
        position: vec2(204.0, 499.0),
        size: vec2(400.0, 100.0),
        text: Some(ButtonTextType::Single(TextLineBuilder {
            id: "RAMs".to_string(),
            custom_data: Vec::new(),
            text: "RAMs".to_string(),
            font_size: 48.0,
            color: vec3(1.0, 1.0, 1.0),
            bold: false,
            position: vec2(0.0, 0.0)
        }.build(display))),
        normal_face: ButtonFace::Color(vec4(0.78, 0.24, 0.0, 1.0)),
        hovered_face: None,
        clicked_face: None
    }.build(display));

    let gpus_button = get_ui_mut().add_element(ButtonBuilder {
        id: "gpus_button".to_string(),
        custom_data: Vec::new(),
        position: vec2(204.0, 391.0),
        size: vec2(400.0, 100.0),
        text: Some(ButtonTextType::Single(TextLineBuilder {
            id: "Graphics Cards".to_string(),
            custom_data: Vec::new(),
            text: "Graphics Cards".to_string(),
            font_size: 48.0,
            color: vec3(1.0, 1.0, 1.0),
            bold: false,
            position: vec2(0.0, 0.0)
        }.build(display))),
        normal_face: ButtonFace::Color(vec4(0.78, 0.24, 0.0, 1.0)),
        hovered_face: None,
        clicked_face: None
    }.build(display));

    let storages_button = get_ui_mut().add_element(ButtonBuilder {
        id: "storages_button".to_string(),
        custom_data: Vec::new(),
        position: vec2(204.0, 283.0),
        size: vec2(400.0, 100.0),
        text: Some(ButtonTextType::Single(TextLineBuilder {
            id: "Storages".to_string(),
            custom_data: Vec::new(),
            text: "Storages".to_string(),
            font_size: 48.0,
            color: vec3(1.0, 1.0, 1.0),
            bold: false,
            position: vec2(0.0, 0.0)
        }.build(display))),
        normal_face: ButtonFace::Color(vec4(0.78, 0.24, 0.0, 1.0)),
        hovered_face: None,
        clicked_face: None
    }.build(display));

    let fans_button = get_ui_mut().add_element(ButtonBuilder {
        id: "fans_button".to_string(),
        custom_data: Vec::new(),
        position: vec2(204.0, 174.0),
        size: vec2(400.0, 100.0),
        text: Some(ButtonTextType::Single(TextLineBuilder {
            id: "Fans".to_string(),
            custom_data: Vec::new(),
            text: "Fans".to_string(),
            font_size: 48.0,
            color: vec3(1.0, 1.0, 1.0),
            bold: false,
            position: vec2(0.0, 0.0)
        }.build(display))),
        normal_face: ButtonFace::Color(vec4(0.78, 0.24, 0.0, 1.0)),
        hovered_face: None,
        clicked_face: None
    }.build(display));

    let psus_button = get_ui_mut().add_element(ButtonBuilder {
        id: "psus_button".to_string(),
        custom_data: Vec::new(),
        position: vec2(204.0, 65.0),
        size: vec2(400.0, 100.0),
        text: Some(ButtonTextType::Single(TextLineBuilder {
            id: "PSUs".to_string(),
            custom_data: Vec::new(),
            text: "PSUs".to_string(),
            font_size: 48.0,
            color: vec3(1.0, 1.0, 1.0),
            bold: false,
            position: vec2(0.0, 0.0)
        }.build(display))),
        normal_face: ButtonFace::Color(vec4(0.78, 0.24, 0.0, 1.0)),
        hovered_face: None,
        clicked_face: None
    }.build(display));

    [
        cases_button,
        mbs_button,
        cpus_button,
        cpu_coolers_button,
        rams_button,
        gpus_button,
        storages_button,
        fans_button,
        psus_button
    ]
}

fn create_items(profile: &Profile, display: &Display, texture_path: &str) {
    for mut item in profile.items.iter().filter(|item| item.1 == ItemType::Case).map(|item| &item.0).enumerate() {
        item.0 += 1;
        
        let offset = vec2(
            if item.0 == 1 {
                0.0
            } else {
                if item.0 % 2 == 0 {
                    205.0
                } else {
                    0.0
                }
            },
            {
                let mut index = item.0 as i32;
                let mut rows = 0;
                
                loop {
                    if index - 3 < 0 {
                        break;
                    } else {
                        rows += 1;
                        index -= 3;
                    }
                }

                (205.0 * rows as f32) * -1.0
            }
        );
        
        get_ui_mut().add_element(ButtonBuilder {
            id: format!("item{}", item.0),
            custom_data: Vec::new(),
            position: vec2(102.0, 880.0) + offset,
            size: vec2(200.0, 200.0),
            text: Some(ButtonTextType::Single(TextLineBuilder {
                id: item.1.clone(),
                custom_data: Vec::new(),
                text: item.1.clone(),
                font_size: 36.0,
                color: vec3(1.0, 1.0, 1.0),
                bold: false,
                position: vec2(0.0, 0.0)
            }.build(display))),
            normal_face: ButtonFace::Texture(image::load(Cursor::new(std::fs::read(texture_path).unwrap()), image::ImageFormat::Png).unwrap()),
            hovered_face: None,
            clicked_face: None
        }.build(display));
    }
}

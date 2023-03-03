use std::{io::Cursor, fs::File};

use crypt::MagicCryptTrait;
use glium::{Display, Frame};

use crate::{gfx::rect::{Rect, RectBuilder}, math::{vec2::vec2, vec4::vec4, vec3::vec3}, get_window_width, get_window_height, log::{log, save_log}, ui::{button::{ButtonBuilder, ButtonTextType, ButtonFace}, textline::TextLineBuilder, uielement::UiOutput}, get_ui_mut, get_ui};

use super::profile::{get_key, Profile};

pub struct SelectSave {
    background: Rect
}

impl SelectSave {
    pub fn new(display: &Display) -> SelectSave {
        let background = RectBuilder {
            position: vec2(get_window_width() as f32 / 2.0, get_window_height() as f32 / 2.0),
            size: vec2(get_window_width() as f32, get_window_height() as f32),
            color: vec4(1.0, 1.0, 1.0, 1.0),
            texture: Some(image::load(
                Cursor::new(std::fs::read("textures/background.png").unwrap_or_else(|e| {
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

        // creating buttons now so that i can just get them later in 'run()'
        for i in 1..4 {
            let prefix = match File::open(format!("saves/save{i}.save")) {
                Ok(_) => {
                    "Continue"
                }
                Err(_) => {
                    "New"
                }
            }; // if file exists then use either 'New' or 'Continue'

            let save_button = get_ui_mut().add_element(ButtonBuilder {
                id: format!("save{i}_button"),
                position: vec2(378.0, 600.0 - (100.0 * i as f32)),
                size: vec2(300.0, 80.0),
                text: Some(ButtonTextType::Single(TextLineBuilder {
                    text: format!("{prefix} Save"),
                    font_size: 36.0,
                    color: vec3(1.0, 1.0, 1.0),
                    bold: false,
                    position: vec2(378.0, 600.0 - (100.0 * i as f32))
                }.build(display))),
                normal_face: ButtonFace::Color(vec4(0.78, 0.24, 0.0, 1.0)),
                hovered_face: None,
                clicked_face: None
            }.build(display));
        }

        SelectSave {
            background
        }
    }

    pub fn run(&mut self) -> Option<Profile> {
        for i in 1..4 {
            let save_button = get_ui_mut().get_element(format!("save{i}_button").as_str()).unwrap();

            if save_button.output() == UiOutput::LeftClicked {
                match std::fs::read(format!("saves/save{i}.save")) {
                    Ok(file_bytes) => {
                        let encrypted_profile_json = String::from_utf8(file_bytes).unwrap();
                        let profile_json = get_key().decrypt_base64_to_string(encrypted_profile_json).unwrap_or_else(|e| {
                            log(format!("NH Err: {}", e.to_string()));
                            save_log();
                            panic!();
                        });

                        return Some(Profile::load_profile_json(profile_json))
                    }
                    Err(_) => {
                        return Some(Profile {
                            money: 1500,
                            level: 1,
                            points: 0,
                            items: Vec::new()
                        })
                    }
                }
            }
        }

        None
    }

    pub fn draw(&self, target: &mut Frame) {
        self.background.draw(target);

        for i in 1..4 {
            get_ui().get_element(format!("save{i}_button").as_str()).unwrap().draw(target);
        }
    }
}

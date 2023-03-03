use std::io::Cursor;
use glium::{Display, Frame};
use crate::{gfx::rect::{RectBuilder, Rect}, math::{vec2::vec2, vec4::vec4, vec3::vec3}, get_window_width, get_window_height, log::{log, save_log}, get_ui_mut, ui::{button::{ButtonBuilder, ButtonTextType, ButtonFace}, textline::TextLineBuilder, uielement::UiOutput}, get_ui, close};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MainMenuOutput {
    None,
    Play
}

pub struct MainMenu {
    background: Rect
}

impl MainMenu {
    pub fn new(display: &Display) -> MainMenu {
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
        let play_button = get_ui_mut().add_element(ButtonBuilder {
            id: "play_button".to_string(),
            position: vec2(378.0, 600.0),
            size: vec2(300.0, 80.0),
            text: Some(ButtonTextType::Single(TextLineBuilder {
                text: "Play".to_string(),
                font_size: 36.0,
                color: vec3(1.0, 1.0, 1.0),
                bold: false,
                position: vec2(378.0, 600.0)
            }.build(display))),
            normal_face: ButtonFace::Color(vec4(0.78, 0.24, 0.0, 1.0)),
            hovered_face: None,
            clicked_face: None
        }.build(display));

        let exit_button = get_ui_mut().add_element(ButtonBuilder {
            id: "exit_button".to_string(),
            position: vec2(378.0, 500.0),
            size: vec2(300.0, 80.0),
            text: Some(ButtonTextType::Single(TextLineBuilder {
                text: "Exit".to_string(),
                font_size: 36.0,
                color: vec3(1.0, 1.0, 1.0),
                bold: false,
                position: vec2(378.0, 500.0)
            }.build(display))),
            normal_face: ButtonFace::Color(vec4(0.78, 0.24, 0.0, 1.0)),
            hovered_face: None,
            clicked_face: None
        }.build(display));

        MainMenu {
            background
        }
    }

    pub fn run(&mut self) -> MainMenuOutput {
        let play_button = get_ui_mut().get_element("play_button").unwrap();
        let exit_button = get_ui_mut().get_element("exit_button").unwrap();

        if play_button.output() == UiOutput::LeftClicked {
            return MainMenuOutput::Play;
        }

        if exit_button.output() == UiOutput::LeftClicked {
            close();
        }

        MainMenuOutput::None
    }

    pub fn draw(&self, target: &mut Frame) {
        self.background.draw(target);

        get_ui().get_element("play_button").unwrap().draw(target);
        get_ui().get_element("exit_button").unwrap().draw(target);
    }
}

use std::io::Cursor;

use glium::{Display, Frame};

use crate::{gfx::rect::{Rect, RectBuilder}, math::{vec2::vec2, vec4::vec4}, get_window_width, get_window_height, log::{log, save_log}, get_ui_mut, ui::{button::{ButtonBuilder, ButtonFace}, uielement::UiOutput}, get_ui};

pub struct Market {
    background: Rect
}

impl Market {
    pub fn new(display: &Display) -> Market {
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
            position: vec2(1842.0, 1038.0),
            size: vec2(155.0, 84.0),
            text: None,
            normal_face: ButtonFace::Color(vec4(1.0, 1.0, 1.0, 0.6)),
            hovered_face: None,
            clicked_face: None
        }.build(display));

        Market {
            background
        }
    }

    pub fn run(&mut self) -> bool {
        if get_ui().get_element("close_button").unwrap().output() == UiOutput::LeftClicked {
            return true;
        }

        

        false
    }

    pub fn draw(&self, target: &mut Frame) {
        self.background.draw(target);

        // commented out because i want it to be completely transparent and performance
        // get_ui().get_element("close_button").unwrap().draw(target);
    }
}

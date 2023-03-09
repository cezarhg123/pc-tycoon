use std::io::Cursor;

use glium::{Display, Frame};

use crate::{gfx::rect::{Rect, RectBuilder}, math::{vec2::vec2, vec4::vec4}, get_window_width, get_window_height, log::{log, save_log}};

pub struct BuildPC {
    background: Rect
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

        BuildPC {
            background
        }
    }

    pub fn draw(&self, target: &mut Frame) {
        self.background.draw(target);
    }
}

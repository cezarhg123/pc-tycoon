use std::io::Cursor;

use glium::{Display, Frame};

use crate::{gfx::rect::{Rect, RectBuilder}, math::{vec2::vec2, vec4::vec4}, get_window_width, get_window_height};

pub struct Inventory {
    background: Rect
}

impl Inventory {
    pub fn new(display: &Display) -> Inventory {
        let background = RectBuilder {
            position: vec2(get_window_width() as f32 / 2.0, get_window_height() as f32 / 2.0),
            size: vec2(get_window_width() as f32, get_window_height() as f32),
            color: vec4(1.0, 1.0, 1.0, 1.0),
            texture: Some(image::load(
                Cursor::new(std::fs::read("textures/inventory-background.png").unwrap()),
                image::ImageFormat::Png
            ).unwrap())
        }.build(display);

        Inventory {
            background
        }
    }

    pub fn draw(&self, target: &mut Frame) {
        self.background.draw(target);
    }
}

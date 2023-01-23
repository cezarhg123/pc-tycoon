use std::io::Cursor;

use glium::{Display, Frame};

use crate::{gfx::Rect, math::vec2::vec2, DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT, log::log};

#[derive(Debug)]
pub struct MainMenu {
    background: Rect,
}

impl MainMenu {
    pub fn new(display: &Display) -> MainMenu {
        let mut background = Rect::new(vec2(960.0, 540.0), vec2(DEFAULT_WINDOW_WIDTH as f32, DEFAULT_WINDOW_HEIGHT as f32), display);
        let image = image::load(Cursor::new(std::fs::read("textures/background.png").unwrap()), image::ImageFormat::Png).unwrap();
        background.set_texture(&image, display);
        
        log("loaded main menu");
        MainMenu {
            background
        }
    }

    pub fn draw(&self, target: &mut Frame) {
        self.background.draw(target);
    }
}

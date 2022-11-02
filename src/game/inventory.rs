use glfw::Window;

use crate::{gfx::{image_rect::ImageRect, color_rect::ColorRect, texture::Texture, vectors::vec2::vec2}, ui::{Ui, button::Button}, WINDOW_WIDTH, WINDOW_HEIGHT};

#[derive(Debug, Clone)]
pub struct Inventory<'a> {
    background: ImageRect,
    back_button: Button<'a>
}

impl<'a> Inventory<'a> {
    pub fn new(ui: &'a Ui) -> Inventory<'a> {
        Inventory {
            background: ImageRect::new(Texture::from_path("textures/inventory-background.png"), 0.0, 0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
            back_button: ui.button("a", vec2(1750.0, 0.0), vec2(170.0, 85.0))
        }
    }

    pub fn run(&mut self, window: &Window) -> bool {
        self.back_button.clicked(window)
    }

    pub fn draw(&self) {
        self.background.draw();
    }
}

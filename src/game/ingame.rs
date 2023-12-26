use std::io::Cursor;
use gpu_allocator::vulkan::Allocator;
use crate::{primitives::rect::Rect, WINDOW_WIDTH, WINDOW_HEIGHT};

pub struct InGame {
    background: Rect,
}

impl InGame {
    pub fn new(allocator: &mut Allocator) -> InGame {
        InGame {
            background: Rect::builder()
                .texture(
                    image::load(
                        Cursor::new(std::fs::read("textures/ingame-background.png").unwrap()),
                        image::ImageFormat::Png
                    ).unwrap()
                )
                .width(WINDOW_WIDTH as f32)
                .height(WINDOW_HEIGHT as f32)
                .build(allocator)
        }
    }

    pub fn draw(&self) {
        self.background.draw();
    }
}

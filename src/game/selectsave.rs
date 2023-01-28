use glium::Display;

use crate::{gfx::Rect, math::vec2::vec2, get_window_width, get_window_height};

#[derive(Debug)]
pub struct SelectSave {
    background: Rect
}

impl SelectSave {
    pub fn new(display: &Display) -> SelectSave {
        let background = Rect::new(vec2(get_window_width() as f32 / 2.0, get_window_height() as f32 / 2.0), vec2(get_window_width() as f32, get_window_height() as f32), display);

        SelectSave {
            background
        }
    }
}

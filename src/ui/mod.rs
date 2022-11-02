pub mod button;
pub mod listbox;

use std::{fmt::Debug, os::windows, any::Any};
use glfw::{Window, WindowEvent, MouseButton};
use rusttype::Font;
use crate::gfx::{color_rect::ColorRect, image_rect::ImageRect, vectors::{vec2::{Vec2, vec2}, vec3::Vec3}, text::Text};

use self::button::Button;

#[derive(Debug)]
pub struct Ui<'a> {
    font: Font<'a>
}

impl<'a> Ui<'a> {
    pub fn new(font: Font<'a>) -> Ui<'a> {
        Ui {
            font
        }
    }
    
    /// Generate a Text object.
    /// 
    /// Set `position` to none if object is given to another widget
    pub fn text(&self, text: &str, size: f32, color: Vec3<u8>, position: Option<Vec2<f32>>) -> Text {
        match position {
            Some(pos) => {
                Text::new(text, &self.font, size, color, pos)
            }
            None => {
                Text::new(text, &self.font, size, color, vec2(0.0, 0.0))
            }
        }
    }

    pub fn button(&self, text: &str, position: Vec2<f32>, size: Vec2<f32>) -> Button {
        Button::new(text, position, size, self)
    }
}

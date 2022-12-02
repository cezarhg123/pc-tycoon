pub mod button;
pub mod listbox;
pub mod info_popup;

use std::fmt::Debug;
use rusttype::Font;
use crate::gfx::{vectors::{vec2::{Vec2, vec2}, vec3::Vec3}, text::Text};

use self::{button::Button, listbox::ListBox, info_popup::InfoPopup};

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

    pub fn listbox(&self, pos: Vec2<f32>, size: Vec2<f32>, texts: &[String], text_size: f32) -> ListBox {
        ListBox::new(pos, size, texts, text_size, self)
    }

    pub fn info_popup(&self, id: &str, texts: &[String], pos: Vec2<f32>, text_size: f32) -> InfoPopup {
        InfoPopup::new(id, texts, pos, text_size, self)
    }
}

use glfw::{Window, Action};
use rusttype::Font;

use crate::gfx::{text::Text, rect::Rect, color_rect::ColorRect, vectors::{vec2::Vec2, vec3::vec3}};

use super::Ui;


#[derive(Debug, Clone)]
pub struct Button<'a> {
    text: Text<'a>,
    rect: ColorRect
}

impl<'a> Button<'a> {
    pub fn new(text: &str, pos: Vec2<f32>, size: Vec2<f32>, ui: &'a Ui) -> Button<'a> {
        let mut text = ui.text(text, size.y / 2.0, vec3(255, 255, 255), None);
        let rect = ColorRect::new(vec3(0.3, 0.3, 0.3), pos.x, pos.y, size.x, size.y);
        text.set_center(rect.get_center());

        Button {
            text,
            rect
        }
    }

    pub fn clicked(&mut self, window: &Window) -> bool {
        if self.rect.contains(window.get_cursor_pos().try_into().unwrap()) {
            self.rect.set_color(vec3(0.37, 0.37, 0.37));

            if window.get_mouse_button(glfw::MouseButton::Button1) == Action::Press {
                self.rect.set_color(vec3(0.44, 0.44, 0.44));
                return true;
            }
        } else {
            self.rect.set_color(vec3(0.3, 0.3, 0.3));
        }

        false
    }

    pub fn get_rects(&self) -> Vec<Rect> {
        vec![
            Rect::Color(self.rect.clone()),
            self.text.get_rect()
        ]
    }

    pub fn draw(&self) {
        self.rect.draw();
        self.text.draw();
    }
}

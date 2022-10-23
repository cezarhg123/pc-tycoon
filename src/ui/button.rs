use glfw::{Window, Action};
use crate::gfx::{text::Text, vectors::{vec2::Vec2, vec3::vec3}, color_rect::ColorRect};

#[derive(Debug, Clone)]
pub struct Button<'a> {
    text: Text<'a>,
    position: Vec2<f32>,
    size: Vec2<f32>,
    rect: ColorRect
}

impl<'a> Button<'a> {
    pub fn new(mut text: Text<'a>, position: Vec2<f32>, size: Vec2<f32>) -> Button<'a> {
        let rect = ColorRect::new(vec3(0.3, 0.3, 0.3), position.x, position.y, size.x, size.y);
        text.set_center(rect.get_center());
        
        Button {
            text,
            position,
            size,
            rect
        }
    }

    pub fn clicked(&mut self, window: &Window) -> bool {
        if self.rect.contains(window.get_cursor_pos().try_into().unwrap()) {
            self.rect.set_color(vec3(0.35, 0.35, 0.35));
            
            if window.get_mouse_button(glfw::MouseButton::Button1) == Action::Press {
                self.rect.set_color(vec3(0.4, 0.4, 0.4));
                return true;
            }
        } else {
            self.rect.set_color(vec3(0.3, 0.3, 0.3));
        }

        false
    }

    pub fn draw(&self) {
        self.rect.draw();
        self.text.draw();
    }

    pub fn get_left(&self) -> f32 {
        self.rect.get_left()
    }

    pub fn set_left(&mut self, left: f32) {
        self.rect.set_left(left);
    }

    pub fn get_top(&self) -> f32 {
        self.rect.get_top()
    }

    pub fn set_top(&mut self, top: f32) {
        self.rect.set_top(top);
    }

    pub fn get_width(&self) -> f32 {
        self.rect.get_width()
    }

    pub fn set_width(&mut self, width: f32) {
        self.rect.set_width(width);
    }

    pub fn get_height(&self) -> f32 {
        self.rect.get_height()
    }

    pub fn set_height(&mut self, height: f32) {
        self.rect.set_height(height);
    }

    pub fn get_center(&self) -> Vec2<f32> {
        self.rect.get_center()
    }
    
    pub fn set_center(&mut self, center: Vec2<f32>) {
        self.rect.set_center(center);
    }
}

use glium::glutin::event::{ElementState, MouseButton};

use crate::{gfx::rect::Rect, ui::glium_position_to_mine};
use super::{uiattributes::UiAttributes, uioutput::UiOutput};

pub struct UiRect {
    output: UiOutput,
    rect: Rect
}

impl UiRect {
    pub fn new(rect: Rect) -> UiRect {
        UiRect {
            output: UiOutput::None,
            rect
        }
    }
}

impl UiAttributes for UiRect {
    fn left(&self) -> f32 {
        self.rect.left()
    }

    fn set_left(&mut self, left: f32) {
        self.rect.set_left(left);
    }

    fn top(&self) -> f32 {
        self.rect.top()
    }

    fn set_top(&mut self, top: f32) {
        self.rect.set_top(top);
    }

    fn right(&self) -> f32 {
        self.rect.right()
    }

    fn set_right(&mut self, right: f32) {
        self.rect.set_right(right);
    }

    fn bottom(&self) -> f32 {
        self.rect.bottom()
    }

    fn set_bottom(&mut self, bottom: f32) {
        self.rect.set_bottom(bottom);
    }

    fn centre(&self) -> crate::math::vec2::Vec2<f32> {
        self.rect.centre()
    }

    fn set_centre(&mut self, centre: crate::math::vec2::Vec2<f32>) {
        self.rect.set_centre(centre);
    }

    fn size(&self) -> crate::math::vec2::Vec2<f32> {
        self.rect.size()
    }

    fn set_size(&mut self, size: crate::math::vec2::Vec2<f32>) {
        self.rect.set_size(size);
    }

    fn width(&self) -> f32 {
        self.rect.width()
    }

    fn set_width(&mut self, width: f32) {
        self.rect.set_width(width);
    }

    fn height(&self) -> f32 {
        self.rect.height()
    }

    fn set_height(&mut self, height: f32) {
        self.rect.set_height(height);
    }

    fn output(&self) -> UiOutput {
        self.output
    }

    fn handle_events(&mut self, event: &glium::glutin::event::WindowEvent) -> bool {
        println!("{:#?}", self.output);
        
        use glium::glutin::event::WindowEvent;
        match event {
            WindowEvent::CursorMoved {position, .. } => {
                let position = glium_position_to_mine(position);

                if self.rect.contains(position) && (self.output != UiOutput::LeftClicked || self.output != UiOutput::RightClicked) {
                    self.output = UiOutput::Hovered;
                    return true;
                } else {
                    self.output = UiOutput::None;
                }
            }
            WindowEvent::MouseInput {state, button, ..} => {
                match (button, state) {
                    (MouseButton::Left, ElementState::Pressed) => {
                        if self.output == UiOutput::Hovered {
                            self.output = UiOutput::LeftClicked;
                            return true;
                        }
                    }
                    (MouseButton::Left, ElementState::Released) => {
                        if self.output == UiOutput::LeftClicked {
                            self.output = UiOutput::Hovered;
                            return true;
                        }
                    }
                    (MouseButton::Right, ElementState::Pressed) => {
                        if self.output == UiOutput::Hovered {
                            self.output = UiOutput::RightClicked;
                            return true;
                        }
                    }
                    (MouseButton::Right, ElementState::Released) => {
                        if self.output == UiOutput::RightClicked {
                            self.output = UiOutput::Hovered;
                            return true;
                        }
                    }
                    _ => {}
                }
            }
            _ => {
                self.output = UiOutput::None;
            }
        }
        
        false
    }

    fn draw(&self, target: &mut glium::Frame) {
        self.rect.draw(target);
    }
}

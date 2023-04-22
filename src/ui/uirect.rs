use glium::glutin::event::{MouseButton, ElementState, VirtualKeyCode};
use crate::{gfx::rect::{Rect, RectBuilder}, ui::uielement::UiOutput, MOVE_UI};
use super::{uielement::UiElement, customuidata::CustomUIData};

pub struct UiRect {
    id: String,
    custom_data: Vec<CustomUIData>,
    rect: Rect,
    output: UiOutput
}

impl UiRect {
    pub fn new(id: &str, rect: Rect, custom_data: Vec<CustomUIData>) -> UiRect {
        UiRect {
            id: id.to_string(),
            custom_data, 
            rect,
            output: UiOutput::None
        }
    }
}

impl UiElement for UiRect {
    fn handle_event(&mut self, event: &glium::glutin::event::WindowEvent, cursor_pos: crate::math::vec2::Vec2<f32>, display: &glium::Display) -> bool {
        use glium::glutin::event::WindowEvent;
        if self.rect.contains(cursor_pos) {
            self.output = UiOutput::Hovered;

            match event {
                WindowEvent::MouseInput {button, state, ..} => {
                    match (button, state) {
                        (MouseButton::Left, ElementState::Pressed) => {
                            self.output = UiOutput::LeftClicked;
                            true
                        },
                        (MouseButton::Right, ElementState::Pressed) => {
                            self.output = UiOutput::RightClicked;
                            true
                        },
                        _ => {false}
                    }
                }
                WindowEvent::KeyboardInput {input, ..} => {
                    if MOVE_UI {
                        match (input.virtual_keycode.unwrap(), input.state) {
                            (VirtualKeyCode::Up, ElementState::Pressed) => {
                                self.set_top(self.top() + 1.0);
                                true
                            }
                            (VirtualKeyCode::Down, ElementState::Pressed) => {
                                self.set_bottom(self.bottom() - 1.0);
                                true
                            }
                            (VirtualKeyCode::Right, ElementState::Pressed) => {
                                self.set_right(self.right() + 1.0);
                                true
                            }
                            (VirtualKeyCode::Left, ElementState::Pressed) => {
                                self.set_left(self.left() - 1.0);
                                true
                            }
                            (VirtualKeyCode::Return, ElementState::Pressed) => {
                                println!("{:#?}", self.centre());
                                true
                            }
                            (VirtualKeyCode::Numpad4, ElementState::Pressed) => {
                                self.set_width(self.width() - 1.0);
                                true
                            }
                            (VirtualKeyCode::Numpad6, ElementState::Pressed) => {
                                self.set_width(self.width() + 1.0);
                                true
                            }
                            (VirtualKeyCode::Numpad2, ElementState::Pressed) => {
                                self.set_height(self.height() - 1.0);
                                true
                            }
                            (VirtualKeyCode::Numpad8, ElementState::Pressed) => {
                                self.set_height(self.height() + 1.0);
                                true
                            }
                            (VirtualKeyCode::Numpad5, ElementState::Pressed) => {
                                println!("size: {}, {}", self.width(), self.height());
                                true
                            }
                            _ => {false}
                        }
                    } else {false}
                }
                _ => {false}
            }
        } else {
            self.output = UiOutput::None;
            false
        }
    }

    fn output(&self) -> UiOutput {
        self.output
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn custon_data(&self) -> &[CustomUIData] {
        self.custom_data.as_slice()
    }

    fn left(&self) -> f32 {
        self.rect.left()
    }

    fn set_left(&mut self, left: f32) {
        self.rect.set_left(left)
    }

    fn top(&self) -> f32 {
        self.rect.top()
    }

    fn set_top(&mut self, top: f32) {
        self.rect.set_top(top)
    }

    fn right(&self) -> f32 {
        self.rect.right()
    }

    fn set_right(&mut self, right: f32) {
        self.rect.set_right(right)
    }

    fn bottom(&self) -> f32 {
        self.rect.bottom()
    }

    fn set_bottom(&mut self, bottom: f32) {
        self.rect.set_bottom(bottom)
    }

    fn centre(&self) -> crate::math::vec2::Vec2<f32> {
        self.rect.centre()
    }

    fn set_centre(&mut self, centre: crate::math::vec2::Vec2<f32>) {
        self.rect.set_centre(centre)
    }

    fn width(&self) -> f32 {
        self.rect.width()
    }

    fn set_width(&mut self, width: f32) {
        self.rect.set_width(width)
    }

    fn height(&self) -> f32 {
        self.rect.height()
    }

    fn set_height(&mut self, height: f32) {
        self.rect.set_height(height)
    }

    fn draw(&self, target: &mut glium::Frame) {
        self.rect.draw(target);
    }
}

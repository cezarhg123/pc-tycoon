use glium::{Display, glutin::event::{MouseButton, ElementState, VirtualKeyCode}};

use crate::{gfx::rect::{Rect, RectBuilder}, math::{vec2::{Vec2, vec2}, vec3::{Vec3, vec3}, vec4::vec4}, MOVE_UI};
use super::{textline::{TextLine, TextLineBuilder}, uielement::{UiOutput, UiElement}};

pub enum TextLayout {
    Left,
    Middle,
    Right
}

pub struct MultiTextLine {
    output: UiOutput,
    id: String,
    textlines: Vec<TextLine>,
    layout: TextLayout,
    rect: Rect
}

impl UiElement for MultiTextLine {
    fn handle_event(&mut self, event: &glium::glutin::event::WindowEvent, cursor_pos: crate::math::vec2::Vec2<f32>, display: &Display) -> bool {
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
        self.id.as_str()
    }

    fn left(&self) -> f32 {
        self.rect.left()
    }

    fn set_left(&mut self, left: f32) {
        let diff = left - self.rect.left();
        self.rect.set_left(left);

        for textline in &mut self.textlines {
            textline.set_left(textline.left() + diff);
        }
    }

    fn top(&self) -> f32 {
        self.rect.top()
    }

    fn set_top(&mut self, top: f32) {
        let diff = top - self.rect.top();
        self.rect.set_top(top);

        for textline in &mut self.textlines {
            textline.set_top(textline.top() + diff);
        }
    }

    fn right(&self) -> f32 {
        self.rect.right()
    }

    fn set_right(&mut self, right: f32) {
        let diff = right - self.rect.right();
        self.rect.set_right(right);

        for textline in &mut self.textlines {
            textline.set_right(textline.right() + diff);
        }
    }

    fn bottom(&self) -> f32 {
        self.rect.bottom()
    }

    fn set_bottom(&mut self, bottom: f32) {
        let diff = bottom - self.rect.bottom();
        self.rect.set_bottom(bottom);

        for textline in &mut self.textlines {
            textline.set_bottom(textline.bottom() + diff);
        }
    }

    fn centre(&self) -> Vec2<f32> {
        self.rect.centre()
    }

    fn set_centre(&mut self, centre: Vec2<f32>) {
        let diff = centre - self.rect.centre();
        self.rect.set_centre(centre);

        for textline in &mut self.textlines {
            textline.set_centre(textline.centre() + diff);
        }
    }

    fn width(&self) -> f32 {
        self.rect.width()
    }

    fn set_width(&mut self, width: f32) {}

    fn height(&self) -> f32 {
        self.rect.height()
    }

    fn set_height(&mut self, height: f32) {}

    fn draw(&self, target: &mut glium::Frame) {
        self.rect.draw(target);
        
        for textline in &self.textlines {
            textline.draw(target);
        }
    }
}

/// Split text with \n to actually make it multiline
pub struct MultiTextLineBuilder {
    pub id: String,
    pub text: String,
    pub layout: TextLayout,
    pub font_size: f32,
    pub color: Vec3<f32>,
    pub bold: bool,
    pub position: Vec2<f32>
}

impl Default for MultiTextLineBuilder {
    fn default() -> Self {
        MultiTextLineBuilder {
            id: "Default".to_string(),
            text: "Default\nDefault".to_string(),
            layout: TextLayout::Middle,
            font_size: 12.0,
            color: vec3(1.0, 1.0, 1.0),
            bold: false,
            position: vec2(500.0, 400.0)
        }
    }
}

impl MultiTextLineBuilder {
    pub fn build(self, display: &Display) -> MultiTextLine {
        let split_texts = self.text.split("\n").collect::<Vec<_>>();
        let mut textlines = Vec::new();
        textlines.reserve(split_texts.len());

        let mut biggest_x = 0.0; // widest textline
        let mut total_y = 0.0; // total height of textlines
        let mut single_y = 0.0; // height of 1 textline
        for text in split_texts {
            let textline = TextLineBuilder {
                id: text.to_string(),
                text: text.to_string(),
                font_size: self.font_size,
                color: self.color,
                bold: self.bold,
                position: vec2(0.0, 0.0)
            }.build(display);

            if textline.width() > biggest_x {
                biggest_x = textline.width();
            } // pretty much find widest textline
            total_y += textline.height();
            single_y = textline.height();

            textlines.push(textline);
        };

        let rect = RectBuilder {
            position: self.position,
            size: vec2(biggest_x, total_y),
            color: vec4(0.0, 0.0, 0.0, 0.0),
            texture: None
        }.build(display);

        match self.layout {
            TextLayout::Left => {
                for (i, textline) in textlines.iter_mut().enumerate() {
                    textline.set_left(rect.left());
                    textline.set_top(rect.top() + (i as f32 * single_y));
                }
            }
            TextLayout::Middle => {
                for (i, textline) in textlines.iter_mut().enumerate() {
                    textline.set_centre(vec2(self.position.x, 0.0));
                    textline.set_top(rect.top() + (i as f32 * single_y));
                }
            }
            TextLayout::Right => {
                for (i, textline) in textlines.iter_mut().enumerate() {
                    textline.set_right(rect.right());
                    textline.set_top(rect.top() + (i as f32 * single_y));
                }
            }
        };

        MultiTextLine {
            id: self.id,
            output: UiOutput::None,
            textlines,
            layout: self.layout,
            rect
        }
    }
}
use std::io::Cursor;

use glium::{texture::{SrgbTexture2d, RawImage2d}, Display, glutin::event::{MouseButton, ElementState, VirtualKeyCode}};
use image::{DynamicImage, GenericImageView};

use crate::{math::{vec2::{Vec2, vec2}, vec4::{Vec4, vec4}}, gfx::rect::{Rect, RectBuilder}, MOVE_UI};
use super::{uielement::{UiOutput, UiElement}, textline::{TextLine, TextLineBuilder}, multitextline::MultiTextLine};

pub enum ButtonTextType {
    Single(TextLine),
    Multi(MultiTextLine)
}

#[derive(Debug, Clone)]
pub enum ButtonFace {
    Color(Vec4<f32>),
    Texture(DynamicImage)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ButtonFaceState {
    Normal,
    Hovered,
    Clicked
}

pub struct Button {
    output: UiOutput,
    id: String,
    position: Vec2<f32>,
    size: Vec2<f32>,
    text: Option<ButtonTextType>,
    normal_face: ButtonFace,
    hovered_face: ButtonFace,
    clicked_face: ButtonFace,
    rect: Rect,
    button_face_state: ButtonFaceState,
}

impl UiElement for Button {
    fn handle_event(&mut self, event: &glium::glutin::event::WindowEvent, cursor_pos: Vec2<f32>, display: &Display) -> bool {
        use glium::glutin::event::WindowEvent;
        if self.rect.contains(cursor_pos) {
            self.output = UiOutput::Hovered;
            let new_face_state = ButtonFaceState::Hovered;
            if self.button_face_state != new_face_state {
                match &self.hovered_face {
                    ButtonFace::Color(color) => {
                        self.rect.set_color(color.clone());
                    }
                    ButtonFace::Texture(texture) => {
                        self.rect.set_texture(texture.clone(), display);
                    }
                }

                self.button_face_state = new_face_state;
            }

            match event {
                WindowEvent::MouseInput {button, state, ..} => {
                    match (button, state) {
                        (MouseButton::Left, ElementState::Pressed) => {
                            self.output = UiOutput::LeftClicked;
                            let new_face_state = ButtonFaceState::Clicked;
                            if self.button_face_state != new_face_state {
                                match &self.clicked_face {
                                    ButtonFace::Color(color) => {
                                        self.rect.set_color(color.clone());
                                    }
                                    ButtonFace::Texture(texture) => {
                                        self.rect.set_texture(texture.clone(), display);
                                    }
                                }

                                self.button_face_state = new_face_state;
                            }
                            true
                        },
                        (MouseButton::Right, ElementState::Pressed) => {
                            self.output = UiOutput::RightClicked;
                            let new_face_state = ButtonFaceState::Clicked;
                            if self.button_face_state != new_face_state {
                                match &self.clicked_face {
                                    ButtonFace::Color(color) => {
                                        self.rect.set_color(color.clone());
                                    }
                                    ButtonFace::Texture(texture) => {
                                        self.rect.set_texture(texture.clone(), display);
                                    }
                                }

                                self.button_face_state = new_face_state;
                            }
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
                                self.set_width(self.width() + 1.0);
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
            let new_face_state = ButtonFaceState::Normal;
            if self.button_face_state != new_face_state {
                match &self.normal_face {
                    ButtonFace::Color(color) => {
                        self.rect.set_color(color.clone());
                    }
                    ButtonFace::Texture(texture) => {
                        self.rect.set_texture(texture.clone(), display);
                    }
                }

                self.button_face_state = new_face_state;
            }
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

        if let Some(text) = &mut self.text {
            match text {
                ButtonTextType::Single(text) => {
                    text.set_left(text.left() + diff);
                }
                ButtonTextType::Multi(text) => {
                    text.set_left(text.left() + diff);
                }
            }
        }
        
        self.rect.set_left(left);
    }

    fn top(&self) -> f32 {
        self.rect.top()
    }

    fn set_top(&mut self, top: f32) {
        let diff = top - self.rect.top();

        if let Some(text) = &mut self.text {
            match text {
                ButtonTextType::Single(text) => {
                    text.set_top(text.top() + diff);
                }
                ButtonTextType::Multi(text) => {
                    text.set_top(text.top() + diff);
                }
            }
        }
        
        self.rect.set_top(top);
    }

    fn right(&self) -> f32 {
        self.rect.right()
    }

    fn set_right(&mut self, right: f32) {
        let diff = right - self.rect.right();

        if let Some(text) = &mut self.text {
            match text {
                ButtonTextType::Single(text) => {
                    text.set_right(text.right() + diff);
                }
                ButtonTextType::Multi(text) => {
                    text.set_right(text.right() + diff);
                }
            }
        }
        
        self.rect.set_right(right);
    }

    fn bottom(&self) -> f32 {
        self.rect.bottom()
    }

    fn set_bottom(&mut self, bottom: f32) {
        let diff = bottom - self.rect.bottom();

        if let Some(text) = &mut self.text {
            match text {
                ButtonTextType::Single(text) => {
                    text.set_bottom(text.bottom() + diff);
                }
                ButtonTextType::Multi(text) => {
                    text.set_bottom(text.bottom() + diff);
                }
            }
        }
        
        self.rect.set_bottom(bottom);
    }

    fn centre(&self) -> Vec2<f32> {
        self.rect.centre()
    }

    fn set_centre(&mut self, centre: Vec2<f32>) {
        let diff = centre - self.rect.centre();

        if let Some(text) = &mut self.text {
            match text {
                ButtonTextType::Single(text) => {
                    text.set_centre(text.centre() + diff);
                }
                ButtonTextType::Multi(text) => {
                    text.set_centre(text.centre() + diff);
                }
            }
        }

        self.rect.set_centre(centre);
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

    fn draw(&self, target: &mut glium::Frame) {
        self.rect.draw(target);

        if let Some(text) = &self.text {
            match text {
                ButtonTextType::Single(text) => {
                    text.draw(target);
                }
                ButtonTextType::Multi(text) => {
                    text.draw(target);
                }
            }
        }
    }
}

pub struct ButtonBuilder {
    pub id: String,
    pub position: Vec2<f32>,
    pub size: Vec2<f32>,
    pub text: Option<ButtonTextType>,
    pub normal_face: ButtonFace,
    pub hovered_face: Option<ButtonFace>,
    pub clicked_face: Option<ButtonFace>
}

impl Default for ButtonBuilder {
    fn default() -> Self {
        ButtonBuilder {
            id: "Default".to_string(),
            position: vec2(1000.0, 600.0),
            size: vec2(300.0, 100.0),
            text: None,
            normal_face: ButtonFace::Color(vec4(1.0, 1.0, 1.0, 1.0)),
            hovered_face: None,
            clicked_face: None
        }
    }
}

impl ButtonBuilder {
    pub fn build(self, display: &Display) -> Button {
        let hovered_face = match &self.hovered_face {
            Some(hovered_face) => {hovered_face.clone()} // just use the face that dev set
            None => {
                if let ButtonFace::Color(normal_face_color) = self.normal_face {
                    let average = (normal_face_color.x + normal_face_color.y + normal_face_color.z) / 3.0;
                    if average < 0.5  { // return face if button is overall dark
                        ButtonFace::Color(vec4(normal_face_color.x + 0.1, normal_face_color.y + 0.1, normal_face_color.z + 0.1, normal_face_color.w))
                    } else { // return face if button is overall bright
                        ButtonFace::Color(vec4(normal_face_color.x - 0.1, normal_face_color.y - 0.1, normal_face_color.z - 0.1, normal_face_color.w))
                    }
                } else {
                    self.normal_face.clone() // if the normal face is a texture than just clone that shit into this one
                }
            }
        };

        // read the fucking comments at 'hovered_face'
        let clicked_face = match self.clicked_face {
            Some(clicked_face) => {clicked_face}
            None => {
                if let ButtonFace::Color(normal_face_color) = &self.normal_face {
                    let average = (normal_face_color.x + normal_face_color.y + normal_face_color.z) / 3.0;
                    if average < 0.5  {
                        ButtonFace::Color(vec4(normal_face_color.x + 0.2, normal_face_color.y + 0.2, normal_face_color.z + 0.2, normal_face_color.w))
                    } else {
                        ButtonFace::Color(vec4(normal_face_color.x - 0.2, normal_face_color.y - 0.2, normal_face_color.z - 0.2, normal_face_color.w))
                    }
                } else {
                    self.normal_face.clone()
                }
            }
        };

        let rect = RectBuilder {
            position: self.position,
            size: self.size,
            color: match &self.normal_face {
                ButtonFace::Color(color) => {color.clone()},
                ButtonFace::Texture(_) => {vec4(1.0, 1.0, 1.0, 1.0)}
            },
            texture: match &self.normal_face {
                ButtonFace::Color(_) => {None},
                ButtonFace::Texture(texture) => {Some(texture.clone())}
            }
        }.build(display);

        Button {
            output: UiOutput::None,
            id: self.id,
            position: self.position,
            size: self.size,
            text: self.text,
            normal_face: self.normal_face,
            hovered_face,
            clicked_face,
            rect,
            button_face_state: ButtonFaceState::Normal
        }
    }
}

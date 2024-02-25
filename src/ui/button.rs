use std::ops::{Deref, DerefMut};
use gpu_allocator::vulkan::Allocator;
use crate::{primitives::{rect::{Rect, RectBuilder}, text::{Text, TextBuilder}}};
use super::ui_object::UiObject;

pub struct Button {
    rect: Rect,
    state: ButtonState,
    clicked_once: bool,
    normal_face: ButtonFace,
    hovered_face: Option<ButtonFace>,
    pressed_face: Option<ButtonFace>,
    text: Option<Text>
}

pub enum ButtonFace {
    Color(glm::Vec4),
    Texture(image::DynamicImage),
    Both(glm::Vec4, image::DynamicImage)
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ButtonState {
    Normal,
    Hovered,
    Pressed,
}

impl Button {
    pub fn builder() -> ButtonBuilder {
        ButtonBuilder {
            position: glm::vec2(0.0, 0.0),
            size: glm::vec2(100.0, 100.0),
            normal_face: ButtonFace::Color(glm::vec4(1.0, 1.0, 1.0, 1.0)),
            hovered_face: None,
            pressed_face: None,
            text: None,
        }
    }

    /// change face based on current state
    pub fn change_rect_face(&mut self, flip: bool, allocator: &mut Allocator) {
        // reduce code duplication
        fn change(rect: &mut Rect, face: &ButtonFace, allocator: &mut Allocator) {
            match face {
                ButtonFace::Color(color) => rect.set_color(*color),
                ButtonFace::Texture(texture) => {
                    *rect = Rect::builder()
                        .left(rect.left())
                        .top(rect.top())
                        .width(rect.width())
                        .height(rect.height())
                        .texture(texture.clone())
                        .build(allocator)
                },
                ButtonFace::Both(color, texture) => {
                    *rect = Rect::builder()
                        .left(rect.left())
                        .top(rect.top())
                        .width(rect.width())
                        .height(rect.height())
                        .color(*color)
                        .texture(texture.clone())
                        .build(allocator)
                }
            }
        }

        match &self.state {
            ButtonState::Normal => {
                change(&mut self.rect, &self.normal_face, allocator);
            },
            ButtonState::Hovered => {
                if let Some(hovered_face) = &self.hovered_face {
                    change(&mut self.rect, hovered_face, allocator);
                } else {
                    let color = match &self.normal_face {
                        ButtonFace::Color(color) => *color,
                        _ => glm::vec4(1.0, 1.0, 1.0, 1.0)
                    };
                    // how bright the button is
                    let how_lit = (color.x + color.y + color.z) / 3.0;

                    if flip {
                        if how_lit >= 0.5 {
                            self.rect.undim(0.2);
                        } else { // undim. aka lighten
                            self.rect.dim(0.2);
                        }
                    } else {
                        // dim button if it is "brighter" than 0.5 
                        if how_lit >= 0.5 {
                            self.rect.dim(0.2);
                        } else { // undim. aka lighten
                            self.rect.undim(0.2);
                        }
                    }
                }
            }
            ButtonState::Pressed => {
                if let Some(pressed_face) = &self.pressed_face {
                    change(&mut self.rect, pressed_face, allocator);
                } else {
                    let color = match &self.normal_face {
                        ButtonFace::Color(color) => *color,
                        _ => glm::vec4(1.0, 1.0, 1.0, 1.0)
                    };
                    // how bright the button is
                    let how_lit = (color.x + color.y + color.z) / 3.0;

                    if flip {
                        if how_lit >= 0.5 {
                            self.rect.undim(0.2);
                        } else { // undim. aka lighten
                            self.rect.dim(0.2);
                        }
                    } else {
                        // dim button if it is "brighter" than 0.5 
                        if how_lit >= 0.5 {
                            self.rect.dim(0.2);
                        } else { // undim. aka lighten
                            self.rect.undim(0.2);
                        }
                    }
                }
            }
        }
    }

    pub fn pressed(&self) -> bool {
        self.state == ButtonState::Pressed
    }

    /// returns true if button was pressed once(true after click is released)
    pub fn pressed_once(&self) -> bool {
        if self.clicked_once {
            // i really dont want this function to require mutable self
            unsafe {
                (((self as *const Button).cast_mut()).as_mut().unwrap()).clicked_once = false;
            }

            true
        } else {
            false
        }
    }
}

pub struct ButtonBuilder {
    /// Top left
    position: glm::Vec2,
    /// width and height
    size: glm::Vec2,
    normal_face: ButtonFace,
    hovered_face: Option<ButtonFace>,
    pressed_face: Option<ButtonFace>,
    text: Option<TextBuilder>,
}

impl ButtonBuilder {
    /// Just specify rect dimensions, color and texture is omitted
    pub fn dimensions(mut self, rect: RectBuilder) -> ButtonBuilder {
        self.position.x = rect.left;
        self.position.y = rect.top;
        self.size.x = rect.width;
        self.size.y = rect.height;

        self
    }

    pub fn normal_face(mut self, normal_face: ButtonFace) -> ButtonBuilder {
        self.normal_face = normal_face;
        self
    }

    pub fn hovered_face(mut self, hovered_face: ButtonFace) -> ButtonBuilder {
        self.hovered_face = Some(hovered_face);
        self
    }

    pub fn pressed_face(mut self, pressed_face: ButtonFace) -> ButtonBuilder {
        self.pressed_face = Some(pressed_face);
        self
    }

    /// build will autmatically centre the text
    pub fn text(mut self, text: TextBuilder) -> ButtonBuilder {
        self.text = Some(text);
        self
    }

    pub fn build(self, allocator: &mut Allocator) -> Button {
        let mut rect = Rect::builder()
            .left(self.position.x)
            .top(self.position.y)
            .width(self.size.x)
            .height(self.size.y);

        match &self.normal_face {
            ButtonFace::Color(color) => rect = rect.color(*color),
            ButtonFace::Texture(texture) => rect = rect.texture(texture.clone()),
            ButtonFace::Both(color, texture) => {
                rect = rect.color(*color).texture(texture.clone());
            }
        }

        let rect = rect.build(allocator);
        let rect_color = rect.color();

        let mut button = Button {
            rect,
            state: ButtonState::Normal,
            clicked_once: false,
            normal_face: self.normal_face,
            hovered_face: self.hovered_face,
            pressed_face: self.pressed_face,
            text: None,
        };

        if let Some(mut text) = self.text {
            // set text color to black if rect color is white
            if rect_color == glm::vec4(1.0, 1.0, 1.0, 1.0) {
                text = text.font_color(glm::vec3(0.0, 0.0, 0.0));
            }

            button.text = Some(text.build(allocator));
            // centre text
            button.text.as_mut().unwrap().set_center(button.rect.center());
        }

        button
    }
}

impl UiObject for Button {
    fn contains(&self, pos: glm::Vec2) -> bool {
        self.rect.contains(pos)
    }

    fn left(&self) -> f32 {
        self.rect.left()
    }

    fn set_left(&mut self, left: f32) {
        let diff = left - self.rect.left();
        self.rect.set_left(left);
        if let Some(text) = &mut self.text {
            let left = text.left();
            text.set_left(left + diff);
        }
    }

    fn top(&self) -> f32 {
        self.rect.top()
    }

    fn set_top(&mut self, top: f32) {
        let diff = top - self.rect.top();
        self.rect.set_top(top);
        if let Some(text) = &mut self.text {
            let top = text.top();
            text.set_top(top + diff);
        }
    }

    fn right(&self) -> f32 {
        self.rect.right()
    }

    fn set_right(&mut self, right: f32) {
        let diff = right - self.rect.right();
        self.rect.set_right(right);
        if let Some(text) = &mut self.text {
            let right = text.right();
            text.set_right(right + diff);
        }
    }

    fn bottom(&self) -> f32 {
        self.rect.bottom()
    }

    fn set_bottom(&mut self, bottom: f32) {
        let diff = bottom - self.rect.bottom();
        self.rect.set_bottom(bottom);
        if let Some(text) = &mut self.text {
            let bottom = text.bottom();
            text.set_bottom(bottom + diff);
        }
    }

    fn width(&self) -> f32 {
        self.rect.width()
    }

    fn set_width(&mut self, width: f32) {
        self.rect.set_width(width);
        
        if let Some(text) = &mut self.text {
            text.set_center(self.rect.center());
        }
    }

    fn height(&self) -> f32 {
        self.rect.height()
    }

    fn set_height(&mut self, height: f32) {
        self.rect.set_height(height);
        
        if let Some(text) = &mut self.text {
            text.set_center(self.rect.center());
        }
    }

    fn center(&self) -> glm::Vec2 {
        self.rect.center()
    }

    fn set_center(&mut self, center: glm::Vec2) {
        let diff = center - self.rect.center();
        self.rect.set_center(center);
        if let Some(text) = &mut self.text {
            let center = text.center();
            text.set_center(center + diff);
        }
    }

    fn handle_events(&mut self, event: &glfw::WindowEvent, allocator: &mut Allocator) -> bool {
        match *event {
            glfw::WindowEvent::CursorPos(x, y) => {
                match &self.state {
                    // dont change the state if the button is clicked but the mouse moves around
                    ButtonState::Pressed => return true,
                    ButtonState::Hovered => {
                        if !self.rect.contains(glm::vec2(x as f32, y as f32)) {
                            self.state = ButtonState::Normal;
                            self.change_rect_face(false, allocator);
                        }
                        false
                    }
                    ButtonState::Normal => {
                        if self.rect.contains(glm::vec2(x as f32, y as f32)) {
                            self.state = ButtonState::Hovered;
                            self.change_rect_face(false, allocator);
                            true
                        } else {
                            false
                        }
                    }
                }
            }
            glfw::WindowEvent::MouseButton(button, action, _) => {
                if button == glfw::MouseButton::Button1 && action == glfw::Action::Press && self.state == ButtonState::Hovered {
                    self.state = ButtonState::Pressed;
                    self.change_rect_face(false, allocator);
                    true
                } else if button == glfw::MouseButton::Button1 && action == glfw::Action::Release && self.state == ButtonState::Pressed {
                    self.state = ButtonState::Hovered;
                    self.change_rect_face(true, allocator);

                    self.clicked_once = true;
                    false
                } else {
                    false
                }
            }
            _ => {
                false
            }
        }
    }

    fn draw(&self) {
        self.rect.draw();
        if let Some(text) = &self.text {
            text.draw();
        }
    }
}

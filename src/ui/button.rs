use glium::{Display, Frame, glutin::event::{WindowEvent, KeyboardInput, ElementState, MouseButton}};
use image::DynamicImage;
use crate::{math::{vec2::Vec2, vec3::{Vec3, vec3}, vec4::vec4}, gfx::Rect};
use super::{textline::TextLine, multitextline::{MultiTextLine, TextAlignment}};

#[derive(Debug)]
pub struct Button {
    textline: Option<TextLine>,
    multi_textline: Option<MultiTextLine>,
    position: Vec2<f32>,
    size: Vec2<f32>,
    /// 0 - normal color, 1 - clicked color
    /// 
    /// Just darkens button if 1 aint set
    colors: (Vec3<f32>, Option<Vec3<f32>>),
    /// 0 - normal color, 1 - clicked color
    /// 
    /// If 0 has texture but 1 dont then button will just make texture darker
    textures: (Option<DynamicImage>, Option<DynamicImage>),
    back_rect: Rect,
    clicked: bool,
    previous_clicked: bool
}

impl Button {
    pub fn handle_event(&mut self, cursor_pos: Vec2<f32>, event: &WindowEvent, display: &Display) -> bool {
        match event {
            WindowEvent::MouseInput {state, button, ..} => match (state, button) {
                (ElementState::Pressed, MouseButton::Left) => {
                    if !self.clicked {
                        if self.back_rect.contains(cursor_pos) {
                            if let Some(clicked_color) = self.colors.1 {
                                self.back_rect.set_color_uniform(vec4(clicked_color.x, clicked_color.y, clicked_color.z, 1.0));
                            } else {
                                match &self.textures.1 {
                                    Some(clicked_texture) => {
                                        self.back_rect.set_texture(clicked_texture, display);
                                    }
                                    None => {}
                                }
                            }
                            self.back_rect.set_color_uniform(vec4(0.6, 0.6, 0.6, 1.0));
                            self.clicked = true;
                        }
                    }
                    true
                }
                (ElementState::Released, MouseButton::Left) => {
                    if self.clicked {
                        if let Some(texture) = &self.textures.0 {
                            self.back_rect.set_texture(texture, display);
                            self.back_rect.set_color_uniform(vec4(1.0, 1.0, 1.0, 1.0));
                        } else {
                            let color = self.colors.0;
                            self.back_rect.set_color_uniform(vec4(color.x, color.y, color.z, 1.0));
                        }
                        self.clicked = false;
                    }
                    true
                }
                _ => {false}
            }
            _ => {false}
        }
    }
    
    pub fn clicked(&mut self) -> bool {
        if self.clicked && !self.previous_clicked {
            self.previous_clicked = true;
            true
        } else if self.clicked && self.previous_clicked {
            false
        } else {
            self.previous_clicked = false;
            false
        }
    }

    pub fn is_held(&self) -> bool {
        self.clicked
    }

    pub fn left(&self) -> f32 {
        self.back_rect.left()
    }

    pub fn set_left(&mut self, left: f32) {
        self.back_rect.set_left(left);

        if let Some(textline) = &mut self.textline {
            textline.set_left(left);
        } else if let Some(multi_textline) = &mut self.multi_textline {
            multi_textline.set_left(left);
        }
    }

    pub fn top(&self) -> f32 {
        self.back_rect.top()
    }

    pub fn set_top(&mut self, top: f32) {
        self.back_rect.set_top(top);

        if let Some(textline) = &mut self.textline {
            textline.set_top(top);
        } else if let Some(multi_textline) = &mut self.multi_textline {
            multi_textline.set_top(top);
        }
    }

    pub fn right(&self) -> f32 {
        self.back_rect.right()
    }

    pub fn set_right(&mut self, right: f32) {
        self.back_rect.set_right(right);

        if let Some(textline) = &mut self.textline {
            textline.set_right(right);
        } else if let Some(multi_textline) = &mut self.multi_textline {
            multi_textline.set_right(right);
        }
    }

    pub fn bottom(&self) -> f32 {
        self.back_rect.bottom()
    }

    pub fn set_bottom(&mut self, bottom: f32) {
        self.back_rect.set_bottom(bottom);

        if let Some(textline) = &mut self.textline {
            textline.set_bottom(bottom);
        } else if let Some(multi_textline) = &mut self.multi_textline {
            multi_textline.set_bottom(bottom);
        }
    }

    pub fn width(&self) -> f32 {
        self.back_rect.width()
    }

    pub fn height(&self) -> f32 {
        self.back_rect.height()
    }

    pub fn draw(&mut self, target: &mut Frame, display: &Display) {
        self.back_rect.draw(target);
        if let Some(textline) = &mut self.textline {
            textline.draw(Some(target), display);
        }

        if let Some(multi_textline) = &mut self.multi_textline {
            multi_textline.draw(target, display);
        }
    }
}

/// If `text` contains '\n' then builder is gonna create `MultiTextLine` instead of `TextLine`
pub struct ButtonBuilder {
    pub position: Vec2<f32>,
    pub size: Vec2<f32>,
    pub text: Option<String>,
    pub font_size: Option<f32>,
    pub text_color: Option<Vec3<f32>>,
    /// 0 - normal color, 1 - clicked color
    /// 
    /// Just darkens button if 1 aint set
    pub colors: (Vec3<f32>, Option<Vec3<f32>>),
    /// 0 - normal color, 1 - clicked color
    /// 
    /// If 0 has texture but 1 dont then button will just make texture darker
    pub textures: (Option<DynamicImage>, Option<DynamicImage>),
}

impl ButtonBuilder {
    pub fn build(self, display: &Display) -> Button {
        let font_size = if let Some(font_size) = &self.font_size {
            *font_size
        } else {
            0.0
        };

        let text_color = if let Some(text_color) = &self.text_color {
            *text_color
        } else {
            vec3(1.0, 1.0, 1.0)
        };
        
        let (text_line, multi_text_line) = match &self.text {
            Some(text) => {
                if text.contains("\n") {
                    (None, Some(MultiTextLine::new(&text, self.position, TextAlignment::Middle, display, Some(text_color), Some(font_size), true)))
                } else {
                    let mut textline = TextLine::new(text, self.position);
                    textline.set_bold(true);
                    textline.set_color(text_color);
                    textline.set_font_size(font_size);

                    (Some(textline), None)
                }
            }
            None => {(None, None)}
        };

        let mut rect = Rect::new(self.position, self.size, display);
        if let Some(texture) = &self.textures.0 {
            rect.set_texture(texture, display);
        } else {
            let color = self.colors.0;
            rect.set_color(vec4(color.x, color.y, color.z, 1.0), display)
        }

        Button {
            textline: text_line,
            multi_textline: multi_text_line,
            position: self.position,
            size: self.size,
            colors: self.colors,
            textures: self.textures,
            back_rect: rect,
            clicked: false,
            previous_clicked: false
        }
    }
}

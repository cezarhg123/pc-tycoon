use glium::{Display, Frame};

use crate::math::{vec2::{Vec2, vec2}, vec3::{Vec3, vec3}};

use super::textline::TextLine;

#[derive(Debug)]
pub enum TextAlignment {
    Left,
    Middle,
    Right
}

#[derive(Debug)]
pub struct MultiTextLine {
    text_lines: Vec<TextLine>,
    text_alignment: TextAlignment,
    position: Vec2<f32>,
    size: Vec2<f32>
}

impl MultiTextLine {
    /// Split lines with '\n'.
    pub fn new(text: &str, position: Vec2<f32>, text_alignment: TextAlignment, display: &Display, color: Option<Vec3<f32>>, font_size: Option<f32>, is_bold: bool) -> MultiTextLine {
        let color = match color {
            Some(color) => color,
            None => vec3(1.0, 1.0, 1.0)
        };

        let font_size = match font_size {
            Some(font_size) => font_size,
            None => 12.0
        };
        
        let mut text_lines: Vec<TextLine> = text.split("\n").map(|t| { 
            let mut textline = TextLine::new(t, vec2(0.0, 0.0));
            textline.set_color(color);
            textline.set_bold(is_bold);
            textline.set_font_size(font_size);
            textline.draw(None, display);

            textline
        }).collect();
        
        text_lines.reverse();

        let mut biggest_width = 0.0;
        for textline in &text_lines {
            if textline.width() > biggest_width {
                biggest_width = textline.width();
            }
        }
        
        let single_height = match text_lines.first() {
            Some(textline) => {
                textline.height()
            }
            None => 0.0
        };

        let size = vec2(biggest_width, single_height * text_lines.len() as f32);

        let mut vertical_offset = 0.0;
        match text_alignment {
            TextAlignment::Left => {
                for textline in &mut text_lines {
                    textline.set_left(position.x - size.x);
                    textline.set_top(position.y + size.y + vertical_offset);
                    vertical_offset += single_height;
                }
            }
            TextAlignment::Middle => {
                for textline in &mut text_lines {
                    textline.set_centre(vec2(position.x, position.y + size.y + vertical_offset));
                    vertical_offset += single_height;
                }
            }
            TextAlignment::Right => {
                for textline in &mut text_lines {
                    textline.set_right(position.x + size.x);
                    textline.set_top(position.y + size.y + vertical_offset);
                    vertical_offset += single_height;
                }
            }
        }

        MultiTextLine {
            text_lines,
            text_alignment,
            position,
            size
        }
    }

    pub fn left(&self) -> f32 {
        self.position.x - self.size.x
    }

    pub fn set_left(&mut self, left: f32) {
        let diff = left - self.left();
        self.position.x = left + (self.size.x / 2.0);

        self.text_lines.iter_mut().for_each(|textline| textline.set_left(textline.left() + diff));
    }

    pub fn top(&self) -> f32 {
        self.position.y + (self.size.y / 2.0)
    }

    pub fn set_top(&mut self, top: f32) {
        let diff = top - self.top();
        self.position.y = top - (self.size.y / 2.0);

        self.text_lines.iter_mut().for_each(|textline| textline.set_top(textline.top() + diff));
    }

    pub fn right(&self) -> f32 {
        self.position.x + (self.size.x / 2.0)
    }

    pub fn set_right(&mut self, right: f32) {
        let diff = right - self.right();
        self.position.x = right - (self.size.x / 2.0);

        self.text_lines.iter_mut().for_each(|textline| textline.set_right(textline.right() + diff));
    }

    pub fn bottom(&self) -> f32 {
        self.position.y - (self.size.y / 2.0)
    }

    pub fn set_bottom(&mut self, bottom: f32) {
        let diff = bottom - self.bottom();
        self.position.y = bottom + (self.size.y / 2.0);

        self.text_lines.iter_mut().for_each(|textline| textline.set_bottom(textline.bottom() + diff));
    }

    pub fn centre(&self) -> Vec2<f32> {
        self.position
    }

    pub fn set_centre(&mut self, centre: Vec2<f32>) {
        let diff = centre - self.centre();
        self.position = centre;

        self.text_lines.iter_mut().for_each(|textline| textline.set_centre(textline.centre() + diff));
    }

    pub fn width(&self) -> f32 {
        self.size.x
    }

    pub fn height(&self) -> f32 {
        self.size.y
    }

    pub fn draw(&mut self, target: &mut Frame, display: &Display) {
        for textline in &mut self.text_lines {
            textline.draw(Some(target), display);
        }
    }
}

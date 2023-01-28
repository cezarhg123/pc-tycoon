use glium::{Frame, Display};
use image::{DynamicImage, Rgba, ImageBuffer, imageops::FilterType};
use rusttype::{Scale, point};

use crate::{gfx::Rect, math::{vec4::{Vec4, vec4}, vec2::{Vec2, vec2}, vec3::{vec3, Vec3}}};

use super::{get_global_font, get_global_bold_font};

#[derive(Debug)]
pub struct TextLine {
    is_bold: bool,
    text: String,
    font_size: f32,
    position: Vec2<f32>,
    color: Vec3<f32>,
    rect: Option<Rect>,
    bitmap: Option<DynamicImage>
}

impl TextLine {
    /// `position` is the centre of texline rect. 
    /// 
    /// Default color is white. 
    /// 
    /// Default size is 12.0px.
    /// 
    /// Isnt bold by default.
    /// 
    /// CALL `DRAW` ONCE RIGHT AFTER CREATING, SETTING COLOR OR FONT SIZE OR RESETTING TEXT TO ACTUALLY CHANGE/GET ITS POSITION
    pub fn new(text: impl ToString, position: Vec2<f32>) -> TextLine {
        let text = text.to_string();

        TextLine {
            is_bold: false,
            text,
            position,
            font_size: 12.0,
            color: vec3(1.0, 1.0, 1.0),
            rect: None,
            bitmap: None
        }
    }

    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
        self.rect = None;
        self.bitmap = None;
    }

    pub fn set_color(&mut self, color: Vec3<f32>) {
        self.color = color;
        self.rect = None;
        self.bitmap = None;
    }

    pub fn set_text(&mut self, text: impl ToString) {
        self.text = text.to_string();
        self.rect = None;
        self.bitmap = None;
    }

    pub fn set_bold(&mut self, bold: bool) {
        self.is_bold = bold;
        self.rect = None;
        self.bitmap = None;
    }

    pub fn left(&self) -> f32 {
        self.position.x - (self.rect.as_ref().unwrap().width() / 2.0)
    }

    pub fn set_left(&mut self, left: f32) {
        self.position.x = left + (self.rect.as_ref().unwrap().width() / 2.0);
    }

    pub fn top(&self) -> f32 {
        self.position.y + (self.rect.as_ref().unwrap().height() / 2.0)
    }

    pub fn set_top(&mut self, top: f32) {
        self.position.y = top - (self.rect.as_ref().unwrap().height() / 2.0)
    }

    pub fn right(&self) -> f32 {
        self.position.x + (self.rect.as_ref().unwrap().width() / 2.0)
    }

    pub fn set_right(&mut self, right: f32) {
        self.position.x = right - (self.rect.as_ref().unwrap().width() / 2.0);
    }

    pub fn bottom(&self) -> f32 {
        self.position.y - (self.rect.as_ref().unwrap().height() / 2.0)
    }

    pub fn set_bottom(&mut self, bottom: f32) {
        self.position.y = bottom + (self.rect.as_ref().unwrap().height() / 2.0);
    }

    pub fn centre(&self) -> Vec2<f32> {
        self.position
    }

    pub fn set_centre(&mut self, centre: Vec2<f32>) {
        self.position = centre;
    }

    pub fn width(&self) -> f32 {
        self.rect.as_ref().unwrap().width()
    }

    pub fn height(&self) -> f32 {
        self.rect.as_ref().unwrap().height()
    }

    /// set `target` to `None` if you only want to create 'drawable' objects instead of actually drawing to screen
    pub fn draw(&mut self, mut target: Option<&mut Frame>, display: &Display) {
        match (&self.bitmap, &mut self.rect, &mut target) {
            (Some(bitmap), Some(rect), Some(target)) => {
                rect.set_centre(self.position);
                rect.draw(*target);
            }
            _ => {
                let font = match self.is_bold {
                    true => get_global_bold_font(),
                    false => get_global_font()
                };

                let scale = Scale::uniform(self.font_size);
                let color = self.color * 255.0;
                let v_metrics = font.v_metrics(scale);
                let glyphs: Vec<_> = font
                    .layout(&self.text, scale, point(0.0, v_metrics.ascent))
                    .collect();
                
                let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
                let glyphs_width = {
                    let min_x = glyphs
                        .first()
                        .map(|g| g.pixel_bounding_box().unwrap().min.x)
                        .unwrap();
                    let max_x = glyphs
                        .last()
                        .map(|g| g.pixel_bounding_box().unwrap().max.x)
                        .unwrap();
                    (max_x - min_x) as u32
                };

                let mut biggest_x = 0;
                let mut biggest_y = 0;
                for glyph in &glyphs {
                    if let Some(bounding_box) = glyph.pixel_bounding_box() {
                        glyph.draw(|x, y, _| {
                            let x = x + bounding_box.min.x as u32;
                            let y = y + bounding_box.min.y as u32;
                            if x > biggest_x {
                                biggest_x = x;
                            }

                            if y > biggest_y {
                                biggest_y = y;
                            }
                        });
                    }
                }

                let mut bitmap = DynamicImage::new_rgba8(biggest_x + 1, glyphs_height + 1).into_rgba8();
                
                for glyph in glyphs {
                    if let Some(bounding_box) = glyph.pixel_bounding_box() {
                        // Draw the glyph into the image per-pixel by using the draw closure
                        glyph.draw(|x, y, v| {
                            bitmap.put_pixel(
                                // Offset the position by the glyph bounding box
                                x + bounding_box.min.x as u32,
                                y + bounding_box.min.y as u32,
                                // Turn the coverage into an alpha value
                                Rgba([color.x as u8, color.y as u8, color.z as u8, (v * 255.0) as u8]),
                            )
                        });
                    }
                }

                let mut bitmap = DynamicImage::ImageRgba8(bitmap);

                let mut rect = Rect::new(self.position, vec2(bitmap.width() as f32, bitmap.height() as f32), display);
                rect.set_texture(&bitmap, display);
                rect.set_centre(self.position);

                self.rect = Some(rect);
                self.bitmap = Some(bitmap);

                match target {
                    Some(target) => self.rect.as_ref().unwrap().draw(target),
                    None => {}
                }
            }
        }
    }
}

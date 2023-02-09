use std::{borrow::BorrowMut, cell::{Ref, RefCell}, rc::Rc};
use glium::{texture::{SrgbTexture2d, RawImage2d}, Display};
use image::{DynamicImage, Rgba, GenericImageView};
use rusttype::{Scale, point};
use crate::{math::{vec3::{Vec3, vec3}, vec2::{Vec2, vec2}, vec4::vec4}, gfx::rect::{Rect, RectBuilder}};
use super::{uielement::{UiElement, UiOutput}, get_global_bold_font, get_global_font};

pub struct TextLine {
    output: UiOutput,
    text: String,
    font_size: f32,
    color: Vec3<f32>,
    bold: bool,
    rect: Rect,
    bitmap: SrgbTexture2d
}

impl UiElement for TextLine {
    fn handle_event(&mut self, event: &glium::glutin::event::WindowEvent) -> bool {false}
    
    fn output(&self) -> super::uielement::UiOutput {
        self.output
    }

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

    fn centre(&self) -> Vec2<f32> {
        self.rect.centre()
    }

    fn set_centre(&mut self, centre: Vec2<f32>) {
        self.rect.set_centre(centre);
    }

    fn draw(&self, target: &mut glium::Frame) {
        self.rect.draw(target);
    }
}

pub struct TextLineBuilder {
    pub text: String,
    pub font_size: f32,
    pub color: Vec3<f32>,
    pub bold: bool,
    pub position: Vec2<f32>,
}

impl Default for TextLineBuilder {
    fn default() -> TextLineBuilder {
        TextLineBuilder {
            text: "Default".to_string(),
            font_size: 12.0,
            color: vec3(1.0, 1.0, 1.0),
            bold: false,
            position: vec2(0.0, 0.0)
        }
    }
}

impl TextLineBuilder {
    pub fn build(self, display: &Display) -> TextLine {
        let font = match self.bold {
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
                        x + bounding_box.min.x as u32 - 1,
                        y + bounding_box.min.y as u32,
                        // Turn the coverage into an alpha value
                        Rgba([color.x as u8, color.y as u8, color.z as u8, (v * 255.0) as u8]),
                    )
                });
            }
        }

        let mut bitmap = DynamicImage::ImageRgba8(bitmap);

        let rect = RectBuilder {
            position: self.position,
            size: vec2(bitmap.width() as f32, bitmap.height() as f32),
            color: vec4(1.0, 1.0, 1.0, 1.0),
            texture: Some(bitmap.clone())
        }.build(display);
        
        let bitmap = RawImage2d::from_raw_rgba_reversed(bitmap.as_bytes(), bitmap.dimensions());
        let bitmap = SrgbTexture2d::new(display, bitmap).unwrap();

        TextLine {
            text: self.text,
            font_size: self.font_size,
            color: self.color,
            bold: self.bold,
            bitmap,
            rect,
            output: UiOutput::None
        }
    }
}

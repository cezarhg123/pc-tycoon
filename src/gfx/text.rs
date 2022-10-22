use image::{DynamicImage, Rgba};
use rusttype::{Font, Scale, point};

use super::{vectors::{vec3::Vec3, vec2::Vec2}, image_rect::ImageRect, texture::Texture};

#[derive(Debug, Clone)]
pub struct Text<'a> {
    font: &'a Font<'a>,
    size: f32,
    color: Vec3<u8>,
    rect: ImageRect
}

impl<'a> Text<'a> {
    pub fn new(text: &str, font: &'a Font, size: f32, color: Vec3<u8>, position: Vec2<f32>) -> Text<'a> {
        let scale = Scale::uniform(size);
        let v_metrics = font.v_metrics(scale);

        let glyphs: Vec<_> = font
            .layout(text, scale, point(0.0, v_metrics.ascent))
            .collect();
        
        let width = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let height = {
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

        let mut bitmap = DynamicImage::new_rgba8(width + (size / 2.0) as u32, height + (size / 2.0) as u32).into_rgba8();
        
        for glyph in &glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    bitmap.put_pixel(
                        // Offset the position by the glyph bounding box
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        // Turn the coverage into an alpha value
                        Rgba([color.x, color.y, color.z, (v * 255.0) as u8]),
                    )
                });
            }
        };

        //turning bitmap into dynamic image to flip in vertically
        let bitmap = DynamicImage::ImageRgba8(bitmap).flipv().into_rgba8();

        Text {
            font,
            size,
            color,
            rect: ImageRect::new(Texture::from_memory(bitmap), position.x, position.y, width as f32 + (size / 2.0), height as f32 + (size / 2.0))
        }
    }

    pub fn draw(&self) {
        self.rect.draw();
    }
}

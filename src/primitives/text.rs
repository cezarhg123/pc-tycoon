use std::ops::{Deref, DerefMut};
use ab_glyph::*;
use gpu_allocator::vulkan::Allocator;
use image::{DynamicImage, GenericImage, Rgba, GenericImageView};
use super::{rect::Rect, get_font};

pub struct Text {
    rect: Rect,
    text: String,
    font_size: f32,
    font_color: glm::Vec3
}

impl Text {
    pub fn builder() -> TextBuilder {
        TextBuilder {
            left: 0.0,
            top: 0.0,
            text: String::new(),
            font_size: 12.0,
            font_color: glm::vec3(1.0, 1.0, 1.0)
        }
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn rect_mut(&mut self) -> &mut Rect {
        &mut self.rect
    }

    pub fn create_image(font_size: f32, text: String, font_color: glm::Vec3) -> image::DynamicImage {
        let scale = PxScale::from(font_size);
        let scaled_font = get_font().as_scaled(scale);
        let mut caret = point(0.0, scaled_font.ascent());

        let mut glyphs = Vec::new();
        let mut last_glyph: Option<Glyph> = None;

        for c in text.chars() {
            let mut glyph = scaled_font.scaled_glyph(c);

            if let Some(last) = last_glyph.take() {
                caret.x += scaled_font.kern(last.id, glyph.id);
            }
            glyph.position = caret;

            last_glyph = Some(glyph.clone());
            caret.x += scaled_font.h_advance(glyph.id);

            glyphs.push(glyph);
        }

        let glyphs_width = {
            let min_x = glyphs.first().unwrap().position.x;
            let last_glyph = glyphs.last().unwrap();
            let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);

            (max_x - min_x).ceil() as u32
        };

        let glyphs_height = scaled_font.height().ceil() as u32;

        let mut image = DynamicImage::new_rgba8(glyphs_width * 2, glyphs_height * 2);

        for glyph in glyphs {
            if let Some(outlined) = scaled_font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                outlined.draw(|x, y, v| {
                    image.put_pixel(x + bounds.min.x as u32, y + bounds.min.y as u32, Rgba([(font_color.x * 255.0) as u8, (font_color.y * 255.0) as u8, (font_color.z * 255.0) as u8, (v * 255.0) as u8]));
                });
            }
        }

        // find how much of the image is actually text(not transparent) and create a new image with only the text
        // i gotta do this because every fucking font rasterizer i use shits out a glyphs_width too small
        
        let mut max_x = 0;
        let mut max_y = 0;
        
        for x in 0..image.width() {
            for y in 0..image.height() {
                let pixel = image.get_pixel(x, y).clone();

                if pixel[3] > 0 {
                    if x > max_x {
                        max_x = x;
                    }
                    if y > max_y {
                        max_y = y;
                    }
                }
            }
        }
        
        image.crop_imm(0, 0, max_x, max_y)
    }

    pub fn draw(&self) {
        self.rect.draw();
    }
}

pub struct TextBuilder {
    left: f32,
    top: f32,
    text: String,
    font_size: f32,
    font_color: glm::Vec3
}

// doesnt have other helper methods like `right` or `bottom`, just create it and set it later
impl TextBuilder {
    pub fn left(mut self, left: f32) -> TextBuilder {
        self.left = left;
        self
    }

    pub fn top(mut self, top: f32) -> TextBuilder {
        self.top = top;
        self
    }

    pub fn text(mut self, text: impl ToString) -> TextBuilder {
        self.text = text.to_string();
        self
    }

    pub fn font_size(mut self, font_size: f32) -> TextBuilder {
        self.font_size = font_size;
        self
    }

    pub fn font_color(mut self, font_color: glm::Vec3) -> TextBuilder {
        self.font_color = font_color;
        self
    }

    pub fn build(self, allocator: &mut Allocator) -> Text {
        let image = Text::create_image(self.font_size, self.text.clone(), self.font_color);

        let rect = Rect::builder()
            .left(self.left)
            .top(self.top)
            .width(image.width() as f32)
            .height(image.height() as f32)
            .name(self.text.clone())
            .texture(image)
            .build(allocator);

        Text {
            rect,
            text: self.text,
            font_size: self.font_size,
            font_color: self.font_color
        }
    }
}

impl Deref for Text {
    type Target = Rect;

    fn deref(&self) -> &Self::Target {
        &self.rect
    }
}

impl DerefMut for Text {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rect
    }
}

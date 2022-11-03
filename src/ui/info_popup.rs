use crate::gfx::{text::Text, color_rect::ColorRect, vectors::{vec2::{Vec2, vec2}, vec3::vec3}};

use super::Ui;

#[derive(Debug, Clone)]
pub struct InfoPopup<'a> {
    pub id: String,
    texts: Vec<Text<'a>>,
    rect: ColorRect
}

impl<'a> InfoPopup<'a> {
    pub fn new(id: &str, texts: &[String], pos: Vec2<f32>, text_size: f32, ui: &'a Ui) -> InfoPopup<'a> {
        let mut texts_ui = Vec::new();
        
        let mut biggest_text = vec2(0.0, 0.0);

        let mut i = 0;
        for text in texts {
            let current_pos = vec2(pos.x + (text_size / 4.0), pos.y + (text_size * i as f32));
            let text = ui.text(text.as_str(), text_size, vec3(255, 255, 255), Some(current_pos));
            
            if text.get_width() > biggest_text.x || text.get_height() > biggest_text.y {
                biggest_text = vec2(text.get_width() + (text_size / 2.0), text.get_height());
            }
            
            texts_ui.push(text);
            i += 1;
        }

        InfoPopup {
            id: id.to_string(),
            rect: ColorRect::new(vec3(0.0, 0.0, 0.0), pos.x, pos.y, biggest_text.x, texts_ui.len() as f32 * text_size),
            texts: texts_ui,
        }
    }

    pub fn draw(&self) {
        self.rect.draw();
        for text in &self.texts {
            text.draw();
        }
    }
}

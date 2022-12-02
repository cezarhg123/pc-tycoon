use super::{color_rect::ColorRect, image_rect::ImageRect};

#[derive(Debug, Clone)]
pub enum Rect {
    Color(ColorRect),
    Image(ImageRect)
}

impl Rect {
    pub fn draw(&self) {
        match self {
            Rect::Color(rect) => {
                rect.draw();
            }
            Rect::Image(rect) => {
                rect.draw();
            }
        }
    }
}

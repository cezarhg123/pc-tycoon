use super::{rect::Rect, text::Text};

#[derive(Debug, Clone)]
pub struct Button {
    rect: Rect,
    text: Option<Text>
}

impl Button {
    pub fn new(position: [i32; 2], size: [i32; 2], sprite: &str, text: Option<&str>) -> Self {
        let rect = Rect::new(position[0], position[1], size[0], size[1], sprite);
        
        let text = if text.is_some() {
            let mut text2 = Text::new(text.unwrap(), position, (size[1] / 4) as f32, false);
            let (x_center, y_center) = (text2.get_center()[0], text2.get_center()[1]);
            let (x_center_rect, y_center_rect) = (rect.get_center()[0], rect.get_center()[1]);
            let x_dif = x_center_rect - x_center;
            let y_dif = y_center_rect - y_center;

            text2.set_left(text2.get_left() + x_dif);
            text2.set_top(text2.get_top() - y_dif);

            Some(text2)
        } else {
            None
        };
        
        Button {
            rect,
            text
        }
    }

    pub fn draw(&self) {
        self.rect.draw();
        if self.text.is_some() {
            self.text.as_ref().unwrap().draw();
        }
    }
}

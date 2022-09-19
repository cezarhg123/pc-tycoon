use super::rect::Rect;

#[derive(Debug, Clone)]
pub struct Text {
    size: f32,
    rect: Rect,
    uvs: Vec<[f32; 2]>
}

impl Text {
    pub fn new(text: &str, position: [i32; 2], size: f32, black: bool) -> Self {
        let uvs = parse_string(text);
        
        let image_path = if black {
            "textures/character-map-black.png"
        } else {
            "textures/character-map-white.png"
        };

        let mut rect = Rect::new(position[0], position[1] + size as i32, uvs.len() as i32 * size as i32, size as i32, image_path);
        rect.split_quad(uvs.as_slice());
        
        Text {
            size,
            rect,
            uvs
        }
    }

    pub fn get_center(&self) -> [i32; 2] {
        self.rect.get_center()
    }

    pub fn get_left(&self) -> i32 {
        self.rect.get_left()
    }

    pub fn set_left(&mut self, left: i32) {
        self.rect.set_left(left);
        self.rect.split_quad(self.uvs.as_slice())
    }

    pub fn get_top(&self) -> i32 {
        self.rect.get_top()
    }

    pub fn set_top(&mut self, top: i32) {
        self.rect.set_top(top);
        self.rect.split_quad(self.uvs.as_slice())
    }

    pub fn get_width(&self) -> i32 {
        self.rect.get_width()
    }

    pub fn set_width(&mut self, width: i32) {
        self.rect.set_width(width);
        self.rect.split_quad(self.uvs.as_slice())
    }

    pub fn get_height(&self) -> i32 {
        self.rect.get_height()
    }

    pub fn set_height(&mut self, height: i32) {
        self.rect.set_height(height);
        self.rect.split_quad(self.uvs.as_slice())
    }

    pub fn draw(&self) {
        self.rect.draw();
    }
}

fn parse_string(text: &str) -> Vec<[f32; 2]> {
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    for c in text.to_lowercase().chars() {
        match c {
            'a' => {
                uvs.push([0.0, 0.0]);
            }
            'b' => {
                uvs.push([0.1, 0.0]);
            }
            'c' => {
                uvs.push([0.2, 0.0]);
            }
            'd' => {
                uvs.push([0.3, 0.0]);
            }
            'e' => {
                uvs.push([0.4, 0.0]);
            }
            'f' => {
                uvs.push([0.5, 0.0]);
            }
            'g' => {
                uvs.push([0.6, 0.0]);
            }
            'h' => {
                uvs.push([0.7, 0.0]);
            }
            'i' => {
                uvs.push([0.8, 0.0]);
            }
            'j' => {
                uvs.push([0.9, 0.0]);
            }
            'k' => {
                uvs.push([0.0, 0.2]);
            }
            'l' => {
                uvs.push([0.1, 0.2]);
            }
            'm' => {
                uvs.push([0.2, 0.2]);
            }
            'n' => {
                uvs.push([0.3, 0.2]);
            }
            'o' => {
                uvs.push([0.4, 0.2]);
            }
            'p' => {
                uvs.push([0.5, 0.2]);
            }
            'q' => {
                uvs.push([0.6, 0.2]);
            }
            'r' => {
                uvs.push([0.7, 0.2]);
            }
            's' => {
                uvs.push([0.8, 0.2]);
            }
            't' => {
                uvs.push([0.9, 0.2]);
            }
            'u' => {
                uvs.push([0.0, 0.4]);
            }
            'v' => {
                uvs.push([0.1, 0.4]);
            }
            'w' => {
                uvs.push([0.2, 0.4]);
            }
            'x' => {
                uvs.push([0.3, 0.4]);
            }
            'y' => {
                uvs.push([0.4, 0.4]);
            }
            'z' => {
                uvs.push([0.5, 0.4]);
            }
            '?' => {
                uvs.push([0.6, 0.4]);
            }
            '!' => {
                uvs.push([0.7, 0.4]);
            }
            '.' => {
                uvs.push([0.8, 0.4]);
            }
            ' ' => {
                uvs.push([0.9, 0.4]);
            }
            '1' => {
                uvs.push([0.0, 0.6]);
            }
            '2' => {
                uvs.push([0.1, 0.6]);
            }
            '3' => {
                uvs.push([0.2, 0.6]);
            }
            '4' => {
                uvs.push([0.3, 0.6]);
            }
            '5' => {
                uvs.push([0.4, 0.6]);
            }
            '6' => {
                uvs.push([0.5, 0.6]);
            }
            '7' => {
                uvs.push([0.6, 0.6]);
            }
            '8' => {
                uvs.push([0.7, 0.6]);
            }
            '9' => {
                uvs.push([0.8, 0.6]);
            }
            '0' => {
                uvs.push([0.9, 0.6]);
            }
            _ => {}
        }
    }

    uvs
}

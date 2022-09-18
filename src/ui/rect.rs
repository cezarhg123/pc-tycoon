use crate::{drawable::{Drawable, Vertex}, WINDOW_WIDTH, WINDOW_HEIGHT};

#[derive(Debug, Clone)]
pub struct Rect {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
    drawable: Drawable
}

impl Rect {
    pub fn new(left: i32, top: i32, width: i32, height: i32, image_path: &str) -> Self {
        let drawable = Drawable::new(image_path, "shaders/default.vert", "shaders/default.frag");
        let mut rect = Rect {
            left,
            top,
            width,
            height,
            drawable: drawable
        };

        rect.update_quad();
        
        rect
    }

    fn update_quad(&mut self) {
        let bottom_left = Vertex {
            position: px_to_float(self.left, self.top + self.height),
            uv: [0.0, 0.0]  
        };

        let top_left = Vertex {
            position: px_to_float(self.left, self.top),
            uv: [0.0, 1.0]
        };

        let top_right = Vertex {
            position: px_to_float(self.left + self.width, self.top),
            uv: [1.0, 1.0]
        };

        let bottom_right = Vertex {
            position: px_to_float(self.left + self.width, self.top + self.height),
            uv: [1.0, 0.0]
        };

        self. drawable.set_vbo(&vec![
            bottom_left,
            top_left,
            top_right,

            bottom_left,
            bottom_right,
            top_right
        ]);
    }

    
    /// FOR TEXT UI ONLY
    pub fn split_quad(&mut self, uvs: &[[f32; 2]]) {
        let len = uvs.len();
        let mut vertices: Vec<Vertex> = Vec::new();
        let size = self.width / len as i32;

        for i in 0..len {
            let x = self.left + (i as i32 * size);
            let y = self.top + self.height;

            let uv = &uvs[i];

            vertices.append(&mut vec![
                Vertex {
                    position: px_to_float(x, y),
                    uv: uv.clone()
                },
                Vertex {
                    position: px_to_float(x + size, y - size),
                    uv: [uv[0] + 0.1, uv[1] + 0.2]
                },
                Vertex {
                    position: px_to_float(x, y - size),
                    uv: [uv[0], uv[1] + 0.2]
                },
                Vertex {
                    position: px_to_float(x, y),
                    uv: uv.clone()
                },
                Vertex {
                    position: px_to_float(x + size, y),
                    uv: [uv[0] + 0.1, uv[1]]
                },
                Vertex {
                    position: px_to_float(x + size, y - size),
                    uv: [uv[0] + 0.1, uv[1] + 0.2]
                }
            ]);
        }
        self.drawable.set_vbo(&vertices);
    }

    pub fn draw(&self) {
        self.drawable.draw();
    }

    pub fn get_left(&self) -> i32 {
        self.left
    }

    pub fn set_left(&mut self, left: i32) {
        self.left = left;
        self.update_quad();
    }

    pub fn get_top(&self) -> i32 {
        self.top
    }

    pub fn set_top(&mut self, top: i32) {
        self.top = top;
        self.update_quad();
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn set_width(&mut self, width: i32) {
        self.width = width;
        self.update_quad();
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn set_height(&mut self, height: i32) {
        self.height = height;
        self.update_quad();
    }
}

fn px_to_float(x: i32, y: i32) -> [f32; 2] {    
    let mut x = (x * 2) as f32 / WINDOW_WIDTH as f32;
    x -= 1.0;

    let mut y = (y * 2) as f32 / WINDOW_HEIGHT as f32;
    y -= 1.0;
    y *= -1.0;

    [x, y]
}

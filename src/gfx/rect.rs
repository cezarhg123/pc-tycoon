use glium::{texture::{SrgbTexture2d, RawImage2d}, VertexBuffer, IndexBuffer, Program, Display, Frame, uniform, Surface, DrawParameters, Blend};
use image::{DynamicImage, GenericImageView};

use crate::{math::{vec2::{Vec2, vec2}, vec4::{Vec4, vec4}}, get_window_width, get_window_height};

use super::{Vertex, vertex, WHITE_SQUARE_BYTES, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC};

#[derive(Debug)]
pub struct Rect {
    position: Vec2<f32>,
    size: Vec2<f32>,
    color: Vec4<f32>,
    texture: SrgbTexture2d,
    vbo: VertexBuffer<Vertex>,
    ebo: IndexBuffer<u32>,
    shader: Program
}

impl Rect {
    pub fn set_color(&mut self, color: Vec4<f32>) {
        self.color = color;
    }

    pub fn set_texture(&mut self, texture: DynamicImage, display: &Display) {
        let raw_texture = RawImage2d::from_raw_rgba_reversed(texture.as_bytes(), texture.dimensions());
        let texture = SrgbTexture2d::new(display, raw_texture).unwrap();

        self.texture = texture;
    }

    pub fn left(&self) -> f32 {
        self.position.x - (self.size.x / 2.0)
    }

    pub fn set_left(&mut self, left: f32) {
        self.position.x = left + (self.size.x / 2.0);
    }

    pub fn top(&self) -> f32 {
        self.position.y + (self.size.y / 2.0)
    }

    pub fn set_top(&mut self, top: f32) {
        self.position.y = top - (self.size.y / 2.0);
    }

    pub fn right(&self) -> f32 {
        self.position.x + (self.size.x / 2.0)
    }

    pub fn set_right(&mut self, right: f32) {
        self.position.x = right - (self.size.x / 2.0);
    }

    pub fn bottom(&self) -> f32 {
        self.position.y - (self.size.y / 2.0)
    }

    pub fn set_bottom(&mut self, bottom: f32) {
        self.position.y = bottom + (self.size.y / 2.0);
    }

    pub fn width(&self) -> f32 {
        self.size.x
    }

    pub fn set_width(&mut self, width: f32) {
        self.size.x = width;
    }

    pub fn height(&self) -> f32 {
        self.size.y
    }

    pub fn set_height(&mut self, height: f32) {
        self.size.y = height;
    }

    pub fn centre(&self) -> Vec2<f32> {
        self.position
    }

    pub fn set_centre(&mut self, centre: Vec2<f32>) {
        self.position = centre;
    }

    pub fn contains(&self, pos: Vec2<f32>) -> bool {
        if self.left() < pos.x && self.right() > pos.x {
            if self.top() > pos.y && self.bottom() < pos.y {
                return true;
            }
        }

        false
    }

    pub fn draw(&self, target: &mut Frame) {
        let uniforms = uniform! {
            offset: ((self.position * 2.0) / vec2(get_window_width() as f32, get_window_height() as f32)).as_raw(),
            tex: &self.texture,
            color: self.color.as_raw(),
            size: (self.size / vec2(get_window_width() as f32, get_window_height() as f32)).as_raw()
        };

        target.draw(&self.vbo, &self.ebo, &self.shader, &uniforms, &DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        }).unwrap();
    }
}

#[derive(Debug)]
pub struct RectBuilder {
    pub position: Vec2<f32>,
    pub size: Vec2<f32>,
    pub color: Vec4<f32>,
    pub texture: Option<DynamicImage>
}

impl Default for RectBuilder {
    fn default() -> RectBuilder {
        RectBuilder {
            position: vec2(0.0, 0.0),
            size: vec2(100.0, 100.0),
            color: vec4(1.0, 1.0, 1.0, 1.0),
            texture: None
        }
    }
}

impl RectBuilder {
    pub fn build(self, display: &Display) -> Rect {
        Rect {
            position: self.position,
            size: self.size,
            color: self.color,
            texture: match self.texture { // if texture is Some then use that texture, if not then it loads a pure white image
                Some(texture) => {
                    let raw_texture = RawImage2d::from_raw_rgba_reversed(texture.as_bytes(), texture.dimensions());
                    SrgbTexture2d::new(display, raw_texture).unwrap()
                }
                None => {
                    let image = image::load_from_memory(WHITE_SQUARE_BYTES.as_slice()).unwrap();
                    let raw_texture = RawImage2d::from_raw_rgba_reversed(image.as_bytes(), image.dimensions());
                    SrgbTexture2d::new(display, raw_texture).unwrap()
                }
            },
            vbo: VertexBuffer::new(display, create_vertices(self.size).as_slice()).unwrap(),
            ebo: IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &[0, 2, 1, 0, 3, 2]).unwrap(),
            shader: Program::from_source(display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap()
        }
    }
}

fn create_vertices(size: Vec2<f32>) -> [Vertex; 4] {
    // let x = (size.x / 2.0) / get_window_width() as f32;
    // let y = (size.y / 2.0) / get_window_height() as f32;
    let x = 1.0;
    let y = 1.0;

    [
        vertex(vec2(0.0 - x, 0.0 - y), vec2(0.0, 0.0)),
        vertex(vec2(0.0 - x, 0.0 + y), vec2(0.0, 1.0)),
        vertex(vec2(0.0 + x, 0.0 + y), vec2(1.0, 1.0)),
        vertex(vec2(0.0 + x, 0.0 - y), vec2(1.0, 0.0))
    ]
}

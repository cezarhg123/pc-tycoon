use std::{io::Cursor, error::Error};
use glium::{texture::{SrgbTexture2d, RawImage2d}, VertexBuffer, IndexBuffer, Program, Display, Frame, uniform, Surface, Blend, DrawParameters};
use image::{DynamicImage, GenericImageView};
use crate::{math::{vec2::{Vec2, vec2}, vec4::{Vec4, vec4}}, get_window_width, get_window_height, DEV_WINDOW_WIDTH, DEV_WINDOW_HEIGHT};
use super::{Vertex, WHITE_SQUARE_BYTES, vertex, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC};

pub struct Rect {
    position: Vec2<f32>,
    size: Vec2<f32>,
    color: Vec4<f32>,
    texture: SrgbTexture2d,
    vbo: VertexBuffer<Vertex>,
    ebo: IndexBuffer<u32>,
    program: Program
}

impl Rect {
    pub fn centre(&self) -> Vec2<f32> {
        self.position
    }

    pub fn set_centre(&mut self, centre: Vec2<f32>) {
        self.position = centre;
    }

    pub fn size(&self) -> Vec2<f32> {
        self.size
    }

    pub fn set_size(&mut self, size: Vec2<f32>) {
        self.size = size;
    }

    pub fn left(&self) -> f32 {
        self.position.x - (self.size.x / 2.0)
    }

    pub fn set_left(&mut self, left: f32) {
        self.position.x = left + (self.size.x / 2.0);
    }

    pub fn top(&self) -> f32 {
        self.position.y  + (self.size.y / 2.0)
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

    pub fn color(&self) -> Vec4<f32> {
        self.color
    }

    pub fn set_color(&mut self, color: Vec4<f32>) {
        self.color = color;
    }

    pub fn texture(&self) -> &SrgbTexture2d {
        &self.texture
    }

    pub fn set_texture(&mut self, texture: SrgbTexture2d) {
        self.texture = texture;
    }

    pub fn contains(&self, pos: Vec2<f32>) -> bool {
        if self.left() < pos.x && self.right() > pos.x { //if pos.x is within the rect
            if self.top() > pos.y && self.bottom() < pos.y { //if pos.y is within the rect
                return true;
            }
        }

        false
    }

    pub fn draw(&self, target: &mut Frame) {
        let uniforms = uniform! {
            offset: ((self.position * 2.0) / vec2(DEV_WINDOW_WIDTH as f32, DEV_WINDOW_HEIGHT as f32)).as_raw(),
            tex: &self.texture,
            color: self.color.as_raw(),
            size: (self.size / vec2(DEV_WINDOW_WIDTH as f32, DEV_WINDOW_HEIGHT as f32)).as_raw()
        }; //create uniforms

        target.draw(&self.vbo, &self.ebo, &self.program, &uniforms, &DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        }).unwrap();
    }
}

pub struct RectBuilder {
    position: Vec2<f32>,
    size: Vec2<f32>,
    color: Vec4<f32>,
    texture: Option<DynamicImage>
}

impl RectBuilder {
    pub fn new() -> RectBuilder {
        RectBuilder {
            position: vec2(0.0, 0.0),
            size: vec2(100.0, 100.0),
            color: vec4(1.0, 1.0, 1.0, 1.0),
            texture: None
        }
    }

    pub fn with_position(mut self, position: Vec2<f32>) -> RectBuilder {
        self.position = position;
        self
    }

    pub fn with_size(mut self, size: Vec2<f32>) -> RectBuilder {
        self.size = size;
        self
    }

    pub fn with_color(mut self, color: Vec4<f32>) -> RectBuilder {
        self.color = color;
        self
    }

    pub fn with_texture_from_memory(mut self, texture: DynamicImage) -> RectBuilder {
        self.texture = Some(texture);
        self
    }

    pub fn with_texture_from_path(mut self, path: impl ToString) -> Result<RectBuilder, Box<dyn Error>> {
        let image_bytes = std::fs::read(path.to_string())?;
        let texture = image::load(Cursor::new(image_bytes), image::ImageFormat::Png)?;
        self.texture = Some(texture);
        Ok(self)
    }

    pub fn build(self, display: &Display) -> Rect {
        Rect {
            position: self.position,
            size: self.size,
            color: self.color,
            texture: match self.texture {
                Some(texture) => {
                    let texture = RawImage2d::from_raw_rgba_reversed(texture.as_bytes(), texture.dimensions());
                    SrgbTexture2d::new(display, texture).unwrap()
                }
                None => {
                    let texture = image::load_from_memory(WHITE_SQUARE_BYTES.as_slice()).unwrap();
                    let texture = RawImage2d::from_raw_rgba_reversed(texture.as_bytes(), texture.dimensions());
                    SrgbTexture2d::new(display, texture).unwrap()
                }
            },
            vbo: VertexBuffer::new(display, create_vertices(self.size).as_slice()).unwrap(),
            ebo: IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &[0, 2, 1, 0, 3, 2]).unwrap(),
            program: Program::from_source(display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap()
        }
    }
}

fn create_vertices(size: Vec2<f32>) -> [Vertex; 4] {
    let x = 1.0;
    let y = 1.0;

    [
        vertex(vec2(0.0 - x, 0.0 - y), vec2(0.0, 0.0)),
        vertex(vec2(0.0 - x, 0.0 + y), vec2(0.0, 1.0)),
        vertex(vec2(0.0 + x, 0.0 + y), vec2(1.0, 1.0)),
        vertex(vec2(0.0 + x, 0.0 - y), vec2(1.0, 0.0))
    ]
}

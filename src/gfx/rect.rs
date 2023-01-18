use glium::{vertex::VertexBufferAny, Program, VertexBuffer, Display, IndexBuffer, Frame, Surface, uniforms::EmptyUniforms, uniform, DrawParameters, Blend, texture::{SrgbTexture2d, RawImage2d}};
use image::{DynamicImage, GenericImageView};
use crate::{math::{vec2::{Vec2, vec2}, vec4::{vec4, Vec4}}, get_window_width, get_window_height, DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT};
use super::{ColorVertex, COLOR_VERTEX_SRC, COLOR_FRAG_SRC, color_vertex, UVertex, uv_vertex, IMAGE_VERTEX_SRC, IMAGE_FRAG_SRC};

#[derive(Debug)]
pub struct Rect {
    position: Vec2<f32>,
    size: Vec2<f32>,
    color: Vec4<f32>,
    texture: Option<SrgbTexture2d>,
    vbo: VertexBufferAny,
    ebo: IndexBuffer<u32>,
    shader: Program
}

impl Rect {
    /// `position` is the centre of the rect
    pub fn new(position: Vec2<f32>, size: Vec2<f32>, display: &Display) -> Rect {
        let vbo = VertexBuffer::new(display, create_color_vertices(size).as_slice()).unwrap();
        let ebo = IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &[0, 2, 1, 0, 3, 2]).unwrap();
        let shader = Program::from_source(display, COLOR_VERTEX_SRC, COLOR_FRAG_SRC, None).unwrap();

        Rect {
            position,
            size,
            color: vec4(1.0, 1.0, 1.0, 1.0),
            texture: None,
            vbo: VertexBufferAny::from(vbo),
            ebo,
            shader
        }
    }

    pub fn set_color(&mut self, color: Vec4<f32>, display: &Display) {
        self.shader = Program::from_source(display, &COLOR_VERTEX_SRC, &COLOR_FRAG_SRC, None).unwrap();
        let vbo = VertexBuffer::new(display, create_color_vertices(self.size).as_slice()).unwrap();
        self.vbo = VertexBufferAny::from(vbo);
        
        self.color = color;
    }

    pub fn set_texture(&mut self, texture: DynamicImage, display: &Display) {
        self.shader = Program::from_source(display, &IMAGE_VERTEX_SRC, &IMAGE_FRAG_SRC, None).unwrap();
        let vbo = VertexBuffer::new(display, create_uv_vertices(self.size).as_slice()).unwrap();
        self.vbo = VertexBufferAny::from(vbo);

        let texture = RawImage2d::from_raw_rgba_reversed(texture.as_bytes(), texture.dimensions());
        let texture = SrgbTexture2d::new(display, texture).unwrap();
        self.texture = Some(texture);
    }

    pub fn left(&self) -> f32 {
        self.position.x - self.size.x
    }

    pub fn set_left(&mut self, left: f32) {
        self.position.x = left + self.size.x;
    }

    pub fn top(&self) -> f32 {
        self.position.y + self.size.y
    }

    pub fn set_top(&mut self, top: f32) {
        self.position.y = top - self.size.y;
    }

    pub fn right(&self) -> f32 {
        self.position.x + self.size.x
    }

    pub fn set_right(&mut self, right: f32) {
        self.position.x = right - self.size.x;
    }

    pub fn bottom(&self) -> f32 {
        self.position.y - self.size.y
    }

    pub fn set_bottom(&mut self, bottom: f32) {
        self.position.y = bottom + self.size.y;
    }

    pub fn width(&self) -> f32 {
        self.size.x
    }

    pub fn set_width(&mut self, width: f32, display: &Display) {
        self.size.x = width;

        match &self.texture {
            Some(_) => {
                self.vbo = VertexBufferAny::from(VertexBuffer::new(display, create_uv_vertices(self.size).as_slice()).unwrap());
            }
            None => {
                self.vbo = VertexBufferAny::from(VertexBuffer::new(display, create_color_vertices(self.size).as_slice()).unwrap());
            }
        }
    }

    pub fn height(&self) -> f32 {
        self.size.y
    }

    pub fn set_height(&mut self, height: f32, display: &Display) {
        self.size.y = height;

        match &self.texture {
            Some(_) => {
                self.vbo = VertexBufferAny::from(VertexBuffer::new(display, create_uv_vertices(self.size).as_slice()).unwrap());
            }
            None => {
                self.vbo = VertexBufferAny::from(VertexBuffer::new(display, create_color_vertices(self.size).as_slice()).unwrap());
            }
        }
    }

    pub fn centre(&self) -> Vec2<f32> {
        self.position
    }

    pub fn set_centre(&mut self, centre: Vec2<f32>) {
        self.position = centre;
    }

    pub fn draw(&self, target: &mut Frame) {
        let position = [self.position.x / (DEFAULT_WINDOW_WIDTH as f32 / 2.0), self.position.y / (DEFAULT_WINDOW_HEIGHT as f32 / 2.0)];

        match &self.texture {
            Some(texture) => {
                let uniforms = uniform! {
                    offset: position,
                    tex: texture
                };

                target.draw(&self.vbo, &self.ebo, &self.shader, &uniforms, &DrawParameters {
                    blend: Blend::alpha_blending(),
                    ..Default::default()
                });
            },
            None => {
                let uniforms = uniform! {
                    offset: position,
                    color: self.color.as_raw()
                };

                target.draw(&self.vbo, &self.ebo, &self.shader, &uniforms, &DrawParameters {
                    blend: Blend::alpha_blending(),
                    ..Default::default()
                });
            }
        };

        
    }
}

fn create_color_vertices(size: Vec2<f32>) -> [ColorVertex; 4] {
    let x = ((size.x / 2.0) / get_window_width() as f32);
    let y = ((size.y / 2.0) / get_window_height() as f32);
    [
        color_vertex(vec2(0.0 - x, 0.0 - y)),
        color_vertex(vec2(0.0 - x, 0.0 + y)),
        color_vertex(vec2(0.0 + x, 0.0 + y)),
        color_vertex(vec2(0.0 + x, 0.0 - y)),
    ]
}

fn create_uv_vertices(size: Vec2<f32>) -> [UVertex; 4] {
    let x = ((size.x / 2.0) / get_window_width() as f32);
    let y = ((size.y / 2.0) / get_window_height() as f32);
    [
        uv_vertex(vec2(0.0 - x, 0.0 - y), vec2(0.0, 0.0)),
        uv_vertex(vec2(0.0 - x, 0.0 + y), vec2(0.0, 1.0)),
        uv_vertex(vec2(0.0 + x, 0.0 + y), vec2(1.0, 1.0)),
        uv_vertex(vec2(0.0 + x, 0.0 - y), vec2(1.0, 0.0)),
    ]
}

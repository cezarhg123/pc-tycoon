pub mod rect;

pub use rect::Rect;
use glium::implement_vertex;
use crate::math::vec2::{Vec2, vec2};

#[derive(Debug, Clone, Copy)]
pub struct ColorVertex {
    position: [f32; 2]
}

impl ColorVertex {
    pub fn position(&self) -> Vec2<f32> {
        vec2(self.position[0], self.position[1])
    }

    pub fn set_position(&mut self, position: Vec2<f32>) {
        self.position[0] = position.x;
        self.position[1] = position.y;
    }
}

pub fn color_vertex(position: Vec2<f32>) -> ColorVertex {
    ColorVertex {
        position: [position.x, position.y]
    }
}

implement_vertex!(ColorVertex, position);

#[derive(Debug, Clone, Copy)]
pub struct UVertex {
    v_position: [f32; 2],
    v_uv: [f32; 2]
}

impl UVertex {
    pub fn position(&self) -> Vec2<f32> {
        vec2(self.v_position[0], self.v_position[1])
    }

    pub fn set_position(&mut self, position: Vec2<f32>) {
        self.v_position[0] = position.x;
        self.v_position[1] = position.y;
    }

    pub fn uv(&self) -> Vec2<f32> {
        vec2(self.v_uv[0], self.v_uv[1])
    }

    pub fn set_uv(&mut self, uv: Vec2<f32>) {
        self.v_uv[0] = uv.x;
        self.v_uv[1] = uv.y;
    }
}

pub fn uv_vertex(position: Vec2<f32>, uv: Vec2<f32>) -> UVertex {
    UVertex {
        v_position: [position.x, position.y],
        v_uv: [uv.x, uv.y]
    }
}

implement_vertex!(UVertex, v_position , v_uv);

const COLOR_VERTEX_SRC: &str = r#"
    #version 140
    in vec2 position;

    uniform vec2 offset;

    void main() {
        gl_Position = vec4(position + offset + vec2(-1.0, -1.0), 0.0, 1.0);
    }
"#;

const COLOR_FRAG_SRC: &str = r#"
    #version 140
    out vec4 out_color;

    uniform vec4 color;

    void main() {
        out_color = color;
    }
"#;

const IMAGE_VERTEX_SRC: &str = r#"
    #version 140

    in vec2 v_position;
    in vec2 v_uv;

    uniform vec2 offset;

    out vec2 uv;

    void main() {
        uv = v_uv;
        gl_Position = vec4(v_position + offset + vec2(-1.0, -1.0), 0.0, 1.0);
    }
"#;

const IMAGE_FRAG_SRC: &str = r#"
    #version 140
    
    in vec2 uv;
    out vec4 out_color;

    uniform sampler2D tex;
    uniform vec4 color;

    void main() {
        out_color = texture(tex, uv) * color;
    }
"#;

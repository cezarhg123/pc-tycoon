use glium::implement_vertex;

use crate::math::vec2::Vec2;

pub mod rect;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    v_position: [f32; 2],
    v_uv: [f32; 2]
}
implement_vertex!(Vertex, v_position, v_uv);

pub const fn vertex(position: Vec2<f32>, uv: Vec2<f32>) -> Vertex {
    Vertex {
        v_position: position.as_raw(),
        v_uv: uv.as_raw()
    }
}

pub const WHITE_SQUARE_BYTES: [u8; 207] = *include_bytes!("../../textures/white.png");

pub const VERTEX_SHADER_SRC: &'static str = r#"
    #version 140

    in vec2 v_position;
    in vec2 v_uv;
    out vec2 uv;

    uniform vec2 offset;
    uniform vec2 size;

    void main() {
        uv = v_uv;
        gl_Position = vec4((v_position * size) + offset + vec2(-1.0, -1.0), 0.0, 1.0);
    }
"#;

pub const FRAGMENT_SHADER_SRC: &'static str = r#"
    #version 140

    in vec2 uv;
    out vec4 frag_color;

    uniform sampler2D tex;
    uniform vec4 color;

    void main() {
        frag_color = texture(tex, uv) * color;
    }
"#;

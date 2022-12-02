use crate::vectors::{vec2::Vec2, vec3::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct ColorVertex {
    position: Vec2<f32>,
    color: Vec3<f32>
}

pub fn color_vertex(position: Vec2<f32>, color: Vec3<f32>) -> ColorVertex {
    ColorVertex {
        position,
        color
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UvVertex {
    position: Vec2<f32>,
    uv: Vec2<f32>
}

pub fn uv_vertex(position: Vec2<f32>, uv: Vec2<f32>) -> UvVertex {
    UvVertex {
        position,
        uv
    }
}

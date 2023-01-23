use crate::math::{vec2::Vec2, vec3::Vec3};

use super::textline::TextLine;

pub struct Button {
    textlines: Vec<TextLine>,
    position: Vec2<f32>,
    size: Vec2<f32>,
    color: Vec3<f32>,
    clicked: bool
}
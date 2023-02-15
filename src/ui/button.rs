use crate::math::vec2::Vec2;

use super::uielement::UiOutput;

pub struct Button {
    output: UiOutput,
    position: Vec2<f32>,
    size: Vec2<f32>
}
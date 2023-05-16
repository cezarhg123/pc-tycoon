use glium::{Frame, glutin::event::WindowEvent};
use crate::math::vec2::Vec2;

use super::uioutput::UiOutput;

pub trait UiAttributes {
    fn left(&self) -> f32;
    fn set_left(&mut self, left: f32);
    fn top(&self) -> f32;
    fn set_top(&mut self, top: f32);
    fn right(&self) -> f32;
    fn set_right(&mut self, right: f32);
    fn bottom(&self) -> f32;
    fn set_bottom(&mut self, bottom: f32);
    fn centre(&self) -> Vec2<f32>;
    fn set_centre(&mut self, centre: Vec2<f32>);

    fn size(&self) -> Vec2<f32>;
    fn set_size(&mut self, size: Vec2<f32>);
    fn width(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn height(&self) -> f32;
    fn set_height(&mut self, height: f32);

    fn output(&self) -> UiOutput;
    fn handle_events(&mut self, event: &WindowEvent) -> bool;
    fn draw(&self, target: &mut Frame);
}

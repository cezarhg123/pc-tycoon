use std::{cell::{Ref, RefCell}, rc::Rc};
use glium::{Frame, glutin::event::WindowEvent, Display};
use crate::math::vec2::Vec2;
use super::customuidata::CustomUIData;

pub trait UiElement {
    fn handle_event(&mut self, event: &WindowEvent, cursor_pos: Vec2<f32>, display: &Display) -> bool;
    fn output(&self) -> UiOutput;
    fn id(&self) -> &str;
    fn custon_data(&self) -> &[CustomUIData];

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
    fn width(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn height(&self) -> f32;
    fn set_height(&mut self, height: f32);

    fn draw(&self, target: &mut Frame);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UiOutput {
    None,
    Hovered,
    LeftClicked,
    RightClicked
}
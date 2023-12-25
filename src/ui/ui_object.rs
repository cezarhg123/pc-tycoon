use glfw::WindowEvent;
use gpu_allocator::vulkan::Allocator;

pub trait UiObject {
    fn contains(&self, pos: glm::Vec2) -> bool;

    fn left(&self) -> f32;
    fn set_left(&mut self, left: f32);

    fn top(&self) -> f32;
    fn set_top(&mut self, top: f32);

    fn right(&self) -> f32;
    fn set_right(&mut self, right: f32);

    fn bottom(&self) -> f32;
    fn set_bottom(&mut self, bottom: f32);

    fn width(&self) -> f32;
    fn set_width(&mut self, width: f32);

    fn height(&self) -> f32;
    fn set_height(&mut self, height: f32);

    fn center(&self) -> glm::Vec2;
    fn set_center(&mut self, centre: glm::Vec2);

    fn handle_events(&mut self, event: &WindowEvent, allocator: &mut Allocator) -> bool;
    fn draw(&self);
}
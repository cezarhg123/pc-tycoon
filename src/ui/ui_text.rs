use crate::primitives::text::Text;
use super::ui_object::UiObject;

/// Just a wrapper for `Text` to be used in the Ui
pub struct UiText {
    text: Text
}

impl UiText {
    pub fn new(text: Text) -> UiText {
        UiText { text }
    }

    pub fn change_text(&mut self, text: Text) {
        self.text = text;
    }
}

impl UiObject for UiText {
    fn contains(&self, pos: glm::Vec2) -> bool {
        self.text.contains(pos)
    }

    fn left(&self) -> f32 {
        self.text.left()
    }

    fn set_left(&mut self, left: f32) {
        self.text.set_left(left);
    }

    fn top(&self) -> f32 {
        self.text.top()
    }

    fn set_top(&mut self, top: f32) {
        self.text.set_top(top);
    }

    fn right(&self) -> f32 {
        self.text.right()
    }

    fn set_right(&mut self, right: f32) {
        self.text.set_right(right);
    }

    fn bottom(&self) -> f32 {
        self.text.bottom()
    }

    fn set_bottom(&mut self, bottom: f32) {
        self.text.set_bottom(bottom);
    }

    fn width(&self) -> f32 {
        self.text.width()
    }

    fn set_width(&mut self, _width: f32) { /* dont change width */ }

    fn height(&self) -> f32 {
        self.text.height()
    }

    fn set_height(&mut self, _height: f32) { /* dont change height */ }

    fn center(&self) -> glm::Vec2 {
        self.text.center()
    }

    fn set_center(&mut self, center: glm::Vec2) {
        self.text.set_center(center);
    }

    fn handle_events(&mut self, _event: &glfw::WindowEvent, _allocator: &mut gpu_allocator::vulkan::Allocator) -> bool {
        false
    }

    fn draw(&self) {
        self.text.draw();
    }
}

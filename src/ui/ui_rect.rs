use crate::primitives::rect::Rect;
use super::ui_object::UiObject;

pub struct UiRect {
    rect: Rect
}

impl UiRect {
    pub fn new(rect: Rect) -> UiRect {
        UiRect {
            rect
        }
    }
}

impl UiObject for UiRect {
    fn contains(&self, pos: glm::Vec2) -> bool {
        self.rect.contains(pos)
    }

    fn left(&self) -> f32 {
        self.rect.left()
    }

    fn set_left(&mut self, left: f32) {
        self.rect.set_left(left);
    }

    fn top(&self) -> f32 {
        self.rect.top()
    }

    fn set_top(&mut self, top: f32) {
        self.rect.set_top(top);
    }

    fn right(&self) -> f32 {
        self.rect.right()
    }

    fn set_right(&mut self, right: f32) {
        self.rect.set_right(right);
    }

    fn bottom(&self) -> f32 {
        self.rect.bottom()
    }

    fn set_bottom(&mut self, bottom: f32) {
        self.rect.set_bottom(bottom);
    }

    fn width(&self) -> f32 {
        self.rect.width()
    }

    fn set_width(&mut self, width: f32) {
        self.rect.set_width(width);
    }

    fn height(&self) -> f32 {
        self.rect.height()
    }

    fn set_height(&mut self, height: f32) {
        self.rect.set_height(height);
    }

    fn center(&self) -> glm::Vec2 {
        self.rect.center()
    }

    fn set_center(&mut self, center: glm::Vec2) {
        self.rect.set_center(center);
    }

    fn handle_events(&mut self, _event: &glfw::WindowEvent, _allocator: &mut gpu_allocator::vulkan::Allocator) -> bool {
        false
    }

    fn draw(&self) {
        self.rect.draw()
    }
}

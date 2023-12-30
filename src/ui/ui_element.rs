use glfw::WindowEvent;
use gpu_allocator::vulkan::Allocator;
use super::ui_object::UiObject;

pub struct UiElement {
    pub id: String,
    pub handle_events: bool,
    pub draw: bool,
    pub ui_object: Box<dyn UiObject>,
    /// specifies if element is hovered by mouse to move the element
    /// 
    /// this is only used and compiled in debug mode
    #[cfg(debug_assertions)]
    pub dev_hovered: bool
}

impl UiElement {
    pub fn new(id: impl ToString, handle_events: bool, draw: bool, ui_object: impl UiObject + 'static) -> UiElement {
        UiElement {
            id: id.to_string(),
            handle_events,
            draw,
            ui_object: Box::new(ui_object),
            #[cfg(debug_assertions)]
            dev_hovered: false
        }
    }

    #[cfg(debug_assertions)]
    pub fn dev_move(&mut self, event: &WindowEvent, scale: f32) {
        match event {
            WindowEvent::CursorPos(x, y) => {
                if self.ui_object.contains(glm::vec2(*x as f32, *y as f32)) {
                    self.dev_hovered = true;
                } else {
                    self.dev_hovered = false;
                }
            }
            _ => {}
        }
        
        if self.dev_hovered {
            match event {
                WindowEvent::Key(key, _, action, _) => {
                    if *key == glfw::Key::Left && *action == glfw::Action::Press { // move left
                        self.ui_object.set_left(self.ui_object.left() - (1.0 * scale));
                    } else if *key == glfw::Key::Down && *action == glfw::Action::Press { // move down
                        self.ui_object.set_top(self.ui_object.top() + (1.0 * scale));
                    } else if *key == glfw::Key::Right && *action == glfw::Action::Press { // move right
                        self.ui_object.set_left(self.ui_object.left() + (1.0 * scale));
                    } else if *key == glfw::Key::Up && *action == glfw::Action::Press { // move up
                        self.ui_object.set_top(self.ui_object.top() - (1.0 * scale));
                    } else if *key == glfw::Key::Delete && *action == glfw::Action::Press { // shrink width
                        self.ui_object.set_width(self.ui_object.width() - (1.0 * scale));
                    } else if *key == glfw::Key::End && *action == glfw::Action::Press { // shrink height
                        self.ui_object.set_height(self.ui_object.height() - (1.0 * scale));
                    } else if *key == glfw::Key::PageDown && *action == glfw::Action::Press { // grow width
                        self.ui_object.set_width(self.ui_object.width() + (1.0 * scale));
                    } else if *key == glfw::Key::Home && *action == glfw::Action::Press { // grow height
                        self.ui_object.set_height(self.ui_object.height() + (1.0 * scale));
                    } else if *key == glfw::Key::Enter && *action == glfw::Action::Press { // print position and size
                        println!("left: {}, top: {}, width: {}, height: {}", self.ui_object.left(), self.ui_object.top(), self.ui_object.width(), self.ui_object.height());
                    }
                }
                _ => {}
            }
        }
    }

    pub(super) fn handle_events(&mut self, event: &WindowEvent, allocator: &mut Allocator) -> bool {
        if !self.handle_events {
            return false
        }
        
        self.ui_object.handle_events(event, allocator)
    }

    pub(super) fn draw(&self) {
        if self.draw {
            self.ui_object.draw();
        }
    }
}

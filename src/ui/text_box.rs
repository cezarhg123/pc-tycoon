use glfw::{WindowEvent, MouseButton};
use gpu_allocator::vulkan::Allocator;
use crate::primitives::{rect::{Rect, RectBuilder}, text::Text};
use super::ui_object::UiObject;

pub struct TextBox {
    rect: Rect,
    state: TextBoxState,
    selected: bool,
    text: Option<Text>,
    string: String
}

enum TextBoxState {
    Normal,
    Hovered
}

impl TextBox {
    pub fn new(rect: Rect, placeholder: impl ToString, allocator: &mut Allocator) -> TextBox {
        TextBox {
            text: Some(
                Text::builder()
                    .left(rect.left() + 2.0)
                    .top(rect.top() + 2.0)
                    .font_color({
                        let color_average = (rect.color().x + rect.color().y + rect.color().z) / 3.0;
                        let font_color = if color_average > 0.5 {
                            glm::vec3(0.0, 0.0, 0.0)
                        } else {
                            glm::vec3(1.0, 1.0, 1.0)
                        };
                        font_color
                    })
                    .font_size(rect.height() - 4.0)
                    .text(placeholder.to_string())
                    .build(allocator)
            ),
            rect,
            state: TextBoxState::Normal,
            selected: false,
            string: placeholder.to_string()
        }
    }

    pub fn get_string(&self) -> &str {
        &self.string
    }
}

impl UiObject for TextBox {
    fn contains(&self, pos: glm::Vec2) -> bool {
        self.rect.contains(pos)
    }

    fn left(&self) -> f32 {
        self.rect.left()
    }

    fn set_left(&mut self, left: f32) {
        self.rect.set_left(left)
    }

    fn top(&self) -> f32 {
        self.rect.top()
    }

    fn set_top(&mut self, top: f32) {
        self.rect.set_top(top)
    }

    fn right(&self) -> f32 {
        self.rect.right()
    }

    fn set_right(&mut self, right: f32) {
        self.rect.set_right(right)
    }

    fn bottom(&self) -> f32 {
        self.rect.bottom()
    }

    fn set_bottom(&mut self, bottom: f32) {
        self.rect.set_bottom(bottom)
    }

    fn width(&self) -> f32 {
        self.rect.width()
    }

    fn set_width(&mut self, width: f32) {
        self.rect.set_width(width)
    }

    fn height(&self) -> f32 {
        self.rect.height()
    }

    fn set_height(&mut self, height: f32) {
        self.rect.set_height(height)
    }

    fn center(&self) -> glm::Vec2 {
        self.rect.center()
    }

    fn set_center(&mut self, centre: glm::Vec2) {
        self.rect.set_center(centre)
    }

    fn handle_events(&mut self, event: &WindowEvent, allocator: &mut Allocator) -> bool {
        match event {
            WindowEvent::CursorPos(x, y) => {
                let x = *x as f32;
                let y = *y as f32;

                match &self.state {
                    TextBoxState::Normal => {
                        if self.contains(glm::vec2(x, y)) {
                            self.state = TextBoxState::Hovered;
                            return true;
                        }
                    }
                    TextBoxState::Hovered => {
                        if !self.contains(glm::vec2(x, y)) {
                            self.state = TextBoxState::Normal;
                        }
                    }
                }
            }
            WindowEvent::MouseButton(button, action, _) => {
                match &self.state {
                    TextBoxState::Normal => {
                        if *button == MouseButton::Button1 && *action == glfw::Action::Press {
                            self.selected = false;
                        }
                    }
                    TextBoxState::Hovered => {
                        if *button == MouseButton::Button1 && *action == glfw::Action::Press {
                            self.selected = true;
                            return true
                        }
                    }
                }
            }
            WindowEvent::Char(c) => {
                if self.selected {
                    let color_average = (self.rect.color().x + self.rect.color().y + self.rect.color().z) / 3.0;
                    let font_color = if color_average > 0.5 {
                        glm::vec3(0.0, 0.0, 0.0)
                    } else {
                        glm::vec3(1.0, 1.0, 1.0)
                    };

                    self.string.push(*c);
                    self.text = Some(Text::builder().left(self.rect.left() + 2.0).top(self.rect.top() + 2.0).font_color(font_color).font_size(self.rect.height() - 4.0).text(&self.string).build(allocator));
                }
            }
            WindowEvent::Key(key, _, action, _) => {
                if *key == glfw::Key::Backspace && *action == glfw::Action::Press && !self.string.is_empty() && self.selected {
                    let color_average = (self.rect.color().x + self.rect.color().y + self.rect.color().z) / 3.0;
                    let font_color = if color_average > 0.5 {
                        glm::vec3(0.0, 0.0, 0.0)
                    } else {
                        glm::vec3(1.0, 1.0, 1.0)
                    };

                    self.string.pop();

                    if self.string.is_empty() {
                        self.text = None;
                    } else {
                        self.text = Some(Text::builder().left(self.rect.left() + 2.0).top(self.rect.top() + 2.0).font_color(font_color).font_size(self.rect.height() - 4.0).text(&self.string).build(allocator));
                    }
                }

                if *key == glfw::Key::Enter && *action == glfw::Action::Press && self.selected {
                    self.selected = false;
                }
            }
            _ => {}
        }

        false
    }

    fn draw(&self) {
        self.rect.draw();
        if let Some(text) = &self.text {
            text.draw();
        }
    }
}

use glium::{Display, Frame, glutin::event::{WindowEvent, VirtualKeyCode, ElementState, MouseScrollDelta}};

use crate::{ptrcell::PtrCell, gfx::rect::{Rect, RectBuilder}, math::{vec2::{Vec2, vec2}, vec4::vec4}, log::{log, save_log}, MOVE_UI};

use super::uielement::{UiElement, UiOutput};

pub struct Listbox {
    id: String,
    elements: Vec<PtrCell<dyn UiElement>>,
    cursor: Rect,
    bar: Rect,
    rect: Rect,
    cursor_idx: usize,
    viewable_elements: usize
}

impl UiElement for Listbox {
    fn handle_event(&mut self, event: &WindowEvent, cursor_pos: Vec2<f32>, display: &Display) -> bool {
        use glium::glutin::event::WindowEvent;
        if self.rect.contains(cursor_pos) {
            match event {
                WindowEvent::MouseWheel {delta, ..} => {
                    match delta {
                        MouseScrollDelta::LineDelta(x, y) => {
                            if *y > 0.0 { // scroll up
                                if self.cursor_idx as i32 - self.viewable_elements as i32 > 0 { // checks if the cursor can be moved up without going above bar
                                    self.cursor_idx -= self.viewable_elements;
                                    self.cursor.set_bottom(self.cursor.top());
                                } else {
                                    self.cursor_idx = 0;
                                    self.cursor.set_top(self.rect.top());
                                }

                                // reposition elements
                                let mut previous_bottom = 0.0;
                                for i in self.cursor_idx..(self.cursor_idx + self.viewable_elements) {
                                    if previous_bottom == 0.0 {
                                        self.elements[i].set_top(self.rect.top());
                                        previous_bottom = self.elements[i].bottom();
                                    } else {
                                        self.elements[i].set_top(previous_bottom);
                                        previous_bottom = self.elements[i].bottom();
                                    }
                                }

                                return true;
                            } else if *y < 0.0 { // scroll down
                                if self.cursor_idx + self.viewable_elements > self.elements.len() {
                                    self.cursor_idx += self.viewable_elements;
                                    self.cursor.set_top(self.cursor.bottom());
                                } else {
                                    self.cursor_idx = self.elements.len() - self.viewable_elements;
                                    self.cursor.set_bottom(self.rect.bottom());
                                }

                                // reposition elements
                                let mut previous_bottom = 0.0;
                                for i in self.cursor_idx..(self.cursor_idx + self.viewable_elements) {
                                    if previous_bottom == 0.0 {
                                        self.elements[i].set_top(self.rect.top());
                                        previous_bottom = self.elements[i].bottom();
                                    } else {
                                        self.elements[i].set_top(previous_bottom);
                                        previous_bottom = self.elements[i].bottom();
                                    }
                                }
                                return true;
                            }
                            false
                        }
                        _ => {false}
                    }
                }
                WindowEvent::MouseInput {button, state, ..} => {
                    match (button, state) {
                        _ => {false}
                    }
                }
                WindowEvent::KeyboardInput {input, ..} => {
                    if MOVE_UI {
                        match (input.virtual_keycode.unwrap(), input.state) {
                            (VirtualKeyCode::Up, ElementState::Pressed) => {
                                self.set_top(self.top() + 1.0);
                                true
                            }
                            (VirtualKeyCode::Down, ElementState::Pressed) => {
                                self.set_bottom(self.bottom() - 1.0);
                                true
                            }
                            (VirtualKeyCode::Right, ElementState::Pressed) => {
                                self.set_right(self.right() + 1.0);
                                true
                            }
                            (VirtualKeyCode::Left, ElementState::Pressed) => {
                                self.set_left(self.left() - 1.0);
                                true
                            }
                            (VirtualKeyCode::Return, ElementState::Pressed) => {
                                println!("{:#?}", self.centre());
                                true
                            }
                            (VirtualKeyCode::Numpad4, ElementState::Pressed) => {
                                self.set_width(self.width() - 1.0);
                                true
                            }
                            (VirtualKeyCode::Numpad6, ElementState::Pressed) => {
                                self.set_width(self.width() + 1.0);
                                true
                            }
                            (VirtualKeyCode::Numpad2, ElementState::Pressed) => {
                                self.set_height(self.height() - 1.0);
                                true
                            }
                            (VirtualKeyCode::Numpad8, ElementState::Pressed) => {
                                self.set_height(self.height() + 1.0);
                                true
                            }
                            (VirtualKeyCode::Numpad5, ElementState::Pressed) => {
                                println!("size: {}, {}", self.width(), self.height());
                                true
                            }
                            _ => {false}
                        }
                    } else {false}
                }
                _ => {false}
            }
        } else {
            false
        }
    }

    fn output(&self) -> UiOutput {
        UiOutput::None
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn left(&self) -> f32 {
        self.rect.left()
    }

    fn set_left(&mut self, left: f32) {
        let diff = left - self.rect.left();

        self.rect.set_left(left);
        self.bar.set_left(self.bar.left() + diff);
        self.cursor.set_left(self.bar.left() + diff);

        // gotta do this because of rust's strict ass single mutability rules >:(
        let lefts = self.elements.iter_mut().map(|element| element.left()).collect::<Vec<_>>();
        for element in self.elements.iter_mut().enumerate() {
            element.1.set_left(lefts[element.0] + diff);
        }
    }

    fn top(&self) -> f32 {
        self.rect.top()
    }

    fn set_top(&mut self, top: f32) {
        let diff = top - self.rect.top();

        self.rect.set_top(top);
        self.bar.set_top(self.bar.top() + diff);
        self.cursor.set_top(self.bar.top() + diff);

        // gotta do this because of rust's strict ass single mutability rules >:(
        let tops = self.elements.iter_mut().map(|element| element.top()).collect::<Vec<_>>();
        for element in self.elements.iter_mut().enumerate() {
            element.1.set_top(tops[element.0] + diff);
        }
    }

    fn right(&self) -> f32 {
        self.rect.right()
    }

    fn set_right(&mut self, right: f32) {
        let diff = right - self.rect.right();

        self.rect.set_right(right);
        self.bar.set_right(self.bar.right() + diff);
        self.cursor.set_right(self.bar.right() + diff);

        // gotta do this because of rust's strict ass single mutability rules >:(
        let rights = self.elements.iter_mut().map(|element| element.right()).collect::<Vec<_>>();
        for element in self.elements.iter_mut().enumerate() {
            element.1.set_right(rights[element.0] + diff);
        }
    }

    fn bottom(&self) -> f32 {
        self.rect.bottom()
    }

    fn set_bottom(&mut self, bottom: f32) {
        let diff = bottom - self.rect.bottom();

        self.rect.set_bottom(bottom);
        self.bar.set_bottom(self.bar.bottom() + diff);
        self.cursor.set_bottom(self.bar.bottom() + diff);

        // gotta do this because of rust's strict ass single mutability rules >:(
        let bottoms = self.elements.iter_mut().map(|element| element.bottom()).collect::<Vec<_>>();
        for element in self.elements.iter_mut().enumerate() {
            element.1.set_bottom(bottoms[element.0] + diff);
        }
    }

    fn centre(&self) -> Vec2<f32> {
        self.rect.centre()
    }

    fn set_centre(&mut self, centre: Vec2<f32>) {
        let diff = centre - self.rect.centre();

        self.rect.set_centre(centre);
        self.bar.set_centre(self.bar.centre() + diff);
        self.cursor.set_centre(self.bar.centre() + diff);

        // gotta do this because of rust's strict ass single mutability rules >:(
        let centres = self.elements.iter_mut().map(|element| element.centre()).collect::<Vec<_>>();
        for element in self.elements.iter_mut().enumerate() {
            element.1.set_centre(centres[element.0] + diff);
        }
    }

    fn width(&self) -> f32 {
        self.rect.width()
    }

    fn set_width(&mut self, width: f32) {
        let diff = width - self.rect.width();

        self.rect.set_width(width);
        self.bar.set_width(self.bar.width() + diff);
        self.cursor.set_width(self.bar.width() + diff);

        // gotta do this because of rust's strict ass single mutability rules >:(
        let widths = self.elements.iter_mut().map(|element| element.width()).collect::<Vec<_>>();
        for element in self.elements.iter_mut().enumerate() {
            element.1.set_width(widths[element.0] + diff);
        }
    }

    fn height(&self) -> f32 {
        self.rect.height()
    }

    fn set_height(&mut self, height: f32) {
        let diff = height - self.rect.height();

        self.rect.set_height(height);
        self.bar.set_height(self.bar.height() + diff);
        self.cursor.set_height(self.bar.height() + diff);

        // gotta do this because of rust's strict ass single mutability rules >:(
        let heights = self.elements.iter_mut().map(|element| element.height()).collect::<Vec<_>>();
        for element in self.elements.iter_mut().enumerate() {
            element.1.set_height(heights[element.0] + diff);
        }
    }

    fn draw(&self, target: &mut Frame) {
        self.rect.draw(target);
        self.bar.draw(target);
        self.cursor.draw(target);

        for i in self.cursor_idx..(self.cursor_idx + self.viewable_elements) {
            self.elements[i].draw(target);
        }
    }
}

/// all elements better have the same height or else big problems
pub struct ListboxBuilder {
    pub id: String,
    pub elements: Vec<PtrCell<dyn UiElement>>,
    pub bar_width: f32,
    pub position: Vec2<f32>,
    pub size: Vec2<f32>
}

impl Default for ListboxBuilder {
    fn default() -> Self {
        ListboxBuilder {
            id: "Default Listbox".to_string(),
            elements: Vec::new(),
            bar_width: 10.0,
            position: vec2(400.0, 400.0),
            size: vec2(200.0, 160.0)
        }
    }
}

impl ListboxBuilder {
    pub fn build(mut self, display: &Display) -> Listbox {
        if self.elements.len() == 0 {
            log("CRITICAL ERROR: no elements in listbox");
            save_log();
            panic!("no elements in listbox");
        }

        let rect = RectBuilder {
            position: self.position,
            size: self.size,
            color: vec4(0.0, 0.0, 0.0, 1.0),
            texture: None
        }.build(display);

        let mut bar = RectBuilder {
            position: vec2(0.0, 0.0),
            size: vec2(self.bar_width, rect.height()),
            color: vec4(0.2, 0.2, 0.2, 1.0),
            texture: None
        }.build(display);
        bar.set_top(rect.top());
        bar.set_right(rect.right());

        let viewable_elements = (rect.height() / self.elements[0].height()).floor(); // get max allowed of viewable elements
        let cursor_height = if viewable_elements < self.elements.len() as f32 {
            bar.height() * (viewable_elements / self.elements.len() as f32)
        } else {
            bar.height()
        };

        let mut cursor = RectBuilder {
            position: vec2(0.0, 0.0),
            size: vec2(self.bar_width, cursor_height),
            color: vec4(0.6, 0.6, 0.6, 1.0),
            texture: None
        }.build(display);
        cursor.set_top(bar.top());
        cursor.set_right(bar.right());

        let mut previous_bottom = 0.0;
        for element in self.elements.iter_mut() {
            element.set_left(rect.left());

            // if previous bottom is 0.0 then allign element to the top of the rect
            // else align element to bottom of the previous element
            // + a 2 pixel offset for it to look good
            if previous_bottom == 0.0 {
                element.set_top(rect.top() - 2.0);
                previous_bottom = element.bottom();
            } else {
                element.set_top(previous_bottom - 2.0);
                previous_bottom = element.bottom();
            }
        }

        Listbox {
            id: self.id,
            elements: self.elements,
            cursor,
            bar,
            rect,
            cursor_idx: 0,
            viewable_elements: viewable_elements as usize
        }
    }
}

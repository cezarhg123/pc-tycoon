use std::{rc::Rc, cell::{RefMut, RefCell, Ref}};
use glium::{glutin::{event::{VirtualKeyCode, WindowEvent, KeyboardInput, ElementState}, dpi::PhysicalPosition}, Frame};
use crate::{math::vec2::{Vec2, vec2}, get_window_height, DEV_WINDOW_WIDTH, get_window_width, DEV_WINDOW_HEIGHT};
use self::uielement::UiElement;

pub mod uiattributes;
pub mod uielement;
pub mod uioutput;
pub mod uirect;

pub struct Ui<'a> {
    elements: Vec<Rc<RefCell<UiElement<'a>>>>,
    key_pressed: Option<VirtualKeyCode>,
    cursor_pos: Vec2<f32>
}

impl<'b: 'a, 'a> Ui<'a> {
    pub const fn new() -> Ui<'a> {
        Ui {
            elements: Vec::new(),
            key_pressed: None,
            cursor_pos: vec2(0.0, 0.0)
        }
    }

    /// adds element and returns a `Ref` of it
    pub fn add_element(&'b mut self, element: UiElement<'a>) -> Ref<UiElement<'a>> {
        let id = element.id().to_string().clone();

        // i would use 'find' but it causes issues with mutability
        match self.elements.iter().position(|e| e.borrow().id() == id.as_str()) {
            Some(element) => {
                self.elements[element].borrow()
            }
            None => {
                self.elements.push(Rc::new(RefCell::new(element)));
                self.elements.last().unwrap().borrow()
            }
        }
    }

    pub fn get_element(&'a self, id: impl ToString) -> Option<Ref<UiElement<'a>>> {
        let id = id.to_string();
        self.elements.iter().find_map(|e| if e.borrow().id() == id {Some(e.borrow())} else {None})
    }

    pub fn handle_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved {position, ..} => {
                self.cursor_pos = glium_position_to_mine(position);
                self.key_pressed = None;
            }
            WindowEvent::KeyboardInput {input, ..} => {
                if input.state == ElementState::Pressed {
                    self.key_pressed = input.virtual_keycode;
                } else if input.state == ElementState::Released {
                    self.key_pressed = None;
                }
            }
            _ => {}
        }

        for element in self.elements.iter_mut() {
            if !element.borrow().enabled() {
                continue;
            }

            if element.borrow_mut().handle_events(event) {
                return true;
            }
        }
        
        false
    }

    pub fn draw(&self, target: &mut Frame) {
        for element in self.elements.iter() {
            if !element.borrow().enabled() {
                continue;
            }

            element.borrow().draw(target);
        }
    }
}

// this functions only exists because glium
pub fn glium_position_to_mine(position: &PhysicalPosition<f64>) -> Vec2<f32> {
    let x = position.x as f32;
    let y = get_window_height() as f32 - position.y as f32;

    vec2(x * (DEV_WINDOW_WIDTH as f32 / get_window_width() as f32), y * (DEV_WINDOW_HEIGHT as f32 / get_window_height() as f32))
}

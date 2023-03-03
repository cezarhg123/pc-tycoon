pub mod uielement;
pub mod textline;
pub mod multitextline;
pub mod button;

use std::{rc::Rc, cell::{RefCell, RefMut, Cell}, borrow::{Borrow, BorrowMut}, ops::Deref};
use glium::{glutin::event::WindowEvent, Display};
use rusttype::Font;
use crate::{log::{log, save_log}, math::vec2::{Vec2, vec2}, get_window_height, ptrcell::PtrCell};
use self::uielement::UiElement;

static mut GLOBAL_FONT: Option<Font> = None;
static mut GLOBAL_BOLD_FONT: Option<Font> = None;

pub fn get_global_font() -> &'static Font<'static> {
    unsafe {
        GLOBAL_FONT.as_ref().unwrap()
    }
}

pub fn set_global_font(font_path: &str) {
    unsafe {
        GLOBAL_FONT = Some(Font::try_from_vec(std::fs::read(font_path).unwrap_or_else(|err| {
            log(format!("ERROR: cant load font at '{}'. trying to load default path", font_path));
            log(format!("ERROR: error output for the error above: {}", err.to_string()));
            //try default
            std::fs::read("fonts/font.ttf").unwrap_or_else(|err| {
                log("CRITICAL ERROR: cant load default font");
                save_log();
                panic!();
            })
        })).unwrap());
    }
}

pub fn get_global_bold_font() -> &'static Font<'static> {
    unsafe {
        GLOBAL_BOLD_FONT.as_ref().unwrap()
    }
}

pub fn set_global_bold_font(font_path: &str) {
    unsafe {
        GLOBAL_BOLD_FONT = Some(Font::try_from_vec(std::fs::read(font_path).unwrap_or_else(|err| {
            log(format!("ERROR: cant load bold font at '{}'. trying to load default path", font_path));
            log(format!("ERROR: error output for the error above: {}", err.to_string()));
            //try default
            std::fs::read("fonts/bold_font.ttf").unwrap_or_else(|err| {
                log("CRITICAL ERROR: cant load default bold font");
                save_log();
                panic!();
            })
        })).unwrap());
    }
}

pub struct Ui {
    /// Brief explanation on how elements work
    /// 
    /// `add_element()` either adds the element to the list and returns a `PtrCell` or finds the element with the same id and returns a `PtrCell` of that
    /// 
    /// Doing this is highly unsafe because i dont know how `Box` handles its memory.
    elements: Vec<Box<dyn UiElement>>,
    cursor_pos: Vec2<f32>
}

impl Ui {
    pub const fn new() -> Ui {
        Ui {
            elements: Vec::new(),
            cursor_pos: vec2(0.0, 0.0)
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent, display: &Display) -> bool {
        match event {
            WindowEvent::CursorMoved {position, ..} => {
                let x = position.x as f32;
                // gotta do this because the fucking cursor y position starts at the top instead of bottom
                let y = get_window_height() as f32 - position.y as f32;

                self.cursor_pos = vec2(x, y);
            },
            _ => {}
        }

        // go over elements and handle event
        self.elements.iter_mut().for_each(|element| {
            element.handle_event(event, self.cursor_pos, display);
        });

        false
    }

    pub fn add_element<T: UiElement + 'static>(&mut self, other_element: T) -> PtrCell<dyn UiElement> {
        let mut element_found;

        match self.elements.iter_mut().find(|e| e.id() == other_element.id()) { // find element with the same id
            Some(element) => {
                unsafe {element_found = Some(element.as_mut() as *mut dyn UiElement);}
            }
            None => {
                unsafe {element_found = None}
            }
        }

        match element_found {
            Some(element) => {
                unsafe {
                    PtrCell::new_raw(element) // really dont think this is safe at all
                }
            }
            None => {
                // push 'other_element' and return a 'PtrCell'
                unsafe {
                    self.elements.push(Box::new(other_element));
                    PtrCell::new_raw(self.elements.last_mut().unwrap().as_mut() as *mut dyn UiElement)
                }
            }
        }
    }

    pub fn get_element(&self, id: &str) -> Option<PtrCell<dyn UiElement>> {
        match self.elements.iter().find(|e| e.id() == id) {
            Some(element) => {
                Some(PtrCell::new_raw((element.as_ref() as *const dyn UiElement).cast_mut()))
            }
            None => {
                None
            }
        }
    }

    pub fn remove_element(&mut self, id: &str) {
        let index = self.elements.iter().position(|e| e.id() == id);
        
        match index {
            Some(index) => {
                self.elements.remove(index);
            }
            None => {}
        }
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }
}

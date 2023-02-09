pub mod uielement;
pub mod textline;

use glium::glutin::event::WindowEvent;
use rusttype::Font;
use crate::{log::{log, save_log}, math::vec2::{Vec2, vec2}, get_window_height};
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
    elements: Vec<Box<dyn UiElement>>,
    mouse_pos: Vec2<f32>
}

impl Ui {
    pub fn new() -> Ui {
        Ui {
            elements: Vec::new(),
            mouse_pos: vec2(0.0, 0.0)
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved {position, ..} => {
                let x = position.x as f32;
                // gotta do this because the fucking cursor y position starts at the top instead of bottom
                let y = get_window_height() as f32 - position.y as f32;

                self.mouse_pos = vec2(x, y);

                return true;
            },
            _ => {
                // go over elements and handle event
                for element in self.elements.iter_mut() {
                    if element.handle_event(event) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }
}

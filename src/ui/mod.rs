pub mod textline;
pub mod button;
pub mod multitextline;

use rusttype::Font;
use crate::log::{log, save_log};

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
            log(format!("ERROR: cant load font at '{}'. trying to load default path", font_path));
            log(format!("ERROR: error output for the error above: {}", err.to_string()));
            //try default
            std::fs::read("fonts/bold_font.ttf").unwrap_or_else(|err| {
                log("CRITICAL ERROR: cant load default font");
                save_log();
                panic!();
            })
        })).unwrap());
    }
}

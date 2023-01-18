use rusttype::Font;

static mut GLOBAL_FONT: Option<Font> = None;

pub fn get_global_font() -> &'static Font<'static> {
    unsafe {
        GLOBAL_FONT.as_ref().unwrap()
    }
}

pub fn set_global_font(font_path: &str) {
    unsafe {
        GLOBAL_FONT = Some(Font::try_from_vec(std::fs::read(font_path).unwrap()).unwrap());
    }
}
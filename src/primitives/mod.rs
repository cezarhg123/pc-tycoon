// THIS CONTAINS PRIMITIVE TYPES SUCH AS RECT, TEXT

pub mod rect;
/// SETTER AND GETTERS FOR `Rect` IN THIS SO THE IMPLEMENTATION IN `rect.rs` IS CLEANER
/// ALSO CONTAINS A FEW SIMPLE FUNCTIONS
pub mod rect_impl;
pub mod text;

static mut FONT: Option<ab_glyph::FontRef>  = None;

pub fn get_font() -> &'static ab_glyph::FontRef<'static> {
    unsafe {
        FONT.as_ref().unwrap()
    }
}

pub fn load_font() {
    unsafe {
        FONT = Some(
            ab_glyph::FontRef::try_from_slice(include_bytes!("../../fonts/font.ttf")).unwrap()
        );
    }
}

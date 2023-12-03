use super::ui_object::UiObject;

pub struct UiElement {
    pub id: String,
    pub handle_events: bool,
    pub draw: bool,
    pub ui_object: UiObject
}

impl UiElement {
    pub fn new(id: String, handle_events: bool, draw: bool, ui_object: UiObject) -> UiElement {
        UiElement {
            id,
            handle_events,
            draw,
            ui_object
        }
    }
}

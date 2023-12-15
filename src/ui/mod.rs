pub mod ui_element;
pub mod button;
pub mod ui_object;

use std::{rc::Rc, cell::RefCell};
use self::ui_element::UiElement;

pub struct Ui {
    elements: Vec<Rc<RefCell<UiElement>>>
}

impl Ui {
    pub fn new() -> Ui {
        Ui {
            elements: Vec::new()
        }
    }

    /// DONT HOLD THE REFCELL IN A STRUCT MAKE SURE IT GOES OUT OF SCOPE BY THE TIME `Ui` USES THEM
    pub fn add_element(&mut self, element: UiElement) -> Rc<RefCell<UiElement>> {
        if self.elements.iter().find(|e| e.borrow().id == element.id).is_none() {
            self.elements.push(Rc::new(RefCell::new(element)));

            self.elements.last().unwrap().clone()
        } else {
            self.elements.iter().find(|e| e.borrow().id == element.id).unwrap().clone()
        }
    }

    /// DONT HOLD THE REFCELL IN A STRUCT MAKE SURE IT GOES OUT OF SCOPE BY THE TIME `Ui` USES THEM
    pub fn get_element(&self, id: &str) -> Option<Rc<RefCell<UiElement>>> {
        self.elements.iter()
            .find(
                |e| e.borrow().id == id
            )
            .map(
                |e| e.clone()
            )
    }

    pub fn remove_element(&mut self, id: &str) {
        self.elements.retain(|e| e.borrow().id != id);
    }
}

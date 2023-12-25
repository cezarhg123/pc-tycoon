/// i do not like this ui system
/// i will 100% rewrite it later
/// i just wanna finish the project first


pub mod ui_element;
pub mod button;
pub mod ui_object;

use std::{rc::Rc, cell::{RefCell, Ref}};
use glfw::WindowEvent;
use gpu_allocator::vulkan::Allocator;
use self::{ui_element::UiElement, ui_object::UiObject};

pub struct Ui {
    elements: Vec<Rc<RefCell<UiElement>>>,
    #[cfg(debug_assertions)]
    dev_change_scale: f32
}

impl Ui {
    pub fn new() -> Ui {
        Ui {
            elements: Vec::new(),
            #[cfg(debug_assertions)]
            dev_change_scale: 1.0
        }
    }

    pub fn handle_events(&mut self, events: impl Iterator<Item = WindowEvent>, allocator: &mut Allocator) {
        for ref event in events {
            #[cfg(debug_assertions)] {
                match event {
                    WindowEvent::Key(key, _, action, _) => {
                        if *key == glfw::Key::KpAdd && *action == glfw::Action::Press {
                            self.dev_change_scale += 0.1;
                        } else if *key == glfw::Key::KpSubtract && *action == glfw::Action::Press {
                            self.dev_change_scale -= 0.1;
                        }
                    }
                    _ => {}
                }
            }

            for element in self.elements.iter() {
                #[cfg(debug_assertions)] {
                    element.borrow_mut().dev_move(&event, self.dev_change_scale);
                }

                if element.borrow_mut().handle_events(event, allocator) {
                    break;
                }
            }
        }
    }

    pub fn draw(&self) {
        for element in self.elements.iter() {
            element.borrow().draw();
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

pub fn as_ui_type<T>(element: Ref<UiElement>) -> &T {
    let ptr: *const dyn UiObject = element.ui_object.as_ref();
    
    unsafe { &*(ptr as *const T) }
}

pub fn as_ui_type_mut<T>(element: Ref<UiElement>) -> &mut T {
    let ptr: *const dyn UiObject = element.ui_object.as_ref();
    
    unsafe { &mut *(ptr as *mut T) }
}

use std::{rc::Rc, cell::{Cell, RefCell, Ref, RefMut}};
use glium::{Frame, glutin::event::WindowEvent};
use super::{uiattributes::UiAttributes, uioutput::UiOutput};

pub struct UiElement<'a> {
    id: String,
    output: Cell<UiOutput>,
    element: Rc<RefCell<dyn UiAttributes + 'a>>,
    enabled: Cell<bool>,
    children: Vec<&'a UiElement<'a>>
}

impl<'a> UiElement<'a> {
    /// element disabled by default
    pub fn new<T: UiAttributes + 'a>(id: impl ToString, element: T) -> UiElement<'a> {
        UiElement {
            id: id.to_string(),
            output: Cell::new(UiOutput::None),
            element: Rc::new(RefCell::new(element)),
            enabled: Cell::new(false),
            children: Vec::new()
        }
    }

    pub fn inner(&self) -> Ref<dyn UiAttributes> {
        self.element.borrow()
    }

    pub fn inner_mut(&self) -> RefMut<dyn UiAttributes> {
        self.element.borrow_mut()
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn add_child(&'a mut self, child: &'a UiElement<'a>) {
        self.children.push(child);
    }

    pub fn enabled(&self) -> bool {
        self.enabled.get().clone()
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.set(enabled);
    }

    pub fn output(&self) -> UiOutput {
        self.element.borrow().output()
    }

    pub fn handle_events(&self, event: &WindowEvent) -> bool {
        if self.element.borrow_mut().handle_events(event) {
            return true;
        }

        false
    }

    pub fn draw(&self, target: &mut Frame) {
        if self.enabled.get() {
            self.element.borrow().draw(target)
        }
    }
}

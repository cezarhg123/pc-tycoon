use std::ops::{Deref, DerefMut};
use delegate::delegate;
use glium::glutin::event::WindowEvent;
use crate::math::vec2::Vec2;

#[derive(Debug, Clone, Copy)]
/// Handles pointers 'safely'
pub struct PtrCell<T: ?Sized>(*mut T);

impl<T: ?Sized> PtrCell<T> {
    pub fn new(value: &mut T) -> PtrCell<T> {
        PtrCell(value as *mut T)
    }

    pub fn new_raw(value: *mut T) -> PtrCell<T> {
        PtrCell(value)
    }

    pub fn into_ptr(self) -> *mut T {
        self.0
    }
}

impl<T: ?Sized> Deref for PtrCell<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {&*self.0}
    }
}

impl<T: ?Sized> DerefMut for PtrCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {&mut *self.0}
    }
}
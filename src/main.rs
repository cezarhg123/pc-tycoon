#![allow(non_upper_case_globals, unused)]
// #![windows_subsystem = "windows"]

use glium::{glutin::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, dpi::LogicalSize, ContextBuilder, event::{WindowEvent, Event}}, Display, Surface};

pub mod game;
pub mod part_loader;
pub mod timer;

fn main() {
    let mut event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(get_window_width::<i32>(), get_window_height::<i32>()))
        .with_title("PC Tycoon");

    let cb = ContextBuilder::new();

    let display = Display::new(wb, cb, &event_loop).unwrap();

    // main loop
    event_loop.run(move |ev, _, control_flow| {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.finish();
        
        *control_flow = ControlFlow::Poll;
        match ev {
            Event::WindowEvent {
                event,
                ..
            } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                _ => return
            }
            _ => {}
        }
    });
}

static mut WINDOW_WIDTH: i32 = 1280;
static mut WINDOW_HEIGHT: i32 = 720;

pub fn get_window_width<T: From<i32>>() -> T {
    unsafe {
        WINDOW_WIDTH.try_into().unwrap()
    }
}

pub fn set_window_width(width: i32) {
    unsafe {
        WINDOW_WIDTH = width;
    }
}

pub fn get_window_height<T: From<i32>>() -> T {
    unsafe {
        WINDOW_HEIGHT.try_into().unwrap()
    }
}

pub fn set_window_height(height: i32) {
    unsafe {
        WINDOW_HEIGHT = height;
    }
}

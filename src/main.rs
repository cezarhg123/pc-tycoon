#![allow(non_upper_case_globals, unused)]
// #![windows_subsystem = "windows"]

use std::{rc::Rc, cell::RefCell, io::Cursor};
use gfx::rect::RectBuilder;
use glium::{glutin::{event_loop::{EventLoop, ControlFlow}, window::{WindowBuilder, Fullscreen}, dpi::{LogicalSize, PhysicalPosition}, ContextBuilder, event::{Event, WindowEvent, VirtualKeyCode, ElementState}}, Display, Surface};
use log::{create_log_file, log};
use math::{vec2::vec2, vec3::vec3, vec4::vec4};
use timer::Timer;
use ui::{Ui, uielement::UiElement, uirect::UiRect};

pub mod timer;
pub mod gfx;
pub mod math;
pub mod ui;
pub mod log;

fn main() {
    create_log_file();

    let mut event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(get_window_width(), get_window_height()))
        .with_decorations(true)
        .with_position(PhysicalPosition::new(0, 0))
        .with_resizable(true)
        .with_title("PC Tycoon");

    let cb = ContextBuilder::new();

    let display = match Display::new(wb, cb, &event_loop) {
        Ok(display) => {
            display
        }
        Err(err) => {
            panic!();
        }
    };

    log("created all necessary objects");

    let mut fullscreen = false;
    // main loop
    event_loop.run(move |ev, _, control_flow| {
        if is_closed() {
            *control_flow = ControlFlow::Exit;
        } else {
            *control_flow = ControlFlow::Poll;
        }
        match ev {
            Event::WindowEvent {
                event,
                ..
            } => if !ui().handle_events(&event) {
                match event {
                    WindowEvent::CloseRequested => close(),
                    WindowEvent::Resized(new_size) => {
                        set_window_width(new_size.width);
                        set_window_height(new_size.height);
                    }
                    WindowEvent::KeyboardInput {input, ..} => {
                        if input.virtual_keycode == Some(VirtualKeyCode::F11) && input.state == ElementState::Pressed {
                            if fullscreen {
                                display.gl_window().window().set_fullscreen(None);
                                fullscreen = false;
                            } else {
                                display.gl_window().window().set_fullscreen(Some(Fullscreen::Borderless(None)));
                                fullscreen = true;
                            }
                        }

                        if input.virtual_keycode == Some(VirtualKeyCode::A) && input.state == ElementState::Pressed {
                            let rect = RectBuilder::new()
                                .with_position(vec2(400.0, 400.0))
                                .build(&display);

                            let rect = ui().add_element(UiElement::new("test", UiRect::new(rect)));
                            rect.set_enabled(true);
                        }
                    }
                    _ => return
                }
            }
            Event::MainEventsCleared => {
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                //drawing
                ui().draw(&mut target);
                target.finish().unwrap();
            }
            _ => {}
        }
    });
}

/// when true, hover the desired element and then press arrow keys.
/// then press enter to print centre position
pub const MOVE_UI: bool = true;
const DEFAULT_WINDOW_WIDTH: u32 = 1280;
const DEFAULT_WINDOW_HEIGHT: u32 = 720;
static mut WINDOW_WIDTH: u32 = DEFAULT_WINDOW_WIDTH;
static mut WINDOW_HEIGHT: u32 = DEFAULT_WINDOW_HEIGHT;

// these are just for me to do maths n shit
pub const DEV_WINDOW_WIDTH: u32 = 1920;
pub const DEV_WINDOW_HEIGHT: u32 = 1080;

pub fn get_window_width() -> u32 {
    unsafe {
        WINDOW_WIDTH.try_into().unwrap()
    }
}

pub fn get_window_height() -> u32 {
    unsafe {
        WINDOW_HEIGHT.try_into().unwrap()
    }
}

pub fn set_window_width(width: u32) {
    unsafe {
        WINDOW_WIDTH = width;
    }
}

pub fn set_window_height(height: u32) {
    unsafe {
        WINDOW_HEIGHT = height;
    }
}

static mut close_window: bool = false;
/// exists so i can close safely and save log from anywhere in the project
pub fn close() {
    unsafe {
        close_window = true;
    }
}

pub fn is_closed() -> bool {
    unsafe {
        close_window
    }
}

static mut ui_context: Ui = Ui::new();
pub fn ui() -> &'static mut Ui<'static> {
    unsafe {
        &mut ui_context
    }
}

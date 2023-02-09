#![allow(non_upper_case_globals, unused)]
// #![windows_subsystem = "windows"]

use gfx::rect::RectBuilder;
use glium::{glutin::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, dpi::{LogicalSize, PhysicalPosition}, ContextBuilder, event::{Event, WindowEvent}}, Display, Surface};
use math::{vec2::vec2, vec3::vec3};
use part_loader::load_parts;
use ui::{set_global_font, set_global_bold_font, textline::TextLineBuilder, uielement::UiElement};
use log::{log, save_log};

pub mod game;
pub mod part_loader;
pub mod timer;
pub mod gfx;
pub mod math;
pub mod ui;
pub mod log;

fn main() {
    load_parts();
    set_global_font("fonts/font.ttf");
    set_global_bold_font("fonts/bold_font.ttf");

    let mut event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(get_window_width(), get_window_height()))
        .with_decorations(false)
        .with_position(PhysicalPosition::new(0, 0))
        .with_resizable(false)
        .with_title("PC Tycoon");

    let cb = ContextBuilder::new();

    log("creating display");
    let display = match Display::new(wb, cb, &event_loop) {
        Ok(display) => {
            log("created display");
            display
        }
        Err(err) => {
            log("CRITICAL ERROR: fucked up creating display");
            log(format!("CRITICAL ERROR: more info for the error above:\n{}", err.to_string()));
            panic!();
        }
    };

    let mut test = RectBuilder {
        position: vec2(960.0, 540.0),
        size: vec2(100.0, 100.0),
        ..Default::default()
    }.build(&display);

    let textline = TextLineBuilder {
        text: "te .st=12 !?".to_string(),
        font_size: 36.0,
        color: vec3(1.0, 1.0, 1.0),
        bold: false,
        position: vec2(400.0, 300.0)
    }.build(&display);
    
    // main loop
    event_loop.run(move |ev, _, control_flow| {
        //timings

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        //drawing
        test.draw(&mut target);
        textline.draw(&mut target);
        target.finish().unwrap();
        
        if is_closed() {
            *control_flow = ControlFlow::Exit;
            save_log();
        } else {
            *control_flow = ControlFlow::Poll;
        }
        match ev {
            Event::WindowEvent {
                event,
                ..
            } => {
                match event {
                    WindowEvent::CloseRequested => close(),
                    _ => return
                }
            }
            _ => {}
        }
    });
}

const DEFAULT_WINDOW_WIDTH: i32 = 1920;
const DEFAULT_WINDOW_HEIGHT: i32 = 1080;
static mut WINDOW_WIDTH: i32 = DEFAULT_WINDOW_WIDTH;
static mut WINDOW_HEIGHT: i32 = DEFAULT_WINDOW_HEIGHT;

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

static mut close_window: bool = false;
/// exists so i can close safely from anywhere in the project
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

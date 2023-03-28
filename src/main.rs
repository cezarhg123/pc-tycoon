#![allow(non_upper_case_globals, unused)]
// #![windows_subsystem = "windows"]

use std::{rc::Rc, cell::RefCell};

use game::{Game, profile::create_encryption_key};
use gfx::rect::RectBuilder;
use glium::{glutin::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, dpi::{LogicalSize, PhysicalPosition}, ContextBuilder, event::{Event, WindowEvent}}, Display, Surface};
use math::{vec2::vec2, vec3::vec3, vec4::vec4};
use part_loader::load_parts;
use timer::Timer;
use ui::{set_global_font, set_global_bold_font, textline::TextLineBuilder, uielement::UiElement, multitextline::MultiTextLineBuilder, Ui, button::{ButtonBuilder, ButtonFace}, listbox::ListboxBuilder};
use log::{log, save_log};

pub mod game;
pub mod part_loader;
pub mod timer;
pub mod gfx;
pub mod math;
pub mod ui;
pub mod log;
pub mod ptrcell;

fn main() {
    load_parts();
    create_encryption_key();
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
    
    let mut game = Game::new(&display);

    // fps shit
    let mut fps_timer = Timer::new();
    let mut frames = 0;
    let mut fps_textline = TextLineBuilder {
        id: "".to_string(),
        text: frames.to_string(),
        font_size: 12.0,
        color: vec3(0.0, 0.0, 0.0),
        bold: false,
        position: vec2(0.0, 0.0)
    }.build(&display);

    // main loop
    event_loop.run(move |ev, _, control_flow| {        
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
            } => if !get_ui_mut().handle_event(&event, &display) {
                match event {
                    WindowEvent::CloseRequested => close(),
                    _ => return
                }
            }
            Event::MainEventsCleared => {
                frames += 1;
                fps_timer.tick();
                if fps_timer.elapsed() >= 1.0 {
                    fps_textline = TextLineBuilder {
                        id: "".to_string(),
                        text: frames.to_string(),
                        font_size: 18.0,
                        color: vec3(0.0, 0.0, 0.0),
                        bold: false,
                        position: vec2(0.0, 0.0)
                    }.build(&display);
                    fps_textline.set_left(0.0);
                    fps_textline.set_bottom(0.0);

                    frames = 0;
                    fps_timer.reset();
                }

                game.run(&display);

                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                //drawing
                game.draw(&mut target);
                fps_textline.draw(&mut target);
                target.finish().unwrap();
            }
            _ => {}
        }
    });
}

/// when true, hover the desired element and then press arrow keys.
/// then press enter to print centre position
pub const MOVE_UI: bool = true;
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

static mut ui: Ui = Ui::new();

pub fn get_ui_mut() -> &'static mut Ui {
    unsafe {
       &mut ui
    }
}

pub fn get_ui() -> &'static Ui {
    unsafe {
        &ui
    }
}

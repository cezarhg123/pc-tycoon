#![allow(non_upper_case_globals, unused)]
// #![windows_subsystem = "windows"]

use std::{io::Cursor, ptr::{null, null_mut}};
use game::{Game, profile::Profile};
use glium::{glutin::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, dpi::{LogicalSize, PhysicalPosition}, ContextBuilder, event::{WindowEvent, Event}}, Display, Surface};
use log::{save_log, log};
use math::{vec2::vec2, vec3::vec3};
use part_loader::load_parts;
use timer::Timer;
use ui::{set_global_font, textline::TextLine, set_global_bold_font, multitextline::{MultiTextLine, TextAlignment}, button::ButtonBuilder};
use glium::backend::glutin::DisplayCreationError::*;

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
    
    let mut fps_timer = Timer::new();
    let mut fps_counter = 0;
    let mut fps_text = TextLine::new("0", vec2(15.0, 6.0));
    fps_text.set_font_size(18.0);
    fps_text.set_color(vec3(0.0, 0.0, 0.0));
    fps_text.set_bold(true);

    // main loop
    event_loop.run(move |ev, _, control_flow| {
        //timings
        fps_timer.tick();
        fps_counter += 1;
        if fps_timer.elapsed() >= 1.0 {
            fps_text.set_text(fps_counter);
            fps_counter = 0;
            fps_timer.reset();
        }
        
        game.run();

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        //drawing
        game.draw(&mut target, &display);
        fps_text.draw(Some(&mut target), &display);
        //finish drawing
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
            } => if !game.handle_event(&event, &display) {
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

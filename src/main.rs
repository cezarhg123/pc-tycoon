#![allow(non_upper_case_globals, unused)]
// #![windows_subsystem = "windows"]

use std::io::Cursor;
use game::Game;
use glium::{glutin::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, dpi::LogicalSize, ContextBuilder, event::{WindowEvent, Event}}, Display, Surface};
use log::{save_log, log};
use math::{vec2::vec2, vec3::vec3};
use part_loader::load_parts;
use timer::Timer;
use ui::{set_global_font, textline::TextLine, set_global_bold_font, multitextline::{MultiTextLine, TextAlignment}};
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

    let game = Game::new(&display);
    
    let mut test = MultiTextLine::new("cheese\nyes\n123123\nlol", vec2(500.0, 500.0), TextAlignment::Middle, &display, None, Some(30.0), true);

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
        
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        //drawing
        game.draw(&mut target);
        fps_text.draw(Some(&mut target), &display);
        test.draw(&mut target, &display);
        //finish drawing
        target.finish().unwrap();
        
        *control_flow = ControlFlow::Poll;
        match ev {
            Event::WindowEvent {
                event,
                ..
            } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    save_log();
                    return;
                }
                _ => return
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

pub fn set_window_width(width: i32) {
    unsafe {
        WINDOW_WIDTH = width;
    }
}

pub fn get_window_height() -> u32 {
    unsafe {
        WINDOW_HEIGHT.try_into().unwrap()
    }
}

pub fn set_window_height(height: i32) {
    unsafe {
        WINDOW_HEIGHT = height;
    }
}

#![allow(non_upper_case_globals, unused)]
// #![windows_subsystem = "windows"]

use std::io::Cursor;
use game::Game;
use glium::{glutin::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, dpi::LogicalSize, ContextBuilder, event::{WindowEvent, Event}}, Display, Surface};
use log::{save_log, log};
use math::vec2::vec2;

pub mod game;
pub mod part_loader;
pub mod timer;
pub mod gfx;
pub mod math;
pub mod ui;
pub mod log;

fn main() {
    let mut event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(get_window_width(), get_window_height()))
        .with_title("PC Tycoon");

    let cb = ContextBuilder::new();

    let display = Display::new(wb, cb, &event_loop).unwrap();

    let game = Game::new(&display);

    // main loop
    event_loop.run(move |ev, _, control_flow| {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        //drawing
        game.draw(&mut target);
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

const DEFAULT_WINDOW_WIDTH: i32 = 1280;
const DEFAULT_WINDOW_HEIGHT: i32 = 720;
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

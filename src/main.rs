#![allow(non_upper_case_globals)]
#![windows_subsystem = "windows"]

pub mod gl;
pub mod gfx;
pub mod ui;
pub mod game;
pub mod part_loader;
pub mod timer;

use game::Game;
use gfx::vectors::{self, vec2::vec2};
use glfw::{Context, Key, Action};
use part_loader::load_parts;
use rusttype::Font;
use timer::Timer;
use ui::Ui;
use vectors::vec3::vec3;

pub const WINDOW_WIDTH: u32 = 1920;
pub const WINDOW_HEIGHT: u32 = 1080;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::Decorated(false));
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    
    let (mut window, events) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "PC Tycoon", glfw::WindowMode::Windowed).unwrap();
    window.set_all_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s));
    unsafe {
        gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let font = Font::try_from_vec(std::fs::read("fonts/font.ttf").unwrap()).unwrap();

    load_parts();
    let ui = Ui::new(font);
    let mut game = Game::new(&ui);

    let mut fps_text = ui.text("0", 20.0, vec3(0, 255, 0), Some(vec2(0.0, 0.0)));
    let mut fps = 0;
    let mut fps_timer = Timer::new();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        if fps_timer.elapsed() >= 1.0 {
            fps_text = ui.text(fps.to_string().as_str(), 20.0, vec3(0, 255, 0), Some(vec2(0.0, 0.0)));
            fps = 0;
            fps_timer.reset();
        } else {
            fps += 1;
            fps_timer.tick();
        }

        if window.get_key(Key::Escape) == Action::Press {
            window.set_should_close(true);
        }

        game.run(&mut window);
        game.draw();
        fps_text.draw();

        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            game.scroll(event);
        }
    }

    game.save();
}

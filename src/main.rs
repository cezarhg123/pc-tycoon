pub mod gl;
pub mod gfx;
pub mod ui;
pub mod game;

use game::Game;
use gfx::{color_rect, vectors::{self, vec2::vec2}, image_rect::ImageRect, texture::Texture, text::Text};
use color_rect::ColorRect;
use glfw::{Context, Key, Action};
use rusttype::Font;
use ui::{Ui, listbox::ListBox};
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

    let ui = Ui::new(font);

    let mut game = Game::new(&ui);

    let texts = vec![
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
        "6".to_string()
    ];
    let mut test = ListBox::new(vec2(1000.0, 500.0), vec2(150.0, 60.0), texts.as_slice(), 20.0, &ui);
    
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        if window.get_key(Key::Escape) == Action::Press {
            window.set_should_close(true);
        }

        game.run(&mut window);
        game.draw();

        test.run(&window, &mut game.scrolls);
        test.draw();

        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            game.scroll(event);
        }
    }
}

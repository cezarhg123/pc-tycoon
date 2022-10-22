pub mod gl;
pub mod gfx;

use gfx::{color_rect, vectors, image_rect::ImageRect, texture::Texture};
use color_rect::ColorRect;
use glfw::{Context, Key, Action};
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

    let test = ColorRect::new(vec3(0.0, 1.0, 1.0), 0.0, 0.0, 100.0, 200.0);
    let test2 = ImageRect::new(Texture::from_path("textures/nigward.png"), 100.0, 0.0, 100.0, 100.0);

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        if window.get_key(Key::Escape) == Action::Press {
            window.set_should_close(true);
        }

        test.draw();
        test2.draw();

        window.swap_buffers();
        glfw.poll_events();
        for (_, _) in glfw::flush_messages(&events) {}
    }
}

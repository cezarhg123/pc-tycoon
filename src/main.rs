mod gl;

use glfw::Context;
use gl::*;

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::Decorated(false));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (mut window, _) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "PC Tycoon", glfw::WindowMode::Windowed).expect("failed to create window you dumb fuck");
    window.make_current();

    let gl = Gl::load_with(|s| window.get_proc_address(s));

    while !window.should_close() {
        glfw.poll_events();

        unsafe {
            gl.ClearColor(0.0, 0.0, 0.0, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
            window.set_should_close(true);
        }

        window.swap_buffers();
    }
}

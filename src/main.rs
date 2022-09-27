pub mod drawable;
pub mod rect;
mod gl;
mod game;
mod timer;

use std::{time::Instant, sync::mpsc::Receiver};

use game::{save_save, Save, load_save, Game};
use imgui_glfw_rs::{glfw::{self, Context}, imgui::{self, ImStr, im_str}};
use timer::Timer;

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::Decorated(false));
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::Visible(true));

    let (mut window, events) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "PC Tycoon", glfw::WindowMode::Windowed).expect("failed to create window you dumb fuck");
    window.set_all_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s));

    unsafe {
        gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
    }

    let mut imgui_context = imgui::Context::create();
    let mut imgui_glfw = imgui_glfw_rs::ImguiGLFW::new(&mut imgui_context, &mut window); 

    let mut game = Game::new();
    game.start();

    let mut fps_timer = Timer::new();
    let mut fps_ticks = 0;

    let mut fps_string = String::new();

    while !window.should_close() {        
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        //TIMINGS
        fps_timer.update();
        fps_ticks += 1;
        if fps_timer.elapsed().as_secs_f32() >= 1.0 {
            fps_string = format!("fps: {}", fps_ticks);
            fps_ticks = 0;
            fps_timer.reset();
        }
        //TIMINGS
        
        let ui = imgui_glfw.frame(&mut window, &mut imgui_context);
        let test = ui.window(unsafe {&ImStr::from_utf8_with_nul_unchecked(b"hello\0")}).build(|| {
            ui.text("text");
        });
        
        if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
            window.set_should_close(true);
        }
        
        game.run(&mut window);
        imgui_glfw.draw(ui, &mut window);
        
        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            imgui_glfw.handle_event(&mut imgui_context, &event);
        }
    }
}

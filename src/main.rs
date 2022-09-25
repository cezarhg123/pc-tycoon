pub mod drawable;
pub mod rect;
mod gl;
mod game;
mod timer;

use std::{time::Instant, sync::mpsc::Receiver};

use egui_backend::{glfw::{Window, self, Context, WindowEvent}, Painter, egui::{CtxRef, epaint::ClippedShape}, EguiInputState};
use egui_glfw_gl as egui_backend;
use game::{save_save, Save, load_save, Game};
use timer::Timer;

const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::Decorated(false));
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::Visible(true));

    let (mut window, events) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "PC Tycoon", glfw::WindowMode::Windowed).expect("failed to create window you dumb fuck");
    window.set_char_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s));

    unsafe {
        gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);
    }

    let mut game = Game::new();
    game.start();
    
    let (mut painter, mut egui_ctx, mut egui_input_state, native_pixels_per_point, start_time) = init_egui(&mut window);

    let mut fps_timer = Timer::new();
    let mut fps_ticks = 0;

    while !window.should_close() {
        egui_input_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_input_state.input.take());
        egui_input_state.input.pixels_per_point = Some(native_pixels_per_point);
        
        
        fps_timer.update();
        fps_ticks += 1;
        if fps_timer.elapsed().as_secs_f32() >= 1.0 {
            fps_ticks = 0;
            fps_timer.reset()
        }
        
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        
        if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
            window.set_should_close(true);
        }
        
        egui_backend::egui::Window::new("egui with glfw").show(&egui_ctx, |ui| {
            ui.label("test");
        });
        
        game.run(&mut window);
        
        let (_, paint_cmds) = egui_ctx.end_frame();
        paint_and_handle_events(&mut egui_ctx, paint_cmds, &mut painter, native_pixels_per_point, &events, &mut egui_input_state);
        

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn init_egui(window: &mut Window) -> (Painter, CtxRef, EguiInputState, f32, Instant) {
    let mut painter = egui_backend::Painter::new(window, WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut egui_ctx = egui_backend::egui::CtxRef::default();
    let (width, height) = window.get_framebuffer_size();

    let native_pixels_per_point = window.get_content_scale().0;

    let mut egui_input_state = egui_backend::EguiInputState::new(egui_backend::egui::RawInput {
        screen_rect: Some(egui_backend::egui::Rect::from_min_size(egui_backend::egui::pos2(0.0, 0.0), egui_backend::egui::vec2(width as f32, height as f32) / native_pixels_per_point)),
        pixels_per_point: Some(native_pixels_per_point),
        ..Default::default()
    });

    let start_time = std::time::Instant::now();

    (painter, egui_ctx, egui_input_state, native_pixels_per_point, start_time)
}

fn paint_and_handle_events(egui_ctx: &mut CtxRef, paint_cmds: Vec<ClippedShape>, painter: &mut Painter, native_pixels_per_point: f32, events: &Receiver<(f64, WindowEvent)>, egui_input_state: &mut EguiInputState ) {
    let paint_jobs = egui_ctx.tessellate(paint_cmds);
    painter.paint_jobs(None, paint_jobs, &egui_ctx.texture(), native_pixels_per_point);
    
    for (_, event) in glfw::flush_messages(events) {
        match event {
            _ => {egui_backend::handle_event(event, egui_input_state);}
        }
    }
}

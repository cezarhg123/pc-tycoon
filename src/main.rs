pub mod vust;
pub mod primitives;
pub mod ui;

use std::{io::Cursor, mem::size_of_val};
use ash::vk;
use gpu_allocator::vulkan::{Allocator, AllocatorCreateDesc, AllocationCreateDesc};
use primitives::{rect::Rect, load_font, text::Text};
use ui::button::Button;
use vust::{vertex::Vertex, transition_image_layout, instance::DrawCall};

pub const WINDOW_WIDTH: u32 = 1920;
pub const WINDOW_HEIGHT: u32 = 1080;
pub const WINDOW_TITLE: &str = "PC Tycoon";

fn main() {
    let mut glfw = glfw::init(|err, msg| {glfw::fail_on_errors(err, msg)}).unwrap();
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::Decorated(false));

    let (mut window, events) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE, glfw::WindowMode::Windowed).unwrap();
    window.set_all_polling(true);

    vust::instance::init(&glfw, &window);
    load_font();

    let mut allocator = Allocator::new(&AllocatorCreateDesc {
        instance: vust::instance::get_instance().clone(),
        device: vust::instance::get_device().clone(),
        physical_device: vust::instance::get_gpu().clone(),
        allocation_sizes: Default::default(),
        buffer_device_address: false,
        debug_settings: Default::default()
    }).unwrap();

    let mut button = Button::builder()
        .dimensions(Rect::builder().width(400.0).height(150.0).center(glm::vec2(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0)))
        .text(Text::builder().text("i like men").font_size(24.0).font_color(glm::vec3(0.0, 0.0, 0.0)))
        .build(&mut allocator);

    while !window.should_close() {
        glfw.poll_events();
        
        for (_, event) in glfw::flush_messages(&events) {
            button.handle_events(event, &mut allocator);
        }

        if button.pressed_once() {
            println!("pressed once");
        }

        vust::instance::reset_command_buffer();
        
        button.draw();

        vust::instance::render_surface();
    }

    unsafe { vust::instance::get_device().device_wait_idle().unwrap() };
}

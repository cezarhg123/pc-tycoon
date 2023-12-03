pub mod vust;
pub mod primitives;
pub mod ui;

use std::{io::Cursor, mem::size_of_val};
use ash::vk;
use gpu_allocator::vulkan::{Allocator, AllocatorCreateDesc, AllocationCreateDesc};
use primitives::{rect::Rect, load_font, text::Text};
use vust::{vertex::Vertex, transition_image_layout, instance::DrawCall};

pub const WINDOW_WIDTH: u32 = 1920;
pub const WINDOW_HEIGHT: u32 = 1080;
pub const WINDOW_TITLE: &str = "PC Tycoon";

fn main() {
    let mut glfw = glfw::init(|err, msg| {glfw::fail_on_errors(err, msg)}).unwrap();
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::Decorated(false));

    let (window, _) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE, glfw::WindowMode::Windowed).unwrap();
    
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

    let mut rect = Rect::builder()
        .left(0.0)
        .top(0.0)
        .width(100.0)
        .height(100.0)
        .name("Rect".to_string())
        .color(glm::vec3(0.0, 0.0, 0.0))
        .build(&mut allocator);
    
    let mut text = Text::builder()
        .text("i like men!".to_string())
        .font_size(24.0)
        .font_color(glm::vec3(1.0, 1.0, 1.0))
        .build(&mut allocator);

    text.rect_mut().set_center(glm::vec2(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0));
    rect.set_width(text.rect().width() + 10.0);
    rect.set_height(text.rect().height() + 10.0);
    rect.set_center(text.rect().center());

    while !window.should_close() {
        glfw.poll_events();

        vust::instance::reset_command_buffer();
        
        rect.draw();
        text.draw();

        vust::instance::render_surface();
    }

    unsafe { vust::instance::get_device().device_wait_idle().unwrap() };
}

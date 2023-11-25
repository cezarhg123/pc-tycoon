pub mod vust;
pub mod primitives;

use std::{io::Cursor, mem::size_of_val};
use ash::vk;
use gpu_allocator::vulkan::{Allocator, AllocatorCreateDesc, AllocationCreateDesc};
use primitives::rect::Rect;
use vust::{vertex::Vertex, transition_image_layout, instance::DrawCall};

pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;
pub const WINDOW_TITLE: &str = "PC Tycoon";

fn main() {
    let mut glfw = glfw::init(|err, msg| {glfw::fail_on_errors(err, msg)}).unwrap();
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

    let (window, _) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE, glfw::WindowMode::Windowed).unwrap();
    
    vust::instance::init(&glfw, &window);

    let mut allocator = Allocator::new(&AllocatorCreateDesc {
        instance: vust::instance::get_instance().clone(),
        device: vust::instance::get_device().clone(),
        physical_device: vust::instance::get_gpu().clone(),
        allocation_sizes: Default::default(),
        buffer_device_address: false,
        debug_settings: Default::default()
    }).unwrap();

    let rect = Rect::builder()
        .left(0.0)
        .top(0.0)
        .width(100.0)
        .height(100.0)
        .name("Rect".to_string())
        .color(glm::vec3(0.0, 1.0, 1.0))
        .build(&mut allocator);
    
    let rect2 = Rect::builder()
        .width(100.0)
        .height(100.0)
        .right(1280.0)
        .top(0.0)
        .name("Texture Rect".to_string())
        .texture(image::load(Cursor::new(std::fs::read("textures/icon.png").unwrap()), image::ImageFormat::Png).unwrap())
        .build(&mut allocator);

    while !window.should_close() {
        glfw.poll_events();

        vust::instance::reset_command_buffer();
        
        rect.draw();
        rect2.draw();

        vust::instance::render_surface();
    }

    unsafe { vust::instance::get_device().device_wait_idle().unwrap() };
}

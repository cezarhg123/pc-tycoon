pub mod vust;

use std::{io::Cursor, mem::size_of_val};

use ash::vk;
use gpu_allocator::vulkan::{Allocator, AllocatorCreateDesc, AllocationCreateDesc};
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

    let vertices = vec![
        Vertex::new(glm::vec2(-1.0, -1.0), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 0.0)),
        Vertex::new(glm::vec2(-1.0, 1.0), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 1.0)),
        Vertex::new(glm::vec2(1.0, 1.0), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 1.0)),

        Vertex::new(glm::vec2(-1.0, -1.0), glm::vec3(1.0, 1.0, 1.0), glm::vec2(0.0, 0.0)),
        Vertex::new(glm::vec2(1.0, 1.0), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 1.0)),
        Vertex::new(glm::vec2(1.0, -1.0), glm::vec3(1.0, 1.0, 1.0), glm::vec2(1.0, 0.0))
    ];

    let (buffer, requirements) = unsafe {
        let buffer = vust::instance::get_device().create_buffer(
            &vk::BufferCreateInfo::builder()
                .size(vertices.len() as u64 * std::mem::size_of::<Vertex>() as u64)
                .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
                .sharing_mode(vk::SharingMode::EXCLUSIVE),
            None
        ).unwrap();

        let requirements = vust::instance::get_device().get_buffer_memory_requirements(buffer);

        (buffer, requirements)
    };

    let allocation = allocator
        .allocate(&AllocationCreateDesc {
            name: "Vertex Buffer",
            requirements,
            location: gpu_allocator::MemoryLocation::CpuToGpu,
            linear: true,
            allocation_scheme: gpu_allocator::vulkan::AllocationScheme::GpuAllocatorManaged
        }).unwrap();

    unsafe {
        vust::instance::get_device().bind_buffer_memory(buffer, allocation.memory(), allocation.offset()).unwrap();
        let ptr = allocation.mapped_ptr().unwrap().as_ptr() as *mut Vertex;

        ptr.copy_from_nonoverlapping(vertices.as_ptr(), vertices.len());
    }

    let (image, image_allocation, view, sampler) = unsafe {
        let image = image::load(
            Cursor::new(std::fs::read("textures/background.png").unwrap()),
            image::ImageFormat::Png
        ).unwrap();
        let image_dims = (image.width(), image.height());

        let raw_byte_buffer = vust::instance::get_device().create_buffer(
            &vk::BufferCreateInfo::builder()
                .size(size_of_val(image.as_bytes()) as u64)
                .usage(vk::BufferUsageFlags::TRANSFER_SRC)
                .sharing_mode(vk::SharingMode::EXCLUSIVE),
            None
        ).unwrap();

        let requirements = vust::instance::get_device().get_buffer_memory_requirements(raw_byte_buffer);

        let raw_byte_buffer_allocation = allocator.allocate(&AllocationCreateDesc {
            name: "Raw Byte Buffer",
            requirements,
            location: gpu_allocator::MemoryLocation::CpuToGpu,
            linear: true,
            allocation_scheme: gpu_allocator::vulkan::AllocationScheme::GpuAllocatorManaged
        }).unwrap();

        vust::instance::get_device().bind_buffer_memory(raw_byte_buffer, raw_byte_buffer_allocation.memory(), raw_byte_buffer_allocation.offset()).unwrap();
        let ptr = raw_byte_buffer_allocation.mapped_ptr().unwrap().as_ptr() as *mut u8;
        ptr.copy_from_nonoverlapping(image.as_bytes().as_ptr(), image.as_bytes().len());

        let image = vust::instance::get_device().create_image(
            &vk::ImageCreateInfo::builder()
                .image_type(vk::ImageType::TYPE_2D)
                .extent(vk::Extent3D {
                    width: image_dims.0,
                    height: image_dims.1,
                    depth: 1
                })
                .mip_levels(1)
                .array_layers(1)
                .format(vk::Format::R8G8B8A8_SRGB)
                .tiling(vk::ImageTiling::OPTIMAL)
                .initial_layout(vk::ImageLayout::UNDEFINED)
                .usage(vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED)
                .sharing_mode(vk::SharingMode::EXCLUSIVE)
                .samples(vk::SampleCountFlags::TYPE_1),
            None
        ).unwrap();

        let requirements = vust::instance::get_device().get_image_memory_requirements(image);

        let image_allocation = allocator.allocate(&AllocationCreateDesc {
            name: "Image",
            requirements,
            location: gpu_allocator::MemoryLocation::GpuOnly,
            linear: true,
            allocation_scheme: gpu_allocator::vulkan::AllocationScheme::GpuAllocatorManaged
        }).unwrap();

        vust::instance::get_device().bind_image_memory(image, image_allocation.memory(), image_allocation.offset()).unwrap();

        transition_image_layout(image, vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL);
        let copy_command_buffer = vust::instance::begin_single_exec_command();
        vust::instance::get_device().cmd_copy_buffer_to_image(
            copy_command_buffer,
            raw_byte_buffer,
            image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &[vk::BufferImageCopy {
                buffer_offset: 0,
                buffer_row_length: 0,
                buffer_image_height: 0,
                image_subresource: vk::ImageSubresourceLayers {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    mip_level: 0,
                    base_array_layer: 0,
                    layer_count: 1
                },
                image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                image_extent: vk::Extent3D { width: image_dims.0, height: image_dims.1, depth: 1 }
            }]
        );
        vust::instance::end_single_exec_command(copy_command_buffer);
        transition_image_layout(image, vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);

        let view = vust::instance::get_device().create_image_view(
            &vk::ImageViewCreateInfo::builder()
                .image(image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(vk::Format::R8G8B8A8_SRGB)
                .subresource_range(
                    vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1
                    }
                )
                .build(),
            None
        ).unwrap();

        let sampler = vust::instance::get_device().create_sampler(
            &vk::SamplerCreateInfo::builder()
                .mag_filter(vk::Filter::LINEAR)
                .min_filter(vk::Filter::LINEAR)
                .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
                .address_mode_u(vk::SamplerAddressMode::REPEAT)
                .address_mode_v(vk::SamplerAddressMode::REPEAT)
                .address_mode_w(vk::SamplerAddressMode::REPEAT)
                .anisotropy_enable(true)
                .max_anisotropy(16.0)
                .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
                .unnormalized_coordinates(false)
                .compare_enable(false)
                .compare_op(vk::CompareOp::ALWAYS)
                .mip_lod_bias(0.0)
                .min_lod(0.0)
                .max_lod(0.0)
                .build(),
            None
        ).unwrap();

        (image, image_allocation, view, sampler)
    };

    let descriptor_pool = vust::instance::create_descriptor_pool();

    let descriptor_set = unsafe {
        let descriptor_set_layout = [*vust::instance::get_descriptor_set_layout()];

        vust::instance::get_device().allocate_descriptor_sets(
            &vk::DescriptorSetAllocateInfo::builder()
                .descriptor_pool(descriptor_pool)
                .set_layouts(&descriptor_set_layout)
                .build()
        ).unwrap()[0]
    };

    while !window.should_close() {
        glfw.poll_events();

        vust::instance::reset_command_buffer();
        
        unsafe {
            vust::instance::get_device().update_descriptor_sets(
                &[vk::WriteDescriptorSet::builder()
                    .dst_set(descriptor_set)
                    .dst_binding(0)
                    .dst_array_element(0)
                    .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                    .image_info(&[vk::DescriptorImageInfo {
                        image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                        image_view: view,
                        sampler: sampler
                    }])
                    .build()],
                &[]
            );
        }

        vust::instance::draw(DrawCall {
            buffer,
            descriptor_set,
            vertex_count: 6
        });

        vust::instance::render_surface();
    }

    unsafe { vust::instance::get_device().device_wait_idle().unwrap() };
}

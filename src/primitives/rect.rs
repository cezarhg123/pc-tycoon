use std::mem::{size_of_val, size_of};
use ash::vk;
use gpu_allocator::vulkan::{Allocator, AllocationCreateDesc, Allocation};
use image::{DynamicImage, GenericImage, Rgba};
use crate::{vust::{vertex::Vertex, self, transition_image_layout, instance::DrawCall}, WINDOW_WIDTH, WINDOW_HEIGHT};

pub struct Rect {
    pub(super) left: f32,
    pub(super) top: f32,
    pub(super) width: f32,
    pub(super) height: f32,
    /// Name is only used for debug purposes
    pub(super) name: String,
    pub(super) color: glm::Vec3,
    pub(super) vertex_buffer: (vk::Buffer, Allocation),
    pub(super) image: (vk::Image, Allocation, vk::ImageView, vk::Sampler),
    pub(super) descriptor_pool: vk::DescriptorPool,
    pub(super) descriptor_set: vk::DescriptorSet,
    pub(super) uniform: (vk::Buffer, Allocation)
}

impl Rect {
    pub fn builder() -> RectBuilder {
        RectBuilder {
            left: 0.0,
            top: 0.0,
            width: 0.0,
            height: 0.0,
            name: String::new(),
            color: glm::vec3(1.0, 1.0, 1.0),
            texture: None
        }
    }

    pub fn update_descriptor_set(&self) {
        unsafe {
            let device = vust::instance::get_device();
            device.update_descriptor_sets(
                &[
                    vk::WriteDescriptorSet::builder()
                        .dst_set(self.descriptor_set)
                        .dst_binding(0)
                        .dst_array_element(0)
                        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                        .image_info(&[
                            vk::DescriptorImageInfo::builder()
                                .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                                .image_view(self.image.2)
                                .sampler(self.image.3)
                                .build()
                        ])
                        .build(),
                    vk::WriteDescriptorSet::builder()
                        .dst_set(self.descriptor_set)
                        .dst_binding(1)
                        .dst_array_element(0)
                        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                        .buffer_info(&[
                            vk::DescriptorBufferInfo::builder()
                                .buffer(self.uniform.0)
                                .offset(0)
                                .range(size_of::<RectUniform>() as u64)
                                .build()
                        ])
                        .build()
                ],
                &[]
            );
        }
    }

    pub fn draw(&self) {
        vust::instance::draw(DrawCall {
            buffer: self.vertex_buffer.0,
            descriptor_set: self.descriptor_set,
            vertex_count: 6
        });
    }
}

pub struct RectBuilder {
    left: f32,
    top: f32,
    width: f32,
    height: f32,
    /// Name is only used for debug purposes
    name: String,
    color: glm::Vec3,
    texture: Option<DynamicImage>
}

impl RectBuilder {
    pub fn left(mut self, left: f32) -> RectBuilder {
        self.left = left;
        self
    }

    pub fn top(mut self, top: f32) -> RectBuilder {
        self.top = top;
        self
    }

    pub fn width(mut self, width: f32) -> RectBuilder {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> RectBuilder {
        self.height = height;
        self
    }

    pub fn right(mut self, right: f32) -> RectBuilder {
        self.left = right - self.width;
        self
    }

    pub fn bottom(mut self, bottom: f32) -> RectBuilder {
        self.top = bottom - self.height;
        self
    }

    pub fn center(mut self, center: glm::Vec2) -> RectBuilder {
        self.left = center.x - self.width / 2.0; // might be undefined behavior if width is 0 but i trust rust
        self.top = center.y - self.height / 2.0;
        self
    }

    /// Name is only used for debug purposes
    pub fn name(mut self, name: String) -> RectBuilder {
        self.name = name;
        self
    }

    pub fn color(mut self, color: glm::Vec3) -> RectBuilder {
        self.color = color;
        self
    }

    pub fn texture(mut self, texture: DynamicImage) -> RectBuilder {
        self.texture = Some(texture);
        self
    }

    pub fn build(self, allocator: &mut Allocator) -> Rect {
        let name = if cfg!(debug_assertions) {
            self.name
        } else {
            // save some space on release
            String::new()
        };

        // future optimization
        // since the position, color and size of rect are done dynamically with uniforms
        // this could allow just one vertex buffer to be created and then draw it each time with different uniforms
        let (vertex_buffer, vertex_buffer_allocation) = unsafe {
            let device = vust::instance::get_device();
            let buffer = device.create_buffer(
                &vk::BufferCreateInfo::builder()
                    .size(RECT_VERTICES_SIZE as u64)
                    .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
                    .sharing_mode(vk::SharingMode::EXCLUSIVE),
                None
            ).unwrap();

            let requirements = device.get_buffer_memory_requirements(buffer);

            let vertex_buffer_allocation = allocator.allocate(
                &AllocationCreateDesc {
                    name: name.as_str(),
                    requirements,
                    location: gpu_allocator::MemoryLocation::CpuToGpu,
                    linear: true,
                    allocation_scheme: gpu_allocator::vulkan::AllocationScheme::GpuAllocatorManaged
                }
            ).unwrap();

            device.bind_buffer_memory(buffer, vertex_buffer_allocation.memory(), vertex_buffer_allocation.offset()).unwrap();

            let ptr = vertex_buffer_allocation.mapped_ptr().unwrap().as_ptr() as *mut Vertex;
            ptr.copy_from_nonoverlapping(RECT_VERTICES.as_ptr(), RECT_VERTICES.len());

            (buffer, vertex_buffer_allocation)
        };

        let dynamic_image = match self.texture {
            Some(image) => image.to_rgba8(),
            None => {
                let mut image = DynamicImage::new_rgba8(1, 1).to_rgba8();
                image.put_pixel(0, 0, Rgba([255, 255, 255, 255]));
                image
            }
        };
        let dynamic_image_bytes = dynamic_image.as_raw();

        let (image, image_buffer_allocation, view, sampler) = unsafe {
            let device = vust::instance::get_device();

            let raw_byte_buffer = device.create_buffer(
                &vk::BufferCreateInfo::builder()
                    .size(size_of_val(dynamic_image_bytes) as u64)
                    .usage(vk::BufferUsageFlags::TRANSFER_SRC)
                    .sharing_mode(vk::SharingMode::EXCLUSIVE),
                None
            ).unwrap();

            let requirements = device.get_buffer_memory_requirements(raw_byte_buffer);

            let raw_byte_buffer_allocation = allocator.allocate(
                &AllocationCreateDesc {
                    name: name.as_str(),
                    requirements,
                    location: gpu_allocator::MemoryLocation::CpuToGpu,
                    linear: true,
                    allocation_scheme: gpu_allocator::vulkan::AllocationScheme::GpuAllocatorManaged
                }
            ).unwrap();

            device.bind_buffer_memory(raw_byte_buffer, raw_byte_buffer_allocation.memory(), raw_byte_buffer_allocation.offset()).unwrap();

            let ptr = raw_byte_buffer_allocation.mapped_ptr().unwrap().as_ptr() as *mut u8;
            ptr.copy_from_nonoverlapping(dynamic_image_bytes.as_ptr(), dynamic_image_bytes.len());

            let image = device.create_image(
                &vk::ImageCreateInfo::builder()
                    .image_type(vk::ImageType::TYPE_2D)
                    .extent(vk::Extent3D {
                        width: dynamic_image.width(),
                        height: dynamic_image.height(),
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

            let requirements = device.get_image_memory_requirements(image);

            let image_buffer_allocation = allocator.allocate(
                &AllocationCreateDesc {
                    name: name.as_str(),
                    requirements,
                    location: gpu_allocator::MemoryLocation::GpuOnly,
                    linear: true,
                    allocation_scheme: gpu_allocator::vulkan::AllocationScheme::GpuAllocatorManaged
                }
            ).unwrap();

            device.bind_image_memory(image, image_buffer_allocation.memory(), image_buffer_allocation.offset()).unwrap();

            transition_image_layout(image, vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL);
            let copy_command_buffer = vust::instance::begin_single_exec_command();
            device.cmd_copy_buffer_to_image(
                copy_command_buffer,
                raw_byte_buffer,
                image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[
                    vk::BufferImageCopy::builder()
                        .buffer_offset(0)
                        .buffer_row_length(0)
                        .buffer_image_height(0)
                        .image_subresource(vk::ImageSubresourceLayers {
                            aspect_mask: vk::ImageAspectFlags::COLOR,
                            mip_level: 0,
                            base_array_layer: 0,
                            layer_count: 1
                        })
                        .image_offset(vk::Offset3D { x: 0, y: 0, z: 0 })
                        .image_extent(vk::Extent3D {
                            width: dynamic_image.width(),
                            height: dynamic_image.height(),
                            depth: 1
                        })
                        .build()
                ]
            );
            vust::instance::end_single_exec_command(copy_command_buffer);

            transition_image_layout(image, vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);

            let view = device.create_image_view(
                &vk::ImageViewCreateInfo::builder()
                    .image(image)
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(vk::Format::R8G8B8A8_SRGB)
                    .subresource_range(
                        vk::ImageSubresourceRange::builder()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .base_mip_level(0)
                            .level_count(1)
                            .base_array_layer(0)
                            .layer_count(1)
                            .build()
                    )
                    .build(),
                None
            ).unwrap();

            let sampler = device.create_sampler(
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

            (image, image_buffer_allocation, view, sampler)
        };

        let descriptor_pool = vust::instance::create_descriptor_pool();

        let descriptor_set = unsafe {
            vust::instance::get_device().allocate_descriptor_sets(
                &vk::DescriptorSetAllocateInfo::builder()
                    .descriptor_pool(descriptor_pool)
                    .set_layouts(&[*vust::instance::get_descriptor_set_layout()])
                    .build()
            ).unwrap()[0]
        };

        let position = glm::vec2(
            ((self.left / WINDOW_WIDTH as f32) + (self.width / 2.0 / WINDOW_WIDTH as f32)) * 2.0 - 1.0,
            ((self.top / WINDOW_HEIGHT as f32) + (self.height / 2.0 / WINDOW_HEIGHT as f32)) * 2.0 - 1.0
        );

        let size = glm::vec2(
            self.width / WINDOW_WIDTH as f32,
            self.height / WINDOW_HEIGHT as f32
        );
        
        let (uniform_buffer, uniform_buffer_allocation) = unsafe {
            let device = vust::instance::get_device();
            let uniform_buffer = device.create_buffer(
                &vk::BufferCreateInfo::builder()
                    .size(size_of::<RectUniform>() as u64)
                    .usage(vk::BufferUsageFlags::UNIFORM_BUFFER)
                    .sharing_mode(vk::SharingMode::EXCLUSIVE),
                None
            ).unwrap();

            let requirements = device.get_buffer_memory_requirements(uniform_buffer);

            let uniform_buffer_allocation = allocator.allocate(
                &AllocationCreateDesc {
                    name: name.as_str(),
                    requirements,
                    location: gpu_allocator::MemoryLocation::CpuToGpu,
                    linear: true,
                    allocation_scheme: gpu_allocator::vulkan::AllocationScheme::GpuAllocatorManaged
                }
            ).unwrap();

            device.bind_buffer_memory(uniform_buffer, uniform_buffer_allocation.memory(), uniform_buffer_allocation.offset()).unwrap();

            let ptr = uniform_buffer_allocation.mapped_ptr().unwrap().as_ptr() as *mut RectUniform;
            ptr.write(RectUniform::new(
                position,
                self.color,
                size
            ));

            device.update_descriptor_sets(
                &[
                    vk::WriteDescriptorSet::builder()
                        .dst_set(descriptor_set)
                        .dst_binding(0)
                        .dst_array_element(0)
                        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                        .image_info(&[
                            vk::DescriptorImageInfo::builder()
                                .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                                .image_view(view)
                                .sampler(sampler)
                                .build()
                        ])
                        .build(),
                    vk::WriteDescriptorSet::builder()
                        .dst_set(descriptor_set)
                        .dst_binding(1)
                        .dst_array_element(0)
                        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                        .buffer_info(&[
                            vk::DescriptorBufferInfo::builder()
                                .buffer(uniform_buffer)
                                .offset(0)
                                .range(size_of::<RectUniform>() as u64)
                                .build()
                        ])
                        .build()
                ],
                &[]
            );

            (uniform_buffer, uniform_buffer_allocation)
        };

        Rect {
            left: self.left,
            top: self.top,
            width: self.width,
            height: self.height,
            name,
            color: self.color,
            vertex_buffer: (vertex_buffer, vertex_buffer_allocation),
            image: (image, image_buffer_allocation, view, sampler),
            descriptor_pool,
            descriptor_set,
            uniform: (uniform_buffer, uniform_buffer_allocation)
        }
    }
}

const RECT_VERTICES: [Vertex; 6] = [
    Vertex {
        position: [-1.0, -1.0],
        uv: [0.0, 0.0]
    },
    Vertex {
        position: [-1.0, 1.0],
        uv: [0.0, 1.0]
    },
    Vertex {
        position: [1.0, -1.0],
        uv: [1.0, 0.0]
    },

    Vertex {
        position: [1.0, -1.0],
        uv: [1.0, 0.0]
    },
    Vertex {
        position: [-1.0, 1.0],
        uv: [0.0, 1.0]
    },
    Vertex {
        position: [1.0, 1.0],
        uv: [1.0, 1.0]
    },
];

const RECT_VERTICES_SIZE: usize = size_of::<Vertex>() * 6;

#[repr(C)]
struct RectUniform {
    pub position: glm::Vec2,
    padding: [f32; 2],
    pub color: glm::Vec3,
    padding2: [f32; 1],
    pub size: glm::Vec2
}

impl RectUniform {
    fn new(position: glm::Vec2, color: glm::Vec3, size: glm::Vec2) -> RectUniform {
        RectUniform {
            position,
            padding: [0.0, 0.0],
            color,
            padding2: [0.0],
            size
        }
    }
}
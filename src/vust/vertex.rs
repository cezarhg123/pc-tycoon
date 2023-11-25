use std::mem::size_of;
use ash::vk;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 2],
    pub uv: [f32; 2]
}

impl Vertex {
    pub fn new(position: glm::Vec2, uv: glm::Vec2) -> Vertex {
        Vertex {
            position: [position.x, position.y],
            uv: [uv.x, uv.y]
        }
    }

    pub const fn get_binding_description() -> [vk::VertexInputBindingDescription; 1] {
        [
            vk::VertexInputBindingDescription {
                binding: 0,
                stride: size_of::<Vertex>() as u32,
                input_rate: vk::VertexInputRate::VERTEX
            }
        ]
    }

    pub const fn get_attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        [
            vk::VertexInputAttributeDescription {
                binding: 0,
                location: 0,
                format: vk::Format::R32G32_SFLOAT,
                offset: 0
            },
            vk::VertexInputAttributeDescription {
                binding: 0,
                location: 1,
                format: vk::Format::R32G32_SFLOAT,
                offset: size_of::<f32>() as u32 * 2
            }
        ]
    }
}

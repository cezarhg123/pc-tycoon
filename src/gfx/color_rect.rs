use std::mem::{size_of_val, size_of};
use crate::gl;

use super::{vertexs::{ColorVertex, color_vertex}, vectors::{vec3::Vec3, vec2::vec2}};


#[derive(Debug, Clone)]
pub struct ColorRect {
    vertices: Vec<ColorVertex>,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    width: f32,
    height: f32,
    program: u32,
    vao: u32,
    vbo: u32
}

impl ColorRect {
    pub fn new(color: Vec3<f32>, left: f32, top: f32, width: f32, height: f32) -> ColorRect {
        let right = left + width;
        let bottom = top + height;

        let vertices = vec![
            color_vertex(vec2(left, bottom), color),
            color_vertex(vec2(left, top), color),
            color_vertex(vec2(right, top), color),

            color_vertex(vec2(left, bottom), color),
            color_vertex(vec2(right, top), color),
            color_vertex(vec2(right, bottom), color)
        ];

        let program;
        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            let vert_src = std::fs::read_to_string("shaders/color.vert").unwrap();
            let frag_src = std::fs::read_to_string("shaders/color.frag").unwrap();

            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader, 1, &(vert_src.as_bytes().as_ptr().cast()), &(vert_src.len().try_into().unwrap()));
            gl::CompileShader(vertex_shader);

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_shader, 1, &(frag_src.as_bytes().as_ptr().cast()), &(frag_src.len().try_into().unwrap()));
            gl::CompileShader(fragment_shader);

            program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, size_of_val(vertices.as_slice()) as isize, vertices.as_ptr().cast(), gl::STATIC_DRAW);

            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, size_of::<ColorVertex>() as i32, 0 as *const _);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, size_of::<ColorVertex>() as i32, 8 as *const _);
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        ColorRect {
            vertices,
            left,
            top,
            right,
            bottom,
            width,
            height,
            program,
            vao,
            vbo
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::UseProgram(self.program);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
        }
    }
}

impl Drop for ColorRect {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

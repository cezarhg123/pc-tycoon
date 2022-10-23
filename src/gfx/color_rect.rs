use std::mem::{size_of_val, size_of};
use glfw::Window;

use crate::gl;
use super::{vertexs::{ColorVertex, color_vertex}, vectors::{vec3::Vec3, vec2::{vec2, Vec2}}};


#[derive(Debug, Clone)]
pub struct ColorRect {
    color: Vec3<f32>,
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
            color,
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
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }

    pub fn get_left(&self) -> f32 {
        self.left
    }

    pub fn set_left(&mut self, left: f32) {
        self.left = left;
        self.right = left + self.width;

        let vertices = vec![
            color_vertex(vec2(self.left, self.bottom), self.color),
            color_vertex(vec2(self.left, self.top), self.color),
            color_vertex(vec2(self.right, self.top), self.color),

            color_vertex(vec2(self.left, self.bottom), self.color),
            color_vertex(vec2(self.right, self.top), self.color),
            color_vertex(vec2(self.right, self.bottom), self.color)
        ];

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, size_of_val(vertices.as_slice()) as isize, vertices.as_ptr().cast(), gl::STATIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn get_top(&self) -> f32 {
        self.top
    }

    pub fn set_top(&mut self, top: f32) {
        self.top = top;
        self.bottom = top + self.height;

        let vertices = vec![
            color_vertex(vec2(self.left, self.bottom), self.color),
            color_vertex(vec2(self.left, self.top), self.color),
            color_vertex(vec2(self.right, self.top), self.color),

            color_vertex(vec2(self.left, self.bottom), self.color),
            color_vertex(vec2(self.right, self.top), self.color),
            color_vertex(vec2(self.right, self.bottom), self.color)
        ];

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, size_of_val(vertices.as_slice()) as isize, vertices.as_ptr().cast(), gl::STATIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn get_right(&self) -> f32 {
        self.right
    }

    pub fn set_right(&mut self, right: f32) {
        self.right = right;
        self.left = right - self.width;

        let vertices = vec![
            color_vertex(vec2(self.left, self.bottom), self.color),
            color_vertex(vec2(self.left, self.top), self.color),
            color_vertex(vec2(self.right, self.top), self.color),

            color_vertex(vec2(self.left, self.bottom), self.color),
            color_vertex(vec2(self.right, self.top), self.color),
            color_vertex(vec2(self.right, self.bottom), self.color)
        ];

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, size_of_val(vertices.as_slice()) as isize, vertices.as_ptr().cast(), gl::STATIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn get_bottom(&self) -> f32 {
        self.bottom
    }

    pub fn set_bottom(&mut self, bottom: f32) {
        self.bottom = bottom;
        self.top = bottom - self.height;

        let vertices = vec![
            color_vertex(vec2(self.left, self.bottom), self.color),
            color_vertex(vec2(self.left, self.top), self.color),
            color_vertex(vec2(self.right, self.top), self.color),

            color_vertex(vec2(self.left, self.bottom), self.color),
            color_vertex(vec2(self.right, self.top), self.color),
            color_vertex(vec2(self.right, self.bottom), self.color)
        ];

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, size_of_val(vertices.as_slice()) as isize, vertices.as_ptr().cast(), gl::STATIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
        self.right = self.left + width;

        let vertices = vec![
            color_vertex(vec2(self.left, self.bottom), self.color),
            color_vertex(vec2(self.left, self.top), self.color),
            color_vertex(vec2(self.right, self.top), self.color),

            color_vertex(vec2(self.left, self.bottom), self.color),
            color_vertex(vec2(self.right, self.top), self.color),
            color_vertex(vec2(self.right, self.bottom), self.color)
        ];

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, size_of_val(vertices.as_slice()) as isize, vertices.as_ptr().cast(), gl::STATIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
        self.bottom = self.top + height;

        let vertices = vec![
            color_vertex(vec2(self.left, self.bottom), self.color),
            color_vertex(vec2(self.left, self.top), self.color),
            color_vertex(vec2(self.right, self.top), self.color),

            color_vertex(vec2(self.left, self.bottom), self.color),
            color_vertex(vec2(self.right, self.top), self.color),
            color_vertex(vec2(self.right, self.bottom), self.color)
        ];

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, size_of_val(vertices.as_slice()) as isize, vertices.as_ptr().cast(), gl::STATIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn get_color(&self) -> Vec3<f32> {
        self.color
    }

    pub fn get_center(&self) -> Vec2<f32> {
        let x = self.left + (self.width / 2.0);
        let y = self.top + (self.height / 2.0);
        vec2(x, y)
    }

    pub fn set_center(&mut self, center: Vec2<f32>) {
        let left = center.x - (self.width / 2.0);
        let top = center.y - (self.height / 2.0);

        self.set_left(left);
        self.set_top(top);
    }

    pub fn set_color(&mut self, color: Vec3<f32>) {
        self.color = color;

        let vertices = vec![
            color_vertex(vec2(self.left, self.bottom), self.color),
            color_vertex(vec2(self.left, self.top), self.color),
            color_vertex(vec2(self.right, self.top), self.color),

            color_vertex(vec2(self.left, self.bottom), self.color),
            color_vertex(vec2(self.right, self.top), self.color),
            color_vertex(vec2(self.right, self.bottom), self.color)
        ];

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, size_of_val(vertices.as_slice()) as isize, vertices.as_ptr().cast(), gl::STATIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn contains(&self, pos: Vec2<f32>) -> bool {
        if pos.x >= self.left && pos.x <= self.right {
            if pos.y >= self.top && pos.y <= self.bottom {
                return true;
            }
        }

        false
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

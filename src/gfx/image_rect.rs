use std::mem::{size_of_val, size_of};

use crate::gl;

use super::{vertexs::{UvVertex, uv_vertex}, texture::Texture, vectors::vec2::vec2};

#[derive(Debug, Clone)]
pub struct ImageRect {
    vertices: Vec<UvVertex>,
    texture: Texture,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    width: f32,
    height: f32,
    program: u32,
    vao: u32,
    vbo: u32,
    texture_id: u32
}

impl ImageRect {
    pub fn new(texture: Texture, left: f32, top: f32, width: f32, height: f32) -> ImageRect {
        let right = left + width;
        let bottom = top + height;

        let vertices = vec![
            uv_vertex(vec2(left, bottom), vec2(0.0, 0.0)),
            uv_vertex(vec2(left, top), vec2(0.0, 1.0)),
            uv_vertex(vec2(right, top), vec2(1.0, 1.0)),

            uv_vertex(vec2(left, bottom), vec2(0.0, 0.0)),
            uv_vertex(vec2(right, top), vec2(1.0, 1.0)),
            uv_vertex(vec2(right, bottom), vec2(1.0, 0.0))
        ];

        let program;
        let mut vao = 0;
        let mut vbo = 0;
        let mut texture_id = 0;

        unsafe {
            let vert_src = std::fs::read_to_string("shaders/image.vert").unwrap();
            let frag_src = std::fs::read_to_string("shaders/image.frag").unwrap();

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

            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, size_of::<UvVertex>() as i32, 0 as *const _);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, size_of::<UvVertex>() as i32, 8 as *const _);
            gl::EnableVertexAttribArray(1);

            gl::GenTextures(1, &mut texture_id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, texture.width as i32, texture.height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, texture.buffer.as_raw().as_ptr().cast());
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::UseProgram(program);
            gl::Uniform1i(gl::GetUniformLocation(program, "tex".as_ptr().cast()), gl::TEXTURE0 as i32);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        ImageRect {
            vertices,
            texture,
            left,
            top,
            right,
            bottom,
            width,
            height,
            program,
            vao,
            vbo,
            texture_id
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::UseProgram(self.program);
            gl::BindVertexArray(self.vao);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
        }
    }
}

impl Drop for ImageRect {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}

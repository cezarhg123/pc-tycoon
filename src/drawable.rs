use std::{mem::{size_of, size_of_val}, io::Cursor, convert::TryInto};

use image::GenericImageView;

use crate::gl;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub uv: [f32; 2]
}

#[derive(Debug, Clone)]
pub struct Drawable {
    vao: u32,
    vbo: u32,
    shader: u32,
    texture: u32,
    count: u32
}

impl Drawable {
    pub fn new(image_path: &str) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let shader;
        let mut texture = 0;
        
        let image = image::load(
            Cursor::new(
                std::fs::read(image_path).unwrap()
            ),
            image::ImageFormat::Png
        ).unwrap();
        let image = image.flipv();
        let dims = image.dimensions();
        let image = image.into_rgba8();
        let bytes = image.into_raw();

        unsafe {
            shader = create_shader_program("shaders/default.vert", "shaders/default.frag");

            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, size_of::<Vertex>() as i32, 0 as *const _);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, size_of::<Vertex>() as i32, 8 as *const _);
            gl::EnableVertexAttribArray(1);

            gl::GenTextures(1, &mut texture);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, dims.0 as i32, dims.1 as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, bytes.as_ptr().cast());
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::UseProgram(shader);
            gl::Uniform1i(gl::GetUniformLocation(shader, "tex".as_ptr().cast()), gl::TEXTURE0 as i32);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Drawable {
            vao,
            vbo,
            shader,
            texture,
            count: 0
        }
    }

    pub fn set_vbo(&mut self, vertices: &Vec<Vertex>) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, size_of_val(vertices.as_slice()) as isize, vertices.as_ptr().cast(), gl::STATIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        self.count = vertices.len() as u32;
    }

    pub fn draw(&self) {
        unsafe {
            gl::UseProgram(self.shader);
            gl::BindVertexArray(self.vao);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::DrawArrays(gl::TRIANGLES, 0, self.count as i32);
        }
    }
}

fn create_shader_program(vertex_path: &str, fragment_path: &str) -> u32 {
    let vertex_source = std::fs::read_to_string(vertex_path).unwrap();
    let fragment_source = std::fs::read_to_string(fragment_path).unwrap();

    let program;

    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &(vertex_source.as_ptr().cast()), &(vertex_source.len().try_into().unwrap()));
        gl::CompileShader(vertex_shader);

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, &(fragment_source.as_ptr().cast()), &(fragment_source.len().try_into().unwrap()));
        gl::CompileShader(fragment_shader);

        program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    program
}

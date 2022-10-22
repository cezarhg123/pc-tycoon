use std::io::Cursor;
use image::{ImageBuffer, Rgba, GenericImageView};

#[derive(Debug, Clone)]
pub struct Texture {
    pub buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub width: u32,
    pub height: u32
}

impl Texture {
    pub fn from_path(path: &str) -> Texture {
        let image = image::load(Cursor::new(std::fs::read(path).unwrap()), image::ImageFormat::Png).unwrap();
        let dims = image.dimensions();
        let image = image.flipv();
        let image = image.into_rgba8();

        Texture {
            buffer: image,
            width: dims.0,
            height: dims.1
        }
    }

    pub fn from_memory(buffer: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Texture {
        Texture {
            buffer: buffer.clone(),
            width: buffer.width(),
            height: buffer.height()
        }
    }
}

use vector_x::Vector2;

use crate::imagehandler::SvgHandle;

pub struct ImageData {
    pixels: Vec<u8>,
    size: Vector2<u32>,
}

impl ImageData {
    pub fn new(pixels: Vec<u8>, size: Vector2<u32>) -> Self {
        return Self {
            pixels: pixels,
            size: size,
        };
    }
}

impl Into<SvgHandle> for ImageData {
    fn into(self) -> SvgHandle {
        return SvgHandle::from_memory(self.pixels);
    }
}

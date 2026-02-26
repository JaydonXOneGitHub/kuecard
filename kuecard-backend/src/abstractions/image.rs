use vector_x::Vector2;

use crate::imagehandler::ImageHandle;

pub struct ImageData {
    pixels: Vec<u8>,
    size: Vector2<u32>
}

impl ImageData {
    pub fn new(pixels: Vec<u8>, size: Vector2<u32>) -> Self {
        return Self {
            pixels: pixels,
            size: size
        };
    }
}

impl ImageData {
    pub fn make_handle(&self) -> ImageHandle {
        return ImageHandle::from_rgba(
            self.size.one, 
            self.size.two, 
            self.pixels.clone()
        );
    }
}

impl Into<ImageHandle> for ImageData {
    fn into(self) -> ImageHandle {
        return ImageHandle::from_rgba(
            self.size.one, 
            self.size.two, 
            self.pixels
        );
    }
}
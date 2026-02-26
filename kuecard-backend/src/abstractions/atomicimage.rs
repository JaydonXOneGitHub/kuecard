use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use vector_x::Vector2;

use crate::abstractions::ImageData;

pub struct AtomicImageData {
    pub image: Arc<Mutex<ImageData>>
}

impl AtomicImageData {
    pub fn new(pixels: Vec<u8>, size: Vector2<u32>) -> Self {
        return Self {
            image: Arc::new(Mutex::new(ImageData::new(pixels, size)))
        };
    }
}

impl AtomicImageData {
    pub fn try_use_image_data<T>(
        &self, 
        callback: impl FnOnce(MutexGuard<'_, ImageData>) -> T
    ) -> Result<T, PoisonError<MutexGuard<'_, ImageData>>> {
        let res = self.image.lock();

        if res.is_err() {
            return Result::Err(res.err().unwrap());
        }

        let img: MutexGuard<'_, ImageData> = res.unwrap();

        let value: T = callback(img);

        return Result::Ok(value);
    }

    pub fn use_image_data<T>(
        &self, 
        callback: impl FnOnce(MutexGuard<'_, ImageData>) -> T
    ) -> T {
        return self.try_use_image_data(callback).unwrap();
    }
}
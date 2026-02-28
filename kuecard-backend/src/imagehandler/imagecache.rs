use std::{collections::HashMap, num::NonZeroUsize};

use lru::LruCache;

pub type ImageHandle = iced::widget::image::Handle;

pub struct ImageCache {
    lru_cache: LruCache<String, ImageHandle>,
    main_cache: HashMap<String, ImageHandle>,
    max_image_count: usize
}

impl ImageCache {
    pub fn new(max_image_count: usize) -> Self {
        return Self {
            lru_cache: LruCache::new(
                NonZeroUsize::new(max_image_count).unwrap()
            ),
            main_cache: HashMap::new(),
            max_image_count: max_image_count
        };
    }
}

impl ImageCache {
    pub fn get_max_image_count(&self) -> usize {
        return self.max_image_count;
    }

    pub fn get_lru_cache(&self) -> &LruCache<String, ImageHandle> {
        return &self.lru_cache;
    }

    pub fn get_main_cache(&self) -> &HashMap<String, ImageHandle> {
        return &self.main_cache;
    }
}

impl ImageCache {
    pub fn insert_into_lru_cache(
        &mut self, path: String, image: ImageHandle
    ) -> Option<(String, ImageHandle)> {
        return self.lru_cache.push(path, image);
    }

    pub fn insert_into_main_cache(
        &mut self, path: String, image: ImageHandle
    ) -> Option<ImageHandle> {
        return self.main_cache.insert(path, image);
    }

    pub fn get_lru_cache_mut(&mut self) -> &mut LruCache<String, ImageHandle> {
        return &mut self.lru_cache;
    }

    pub fn get_main_cache_mut(&mut self) -> &mut HashMap<String, ImageHandle> {
        return &mut self.main_cache;
    }

    pub fn try_get_image(&mut self, path: String) -> Result<&ImageHandle, String> {
        if self.lru_cache.contains(&path) {
            return Result::Ok(self.lru_cache.get(&path).unwrap());
        }

        if self.main_cache.contains_key(&path) {
            return Result::Ok(self.main_cache.get(&path).unwrap());
        }

        return Result::Err("Image not found!".into());
    }
}
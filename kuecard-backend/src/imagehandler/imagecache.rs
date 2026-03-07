use std::{collections::HashMap, num::NonZeroUsize};

use lru::LruCache;

pub type ImageHandle = iced::widget::image::Handle;
pub type SvgHandle = iced::widget::svg::Handle;

pub struct ImageCache {
    lru_cache: LruCache<String, SvgHandle>,
    main_cache: HashMap<String, SvgHandle>,
    max_image_count: usize,
}

impl ImageCache {
    pub fn new(max_image_count: usize) -> Self {
        return Self {
            lru_cache: LruCache::new(NonZeroUsize::new(max_image_count).unwrap()),
            main_cache: HashMap::new(),
            max_image_count: max_image_count,
        };
    }
}

impl ImageCache {
    pub fn get_max_image_count(&self) -> usize {
        return self.max_image_count;
    }

    pub fn get_lru_cache(&self) -> &LruCache<String, SvgHandle> {
        return &self.lru_cache;
    }

    pub fn get_main_cache(&self) -> &HashMap<String, SvgHandle> {
        return &self.main_cache;
    }
}

impl ImageCache {
    pub fn insert_into_lru_cache(
        &mut self,
        path: String,
        image: SvgHandle,
    ) -> Option<(String, SvgHandle)> {
        return self.lru_cache.push(path, image);
    }

    pub fn insert_into_main_cache(&mut self, path: String, image: SvgHandle) -> Option<SvgHandle> {
        return self.main_cache.insert(path, image);
    }

    pub fn get_lru_cache_mut(&mut self) -> &mut LruCache<String, SvgHandle> {
        return &mut self.lru_cache;
    }

    pub fn get_main_cache_mut(&mut self) -> &mut HashMap<String, SvgHandle> {
        return &mut self.main_cache;
    }

    pub fn try_get_image(&mut self, path: String) -> Result<&SvgHandle, String> {
        if self.lru_cache.contains(&path) {
            return Result::Ok(self.lru_cache.get(&path).unwrap());
        }

        if self.main_cache.contains_key(&path) {
            return Result::Ok(self.main_cache.get(&path).unwrap());
        }

        return Result::Err("Image not found!".into());
    }
}

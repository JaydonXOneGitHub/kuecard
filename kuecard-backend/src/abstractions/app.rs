use kutamun::MultiGrid;
use vector_x::Vector2;

use crate::{
    elements::uibutton::UIButton, 
    imagehandler::{
        AtomicImageCache, 
        MAX_IMAGE_COUNT
    }
};

pub struct App {
    multi_grid: MultiGrid<UIButton>,
    image_handler: AtomicImageCache,
    pub target: Vector2<f32>,
    pub window_size: Vector2<f32>
}

impl Default for App {
    fn default() -> Self {
        return Self::make(
            || MultiGrid::new(),
            || AtomicImageCache::new(MAX_IMAGE_COUNT)
        );
    }
}

impl App {
    pub fn make(
        mg_callback: impl FnOnce() -> MultiGrid<UIButton>,
        img_callback: impl FnOnce() -> AtomicImageCache,
    ) -> Self {
        return Self {
            multi_grid: mg_callback(),
            image_handler: img_callback(),
            target: Vector2::new(1280.0, 720.0),
            window_size: Vector2::new(1280.0, 720.0)
        };
    }

    pub fn get_multi_grid(&self) -> &MultiGrid<UIButton> {
        return &self.multi_grid;
    }

    pub fn get_image_cache(&self) -> &AtomicImageCache {
        return &self.image_handler;
    }
}
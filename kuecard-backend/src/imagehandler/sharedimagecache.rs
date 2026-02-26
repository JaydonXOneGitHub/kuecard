use std::{cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut}, rc::{Rc, Weak}};

use crate::imagehandler::ImageCache;

pub struct SharedImageCache {
    image_cache: Rc<RefCell<ImageCache>>
}

impl SharedImageCache {
    pub fn new(max_image_count: usize) -> Self {
        return Self {
            image_cache: Rc::new(RefCell::new(ImageCache::new(max_image_count)))
        };
    }
}

impl SharedImageCache {
    pub fn get_weak_image_cache(&self) -> Weak<RefCell<ImageCache>> {
        return Rc::downgrade(&self.image_cache);
    }

    pub fn get_image_cache(&self) -> &Rc<RefCell<ImageCache>> {
        return &self.image_cache;
    }

    pub fn try_use_cache<T>(
        &self,
        callback: impl FnOnce(Ref<'_, ImageCache>) -> T
    ) -> Result<T, BorrowError> {
        let res = self.image_cache.try_borrow();

        if res.is_err() {
            return Result::Err(res.err().unwrap());
        }

        let guard: Ref<'_, ImageCache> = res.ok().unwrap();

        let value: T = callback(guard);

        return Result::Ok(value);
    }

    pub fn try_use_cache_mut<T>(
        &self,
        callback: impl FnOnce(RefMut<'_, ImageCache>) -> T
    ) -> Result<T, BorrowMutError> {
        let res = self.image_cache.try_borrow_mut();

        if res.is_err() {
            return Result::Err(res.err().unwrap());
        }

        let guard: RefMut<'_, ImageCache> = res.ok().unwrap();

        let value: T = callback(guard);

        return Result::Ok(value);
    }

    pub fn use_cache<T>(
        &self,
        callback: impl FnOnce(Ref<'_, ImageCache>) -> T
    ) -> T {
        return self.try_use_cache(callback).unwrap();
    }

    pub fn use_cache_mut<T>(
        &self,
        callback: impl FnOnce(RefMut<'_, ImageCache>) -> T
    ) -> T {
        return self.try_use_cache_mut(callback).unwrap();
    }
}

impl Clone for SharedImageCache {
    fn clone(&self) -> Self {
        return Self {
            image_cache: self.image_cache.clone()
        };
    }
}
use std::sync::{Arc, Mutex, MutexGuard, PoisonError, TryLockError, Weak};

use crate::imagehandler::ImageCache;

pub struct AtomicImageCache {
    image_cache: Arc<Mutex<ImageCache>>
}

impl AtomicImageCache {
    pub fn new(max_image_count: usize) -> Self {
        return Self {
            image_cache: Arc::new(Mutex::new(ImageCache::new(max_image_count)))
        };
    }
}

impl AtomicImageCache {
    pub fn get_weak_image_cache(&self) -> Weak<Mutex<ImageCache>> {
        return Arc::downgrade(&self.image_cache);
    }

    pub fn get_image_cache(&self) -> &Arc<Mutex<ImageCache>> {
        return &self.image_cache;
    }

    pub fn try_use_cache<T>(
        &self,
        callback: impl FnOnce(MutexGuard<'_, ImageCache>) -> T
    ) -> Result<T, TryLockError<MutexGuard<'_, ImageCache>>> {
        let res = self.image_cache.try_lock();

        if res.is_err() {
            return Result::Err(res.err().unwrap());
        }

        let guard: MutexGuard<'_, ImageCache> = res.ok().unwrap();

        let value: T = callback(guard);

        return Result::Ok(value);
    }

    pub fn use_cache<T>(
        &self,
        callback: impl FnOnce(MutexGuard<'_, ImageCache>) -> T
    ) -> T {
        return self.try_use_cache(callback).unwrap();
    }

    pub fn try_use_cache_blocking<T>(
        &self,
        callback: impl FnOnce(MutexGuard<'_, ImageCache>) -> T
    ) -> Result<T, PoisonError<MutexGuard<'_, ImageCache>>> {
        let res = self.image_cache.lock();

        if res.is_err() {
            return Result::Err(res.err().unwrap());
        }

        let guard: MutexGuard<'_, ImageCache> = res.ok().unwrap();

        let value: T = callback(guard);

        return Result::Ok(value);
    }

    pub fn use_cache_blocking<T>(
        &self,
        callback: impl FnOnce(MutexGuard<'_, ImageCache>) -> T
    ) -> T {
        return self.try_use_cache_blocking(callback).unwrap();
    }
}

impl Clone for AtomicImageCache {
    fn clone(&self) -> Self {
        return Self {
            image_cache: self.image_cache.clone()
        };
    }
}
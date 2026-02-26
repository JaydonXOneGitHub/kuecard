use std::sync::{Arc, Mutex, MutexGuard, PoisonError, TryLockError};

pub struct ImageLoadList {
    data: Arc<Mutex<Vec<String>>>
}

impl ImageLoadList {
    pub fn new(vec: Vec<String>) -> Self {
        return Self {
            data: Arc::new(Mutex::new(vec))
        };
    }

    pub fn make_from_ref<T>(
        vec: &Vec<T>,
        callback: impl Fn(&T) -> String
    ) -> Self {
        let mut vec_str: Vec<String> = Vec::new();

        for t in vec {
            vec_str.push(callback(t));
        }

        return Self::new(vec_str);
    }
}

impl ImageLoadList {
    pub fn try_use_data<T>(
        &self,
        callback: impl FnOnce(MutexGuard<'_, Vec<String>>) -> T
    ) -> Result<T, TryLockError<MutexGuard<'_, Vec<String>>>> {
        let res = self.data.try_lock();

        if res.is_err() {
            return Result::Err(res.err().unwrap());
        }

        let guard: MutexGuard<'_, Vec<String>> = res.ok().unwrap();

        let value: T = callback(guard);

        return Result::Ok(value);
    }

    pub fn use_data<T>(
        &self,
        callback: impl FnOnce(MutexGuard<'_, Vec<String>>) -> T
    ) -> T {
        return self.try_use_data(callback).unwrap();
    }

    pub fn try_use_data_blocking<T>(
        &self,
        callback: impl FnOnce(MutexGuard<'_, Vec<String>>) -> T
    ) -> Result<T, PoisonError<MutexGuard<'_, Vec<String>>>> {
        let res = self.data.lock();

        if res.is_err() {
            return Result::Err(res.err().unwrap());
        }

        let guard: MutexGuard<'_, Vec<String>> = res.ok().unwrap();

        let value: T = callback(guard);

        return Result::Ok(value);
    }

    pub fn use_data_blocking<T>(
        &self,
        callback: impl FnOnce(MutexGuard<'_, Vec<String>>) -> T
    ) -> T {
        return self.try_use_data_blocking(callback).unwrap();
    }
}

impl Default for ImageLoadList {
    fn default() -> Self {
        return Self::new(Vec::new());
    }
}

impl Clone for ImageLoadList {
    fn clone(&self) -> Self {
        return Self {
            data: self.data.clone()
        };
    }
}
use std::{path::PathBuf, str::FromStr, sync::MutexGuard};

use vector_x::Vector2;

use kuecard_backend::{
    abstractions::{ImageData, ImageLoadList},
    imagehandler::{AtomicImageCache, ImageCache},
};

fn check_and_load_images(vec: &Vec<String>, cache: &mut MutexGuard<'_, ImageCache>) {
    for entry in vec {
        let image_loaded: bool = cache.get_main_cache().get(entry).is_some();

        if image_loaded {
            continue;
        }

        let path: PathBuf = PathBuf::from_str(entry.as_str()).unwrap();

        let res: Result<Vec<u8>, std::io::Error> = std::fs::read(path);

        if res.is_err() {
            eprintln!(
                "Couldn't load image \"{}\" : \"{}\"",
                entry,
                res.err().unwrap().to_string()
            );
            return;
        }

        let bytes: Vec<u8> = res.unwrap();

        let img_size: Vector2<u32> = Vector2::new(100, 100);

        let _ = cache
            .get_main_cache_mut()
            .insert(entry.clone(), ImageData::new(bytes, img_size).into());
    }
}

pub async fn load_images_for_row(
    image_load_list: ImageLoadList,
    image_cache: AtomicImageCache,
) -> Result<(), String> {
    let mut res2: Result<(), String> = Result::Ok(());

    let res = image_load_list.try_use_data_blocking(|load_list| {
        let vec: &Vec<String> = load_list.as_ref();

        let res = image_cache.try_use_cache_blocking(|mut cache| {
            check_and_load_images(vec, &mut cache);
        });

        if res.is_err() {
            res2 = Result::Err(res.err().unwrap().to_string());
        }
    });

    return match res {
        Result::Ok(_) => res2,
        Result::Err(e) => Result::Err(e.to_string()),
    };
}

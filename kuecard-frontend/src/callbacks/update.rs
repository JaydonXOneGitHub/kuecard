use iced::Task;

use kuecard_backend::{
    abstractions::ImageLoadList, imagehandler::AtomicImageCache, message::Message,
};

use crate::{
    callbacks::{load_images_for_row, on_custom_event, on_iced_event, on_nav_event},
    custommessage::CustomMessage,
    helpers::MainApp,
};

pub fn update(
    main_app: &mut MainApp,
    _msg: Message<CustomMessage>,
) -> Task<Message<CustomMessage>> {
    let task: Task<Message<CustomMessage>> = match _msg {
        Message::IcedEvent(e) => on_iced_event(&mut main_app.app, e),
        Message::NavEvent(ne) => on_nav_event(main_app, ne),
        Message::PrintErr(err) => {
            eprintln!("Error: {}", err);
            Task::none()
        }
        Message::LoadImageSet(image_load_list) => {
            let ill: ImageLoadList = image_load_list.clone();
            let aic: AtomicImageCache = main_app.app.get_image_cache().clone();

            Task::perform(load_images_for_row(ill.clone(), aic.clone()), |res| {
                return match res {
                    Result::Ok(_) => Message::ImagesLoaded(()),
                    Result::Err(e) => Message::PrintErr(e),
                };
            })
        }
        Message::Custom(cm) => on_custom_event(main_app, cm),
        _ => Task::none(),
    };

    main_app.scale_factor = f32::min(
        main_app.app.window_size.one / main_app.app.target.one,
        main_app.app.window_size.two / main_app.app.target.two,
    );

    return task;
}

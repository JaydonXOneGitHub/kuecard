use crate::helpers::MainApp;

pub fn get_scale_factor(main_app: &MainApp) -> f32 {
    let scale_factor: f32 = f32::min(
        main_app.app.window_size.one / main_app.app.target.one,
        main_app.app.window_size.two / main_app.app.target.two
    );
    return scale_factor;
}
use std::{fs::DirEntry, path::Path};

use iced::Task;
use kutamun::{Grid, GridState, MultiGrid};
use vector_x::Vector2;

use kuecard_backend::{
    abstractions::{App, ImageLoadList},
    elements::uibutton::UIButton,
    imagehandler::AtomicImageCache,
    message::Message,
};

use crate::{
    custommessage::CustomMessage, globals::{BUTTON_AMOUNT, GRID_MAIN, MAX_IMAGE_COUNT}, helpers::{AppTileData, Config, CustomThemeData, MainApp}
};

fn read_theme<'a>(path: impl Into<Option<&'a str>>) -> CustomThemeData {
    let str: Option<&str> = path.into();
    let path: &Path = Path::new(match str {
        Option::Some(s) => s,
        Option::None => "theme.json",
    });

    let res = std::fs::read(path);

    if res.is_err() {
        return CustomThemeData::default();
    }

    let res = String::from_utf8(res.ok().unwrap());

    if res.is_err() {
        return CustomThemeData::default();
    }

    let json: String = res.ok().unwrap();

    let res: Result<CustomThemeData, serde_json::Error> = serde_json::from_str(&json);

    if res.is_err() {
        return CustomThemeData::default();
    }

    return res.ok().unwrap();
}

fn read_config<'a>(path: impl Into<Option<&'a str>>) -> Config {
    let str: Option<&str> = path.into();
    let path: &Path = Path::new(match str {
        Option::Some(s) => s,
        Option::None => "config.json",
    });

    let res = std::fs::read(path);

    if res.is_err() {
        return Config::default();
    }

    let res = String::from_utf8(res.ok().unwrap());

    if res.is_err() {
        return Config::default();
    }

    let json: String = res.ok().unwrap();

    let res: Result<Config, serde_json::Error> = serde_json::from_str(&json);

    if res.is_err() {
        return Config::default();
    }

    return res.ok().unwrap();
}

fn get_app_data_list(dir: &Path) -> Vec<Vec<AppTileData>> {
    let res = dir.read_dir();

    if res.is_err() {
        return vec![];
    }

    let file_dir = res.unwrap();

    let mut app_data_full_list: Vec<Vec<AppTileData>> = vec![];
    let mut app_data_list: Vec<AppTileData> = vec![];
    let mut counter: usize = 0;

    for file in file_dir {
        if counter >= BUTTON_AMOUNT {
            counter = 0;
            app_data_full_list.push(app_data_list.clone());
            app_data_list.clear();
        }

        let entry: DirEntry = file.unwrap();

        let contents: String = String::from_utf8(std::fs::read(entry.path()).unwrap()).unwrap();

        println!("Contents: {}", contents);

        let res: Result<AppTileData, serde_json::Error> = serde_json::from_str(&contents);

        if res.is_err() {
            println!("Error: {}", res.err().unwrap());
            continue;
        }

        app_data_list.push(res.ok().unwrap());
        counter += 1;
    }

    app_data_full_list.push(app_data_list.clone());

    return app_data_full_list;
}

fn get_buttons(path: &str, alt_path: &str) -> Vec<Vec<UIButton>> {
    let dir: &Path = Path::new(path);

    let mut app_data_list: Option<Vec<Vec<AppTileData>>> = Option::None;

    if dir.exists() {
        app_data_list = Option::Some(get_app_data_list(dir));
    }

    if app_data_list.is_none() {
        let dir: &Path = Path::new(alt_path);

        if dir.exists() {
            app_data_list = Option::Some(get_app_data_list(dir));
        }
    }

    if app_data_list.is_none() {
        return vec![];
    }

    let app_data_list: Vec<Vec<AppTileData>> = app_data_list.unwrap();

    let mut final_app_tiles: Vec<Vec<UIButton>> = Vec::new();

    for row in app_data_list {
        let mut app_row: Vec<UIButton> = Vec::new();

        for entry in row {
            app_row.push(UIButton::AppTile(entry.into()));
        }

        final_app_tiles.push(app_row);
    }

    return final_app_tiles;
}

fn get_multi_grid(path: &str, alt_path: &str) -> MultiGrid<UIButton> {
    let grid: Grid<UIButton> = Grid::from_callback(|| {
        return get_buttons(path, alt_path);
    })
    .with_position(Vector2::new(0, 0))
    .with_enabled(GridState::Visible);

    return MultiGrid::new()
        .with_grid((GRID_MAIN, grid))
        .with_selected_grid(0);
}

fn get_image_handler() -> AtomicImageCache {
    return AtomicImageCache::new(MAX_IMAGE_COUNT);
}

pub fn initialize() -> (MainApp, Task<Message<CustomMessage>>) {
    let aic: AtomicImageCache = get_image_handler();

    let config: Config = read_config("config.json");
    //let aic2: AtomicImageCache = aic.clone();

    let mg: MultiGrid<UIButton> = get_multi_grid(&config.app_dir, &config.backup_app_dir);

    let app: App = App::make(|| mg.clone(), || aic);

    let theme_data: CustomThemeData = read_theme(Option::None);

    let main_app: MainApp = MainApp {
        app,
        theme: theme_data.into(),
        config: config,
        scale_factor: 1.0,
        ad_metadata: Option::None,
    };

    let task: Task<Message<CustomMessage>> = load_images(mg.clone());

    return (main_app, task);
}

fn load_images(mg: MultiGrid<UIButton>) -> Task<Message<CustomMessage>> {
    let res = mg.get_internal_ref().try_borrow();

    if res.is_err() {
        eprintln!("Error init: {}", res.err().unwrap());
        return Task::none();
    }

    let img = res.unwrap();

    let res = img.get_grid();

    if res.is_error() {
        eprintln!("Error init: {}", res.err().unwrap());
        return Task::none();
    }

    let grid = res.ok().unwrap();

    let mut vec_str: Vec<String> = Vec::new();

    for row in grid.get_buttons() {
        for button in row {
            vec_str.push(button.app_button().unwrap().img_path.clone());
        }
    }

    let task: Task<Message<CustomMessage>> = Task::perform(
        async move {
            return ImageLoadList::new(vec_str);
        },
        Message::LoadImageSet,
    );

    // let msg: Message<CustomMessage> = Message::LoadImageSet(ImageLoadList::new(vec_str));

    // let task: Task<Message<CustomMessage>> = Task::done(Message::Custom(CustomMessage::Delay {
    //     time_in_milliseconds: 2000,
    //     message: Box::new(msg),
    // }));

    return task;
}

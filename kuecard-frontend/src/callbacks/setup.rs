use std::path::Path;

use iced::{
    Color, 
    Task
};
use kutamun::{
    Grid, 
    GridState, 
    MultiGrid
};
use vector_x::Vector2;

use kuecard_backend::{
    abstractions::{
        App, 
        ImageLoadList
    },  
    elements::uibutton::{
        TextTile, 
        UIButton
    }, 
    imagehandler::{
        AtomicImageCache, 
        MAX_IMAGE_COUNT
    }, 
    message::Message
};

use crate::{
    callbacks::GRID_MAIN, 
    custommessage::CustomMessage, 
    helpers::{
        CustomThemeData, 
        MainApp
    }
};

fn read_theme<'a>(path: impl Into<Option<&'a str>>) -> CustomThemeData {
    let str: Option<&str> = path.into();
    let path: &Path = Path::new(match str {
        Option::Some(s) => s,
        Option::None => "theme.json"
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

fn get_buttons() -> Vec<Vec<UIButton>> {
    return vec![];
}

fn get_multi_grid() -> MultiGrid<UIButton> {
    let grid: Grid<UIButton> = Grid::from_callback(get_buttons)
    .with_position(Vector2::new(0, 0))
    .with_enabled(GridState::Visible);

    let grid2: Grid<UIButton> = Grid::from_callback(
        || {
            return vec![
                vec![
                    UIButton::TextTile(TextTile {
                        text: String::from("One"),
                        color: Color::from_rgba(1.0, 0.2, 1.0, 1.0)
                    }),
                    UIButton::TextTile(TextTile {
                        text: String::from("Two"),
                        color: Color::from_rgba(1.0, 0.2, 1.0, 1.0)
                    }),
                    UIButton::TextTile(TextTile {
                        text: String::from("Three"),
                        color: Color::from_rgba(1.0, 0.2, 1.0, 1.0)
                    }),
                    UIButton::TextTile(TextTile {
                        text: String::from("Four"),
                        color: Color::from_rgba(1.0, 0.2, 1.0, 1.0)
                    }),
                ]
            ];
        }
    )
    .with_position(Vector2::new(0, 0))
    .with_enabled(GridState::Visible);
    
    return MultiGrid::new()
    .with_grid((GRID_MAIN, grid))
    .with_grid((1, grid2))
    .with_selected_grid(0);
}

fn get_image_handler() -> AtomicImageCache {
    return AtomicImageCache::new(MAX_IMAGE_COUNT);
}

pub fn initialize() -> (MainApp, Task<Message<CustomMessage>>) {
    let aic: AtomicImageCache = get_image_handler();

    //let aic2: AtomicImageCache = aic.clone();

    let mg: MultiGrid<UIButton> = get_multi_grid();

    let app: App = App::make(
        || mg.clone(),
        || aic
    );

    let theme_data: CustomThemeData = read_theme(Option::None);

    let main_app: MainApp = MainApp { 
        app, 
        theme: theme_data.into()
    };

    let res = mg.get_internal_ref().try_borrow();

    if res.is_err() {
        eprintln!("Error init: {}", res.err().unwrap());
        return (main_app, Task::none());
    }

    let img = res.unwrap();

    let res = img.get_grid();

    if res.is_error() {
        eprintln!("Error init: {}", res.err().unwrap());
        return (main_app, Task::none());
    }

    let grid = res.ok().unwrap();

    let mut vec_str: Vec<String> = Vec::new();

    for row in grid.get_buttons() {
            for button in row {
            vec_str.push(
                button.app_button()
                .unwrap()
                .img_path
                .clone()
            );
        }
    }

    let task: Task<Message<CustomMessage>> = Task::perform(
            async move {
            return ImageLoadList::new(vec_str);
        }, 
        Message::LoadImageSet
    );

    return (main_app, task);
}
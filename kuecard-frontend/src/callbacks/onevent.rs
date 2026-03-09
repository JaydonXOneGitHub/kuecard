use std::{cell::BorrowMutError, process::Command, time::Duration};

use iced::{
    Task,
    keyboard::key::{Code, Physical},
};
use kutamun::{Direction, Grid};
use vector_x::{Vector2, Vector3};

use kuecard_backend::{
    abstractions::App,
    elements::uibutton::UIButton,
    message::{Message, NavEvent},
};

use crate::{callbacks::{navigate, try_play_sound}, custommessage::CustomMessage, helpers::MainApp};

fn on_iced_keyboard_event(
    _app: &mut App,
    ke: iced::keyboard::Event,
) -> Task<Message<CustomMessage>> {
    match ke {
        iced::keyboard::Event::KeyPressed {
            key: _,
            modified_key: _,
            physical_key,
            location: _,
            modifiers: _,
            text: _,
            repeat: _,
        } => {
            if let Physical::Code(code) = physical_key {
                let new_ne: NavEvent = match code {
                    Code::ArrowLeft => NavEvent::Navigate(Direction::Left),
                    Code::ArrowRight => NavEvent::Navigate(Direction::Right),
                    Code::ArrowUp => NavEvent::Navigate(Direction::Up),
                    Code::ArrowDown => NavEvent::Navigate(Direction::Down),
                    Code::Enter => {
                        let mut pos: Vector3<usize> = Vector3::default();

                        let grid = _app.get_multi_grid().get_internal_ref().borrow();

                        pos.one = grid.get_current_grid().unwrap();

                        let grid_pos = grid.get_grid().ok().unwrap().get_position();

                        pos.two = grid_pos.one;
                        pos.three = grid_pos.two;

                        NavEvent::Select(pos)
                    }
                    Code::Escape => NavEvent::Back,
                    _ => NavEvent::Nil,
                };

                return Task::done(Message::NavEvent(new_ne));
            }

            return Task::none();
        }
        _ => {
            return Task::none();
        }
    }
}

fn on_window_event(_app: &mut App, we: iced::window::Event) -> Task<Message<CustomMessage>> {
    return match we {
        iced::window::Event::Resized(size) => {
            _app.window_size = Vector2::new(size.width, size.height);
            Task::none()
        }
        _ => Task::none(),
    };
}

pub fn on_iced_event(_app: &mut App, e: iced::Event) -> Task<Message<CustomMessage>> {
    return match e {
        iced::Event::Keyboard(ke) => on_iced_keyboard_event(_app, ke),
        iced::Event::Window(we) => on_window_event(_app, we),
        _ => Task::none(),
    };
}

pub fn on_nav_event(main_app: &mut MainApp, ne: NavEvent) -> Task<Message<CustomMessage>> {
    match ne {
        NavEvent::Navigate(dir) => {
            let res = main_app.app.get_multi_grid().get_internal_ref().try_borrow_mut();

            if res.is_err() {
                let e: BorrowMutError = res.err().unwrap();
                return Task::done(Message::PrintErr(e.to_string()));
            }

            let mut ig = res.unwrap();

            ig.move_focus(dir.clone(), navigate);

            let _ = try_play_sound(main_app.config.select_sfx.as_ref());

            return Task::none();
        }
        NavEvent::Select(pos) => {
            let _ = try_play_sound(main_app.config.select_sfx.as_ref());

            let res = main_app.app.get_multi_grid().get_internal_ref().try_borrow();

            if res.is_err() {
                return Task::perform(
                    tokio::time::sleep(tokio::time::Duration::from_millis(400)),
                    |_| return Message::NavEvent(ne),
                );
            }

            let multi_grid = res.ok().unwrap();

            let opt: Option<&Grid<UIButton>> = multi_grid.get_grids().get(&pos.one);

            if opt.is_none() {
                return Task::none();
            }

            let grid: &Grid<UIButton> = opt.unwrap();

            let button: &UIButton = grid
                .get_buttons()
                .get(pos.two)
                .unwrap()
                .get(pos.three)
                .unwrap();

            match button {
                UIButton::AppTile(app_tile) => {
                    let mut command: Command = Command::new(app_tile.command.clone());

                    let res = command.spawn();

                    return if res.is_err() {
                        Task::none()
                    } else {
                        Task::perform(async move {
                            tokio::time::sleep(Duration::from_millis(500))
                        }, |_| Message::Custom(CustomMessage::Exit))
                    };
                }
                _ => {}
            }

            return Task::none();
        }
        NavEvent::Back => {
            return Task::none();
        }
        _ => {
            return Task::none();
        }
    }
}

pub fn on_custom_event(main_app: &mut MainApp, cm: CustomMessage) -> Task<Message<CustomMessage>> {
    return match cm {
        CustomMessage::ThemeChanged(theme) => {
            main_app.theme = theme;

            Task::none()
        }
        CustomMessage::AdLoaded(Result::Ok(ad_metadata)) => {
            main_app.ad_metadata = Option::Some(ad_metadata);
            Task::none()
        }
        CustomMessage::AdLoaded(Result::Err(err)) => Task::done(Message::PrintErr(err)),
        CustomMessage::Exit => iced::exit(),
        CustomMessage::Delay {
            time_in_milliseconds,
            message,
        } => {
            let time: u64 = time_in_milliseconds.clone();

            let msg: Message<CustomMessage> = message.as_ref().clone();

            Task::perform(
                async move {
                    tokio::time::sleep(Duration::from_millis(time)).await;
                },
                move |_| msg,
            )
        }
        _ => Task::none(),
    };
}

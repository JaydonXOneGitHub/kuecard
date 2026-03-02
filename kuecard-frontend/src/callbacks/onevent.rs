use std::cell::BorrowMutError;

use iced::{
    Task,
    keyboard::key::{Code, Physical},
};
use kutamun::Direction;
use vector_x::{Vector2, Vector3};

use kuecard_backend::{
    abstractions::App,
    message::{Message, NavEvent},
};

use crate::{callbacks::navigate, custommessage::CustomMessage};

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

pub fn on_nav_event(_app: &mut App, ne: NavEvent) -> Task<Message<CustomMessage>> {
    match ne {
        NavEvent::Navigate(dir) => {
            let res = _app.get_multi_grid().get_internal_ref().try_borrow_mut();
            if res.is_err() {
                let e: BorrowMutError = res.err().unwrap();
                return Task::done(Message::PrintErr(e.to_string()));
            }

            let mut ig = res.unwrap();

            ig.move_focus(dir.clone(), navigate);

            return Task::none();
        }
        NavEvent::Select(_) => {
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

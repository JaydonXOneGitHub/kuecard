use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use iced::{
    Border, Element, advanced::graphics::core::Bytes, widget::{Button, Column, Container, Image, Row, Space, Stack, Svg, Text}
};
use kutamun::{Grid, multigrids::InternalMultiGrid};
use vector_x::Vector2;

use crate::{globals::*, helpers::{AdImage, AdMetadata}};
use kuecard_backend::{
    elements::uibutton::UIButton, imagehandler::{AtomicImageCache, ImageHandle}, message::Message,
};

use crate::{
    custommessage::CustomMessage,
    helpers::{Either, MainApp},
};

pub type ButtonStyle = iced::widget::button::Style;
pub type ContainerStyle = iced::widget::container::Style;

fn custom_min<T: Sized + Ord>(one: T, two: T) -> Either<T, T> {
    return if one < two {
        Either::A(one)
    } else {
        Either::B(two)
    };
}

#[allow(unused_parens)]
fn row_is_unviewable(r: usize, grid_height: usize, grid_pos: &Vector2<usize>) -> bool {
    let max_scroll_value: usize = grid_height.saturating_sub(MAX_ADDITIONAL_ROWS - 1);
    let min: Either<usize, usize> = custom_min(r, max_scroll_value);

    let capped_r: usize = match min {
        Either::A(a) => a,
        Either::B(b) => b,
        Either::Neither => 0,
    };

    return if min.is_a() {
        ((capped_r < grid_pos.one.saturating_sub(ALLOWED_SCROLL_OFFSET))
            || (capped_r > grid_pos.one + MAX_ADDITIONAL_ROWS + ALLOWED_SCROLL_OFFSET))
    } else {
        r < max_scroll_value
    };
}

fn get_buttons<'a>(
    main_app: &'a MainApp,
    mg_handle: &Rc<RefCell<InternalMultiGrid<UIButton>>>,
    image_cache: &AtomicImageCache,
) -> Element<'a, Message<CustomMessage>> {
    let res = mg_handle.try_borrow_mut();

    if res.is_err() {
        eprintln!("Error: {}", res.err().unwrap().to_string());
        return Space::new().into();
    }

    let mg: RefMut<'_, InternalMultiGrid<UIButton>> = res.unwrap();

    let opt: Option<&Grid<UIButton>> = mg.get_grids().get(&mg.get_current_grid().unwrap());

    if opt.is_none() {
        return Space::new().into();
    }

    let grid: &Grid<UIButton> = opt.unwrap();

    let buttons: &Vec<Vec<UIButton>> = grid.get_buttons();

    let mut column: Column<'_, Message<CustomMessage>> = Column::new();

    for (r, row) in buttons.iter().enumerate() {
        let grid_pos: Vector2<usize> = grid.get_position();

        if row_is_unviewable(r, buttons.len(), &grid_pos) {
            continue;
        }

        let mut button_row: Row<'_, Message<CustomMessage>> = Row::new();

        for (b, button) in row.iter().enumerate() {
            button_row = create_button(main_app, button_row, (r, b), grid, button, image_cache)
                .push(Space::new().width(SPACING_AMOUNT));
        }

        column = column
            .push(button_row)
            .push(Space::new().height(SPACING_AMOUNT));
    }

    return column.into();
}

fn create_button<'a>(
    main_app: &'a MainApp,
    button_row: Row<'a, Message<CustomMessage>>,
    position: (usize, usize),
    grid: &Grid<UIButton>,
    button: &UIButton,
    image_cache: &AtomicImageCache,
) -> Row<'a, Message<CustomMessage>> {
    let element: Element<'a, Message<CustomMessage>> = match button {
        UIButton::AppTile(app_tile) => {
            let res = image_cache.get_image_cache().try_lock();

            let internal_element: Element<'a, Message<CustomMessage>> = match res {
                Result::Ok(mut ic) => match ic.get_main_cache_mut().get(&app_tile.img_path) {
                    Option::Some(img) => Svg::new(img.clone())
                        .width(BUTTON_SIZE * 2)
                        .height(BUTTON_SIZE * 2)
                        .into(),
                    Option::None => {
                        let text: String = app_tile.alt_text.clone();
                        Text::new(text).into()
                    }
                },
                Result::Err(_) => Space::new().into(),
            };

            let button = Button::new(internal_element)
                .width(BUTTON_SIZE)
                .height(BUTTON_SIZE)
                .style(move |_, _| -> ButtonStyle {
                    return ButtonStyle {
                        text_color: main_app.theme.text_color.clone(),
                        background: Option::Some(main_app.theme.button_backdrop.to_background()),
                        shadow: main_app.theme.shadow.clone(),
                        ..Default::default()
                    };
                });

            let pos: Vector2<usize> = grid.get_position();
            let current_pos: (usize, usize) = position.clone();

            let container = Container::new(button)
                .width(BUTTON_SIZE)
                .height(BUTTON_SIZE)
                .padding(CONTAINER_SPACING)
                .style(move |_| -> ContainerStyle {
                    let current_pos: Vector2<usize> = current_pos.into();
                    let border = if pos == current_pos {
                        Border::default()
                            .color(main_app.theme.selected_color.clone())
                            .rounded(CONTAINER_SPACING)
                            .width(CONTAINER_SPACING)
                    } else {
                        Border::default()
                            .color(main_app.theme.unselected_color.clone())
                            .rounded(CONTAINER_SPACING)
                            .width(CONTAINER_SPACING)
                    };

                    return ContainerStyle::default().border(border);
                });

            container.into()
        }
        _ => Space::new().into(),
    };

    return button_row.push(element);
}

fn ad_image(ad_image: &AdImage) -> Element<'_, Message<CustomMessage>> {
    return match &ad_image.handle {
        Option::Some(handle) => Image::new(handle.clone()).width(ad_image.width).height(ad_image.height).into(),
        Option::None => Space::new().into()
    };
}

fn ad_widget(ad_metadata: &Option<AdMetadata>) -> Element<'_, Message<CustomMessage>> {
    return match ad_metadata {
        Option::None => Space::new().into(),
        Option::Some(metadata) => Row::new()
            .push(Space::new().width(AD_SPACING))
            .push(Column::new()
                .push(ad_image(&metadata.ad_image))
                .push(Text::new(metadata.content.clone())))
            .into()
    };
}

pub fn view(main_app: &MainApp) -> Element<'_, Message<CustomMessage>> {
    //return Space::new().into();

    let mg_handle: &Rc<RefCell<InternalMultiGrid<UIButton>>> =
        main_app.app.get_multi_grid().get_internal_ref();

    let button_layout: Element<'_, Message<CustomMessage>> =
        get_buttons(main_app, mg_handle, main_app.app.get_image_cache());

    let main_elements: Element<'_, Message<CustomMessage>> = Container::new(
        Row::new()
        .push(Column::new().push(Space::new().width(450)).push(button_layout))
        .push(ad_widget(&main_app.ad_metadata)),
    )
    .padding(UI_PADDING)
    .into();

    let bg: Element<'_, Message<CustomMessage>> = Container::new(Space::new())
        .width(1280)
        .height(720)
        .style(|_| -> ContainerStyle {
            return ContainerStyle {
                background: Option::Some(main_app.theme.backdrop.to_background()),
                ..Default::default()
            };
        })
        .into();

    return Stack::new().push(bg).push(main_elements).into();
}

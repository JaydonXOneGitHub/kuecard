use kutamun::{
    Direction, 
    Grid, 
    NavRes, 
    multigrids::InternalMultiGrid
};
use vector_x::Vector3;

use kuecard_backend::{
    elements::uibutton::UIButton
};

use crate::globals::GRID_MAIN;



pub fn navigate(
    internal_grid: &InternalMultiGrid<UIButton>,
    direction: Direction,
    old_pos: Vector3<usize>
) -> Vector3<usize> {
    let res: NavRes<&Grid<UIButton>, String> = internal_grid.get_grid();

    if res.is_error() {
        return old_pos;
    }

    let grid: &Grid<UIButton> = res.ok().unwrap();

    let new_pos: Vector3<usize> = match internal_grid.get_current_grid() {
        Option::Some(cg) => {
            match cg {
                GRID_MAIN => navigate_grid_main(grid, direction, old_pos),
                _ => old_pos
            }
        },
        Option::None => old_pos
    };

    return new_pos;
}

fn navigate_grid_main(    
    grid: &Grid<UIButton>,
    direction: Direction,
    old_pos: Vector3<usize>
) -> Vector3<usize>  {
    let mut new_pos: Vector3<usize> = old_pos;

    match direction {
        Direction::Left => {
            if new_pos.three > 0 {
                new_pos.three -= 1;
            }
        },
        Direction::Right => {
            let opt: Option<&Vec<UIButton>> = grid.get_buttons().get(
                new_pos.two
            );

            if opt.is_none() {
                return new_pos;
            }

            let row: &Vec<UIButton> = opt.unwrap();

            let row_len: usize = row.len() - 1;

            if new_pos.three < row_len {
                new_pos.three += 1;
            }
        },
        Direction::Down => {
            let grid_height: usize = grid.get_buttons().len() - 1;

            if new_pos.two < grid_height {
                new_pos.two += 1;
            }

            let opt: Option<&Vec<UIButton>> = grid.get_buttons().get(
                new_pos.two
            );

            if opt.is_none() {
                return new_pos;
            }

            let row: &Vec<UIButton> = opt.unwrap();

            let row_len: usize = row.len() - 1;

            new_pos.three = usize::clamp(
                new_pos.three, 
                0, row_len
            );
        },
        Direction::Up => {
            if new_pos.two > 0 {
                new_pos.two -= 1;
            }

            let opt: Option<&Vec<UIButton>> = grid.get_buttons().get(
                new_pos.two
            );

            if opt.is_none() {
                return new_pos;
            }

            let row: &Vec<UIButton> = opt.unwrap();

            let row_len: usize = row.len() - 1;

            new_pos.three = usize::clamp(
                new_pos.three, 
                0, row_len
            );
        }
    }

    return new_pos
}
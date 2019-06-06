#![cfg(feature = "asm")]
extern crate newt;
use newt::components::{Component,Button};
use newt::grid::*;
use std::ptr;

#[test]
fn button_bar_create() {
    let button_bar = ButtonBar::new(&["Ok", "Cancel"]);
    assert!(button_bar.grid() != ptr::null_mut());
    assert_eq!(button_bar.buttons().len(), 2);
}

#[test]
fn grid_create() {
    let grid = Grid::new(2, 2);
    assert!(grid.grid() != ptr::null_mut());
}

#[test]
#[should_panic]
fn grid_invalid_row() {
    let _grid = Grid::new(1, 0);
}

#[test]
#[should_panic]
fn grid_invalid_column() {
    let _grid = Grid::new(0, 1);
}

#[test]
#[should_panic]
fn grid_invalid_column_position() {
    let mut b1 = Button::new(0, 0, "Ok");
    let mut grid = Grid::new(2, 2);
    grid.set_field(2, 1, &mut b1, 1, 1, 1, 1, 0, 0);
}

#[test]
#[should_panic]
fn grid_invalid_row_position() {
    let mut b1 = Button::new(0, 0, "Ok");
    let mut grid = Grid::new(2, 2);
    grid.set_field(1, 2, &mut b1, 1, 1, 1, 1, 0, 0);
}

#[test]
fn vertical_grid_create() {
    let mut b1 = Button::new(0, 0, "Yes");
    let mut b2 = Button::new(0, 0, "No");
    let buttons: &mut [&mut dyn Component] = &mut [&mut b1, &mut b2];
    let grid = VerticalGrid::new(buttons);
    assert!(grid.grid() != ptr::null_mut());

    let mut b1 = Button::new(0, 0, "Yes");
    let mut b2 = Button::new(0, 0, "No");
    let mut b3 = Button::new(0, 0, "Maybe");
    let buttons: &mut [&mut dyn Component] = &mut [&mut b1, &mut b2, &mut b3];
    let grid = VerticalGrid::new(buttons);
    assert!(grid.grid() != ptr::null_mut());

    let mut b1 = Button::new(0, 0, "Yes");
    let mut b2 = Button::new(0, 0, "No");
    let buttons: &mut [&mut dyn Component] = &mut [&mut b1, &mut b2];
    let grid = VerticalGrid::new_close_stacked(buttons);
    assert!(grid.grid() != ptr::null_mut());

    let mut b1 = Button::new(0, 0, "Yes");
    let mut b2 = Button::new(0, 0, "No");
    let mut b3 = Button::new(0, 0, "Maybe");
    let buttons: &mut [&mut dyn Component] = &mut [&mut b1, &mut b2, &mut b3];
    let grid = VerticalGrid::new_close_stacked(buttons);
    assert!(grid.grid() != ptr::null_mut());
}

#[test]
fn horizontal_grid_create() {
    let mut b1 = Button::new(0, 0, "Yes");
    let mut b2 = Button::new(0, 0, "No");
    let buttons: &mut [&mut dyn Component] = &mut [&mut b1, &mut b2];
    let grid = HorizontalGrid::new(buttons);
    assert!(grid.grid() != ptr::null_mut());

    let mut b1 = Button::new(0, 0, "Yes");
    let mut b2 = Button::new(0, 0, "No");
    let mut b3 = Button::new(0, 0, "Maybe");
    let buttons: &mut [&mut dyn Component] = &mut [&mut b1, &mut b2, &mut b3];
    let grid = HorizontalGrid::new(buttons);
    assert!(grid.grid() != ptr::null_mut());

    let mut b1 = Button::new(0, 0, "Yes");
    let mut b2 = Button::new(0, 0, "No");
    let buttons: &mut [&mut dyn Component] = &mut [&mut b1, &mut b2];
    let grid = HorizontalGrid::new_close_stacked(buttons);
    assert!(grid.grid() != ptr::null_mut());

    let mut b1 = Button::new(0, 0, "Yes");
    let mut b2 = Button::new(0, 0, "No");
    let mut b3 = Button::new(0, 0, "Maybe");
    let buttons: &mut [&mut dyn Component] = &mut [&mut b1, &mut b2, &mut b3];
    let grid = HorizontalGrid::new_close_stacked(buttons);
    assert!(grid.grid() != ptr::null_mut());
}

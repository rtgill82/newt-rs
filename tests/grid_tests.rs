#![cfg(feature = "asm")]
extern crate newt;
use newt::Component;
use newt::widgets::Button;
use newt::grid::*;
use std::ptr;

#[test]
fn button_bar_create() {
    let button_bar = ButtonBar::new(&["Ok", "Cancel"]);
    assert!(button_bar.co() != ptr::null_mut());
    assert_eq!(button_bar.buttons().len(), 2);
}

#[test]
fn grid_create() {
    let grid = Grid::new(2, 2);
    assert!(grid.co() != ptr::null_mut());
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
    let b1 = Button::new(0, 0, "Ok");
    let mut grid = Grid::new(2, 2);
    grid.set_field(2, 1, &b1, 1, 1, 1, 1, 0, 0);
}

#[test]
#[should_panic]
fn grid_invalid_row_position() {
    let b1 = Button::new(0, 0, "Ok");
    let mut grid = Grid::new(2, 2);
    grid.set_field(1, 2, &b1, 1, 1, 1, 1, 0, 0);
}

#[test]
fn vertical_grid_create() {
    let b1 = Button::new(0, 0, "Yes");
    let b2 = Button::new(0, 0, "No");
    let buttons: &[&dyn Component] = &[&b1, &b2];
    let grid = VerticalGrid::new(buttons);
    assert!(grid.co() != ptr::null_mut());

    let b1 = Button::new(0, 0, "Yes");
    let b2 = Button::new(0, 0, "No");
    let b3 = Button::new(0, 0, "Maybe");
    let buttons: &[&dyn Component] = &[&b1, &b2, &b3];
    let grid = VerticalGrid::new(buttons);
    assert!(grid.co() != ptr::null_mut());

    let b1 = Button::new(0, 0, "Yes");
    let b2 = Button::new(0, 0, "No");
    let buttons: &[&dyn Component] = &[&b1, &b2];
    let grid = VerticalGrid::new_close_stacked(buttons);
    assert!(grid.co() != ptr::null_mut());

    let b1 = Button::new(0, 0, "Yes");
    let b2 = Button::new(0, 0, "No");
    let b3 = Button::new(0, 0, "Maybe");
    let buttons: &[&dyn Component] = &[&b1, &b2, &b3];
    let grid = VerticalGrid::new_close_stacked(buttons);
    assert!(grid.co() != ptr::null_mut());
}

#[test]
fn horizontal_grid_create() {
    let b1 = Button::new(0, 0, "Yes");
    let b2 = Button::new(0, 0, "No");
    let buttons: &[&dyn Component] = &[&b1, &b2];
    let grid = HorizontalGrid::new(buttons);
    assert!(grid.co() != ptr::null_mut());

    let b1 = Button::new(0, 0, "Yes");
    let b2 = Button::new(0, 0, "No");
    let b3 = Button::new(0, 0, "Maybe");
    let buttons: &[&dyn Component] = &[&b1, &b2, &b3];
    let grid = HorizontalGrid::new(buttons);
    assert!(grid.co() != ptr::null_mut());

    let b1 = Button::new(0, 0, "Yes");
    let b2 = Button::new(0, 0, "No");
    let buttons: &[&dyn Component] = &[&b1, &b2];
    let grid = HorizontalGrid::new_close_stacked(buttons);
    assert!(grid.co() != ptr::null_mut());

    let b1 = Button::new(0, 0, "Yes");
    let b2 = Button::new(0, 0, "No");
    let b3 = Button::new(0, 0, "Maybe");
    let buttons: &[&dyn Component] = &[&b1, &b2, &b3];
    let grid = HorizontalGrid::new_close_stacked(buttons);
    assert!(grid.co() != ptr::null_mut());
}

//
// Copyright (C) 2019 Robert Gill <rtgill82@gmail.com>
//
// This file is a part of newt-rs.
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License version 2.1 as published by the Free Software Foundation.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

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
    let grid = VerticalGrid::new(&[&b1, &b2]);
    assert!(grid.co() != ptr::null_mut());

    let b1 = Button::new(0, 0, "Yes");
    let b2 = Button::new(0, 0, "No");
    let b3 = Button::new(0, 0, "Maybe");
    let grid = VerticalGrid::new(&[&b1, &b2, &b3]);
    assert!(grid.co() != ptr::null_mut());

    let b1 = Button::new(0, 0, "Yes");
    let b2 = Button::new(0, 0, "No");
    let grid = VerticalGrid::new_close_stacked(&[&b1, &b2]);
    assert!(grid.co() != ptr::null_mut());

    let b1 = Button::new(0, 0, "Yes");
    let b2 = Button::new(0, 0, "No");
    let b3 = Button::new(0, 0, "Maybe");
    let grid = VerticalGrid::new_close_stacked(&[&b1, &b2, &b3]);
    assert!(grid.co() != ptr::null_mut());
}

#[test]
fn horizontal_grid_create() {
    let b1 = Button::new(0, 0, "Yes");
    let b2 = Button::new(0, 0, "No");
    let grid = HorizontalGrid::new(&[&b1, &b2]);
    assert!(grid.co() != ptr::null_mut());

    let b1 = Button::new(0, 0, "Yes");
    let b2 = Button::new(0, 0, "No");
    let b3 = Button::new(0, 0, "Maybe");
    let grid = HorizontalGrid::new(&[&b1, &b2, &b3]);
    assert!(grid.co() != ptr::null_mut());

    let b1 = Button::new(0, 0, "Yes");
    let b2 = Button::new(0, 0, "No");
    let grid = HorizontalGrid::new_close_stacked(&[&b1, &b2]);
    assert!(grid.co() != ptr::null_mut());

    let b1 = Button::new(0, 0, "Yes");
    let b2 = Button::new(0, 0, "No");
    let b3 = Button::new(0, 0, "Maybe");
    let grid = HorizontalGrid::new_close_stacked(&[&b1, &b2, &b3]);
    assert!(grid.co() != ptr::null_mut());
}

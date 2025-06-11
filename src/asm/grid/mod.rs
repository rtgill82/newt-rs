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

use std::cell::Cell;
use std::ffi::{CString,c_void};
use std::os::raw::c_char;
use newt_sys::*;

use crate::component::Component;
use crate::grid::r#trait;
use crate::private::traits::ComponentPtr;

#[doc(hidden)]
pub mod basic_window;
#[doc(hidden)]
pub mod button_bar;
#[doc(hidden)]
pub mod horizontal_grid;
#[doc(hidden)]
pub mod simple_window;
#[doc(hidden)]
pub mod vertical_grid;

#[doc(inline)]
pub use self::basic_window::BasicWindow;
#[doc(inline)]
pub use self::button_bar::ButtonBar;
#[doc(inline)]
pub use self::horizontal_grid::HorizontalGrid;
#[doc(inline)]
pub use self::simple_window::SimpleWindow;
#[doc(inline)]
pub use self::vertical_grid::VerticalGrid;

///
/// Arrange `Component`s and sub-grids within a two-dimensional grid.
///
/// Component screen positions will automatically be calculated based
/// on their position in the grid.
///
#[derive(Grid)]
pub struct Grid<'a> {
    co: Cell<newtGrid>,
    added_to_parent: Cell<bool>,
    children: Vec<&'a dyn Component>,
    cols: i32,
    rows: i32
}

impl<'a> Grid<'a> {
    ///
    /// Create a new Grid with the specified columns and rows.
    ///
    /// * `cols` - The number of columns the `Grid` should have.
    /// * `rows` - The number of rows the `Grid` should have.
    ///
    pub fn new(cols: i32, rows: i32) -> Grid<'a> {
        assert!(cols > 0, "`cols` must be greater than 0");
        assert!(rows > 0, "`rows` must be greater than 0");

        Grid {
            co: unsafe { Cell::new(newtCreateGrid(cols, rows)) },
            added_to_parent: Cell::new(false),
            children: Vec::new(),
            cols, rows
        }
    }

    ///
    /// Add a component or sub-grid to the positon (`col`, `row`) in the grid.
    ///
    /// * `col` - The column to position the component.
    /// * `row` - The row to position the component.
    /// * `val` - The `Component` to be added to the `Grid`.
    /// * `pad_left` - The amount of padding towards the left of the cell.
    /// * `pad_top` - The amount of padding towards the top of the cell.
    /// * `pad_right` - The amount of padding towards the right of the cell.
    /// * `pad_bottom` - The amount of padding towards the bottom of the cell.
    /// * `anchor` - Anchor the component towards the specified direction of
    ///              its cell compared to surrounding cells. See
    ///              [anchor flags][anchors].
    /// * `flags` - [Flags][flags] modifying sub-Grid behavior.
    ///
    /// [flags]: crate::constants::grid
    /// [anchors]: crate::constants::grid
    ///
    pub fn set_field(&mut self, col: i32, row: i32, val: &'a dyn Component,
                     pad_left: i32, pad_top: i32, pad_right: i32,
                     pad_bottom: i32, anchor: i32, flags: i32) {

        if col >= self.cols || row >= self.rows {
            panic!("Attempting to set a field at an invalid position ({}, {})", col, row);
        }

        let r#type = val.grid_element_type();
        let co = val.co();
        self.children.push(val);

        unsafe {
            newtGridSetField(self.grid_ptr(), col, row, r#type,
                             co as *mut c_void, pad_left, pad_top, pad_right,
                             pad_bottom, anchor, flags);
        }
    }
}

///
/// Wrap a `Grid` in a centered window.
///
/// Wraps a `Grid` in a centered window and automatically displays it.
///
/// * `grid` - The `Grid` to be wrapped in the window.
/// * `title` - The title of the window to be displayed.
///
pub fn wrapped_window(grid: &dyn r#trait::Grid, title: &str) {
    let c_str = CString::new(title).unwrap();
    unsafe {
        newtGridWrappedWindow(grid.grid_ptr(), c_str.as_ptr() as *mut c_char);
    }
}

///
/// Wrap a `Grid` in a window at a specified location.
///
/// Wraps a `Grid` in a window and displays the window at the specified
/// location.
///
/// * `grid` - The `Grid` to be wrapped in the window.
/// * `title` - The title of the window to be displayed.
/// * `left` - The left-most position of the window.
/// * `top` - The top-most position of the window.
///
pub fn wrapped_window_at(grid: &dyn r#trait::Grid, title: &str,
                         left: i32, top: i32) {
    let c_str = CString::new(title).unwrap();
    unsafe {
        newtGridWrappedWindowAt(grid.grid_ptr(), c_str.as_ptr() as *mut c_char,
                                left, top);
    }
}

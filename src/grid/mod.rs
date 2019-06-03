//!
//! Methods for arranging component placement.
//!
extern crate newt_sys;
use libc::c_void;
use std::ffi::CString;
use newt_sys::*;

use crate::components::Component;

#[doc(hidden)]
pub mod r#trait;
#[doc(hidden)]
pub mod basic_window;
#[doc(hidden)]
pub mod button_bar;
#[doc(hidden)]
pub mod horizontal_stacked_grid;
#[doc(hidden)]
pub mod simple_window;
#[doc(hidden)]
pub mod vertical_stacked_grid;

#[doc(inline)]
pub use self::basic_window::BasicWindow;
#[doc(inline)]
pub use self::button_bar::ButtonBar;
#[doc(inline)]
pub use self::horizontal_stacked_grid::HorizontalStackedGrid;
#[doc(inline)]
pub use self::simple_window::SimpleWindow;
#[doc(inline)]
pub use self::vertical_stacked_grid::VerticalStackedGrid;

///
/// Trait implemented by Grid types.
///
pub use self::r#trait::Grid as GridTrait;

///
/// A grid for arranging components and sub-grids.
///
/// Component screen positions will automatically be calculated based
/// on their position in the grid.
///
#[derive(Grid)]
pub struct Grid<'a> {
    grid: newtGrid,
    cols: i32,
    rows: i32,
    added_to_parent: bool,
    children: Option<Vec<&'a mut dyn Component>>
}

impl<'a> Grid<'a> {
    ///
    /// Create a new Grid with the specified columns and rows.
    ///
    pub fn new(cols: i32, rows: i32) -> Grid<'a> {
        assert!(cols > 0, "`cols` must be greater than 0");
        assert!(rows > 0, "`rows` must be greater than 0");

        Grid {
            grid: unsafe { newtCreateGrid(cols, rows) },
            added_to_parent: false,
            cols, rows,
            children: None
        }
    }

    ///
    /// Add a component or sub-grid to the positon (`col`, `row`) in the grid.
    ///
    pub fn set_field(&mut self, col: i32, row: i32, val: &'a mut dyn Component,
                     pad_left: i32, pad_top: i32, pad_right: i32,
                     pad_bottom: i32, anchor: i32, flags: i32) {

        if col >= self.cols || row >= self.rows {
            panic!("Attempting to set a field at an invalid position ({}, {})", col, row);
        }

        let r#type = val.grid_element_type();
        let co = val.co();

        if let Some(children) = &mut self.children {
            children.push(val);
        } else {
            let mut children = Vec::new();
            children.push(val);
            self.children = Some(children);
        }

        unsafe {
            newtGridSetField(self.grid, col, row, r#type, co as *mut c_void,
                             pad_left, pad_top, pad_right, pad_bottom, anchor,
                             flags);
        }
    }
}

///
/// Wrap a grid in a centered window.
///
pub fn wrapped_window(grid: &dyn self::r#trait::Grid, title: &str) {
    let c_str = CString::new(title).unwrap();
    unsafe { newtGridWrappedWindow(grid.grid(), c_str.as_ptr() as *mut i8); }
}

///
/// Wrap a grid in a window at a specified location.
///
pub fn wrapped_window_at(grid: &dyn self::r#trait::Grid, title: &str,
                         left: i32, top: i32) {
    let c_str = CString::new(title).unwrap();
    unsafe {
        newtGridWrappedWindowAt(grid.grid(), c_str.as_ptr() as *mut i8,
                                left, top);
    }
}

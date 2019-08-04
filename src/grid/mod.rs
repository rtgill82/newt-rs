//!
//! Methods for easily arranging component placement.
//!
//! Grids can be used to automatically arrange widgets in a window without
//! having to manually specify their positions. The standard [`Grid`][grid]
//! can be used to place components in specific arrangements. There are also
//! [horizontal][horizontal] and [vertical][vertical] flavors to easily build
//! rows or columns of components. The [`ButtonBar`][button_bar] will build
//! a horizontal `Grid` full of `Button`s when provided with a list of strings to
//! use as `Button` labels.
//!
//! Simple window management involving grids is also provided.
//! [`grid::wrapped_window`][wrapped] and [`grid::wrapped_window_at`][wrapped_at]
//! will create a titled window wrapping a `Grid` for quick information
//! display. [`SimpleWindow`][simple] and [`BasicWindow`][basic] include
//! `ButtonBar`s as well for simple user interaction.
//!
//! [grid]: struct.Grid.html
//! [horizontal]: struct.HorizontalGrid.html
//! [vertical]: struct.VerticalGrid.html
//! [button_bar]: struct.ButtonBar.html
//! [wrapped]: fn.wrapped_window.html
//! [wrapped_at]: fn.wrapped_window_at.html
//! [simple]: struct.SimpleWindow.html
//! [basic]: struct.BasicWindow.html
//!
//! ## Example
//!
//! ```rust no_run
//! extern crate newt;
//! use newt::grid::*;
//! use newt::prelude::*;
//!
//! pub fn main() {
//!     newt::init().unwrap();
//!     newt::cls();
//!
//!     let rv;
//!
//!     let mut l1 = Label::new(0, 0, "Hello");
//!     let mut l2 = Label::new(0, 0, "World");
//!     let components: &mut [&mut dyn Component] = &mut [&mut l1, &mut l2];
//!
//!     // Be sure to allocate the `Form` outside of the `Grid` subscope.
//!     let mut form = Form::new(None, 0);
//!     let mut stacked = HorizontalGrid::new(components);
//!     let mut button_bar = ButtonBar::new(&["Yes", "No", "Maybe"]);
//!
//!     // Create a subscope so that `button_bar` can be mutably borrowed by
//!     // `grid` and iterated over immutably later.
//!     {
//!         let mut grid = Grid::new(2, 2);
//!         grid.set_field(1, 0, &mut stacked, 1, 1, 1, 1, 0, 0);
//!         grid.set_field(0, 1, &mut button_bar, 1, 1, 1, 1, 0, 0);
//!
//!         wrapped_window(&grid, "Grids");
//!         grid.add_to_form(&mut form).unwrap();
//!         rv = form.run().unwrap();
//!     }
//!     newt::finished();
//!
//!     // This requires that no mutable references to `button_bar` are
//!     // currently held.
//!     for (i, button) in button_bar.buttons().iter().enumerate() {
//!         if rv == *button {
//!             println!("Button {} pressed.", i);
//!         }
//!     }
//! }
//! ```
//!
//! ## Warning
//!
//! It's possible for _use after free_ errors to occur when calling
//! `Component` functions in situations where a `Form` is allocated within
//! a more limited scope than the components being added to it. This is
//! especially tricky when working with `Grid`s and `ButtonBar`s as a subscope
//! allows mutably adding a `ButtonBar` to other `Grid`s while allowing it's
//! buttons to be accessed immutably outside of the scope. Allocating the
//! `Form` within the subscope will destroy it, leaving the `Component`s it
//! contains invalid.
//!
//! The following illustrates what **NOT** to do:
//!
//! ```rust no_run
//! extern crate newt;
//! use newt::grid::*;
//! use newt::prelude::*;
//!
//! pub fn main() {
//!     newt::init().unwrap();
//!     newt::cls();
//!
//!     let rv;
//!
//!     let mut l1 = Label::new(0, 0, "Hello");
//!     let mut l2 = Label::new(0, 0, "World");
//!     let components: &mut [&mut dyn Component] = &mut [&mut l1, &mut l2];
//!
//!     let mut stacked = HorizontalGrid::new(components);
//!     let mut button_bar = ButtonBar::new(&["Yes", "No", "Maybe"]);
//!
//!     // Save the position of the first button on the `ButtonBar`.
//!     let pos1 = button_bar.buttons().first()
//!                          .unwrap().get_position();
//!
//!     // Create a subscope so that `button_bar` can be mutably borrowed by
//!     // `grid` and iterated over immutably later.
//!     {
//!         // Allocate `form` within subscope.
//!         let mut form = Form::new(None, 0);
//!         let mut grid = Grid::new(2, 2);
//!         grid.set_field(1, 0, &mut stacked, 1, 1, 1, 1, 0, 0);
//!         grid.set_field(0, 1, &mut button_bar, 1, 1, 1, 1, 0, 0);
//!
//!         wrapped_window(&grid, "Grids");
//!         grid.add_to_form(&mut form).unwrap();
//!         rv = form.run().unwrap();
//!         // `form` is destroyed when this scope exits.
//!     }
//!     newt::finished();
//!
//!     // Try accessing the `ButtonBar` buttons after the `Form` they've been
//!     // added to has been destroyed.
//!     let pos2 = button_bar.buttons().first()
//!                          .unwrap().get_position();
//!
//!     // This assertion will most likely fail.
//!     assert_eq!(pos1, pos2);
//! }
//! ```
//!
extern crate newt_sys;
use std::ffi::{CString,c_void};
use newt_sys::*;

use crate::Component;

#[doc(hidden)]
pub mod r#trait;
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

#[doc(inline)]
pub use self::r#trait::Grid as GridFns;

#[doc(hidden)]
#[deprecated(since = "0.5.4", note = "Please use `newt::grid::GridFns` instead")]
pub use self::GridFns as GridTrait;

///
/// Arrange `Component`s and sub-grids within a two-dimensional grid.
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
/// Wrap a `Grid` in a centered window.
///
pub fn wrapped_window(grid: &dyn self::r#trait::Grid, title: &str) {
    let c_str = CString::new(title).unwrap();
    unsafe { newtGridWrappedWindow(grid.as_grid(), c_str.as_ptr() as *mut i8); }
}

///
/// Wrap a `Grid` in a window at a specified location.
///
pub fn wrapped_window_at(grid: &dyn self::r#trait::Grid, title: &str,
                         left: i32, top: i32) {
    let c_str = CString::new(title).unwrap();
    unsafe {
        newtGridWrappedWindowAt(grid.as_grid(), c_str.as_ptr() as *mut i8,
                                left, top);
    }
}

//
// Copyright (C) 2022 Robert Gill <rtgill82@gmail.com>
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
//!     let l1 = Label::new(0, 0, "Hello");
//!     let l2 = Label::new(0, 0, "World");
//!
//!     let stacked = HorizontalGrid::new(&[&l1, &l2]);
//!     let button_bar = ButtonBar::new(&["Yes", "No", "Maybe"]);
//!
//!     let mut grid = Grid::new(2, 2);
//!     grid.set_field(1, 0, &stacked, 1, 1, 1, 1, 0, 0);
//!     grid.set_field(0, 1, &button_bar, 1, 1, 1, 1, 0, 0);
//!
//!     wrapped_window(&grid, "Grids");
//!     let mut form = Form::new(None, 0);
//!     grid.add_to_form(&mut form).unwrap();
//!     rv = form.run().unwrap();
//!     newt::finished();
//!
//!     for (i, button) in button_bar.buttons().iter().enumerate() {
//!         if rv == *button {
//!             println!("Button {} pressed.", i);
//!         }
//!     }
//! }
//! ```
//!

#![cfg(feature = "asm")]
pub mod r#trait;
pub use crate::asm::grid::*;

#[doc(inline)]
pub use self::r#trait::GridFns;

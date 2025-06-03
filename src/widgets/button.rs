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
use std::ffi::CString;

use newt_sys::*;

///
/// A widget that when activated causes the currently running
/// [Form][form] to exit.
///
/// [form]: crate::widgets::form::Form
///
#[derive(Component)]
pub struct Button {
    co: Cell<newtComponent>,
    added_to_parent: Cell<bool>
}

impl Button {
    ///
    /// Create a new `Button`.
    ///
    /// * `left` - The left-most position of the button.
    /// * `top` - The top-most position of the button.
    /// * `text` - The text to be displayed as the label on the button.
    ///
    pub fn new(left: i32, top: i32, text: &str) -> Button {
        let c_str = CString::new(text).unwrap();
        Button {
            co: unsafe {
                Cell::new(newtButton(left, top, c_str.as_ptr()))
            },
            added_to_parent: Cell::new(false)
        }
    }

    #[cfg(feature = "asm")]
    pub(crate) fn new_co(co: newtComponent) -> Button {
        Button {
            co: Cell::new(co),
            added_to_parent: Cell::new(false)
        }
    }
}

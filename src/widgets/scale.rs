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

use newt_sys::*;
use crate::component::Component;

///
/// A widget that can display a filled horizontal bar representing a percentage.
///
#[derive(Component)]
pub struct Scale {
    co: Cell<newtComponent>,
    added_to_parent: Cell<bool>
}

impl Scale {
    ///
    /// Create a new `Scale` widget.
    ///
    /// * `left` - The left-most position of the `Scale`.
    /// * `top` - The top-most position of the `Scale`.
    /// * `width` - The width of the `Scale`.
    /// * `maximum` - The maximum value that the `Scale` will represent.
    ///
    pub fn new(left: i32, top: i32, width: i32, maximum: i64) -> Scale {
        Scale {
            co: unsafe {
                let co = newtScale(left, top, width, maximum);
                Cell::new(co)
            },
            added_to_parent: Cell::new(false)
        }
    }

    ///
    /// Set the current value of the `Scale`.
    ///
    /// The `Scale` will be updated to display the percentage of
    /// `amount`/`maximum`.
    ///
    /// * `amount` - The amount to set the current value to.
    ///
    pub fn set(&self, amount: u64) {
        unsafe { newtScaleSet(self.co(), amount); }
    }

    ///
    /// Set the colors of the `Scale`.
    ///
    /// * `empty` - The color used to represent the unfilled percentage.
    /// * `full` - The color used to represent the filled percentage.
    ///
    pub fn set_colors(&self, empty: i32, full: i32) {
        unsafe { newtScaleSetColors(self.co(), empty, full); }
    }
}

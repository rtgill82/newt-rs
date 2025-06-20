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
use crate::component::Component;

///
/// A simple widget for displaying static text.
///
#[derive(Component)]
pub struct Label {
    co: Cell<newtComponent>,
    added_to_parent: Cell<bool>
}

impl Label {
    ///
    /// Create a new `Label`.
    ///
    /// * `left` - The left-most position of the `Label`.
    /// * `top` - The top-most position of the `Label`.
    /// * `text` - The text to be displayed as the `Label`.
    ///
    pub fn new(left: i32, top: i32, text: &str) -> Label {
        let c_text = CString::new(text).unwrap();
        Label {
            co: unsafe {
                let co = newtLabel(left, top, c_text.as_ptr());
                Cell::new(co)
            },
            added_to_parent: Cell::new(false)
        }
    }

    ///
    /// Set the `Label`'s text.
    ///
    /// * `text` - The text to be displayed as the `Label`.
    ///
    pub fn set_text(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe { newtLabelSetText(self.co(), c_text.as_ptr()); }
    }

    ///
    /// Set the colors of the `Label.
    ///
    /// See [COLORSET_CUSTOM][colorset_custom] for defining new color sets.
    ///
    /// * `colorset` - The color set to use for the `Label`.
    ///
    /// [colorset_custom]: crate::constants::COLORSET_CUSTOM
    ///
    pub fn set_colors(&self, colorset: i32) {
        unsafe { newtLabelSetColors(self.co(), colorset); }
    }
}

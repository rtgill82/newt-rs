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
use std::os::raw::c_char;

use newt_sys::*;
use crate::component::Component;

///
/// A widget that can display multiple lines of text.
///
#[derive(Component)]
pub struct Textbox {
    co: Cell<newtComponent>,
    added_to_parent: Cell<bool>
}

impl Textbox {
    ///
    /// Create a new `Textbox`.
    ///
    /// * `left` - The left-most postion of the `Textbox`.
    /// * `top` - The top-most position of the `Textbox`.
    /// * `width` - The width of the `Textbox`.
    /// * `height` - The height of the `Textbox`.
    /// * `flags` - The [flags][textbox] specifying `Textbox` behavior.
    ///             See also [general flags][flags].
    ///
    /// [textbox]: crate::constants::textbox
    /// [flags]: crate::constants::flags
    ///
    pub fn new(left: i32, top: i32, width: i32, height: i32, flags: i32)
      -> Textbox {
        Textbox {
            co: unsafe {
                let co = newtTextbox(left, top, width, height, flags);
                Cell::new(co)
            },
            added_to_parent: Cell::new(false)
        }
    }

    ///
    /// Create a new `Textbox` in which text is automatically reflowed.
    ///
    /// A `Textbox` in which the text is automatically reflowed on whitespace
    /// boundaries, rather than splitting words when word wrapping. Height is
    /// automatically calculated and word wrap is automatically enabled.
    ///
    /// * `left` - The left-most position of the `Textbox`.
    /// * `top` - The top-most position of the `Textbox`.
    /// * `text` - The text for the `Textbox` to display.
    /// * `width` - The width of the `Textbox`.
    /// * `flex_down` - The minimum difference from target width for word
    ///                 wrapping.
    /// * `flex_up` - The maximum difference from target width for word
    ///               wrapping.
    /// * `flags` - The [flags][textbox] specifying widget behavior.
    ///             See also [general flags][flags].
    ///
    /// [textbox]: crate::constants::textbox
    /// [flags]: crate::constants::flags
    ///
    pub fn new_reflowed(left: i32, top: i32, text: &str, width: i32,
                        flex_down: i32, flex_up: i32, flags: i32)
      -> Textbox {
        let c_text = CString::new(text).unwrap();
        Textbox {
            co: unsafe {
                let co = newtTextboxReflowed(left, top,
                                             c_text.as_ptr() as *mut c_char,
                                             width, flex_down, flex_up,
                                             flags);
                Cell::new(co)
            },
            added_to_parent: Cell::new(false)
        }
    }

    ///
    /// Set the text displayed in the `Textbox`.
    ///
    /// * `text` - The text for the `Textbox` to display.
    ///
    pub fn set_text(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe { newtTextboxSetText(self.co(), c_text.as_ptr()); }
    }

    ///
    /// Set the height of the `Textbox`.
    ///
    /// * `height` - The height to set the `Textbox`.
    ///
    pub fn set_height(&self, height: i32) {
        unsafe { newtTextboxSetHeight(self.co(), height); }
    }

    ///
    /// `Returns` the number of lines of text displayed in the `Textbox`.
    ///
    pub fn get_num_lines(&self) -> i32 {
        unsafe { newtTextboxGetNumLines(self.co()) }
    }

    ///
    /// Set the colors of the `Textbox`.
    ///
    /// See [COLORSET_CUSTOM][colorset_custom] for defining new color sets.
    ///
    /// * `normal` - The color set to use when in a normal state.
    /// * `active` - The color set to use when activated or selected.
    ///
    /// [colorset_custom]: crate::constants::COLORSET_CUSTOM
    ///
    pub fn set_colors(&self, normal: i32, active: i32) {
        unsafe { newtTextboxSetColors(self.co(), normal, active); }
    }
}

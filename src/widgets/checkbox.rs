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
use std::ptr;

use newt_sys::*;
use crate::component::Component;
use crate::constants::FlagsSense;
use crate::intern::funcs::char_slice_to_cstring;

///
/// A widget displaying a box which can be cycled through various
/// checked states.
///
#[derive(Component)]
pub struct Checkbox {
    co: Cell<newtComponent>,
    added_to_parent: Cell<bool>
}

impl Checkbox {
    ///
    /// Create a new `Checkbox` widget.
    ///
    /// * `left` - The left-most position of the `Checkbox`.
    /// * `top` - The top-most position of the `Checkbox`.
    /// * `text` - The label to be displayed with the `Checkbox`.
    /// * `default` - The optional default state of the `Checkbox`.
    ///               (The default value to be held from the sequence of
    ///               possible values).
    /// * `sequence` - The optional sequence of values possible in the
    ///                `Checkbox`. (Defaults to ` ` and `*` if not provided).
    ///
    pub fn new(left: i32, top: i32, text: &str, default: Option<char>,
               sequence: Option<&[char]>)
      -> Checkbox {
        let c_text = CString::new(text).unwrap();
        let default: c_char = match default {
            Some(value) => value as c_char,
            None => 0 as c_char
        };

        let cstr: CString;
        let c_seq = match sequence {
            Some(seq) => {
                cstr = char_slice_to_cstring(&seq);
                cstr.as_ptr()
            },
            None => ptr::null()
        };

        Checkbox {
            co: unsafe {
                let co = newtCheckbox(left, top, c_text.as_ptr(), default,
                                      c_seq, ptr::null_mut());
                Cell::new(co)
            },
            added_to_parent: Cell::new(false)
        }
    }

    ///
    /// Get the current value of the `Checkbox`.
    ///
    pub fn get_value(&self) -> char {
        unsafe { newtCheckboxGetValue(self.co()) as u8 as char }
    }

    ///
    /// Set the current value of the `Checkbox`.
    ///
    /// * `value` - The value to set the `Checkbox` to. It does not necessarily
    ///             need to be one specified in `sequence` during the `Checkbox`
    ///             creation.
    ///
    pub fn set_value(&self, value: char) {
        unsafe { newtCheckboxSetValue(self.co(), value as c_char); }
    }

    ///
    /// Set flags modifying the `Checkbox` behavior.
    ///
    /// See [flags][flags] for possible flags.
    ///
    /// * `flags` - A logical `or`ed list of flags.
    /// * `sense` - The operation used to set the flags
    ///             (`Set`, `Reset`, or `Toggle`).
    ///
    /// [flags]: crate::constants::flags
    ///
    pub fn set_flags(&self, flags: i32, sense: FlagsSense) {
        unsafe { newtCheckboxSetFlags(self.co(), flags, sense as u32); }
    }
}

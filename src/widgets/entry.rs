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
use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use std::ptr;

use newt_sys::*;
use crate::component::Component;
use crate::constants::FlagsSense;

///
/// A field for reading text input from the user.
///
/// An [EntryFilter][entry_filter] can be use to filter characters as the user
/// enters them into the `Entry`.
///
/// [entry_filter]: crate::callbacks::EntryFilter
///
/// ## Example
/// ```rust no_run
/// extern crate newt;
/// use newt::callbacks::EntryFilter;
/// use newt::prelude::*;
///
/// pub fn main() {
///     newt::init().unwrap();
///     newt::cls();
///     newt::centered_window(22, 5, None).unwrap();
///
///     // last character entered
///     let mut g_ch: char = '\0';
///     // last cursor position
///     let mut g_cursor: i32 = 0;
///     // user data from last Entry modified
///     let mut g_data: i32 = 0;
///
///     // Create closure to be used to filter the Entry field.
///     let mut f = |_e: &Entry, data: Option<&i32>, ch: char, cursor: i32| {
///         g_data = *data.unwrap(); // set user data
///         g_ch = ch;               // set character entered
///         g_cursor = cursor;       // set cursor position
///
///         // The returned character gets added to the entry.
///         // If for example you want to fill an entry with asterisks despite
///         // what the user entered, then return '*' here.
///         return ch; // Return the entered character.
///     };
///
///     let l1 = Label::new(1, 1, "Entry 1:");
///     let l2 = Label::new(1, 2, "Entry 2:");
///     let e1 = Entry::new(10, 1, None, 10, 0);
///     let e2 = Entry::new(10, 2, None, 10, 0);
///     let ok = CompactButton::new(7, 4, "Ok");
///     let components: &[&dyn Component] = &[&l1, &l2, &e1, &e2, &ok];
///
///     let mut form = Form::new(None, 0);
///     form.add_components(components).unwrap();
///
///     // Filter the first Entry, passing user data `5`.
///     let mut filter = EntryFilter::new(&e1, Some(5), &mut f);
///     // Filter the second Entry, passing user data `10`.
///     filter.add_entry(&e2, Some(10));
///
///     form.run().unwrap();
///     newt::finished();
///
///     println!("Entry 1: {}", e1.get_text());
///     println!("Entry 2: {}", e2.get_text());
///
///     // Display the last values set by the filter.
///     println!("ch = {}", g_ch);
///     println!("cursor = {}", g_cursor);
///     println!("data = {}", g_data);
/// }
/// ```
///
#[derive(Component)]
pub struct Entry {
    co: Cell<newtComponent>,
    added_to_parent: Cell<bool>
}

impl Entry {
    ///
    /// Create a new `Entry`.
    ///
    /// * `left` - The left-most position of the `Entry`.
    /// * `top` - The top-most position of the `Entry`.
    /// * `initial_value` - The optional inital text of the `Entry`.
    /// * `width` - The width of the `Entry`.
    /// * `flags` - [Flags][entry] modifying the behavior of the `Entry`.
    ///             See also [generalized flags][flags].
    ///
    /// [entry]: crate::constants::entry
    /// [flags]: crate::constants::flags
    ///
    pub fn new(left: i32, top: i32, initial_value: Option<&str>, width: i32,
               flags: i32) -> Entry {
        let c_str: CString;
        let ptr = match initial_value {
            Some(text) => {
                c_str = CString::new(text).unwrap();
                c_str.as_ptr()
            },
            None => ptr::null()
        };

        Entry {
            co: unsafe {
                let co = newtEntry(left, top, ptr, width, ptr::null_mut(),
                                   flags);
                Cell::new(co)
            },
            added_to_parent: Cell::new(false)
        }
    }

    ///
    /// Get the text currently in the `Entry`.
    ///
    pub fn get_text(&self) -> String {
        unsafe { CStr::from_ptr(newtEntryGetValue(self.co())) }
            .to_string_lossy()
            .into_owned()
    }

    ///
    /// Set the current text in the `Entry`.
    ///
    /// * `text` - The text to enter into the `Entry`.
    /// * `cursor_at_end` - Move the cursor to the end of the string after
    ///                     setting the text.
    ///
    pub fn set_text(&self, text: &str, cursor_at_end: bool) {
        let c_str = CString::new(text).unwrap();
        unsafe {
            newtEntrySet(self.co(), c_str.as_ptr(), cursor_at_end as c_int);
        }
    }

    ///
    /// Set flags modifying the `Entry`'s behavior.
    ///
    /// See [Entry flags][entry] as well as [flags][flags] for possible flags.
    ///
    /// * `flags` - A logical `or`ed list of flags.
    /// * `sense` - The operation used to set the flags
    ///             (`Set`, `Reset`, or `Toggle`).
    ///
    /// [entry]: crate::constants::entry
    /// [flags]: crate::constants::flags
    ///
    pub fn set_flags(&self, flags: i32, sense: FlagsSense) {
        unsafe { newtEntrySetFlags(self.co(), flags, sense as u32); }
    }

    ///
    /// Set the colors of the `Entry`.
    ///
    /// See [COLORSET_CUSTOM][colorset_custom] for defining new color sets.
    ///
    /// * `normal` - The color set to use when in a normal state.
    /// * `active` - The color set to use when activated or selected.
    ///
    /// [colorset_custom]: crate::constants::COLORSET_CUSTOM
    ///
    pub fn set_colors(&self, normal: i32, disabled: i32) {
        unsafe { newtEntrySetColors(self.co(), normal, disabled); }
    }

    ///
    /// Get the current cursor position in the `Entry`.
    ///
    pub fn get_cursor_position(&self) -> i32 {
        unsafe { newtEntryGetCursorPosition(self.co()) }
    }

    ///
    /// Set the current cursor position in the `Entry`.
    ///
    pub fn set_cursor_position(&self, position: i32) {
        unsafe { newtEntrySetCursorPosition(self.co(), position) }
    }
}

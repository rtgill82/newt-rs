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

//!
//! Convenient windowing functions.
//!
use std::ffi::CString;
use std::os::raw::c_char;

use std::ffi::CStr;
use std::mem::size_of;
use std::os::raw::{c_int,c_void};

use newt_sys::*;

#[doc(no_inline)]
pub use crate::asm::win_menu_new;
#[doc(no_inline)]
pub use crate::asm::win_entries_new;

///
/// A struct used to pass initial [`Entry`][entry] information to the
/// `win_entries()` function.
///
/// [entry]: ../widgets/struct.Entry.html
///
#[derive(Default)]
pub struct WinEntry {
    pub(crate) text: CString,
    pub(crate) value: String,
    pub(crate) flags: c_int
}

impl WinEntry {
    ///
    /// Create a new `WinEntry`.
    ///
    /// * `text` - The text to display as the entry field label.
    /// * `value` - The initial value of the `Entry` field.
    /// * `flags` - The settings flags for the `Entry`.
    ///
    pub fn new(text: &str, value: &str, flags: i32) -> WinEntry {
        WinEntry {
            text: CString::new(text).unwrap(),
            value: String::from(value),
            flags
        }
    }

    ///
    /// Returns the value of the corresponding `Entry`. This is either
    /// the inital `value` set when the `WinEntry` is created, or the user
    /// entered data provided by the [`win_entries()`][win_entries] function
    /// if that has been run.
    ///
    /// [win_entries]: ../windows/fn.win_entries.html
    ///
    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}

pub(crate) struct WinEntryBuf<'a> {
    #[allow(dead_code)]
    values_text: Vec<CString>,
    entries: &'a mut [WinEntry],
    entries_buf: *mut newtWinEntry,
    values_buf: *mut *mut c_char
}

impl<'a> WinEntryBuf<'a> {
    pub fn new(entries: &'a mut [WinEntry]) -> WinEntryBuf<'a> {
        unsafe {
            let mut values_text = Vec::new();
            let size = size_of::<newtWinEntry>() * (entries.len() + 1);
            let entries_buf = libc::malloc(size) as *mut newtWinEntry;
            libc::memset(entries_buf as *mut c_void, 0, size);

            let size = size_of::<*mut c_char>() * (entries.len());
            let values_buf = libc::malloc(size) as *mut *mut c_char;
            libc::memset(values_buf as *mut c_void, 0, size);

            for (i, entry) in entries.iter().enumerate() {
                let entry_buf = entries_buf.add(i);
                let value_buf = values_buf.add(i);
                let value = CString::new(entry.value.as_str()).unwrap();
                *value_buf = value.as_ptr() as *mut c_char;

                (*entry_buf).text = entry.text.as_ptr() as *mut c_char;
                (*entry_buf).value = value_buf;
                (*entry_buf).flags = entry.flags;
                values_text.push(value);
            }

            WinEntryBuf { values_text, entries, entries_buf, values_buf }
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut newtWinEntry {
        self.entries_buf
    }
}

impl<'a> Drop for WinEntryBuf<'a> {
    fn drop(&mut self) {
        unsafe {
            for (i, entry) in self.entries.iter_mut().enumerate() {
                let buf = self.entries_buf.add(i);
                let value = CStr::from_ptr(*(*buf).value).to_str().unwrap();
                entry.value = String::from(value);
                libc::free(*(*buf).value as *mut c_void);
            }

            libc::free(self.entries_buf as *mut c_void);
            libc::free(self.values_buf as *mut c_void);
        }
    }
}

///
/// Open a window containing a [`Listbox`][listbox] menu.
///
/// _Requires that the `asm` feature be enabled._
///
/// * `title` - The window title.
/// * `text` - The message to display in the window.
/// * `suggested_width` - The preferred width for the window.
/// * `flex_down` - The minimum allowed difference between `suggested_width`
///                 and actual width.
/// * `flex_up` - The maximum allowed difference between `suggested_width`
///               and actual width.
/// * `max_list_height` - The maximum height to display the list of items.
/// * `items` - A slice containing the text for each item in the list.
/// * `buttons` - A slice containing the text for a number of buttons to display
///               in the window.
///
/// Returns a tuple pair as `(button, item)` where `button` is the button
/// number pressed to close the window and `item` is the item number in the
/// list that was selected. Returned `button` is numbered starting from `1`.
///
/// [listbox]: ../widgets/struct.Listbox.html
///
#[allow(clippy::too_many_arguments)]
pub fn win_menu(title: &str, text: &str, suggested_width: i32, flex_down: i32,
                flex_up: i32, max_list_height: i32, items: &[&str],
                buttons: &[&str]) -> (i32, i32)
{
    unsafe {
        win_menu_new(
            title,
            text,
            suggested_width,
            flex_down,
            flex_up,
            max_list_height,
            items,
            buttons
        )
    }
}

///
/// Open a window containing a number of text [`Entry`s][entry].
///
/// _Requires that the `asm` feature be enabled._
///
/// * `title` - The window title.
/// * `text` - The message to display in the window.
/// * `suggested_width` - The preferred width for the window.
/// * `flex_down` - The minimum allowed difference between `suggested_width`
///                 and actual width.
/// * `flex_up` - The maximum allowed difference between `suggested_width`
///               and actual width.
/// * `data_width` - The field width for all `Entry`s.
/// * `entries` - A slice containing a list of [`WinEntry`s][win_entry]
///               providing initial settings for each `Entry` field.
/// * `buttons` - A slice containing the text for a number of buttons to
///               display in the window.
///
/// Returns the number of the button pressed to close the window, starting
/// from `1`.
///
/// Each `WinEntry` in the `entries` array will be modified to contain the
/// data entered by the user. This can be accessed via the
/// [`WinEntry.value()`][win_entry_value] function.
///
/// [entry]: ../widgets/struct.Entry.html
/// [win_entry]: ../windows/struct.WinEntry.html
/// [win_entry_value]: ../windows/struct.WinEntry.html#method.value
///
#[allow(clippy::too_many_arguments)]
pub fn win_entries(title: &str, text: &str, suggested_width: i32, flex_down: i32,
                   flex_up: i32, data_width: i32, entries: &mut [WinEntry],
                   buttons: &[&str]) -> i32
{
    unsafe {
        win_entries_new(
            title,
            text,
            suggested_width,
            flex_down,
            flex_up,
            data_width,
            entries,
            buttons
        )
    }
}

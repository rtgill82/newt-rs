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

use newt_sys::*;

#[cfg(feature = "asm")]
pub use crate::asm::windows::*;

///
/// Open a simple message window.
///
pub fn win_message(title: &str, button_text: &str, text: &str) {
    unsafe {
        let c_title = CString::new(title).unwrap();
        let c_button = CString::new(button_text).unwrap();
        let escaped = str::replace(text, "%", "%%");
        let c_text = CString::new(escaped).unwrap();

        newtWinMessage(c_title.as_ptr() as *mut c_char,
                       c_button.as_ptr() as *mut c_char,
                       c_text.as_ptr() as *mut c_char);
    }
}

///
/// Open a window providing a choice of buttons.
///
/// Returns the number of the button pressed.
///
pub fn win_choice(title: &str, button1: &str, button2: &str, text: &str)
  -> i32 {
    unsafe {
        let c_title = CString::new(title).unwrap();
        let c_button1 = CString::new(button1).unwrap();
        let c_button2 = CString::new(button2).unwrap();
        let escaped = str::replace(text, "%", "%%");
        let c_text = CString::new(escaped).unwrap();

        newtWinChoice(c_title.as_ptr() as *mut c_char,
                      c_button1.as_ptr() as *mut c_char,
                      c_button2.as_ptr() as *mut c_char,
                      c_text.as_ptr() as *mut c_char) as i32
    }
}

///
/// Open a window with three buttons.
///
/// Returns the number of the button pressed.
///
pub fn win_ternary(title: &str, button1: &str, button2: &str, button3: &str,
                   text: &str) -> i32 {
    unsafe {
        let c_title = CString::new(title).unwrap();
        let c_button1 = CString::new(button1).unwrap();
        let c_button2 = CString::new(button2).unwrap();
        let c_button3 = CString::new(button3).unwrap();
        let escaped = str::replace(text, "%", "%%");
        let c_text = CString::new(escaped).unwrap();

        newtWinTernary(c_title.as_ptr() as *mut c_char,
                       c_button1.as_ptr() as *mut c_char,
                       c_button2.as_ptr() as *mut c_char,
                       c_button3.as_ptr() as *mut c_char,
                       c_text.as_ptr() as *mut c_char) as i32
    }
}

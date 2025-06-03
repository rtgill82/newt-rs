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
use std::ptr;

use newt_sys::*;
use crate::component::Component;

///
/// A set of widgets similar to [Checkboxes][checkbox] in which only one may
/// be selected at a time.
///
/// [checkbox]: crate::widgets::Checkbox
///
/// ## Example
/// ```rust no_run
/// extern crate newt;
/// use newt::prelude::*;
///
/// pub fn main() {
///     newt::init().unwrap();
///     newt::cls();
///     newt::centered_window(20, 6, Some("Options")).unwrap();
///
///     // Create the first `Radiobutton` in the set, set as default.
///     let radio1 = Radiobutton::new(4, 1, "Option 1", true, None);
///
///     // Create the second `Radiobutton in the set, adding `radio1` as the
///     // previous button.
///     let radio2 = Radiobutton::new(4, 2, "Option 2", false,
///                                   Some(&radio1));
///
///     // Create the third `Radiobutton` in the set, adding `radio2` as the
///     // previous button.
///     let radio3 = Radiobutton::new(4, 3, "Option 3", false,
///                                   Some(&radio2));
///
///     let ok = CompactButton::new(7, 5, "Ok");
///
///     let mut form = Form::new(None, 0);
///     form.add_components(&[&radio1, &radio2, &radio3, &ok]).unwrap();
///     form.run().unwrap();
///     newt::finished();
///
///     let buttons = [(&radio1, "Option 1"),
///                    (&radio2, "Option 2"),
///                    (&radio3, "Option 3")];
///
///     // Get the currently selected `Radiobutton` from the first in the set
///     // (any will do).
///     let current = radio1.get_current();
///
///     // Find the currently selected `Radiobutton` in the array of available
///     // buttons.
///     for val in buttons.iter() {
///         let &(radio, text) = val;
///         if *radio == current {
///             println!("Selected Option: {}", text);
///         }
///     }
/// }
/// ```
///
#[derive(Component)]
pub struct Radiobutton {
    co: Cell<newtComponent>,
    added_to_parent: Cell<bool>
}

impl Radiobutton {
    ///
    /// Create a new `Radiobutton`.
    ///
    /// A set of `Radiobutton`s is created by creating a `Radiobutton` with
    /// `prev_button` set as `None`. This is the first `Radiobutton` in the
    /// set. The next created `Radiobutton` should set the first `Radiobutton`
    /// as `prev_button`, and the next created `Radiobutton` the previous,
    /// and so on until the set is complete.
    ///
    /// * `left` - The left-most position of the `Radiobutton`.
    /// * `top` - The top-most position of the `Radiobutton`.
    /// * `text` - The text to be displayed as the label of the `Radiobutton`.
    /// * `default` - Set this `Radiobutton` as the selected default of its
    ///               set.
    /// * `prev_button` - The optional previous `Radiobutton` in the set.
    ///                   Providing `None` here makes the created
    ///                   `Radiobutton` the first in a set.
    ///
    pub fn new(left: i32, top: i32, text: &str, default: bool,
               prev_button: Option<&Radiobutton>) -> Radiobutton {
        let c_text = CString::new(text).unwrap();
        let ptr = match prev_button {
            Some(radio_button) => radio_button.co(),
            None => ptr::null_mut()
        };

        Radiobutton {
            co: unsafe {
                let co = newtRadiobutton(left, top, c_text.as_ptr(),
                                         default as i32, ptr);
                Cell::new(co)
            },
            added_to_parent: Cell::new(false)
        }
    }

    ///
    /// Get the currently selected `Radiobutton` from the set.
    ///
    pub fn get_current(&self) -> Radiobutton {
        Radiobutton {
            co: unsafe { Cell::new(newtRadioGetCurrent(self.co())) },
            added_to_parent: Cell::new(true)
        }
    }

    ///
    /// Set this `Radiobutton` as the currently selected one in its set.
    ///
    pub fn set_current(&self) {
        unsafe { newtRadioSetCurrent(self.co()) }
    }
}

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
//! newt UI `Component`s.
//!
use std::os::raw::c_int;

use crate::private::ComponentPtr;
use newt_sys::*;

mod vertical_scrollbar;
pub use self::vertical_scrollbar::VerticalScrollbar;

mod button;
pub use self::button::Button;
mod checkbox;
pub use self::checkbox::Checkbox;
mod checkbox_tree;
pub use self::checkbox_tree::CheckboxTree;
mod compact_button;
pub use self::compact_button::CompactButton;
mod entry;
pub use self::entry::Entry;
mod label;
pub use self::label::Label;
mod listbox;
pub use self::listbox::Listbox;
mod radiobutton;
pub use self::radiobutton::Radiobutton;
mod radiobutton_set;
pub use self::radiobutton_set::RadiobuttonSet;
mod scale;
pub use self::scale::Scale;
mod textbox;
pub use self::textbox::Textbox;

///
/// Implement shared functions for newt component widgets.
///
pub trait WidgetFns: ComponentPtr {
    ///
    /// Allow the widget to be focused when its parent [`Form`][form] is run.
    ///
    /// [form]: crate::form::Form
    ///
    fn takes_focus(&self, value: bool) {
        unsafe { newtComponentTakesFocus(self.co_ptr(), value as c_int); }
    }

    ///
    /// Get the position of the widget's top left corner.
    ///
    /// `Returns` a tuple in the form of `(left, top)`.
    ///
    fn get_position(&self) -> (i32, i32) {
        let mut left: i32 = 0;
        let mut top:  i32 = 0;

        unsafe {
            newtComponentGetPosition(self.co_ptr(), &mut left, &mut top)
        };
        (left, top)
    }

    ///
    /// Get the widget's width and height.
    ///
    /// `Returns` a tuple in the form of `(width, height)`.
    ///
    fn get_size(&self) -> (i32, i32) {
        let mut width:  i32 = 0;
        let mut height: i32 = 0;

        unsafe {
            newtComponentGetSize(self.co_ptr(), &mut width, &mut height)
        };
        (width, height)
    }
}

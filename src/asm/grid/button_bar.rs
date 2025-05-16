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
use crate::widgets::Button;

use crate::intern::Parent;
use crate::asm;

///
/// Creates a row of buttons.
///
#[derive(Grid)]
pub struct ButtonBar {
    co: Cell<newtGrid>,
    added_to_parent: Cell<bool>,
    children: Vec<Button>
}

impl ButtonBar {
    ///
    /// Create a new grid containing a row of buttons. The buttons will
    /// be labeled with the strings provided in `buttons`.
    ///
    /// * `buttons` - A list of strings to use as button labels.
    ///
    pub fn new(buttons: &[&str]) -> ButtonBar {
        let grid: newtGrid;

        let len = buttons.len();
        let mut buttons_buf: Vec<newtComponent> = Vec::with_capacity(len);

        unsafe {
            let buttons_ptr: *mut newtComponent = buttons_buf.as_mut_ptr();
            grid = asm::button_bar_new(buttons, buttons_ptr);
            buttons_buf.set_len(len);
        }

        let mut buttons = Vec::new();
        for co in buttons_buf {
            buttons.push(Button::new_co(co));
        }

        ButtonBar {
            co: Cell::new(grid),
            added_to_parent: Cell::new(false),
            children: buttons
        }
    }

    ///
    /// Return the array of buttons contained by the grid.
    ///
    pub fn buttons(&self) -> &[Button] {
        return self.children.as_slice();
    }
}

impl Parent for ButtonBar {
    fn children(&self) -> Vec<&dyn Component> {
        let mut vec: Vec<&dyn Component> = Vec::new();
        for child in self.children.iter() {
            vec.push(child);
        }
        vec
    }
}

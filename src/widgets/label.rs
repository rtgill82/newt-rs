//
// Copyright (C) 2019  Robert Gill <locke@sdf.lonestar.org>
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

    pub fn set_text(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe { newtLabelSetText(self.co(), c_text.as_ptr()); }
    }

    pub fn set_colors(&self, colorset: i32) {
        unsafe { newtLabelSetColors(self.co(), colorset); }
    }
}

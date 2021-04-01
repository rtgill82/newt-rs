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

///
/// A progress bar widget.
///
#[derive(Component)]
pub struct Scale {
    co: Cell<newtComponent>,
    added_to_parent: Cell<bool>
}

impl Scale {
    pub fn new(left: i32, top: i32, width: i32, full_value: i64) -> Scale {
        Scale {
            co: unsafe {
                let co = newtScale(left, top, width, full_value);
                Cell::new(co)
            },
            added_to_parent: Cell::new(false)
        }
    }

    pub fn set(&self, amount: u64) {
        unsafe { newtScaleSet(self.co(), amount); }
    }

    pub fn set_colors(&self, empty: i32, full: i32) {
        unsafe { newtScaleSetColors(self.co(), empty, full); }
    }
}

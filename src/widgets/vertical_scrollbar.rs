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

///
/// A vertical scrollbar which can be attached to a
/// [Form](crate::widgets::form::Form).
///
#[derive(Component)]
pub struct VerticalScrollbar {
    co: Cell<newtComponent>,
    added_to_parent: Cell<bool>
}

impl VerticalScrollbar {
    ///
    /// Create a new `VerticalScrollbar`.
    ///
    /// * `left` - The left-most position of the `VerticalScrollbar`.
    /// * `top` - The top-most position of the `VerticalScrollabar`.
    /// * `height` - The height of the `VerticalScrollbar`.
    /// * `normal_colorset` - The normal colorset of the `VerticalScrollbar`.
    /// * `thumb_colorset` - The colorset when the thumb button is activated.
    ///
    /// (as of `newt-0.52.25`, `thumb_colorset` does not appear to be used)
    ///
    pub fn new(left: i32, top: i32, height: i32, normal_colorset: i32,
               thumb_colorset: i32) -> VerticalScrollbar
    {
        VerticalScrollbar {
            co: unsafe {
                let co = newtVerticalScrollbar (left, top, height,
                                                normal_colorset,
                                                thumb_colorset);
                Cell::new(co)
            },
            added_to_parent: Cell::new(false)
        }
    }
}

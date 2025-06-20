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
use crate::constants::{GRID_COMPONENT,GRID_SUBGRID};

///
/// Create a simple window using sub-grids.
///
#[derive(Grid)]
pub struct BasicWindow<'a> {
    co: Cell<newtGrid>,
    added_to_parent: Cell<bool>,
    children: Vec<&'a dyn Component>
}

impl<'a> BasicWindow<'a> {
    ///
    /// Create a new BasicWindow grid.
    ///
    /// * `text` - A text component to be displayed at the top of the window.
    /// * `middle` - A sub-grid to display in the middle of the window.
    /// * `buttons` - A sub-grid to display at the bottom of the window,
    ///               hopefully containing buttons.
    ///
    pub fn new(text: &'a dyn Component, middle: &'a dyn Component,
               buttons: &'a dyn Component)
        -> BasicWindow<'a>
    {

        assert_eq!(text.grid_element_type(), GRID_COMPONENT);
        assert_eq!(middle.grid_element_type(), GRID_SUBGRID);
        assert_eq!(buttons.grid_element_type(), GRID_SUBGRID);

        let grid = unsafe {
            newtGridBasicWindow(text.co(), middle.grid_ptr(), buttons.grid_ptr())
        };

        let mut children: Vec<&'a dyn Component> = Vec::new();
        children.push(text);
        children.push(middle);
        children.push(buttons);

        BasicWindow {
            co: Cell::new(grid),
            added_to_parent: Cell::new(false),
            children
        }
    }
}

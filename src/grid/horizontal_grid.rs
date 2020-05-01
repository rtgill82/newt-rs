//
// Copyright (C) 2019 Robert Gill <locke@sdf.org>
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

use libc::c_void;
use std::cell::Cell;

use newt_sys::*;
use crate::component::Component;
use crate::intern::asm;

///
/// Arrange components horizontally.
///
#[derive(Grid)]
pub struct HorizontalGrid<'a> {
    grid: Cell<newtGrid>,
    added_to_parent: Cell<bool>,
    children: Vec<&'a dyn Component>
}

impl<'a> HorizontalGrid<'a> {
    ///
    /// Create a new Grid in which the added components are stacked in a
    /// single row.
    ///
    pub fn new<'t>(components: &'t [&'a dyn Component])
      -> HorizontalGrid<'a> {
        let mut types: Vec<newtGridElement> = Vec::new();
        let mut values: Vec<newtComponent> = Vec::new();

        let mut children = Vec::new();
        for component in components.iter() {
            types.push(component.grid_element_type());
            values.push(component.co());
            children.push(*component);
        }

        types.reverse();
        values.reverse();

        let func = newtGridHStacked as *const c_void;
        let len = components.len();
        let grid = asm::grid_new(func, types, values, len);
        HorizontalGrid {
            grid: Cell::new(grid),
            added_to_parent: Cell::new(false),
            children
        }
    }

    ///
    /// Create a new Grid in which the added components are closely
    /// stacked in a single row.
    ///
    pub fn new_close_stacked<'t>(components: &'t [&'a dyn Component])
      -> HorizontalGrid<'a> {
        let mut types: Vec<newtGridElement> = Vec::new();
        let mut values: Vec<newtComponent> = Vec::new();

        let mut children = Vec::new();
        for component in components.iter() {
            types.push(component.grid_element_type());
            values.push(component.co());
            children.push(*component);
        }

        types.reverse();
        values.reverse();

        let func = newtGridHCloseStacked as *const c_void;
        let len = components.len();
        let grid = asm::grid_new(func, types, values, len);
        HorizontalGrid {
            grid: Cell::new(grid),
            added_to_parent: Cell::new(false),
            children
        }
    }
}

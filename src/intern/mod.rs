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

#[macro_use]
pub mod macros;

pub mod data;
pub mod funcs;

use std::os::raw::c_void;
use newt_sys::*;

pub trait Child {
    fn add_to_parent(&self) -> Result<(), &'static str>;
    fn added_to_parent(&self) -> bool;
}

pub trait ComponentPtr {
    fn ptr(&self) -> *mut c_void;
    fn co_ptr(&self) -> newtComponent;
    fn grid_ptr(&self) -> newtGrid;
}

pub trait GridElementType {
    fn grid_element_type(&self) -> u32;
}

pub trait Nullify {
    fn nullify(&self);
}

#[cfg(feature = "asm")]
use crate::Component;
#[cfg(feature = "asm")]
pub trait Parent {
    fn children(&self) -> Vec<&dyn Component>;
}

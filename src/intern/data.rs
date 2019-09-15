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

use std::os::raw::c_void;
use std::ptr;

pub trait Data {
    fn newt_to_ptr(&self) -> *const c_void;
    fn newt_from_ptr(ptr: *const c_void) -> Self;
}

impl Data for () {
    fn newt_to_ptr(&self) -> *const c_void {
        ptr::null()
    }

    fn newt_from_ptr(_ptr: *const c_void) -> Self { }
}

impl Data for char {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as u8 as char
    }
}

impl Data for i8 {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as i8
    }
}

impl Data for i32 {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as i32
    }
}

impl Data for isize {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as isize
    }
}

impl Data for u8 {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as u8
    }
}

impl Data for u32 {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as u32
    }
}

impl Data for usize {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize
    }
}

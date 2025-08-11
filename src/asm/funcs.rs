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

#![cfg(feature = "asm")]
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

macro_rules! init_ptr_arrays {
    ($components:ident, $children:ident, $types:ident, $values:ident, $len:ident) => {
        let $len = $components.len();
        let mut $children = Vec::with_capacity($len);
        let mut types: Vec<newtGridElement> = Vec::with_capacity($len);
        let mut values: Vec<newtComponent> = Vec::with_capacity($len);

        for component in $components.iter() {
            types.push(component.grid_element_type());
            values.push(component.co());
            $children.push(*component);
        }

        let $types = types.as_ptr();
        let $values = values.as_ptr();
    }
}

pub fn str_slice_to_cstring_vec(slice: &[&str]) -> Vec<CString> {
    let mut vec = Vec::with_capacity(slice.len());
    for s in slice.iter() {
        vec.push(CString::new(*s).unwrap());
    }
    vec
}

pub fn cstring_vec_to_ptrs(strings: &[CString]) -> Vec<*const c_char> {
    let mut vec = Vec::with_capacity(strings.len() + 1);
    for s in strings.iter() {
        vec.push(s.as_ptr());
    }
    vec.push(ptr::null());
    vec
}

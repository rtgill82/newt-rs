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

pub fn str_slice_to_cstring_vec(slice: &[&str]) -> Vec<CString> {
    let mut vec = Vec::new();
    for s in slice.iter() {
        vec.push(CString::new(*s).unwrap());
    }
    vec
}

pub fn cstring_vec_to_ptrs(strings: &[CString]) -> Vec<*const c_char> {
    let mut vec = Vec::new();
    for s in strings.iter() {
        vec.push(s.as_ptr());
    }
    vec.push(ptr::null());
    vec
}

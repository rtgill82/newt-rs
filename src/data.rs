//
// Copyright (C) 2025 Robert Gill <rtgill82@gmail.com>
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

//!
//! Trait that allows for conveniently passing data as pointers to the
//! underlying C library.
//!
use std::os::raw::c_void;
use std::ptr;

///
/// Trait that allows for conveniently passing data as pointers to the
/// underlying C library.
///
/// Used with [`Listbox`][listbox] and [`CheckboxTree`][checkbox_tree]
/// for associating data with their list items.
///
/// [listbox]: crate::widgets::Listbox
/// [checkbox_tree]: crate::widgets::CheckboxTree
///
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
        if !self.is_ascii() {
            panic!("UTF-8 characters are not supported.");
        }

        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as u8 as Self
    }
}

impl Data for i8 {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as Self
    }
}

impl Data for u8 {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as Self
    }
}

impl Data for isize {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as Self
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

impl Data for i16 {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as Self
    }
}

impl Data for u16 {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as Self
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl Data for i32 {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as Self
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl Data for u32 {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as Self
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl Data for f32 {
    fn newt_to_ptr(&self) -> *const c_void {
        self.to_bits() as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        Self::from_bits(ptr as u32) as Self
    }
}

#[cfg(target_pointer_width = "64")]
impl Data for i64 {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as Self
    }
}

#[cfg(target_pointer_width = "64")]
impl Data for u64 {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as Self
    }
}

#[cfg(target_pointer_width = "64")]
impl Data for f64 {
    fn newt_to_ptr(&self) -> *const c_void {
        self.to_bits() as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        Self::from_bits(ptr as u64) as Self
    }
}

#[test]
fn char_data_should_accept_ascii() {
    let c = '0';
    let _ptr = c.newt_to_ptr();
}

#[test]
#[should_panic(expected = "UTF-8 characters are not supported.")]
fn char_data_should_not_accept_utf8() {
    let c = '\u{1F603}';
    let _ptr = c.newt_to_ptr();
}

#[test]
fn isize_data_should_cast_correctly() {
    let i: isize = isize::MIN / 3;
    let ptr = i.newt_to_ptr();
    let i = isize::newt_from_ptr(ptr);
    assert!(i == (isize::MIN / 3));
}

#[test]
fn usize_data_should_cast_correctly() {
    let i: usize = usize::MAX / 3;
    let ptr = i.newt_to_ptr();
    let i = usize::newt_from_ptr(ptr);
    assert!(i == (usize::MAX / 3));
}

#[test]
fn i8_data_should_cast_correctly() {
    let i: i8 = i8::MIN / 3;
    let ptr = i.newt_to_ptr();
    let i = i8::newt_from_ptr(ptr);
    assert!(i == (i8::MIN / 3));
}

#[test]
fn u8_data_should_cast_correctly() {
    let i: u8 = u8::MAX / 3;
    let ptr = i.newt_to_ptr();
    let i = u8::newt_from_ptr(ptr);
    assert!(i == (u8::MAX / 3));
}

#[test]
fn i16_data_should_cast_correctly() {
    let i: i16 = i16::MIN / 3;
    let ptr = i.newt_to_ptr();
    let i = i16::newt_from_ptr(ptr);
    assert!(i == (i16::MIN / 3));
}

#[test]
fn u16_data_should_cast_correctly() {
    let i: u16 = u16::MAX / 3;
    let ptr = i.newt_to_ptr();
    let i = u16::newt_from_ptr(ptr);
    assert!(i == (u16::MAX / 3));
}

#[test]
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
fn i32_data_should_cast_correctly() {
    let i: i32 = i32::MIN / 3;
    let ptr = i.newt_to_ptr();
    let i = i32::newt_from_ptr(ptr);
    assert!(i == (i32::MIN / 3));
}

#[test]
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
fn u32_data_should_cast_correctly() {
    let i: u32 = u32::MAX / 3;
    let ptr = i.newt_to_ptr();
    let i = u32::newt_from_ptr(ptr);
    assert!(i == (u32::MAX / 3));
}

#[test]
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
fn f32_data_should_cast_correctly() {
    let f: f32 = f32::MIN / 3.0;
    let ptr = f.newt_to_ptr();
    let f = f32::newt_from_ptr(ptr);
    assert!(f == (f32::MIN / 3.0));
}

#[test]
#[cfg(target_pointer_width = "64")]
fn i64_data_should_cast_correctly() {
    let i: i64 = i64::MIN / 3;
    let ptr = i.newt_to_ptr();
    let i = i64::newt_from_ptr(ptr);
    assert!(i == (i64::MIN / 3));
}

#[test]
#[cfg(target_pointer_width = "64")]
fn u64_data_should_cast_correctly() {
    let i: u64 = u64::MAX / 3;
    let ptr = i.newt_to_ptr();
    let i = u64::newt_from_ptr(ptr);
    assert!(i == (u64::MAX / 3));
}

#[test]
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
fn f64_data_should_cast_correctly() {
    let f: f64 = f64::MIN / 3.0;
    let ptr = f.newt_to_ptr();
    let f = f64::newt_from_ptr(ptr);
    assert!(f == (f64::MIN / 3.0));
}

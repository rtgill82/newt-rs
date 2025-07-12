//
// Copyright (C) 2019,2025 Robert Gill <rtgill82@gmail.com>
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

use std::convert::TryInto;
use std::error::Error;
use std::ffi::CString;
use std::os::raw::{c_char,c_int,c_void};
use std::{char,ptr};

use newt_sys::*;
use crate::component::Component;
use crate::widgets::{Entry,Form};

use crate::Callback;
use crate::callbacks::DestroyCallback;
use crate::callbacks::EntryFilter;
use crate::callbacks::HelpCallback;
use crate::callbacks::SuspendCallback;
use crate::private::Child;

use crate::private::data::Data;

//
// Panic! when memory allocation fails.
//
pub fn malloc_failure() -> ! {
    panic!("memory allocation failed");
}

//
// Convert an array of C pointers to a Boxed slice.
//
pub unsafe
fn c_ptr_array_to_boxed_slice<D>(ptr: *const *const c_void, numitems: i32)
    -> Box<[D]> where D: Data
{
    let mut vec: Vec<D> = Vec::new();
    if !ptr.is_null() && numitems > 0 {
        let mut count = 0;
        let mut p = ptr;
        while count < numitems {
            vec.push(D::newt_from_ptr(*p));
            p = p.add(std::mem::size_of::<c_void>());
            count += 1;
        }
    }
    vec.into_boxed_slice()
}

//
//  Convert a `char` to a C character.
//
pub fn char_to_c_char(ch: char) -> c_char {
    let result: Result<c_char, Box<dyn Error>> =
        match TryInto::<u8>::try_into(ch) {
            Ok(ch) => match TryInto::<c_char>::try_into(ch) {
                Ok(ch) => Ok(ch),
                Err(e) => Err(Box::new(e))
            },
            Err(e) => Err(Box::new(e))
        };

    match result {
        Ok(ch) => ch,
        Err(e) => panic!("cannot convert `char` {} to `c_char: {}`", ch, e)
    }
}

//
// Convert a character slice to a C string.
//
pub fn char_slice_to_cstring(slice: &[char]) -> CString {
    let mut vec: Vec<u8> = Vec::new();
    for ch in slice.iter() {
        vec.push(char_to_c_char(*ch) as u8);
    }

    let string = String::from_utf8_lossy(vec.as_slice());
    CString::new(string.into_owned()).unwrap()
}

//
// Call a Callback.
//
unsafe extern "C"
fn callback<'a, F: 'a, T: 'a>(co: newtComponent, data: *mut c_void)
where
    F: FnMut(&dyn Component, Option<&T>)
{
    let cb = &mut *(data as *mut Callback<'a, F, T>);
    cb.call(co);
}

//
// Call a DestroyCallback.
//
unsafe extern "C"
fn destroy_callback<'a, F: 'a, T: 'a>(co: newtComponent, data: *mut c_void)
where
    F: FnMut(&dyn Component, Option<&T>)
{
    let cb = &mut *(data as *mut DestroyCallback<'a, F, T>);
    cb.call(co);
    newt_unset_destroy_callback(co);
}

//
// Call a HelpCallback.
//
unsafe extern "C"
fn help_callback<F, T>(co: newtComponent, data: *mut c_void)
where
    F: FnMut(&Form, Option<&T>)
{
    if data.is_null() { return; };
    let cb = &mut *(data as *mut HelpCallback<F, T>);
    let form = Form::new_co(co);
    form.add_to_parent().unwrap();
    cb.call(&form);
}

//
// Call a SuspendCallback.
//
unsafe extern "C"
fn suspend_callback<F, T>(data: *mut c_void)
where
    F: FnMut(Option<&T>)
{
    let cb = &mut *(data as *mut SuspendCallback<F, T>);
    cb.call();
}

//
// Call an EntryFilter.
//
unsafe extern "C"
fn entry_filter<'a, F: 'a, T: 'a>(
    entry: newtComponent,
    data: *mut c_void,
    ch: c_int,
    cursor: c_int
) -> i32
where
    F: FnMut(&Entry, Option<&T>, char, i32) -> char
{
    let cb = &mut *(data as *mut EntryFilter<'a, F, T>);
    let ch = char::from_u32(ch as u32).unwrap();
    cb.call(entry, ch, cursor) as i32
}

//
// Set a Callback.
//
pub unsafe fn newt_set_callback<'a, F: 'a, T: 'a>(
    co: newtComponent,
    cb: &Callback<'a, F, T>
)
where
    F: FnMut(&dyn Component, Option<&T>)
{
    let c_ptr = cb as *const _ as *mut c_void;
    newtComponentAddCallback(co, Some(callback::<F, T>), c_ptr);
}

//
// Unset a Callback.
//
pub unsafe fn newt_unset_callback(co: &dyn Component)
{
    newtComponentAddCallback(co.co(), None, ptr::null_mut());
}

//
// Set a DestroyCallback.
//
pub unsafe fn newt_set_destroy_callback<'a, F: 'a, T: 'a>
    (co: newtComponent,
     cb: &DestroyCallback<'a, F, T>)
where
    F: FnMut(&dyn Component, Option<&T>)
{
    let c_ptr = cb as *const _ as *mut c_void;
    newtComponentAddDestroyCallback(co, Some(destroy_callback::<F, T>), c_ptr);
}

//
// Unset a DestroyCallback.
//
pub unsafe fn newt_unset_destroy_callback(co: newtComponent)
{
    newtComponentAddDestroyCallback(co, None, ptr::null_mut());
}

//
// Initialize the HelpCallback.
//
pub unsafe fn newt_init_help_callback<F, T>(_cb: &HelpCallback<F, T>)
where F: FnMut(&Form, Option<&T>)
{
    newtSetHelpCallback(Some(help_callback::<F, T>));
}

//
// Set a SuspendCallback.
//
pub unsafe fn newt_set_suspend_callback<F, T>(cb: &SuspendCallback<F, T>)
where
    F: FnMut(Option<&T>)
{
    let c_ptr = cb as *const _ as *mut c_void;
    newtSetSuspendCallback(Some(suspend_callback::<F, T>), c_ptr);
}

//
// Unset SuspendCallback.
//
pub unsafe fn newt_unset_suspend_callback()
{
    newtSetSuspendCallback(None, ptr::null_mut());
}

//
// Set an EntryFilter.
//
pub unsafe fn newt_entry_set_filter<'a, F: 'a, T: 'a>(
    co: newtComponent,
    cb: &EntryFilter<'a, F, T>
)
where
    F: FnMut(&Entry, Option<&T>, char, i32) -> char
{
    let c_ptr = cb as *const _ as *mut c_void;
    newtEntrySetFilter(co, Some(entry_filter::<F, T>), c_ptr)
}

use std::ffi::CString;
use std::os::raw::{c_int,c_void};
use std::{char,ptr};

use newt_sys::*;
use crate::components::Component;
use crate::components::Entry;
use crate::components::Form;
use crate::components::entry::EntryFilter;
use crate::Callback;

pub fn char_slice_to_cstring(slice: &[char]) -> CString {
    let mut vec: Vec<u8> = Vec::new();
    for c in slice.iter() {
        let mut b = [0; 1];
        let ch = c.encode_utf8(&mut b);
        vec.push(ch.as_bytes()[0]);
    }

    let string = String::from_utf8_lossy(vec.as_slice());
    let cstr = CString::new(string.into_owned()).unwrap();
    return cstr;
}

unsafe extern "C"
fn callback<'a, FN: 'a, T: 'a>(co: newtComponent, data: *mut c_void)
where FN: FnMut(Option<&Component>, Option<&T>)
{
    let cb = &mut *(data as *mut Callback<'a, FN, T>);
    cb.call(co, None);
}

unsafe extern "C"
fn help_callback<'a, FN: 'a, T: 'a>(co: newtComponent, data: *mut c_void)
where FN: FnMut(Option<&Component>, Option<&T>)
{
    if data == ptr::null_mut() { return; };
    let cb = &mut *(data as *mut Callback<'a, FN, T>);
    cb.assert_help_callback();
    let mut form = Form::new_co(co);
    form.attach_to_form();
    cb.call(co, Some(&form));
}

unsafe extern "C"
fn suspend_callback<'a, FN: 'a, T: 'a>(data: *mut c_void)
where FN: FnMut(Option<&Component>, Option<&T>)
{
    let cb = &mut *(data as *mut Callback<'a, FN, T>);
    cb.call(ptr::null_mut(), None);
}

unsafe extern "C"
fn entry_filter<'a, FN: 'a, T: 'a>
  (entry: newtComponent, data: *mut c_void, ch: c_int, cursor: c_int) -> i32
where FN: FnMut(&Entry, Option<&T>, char, i32) -> char
{
    let cb = &mut *(data as *mut EntryFilter<'a, FN, T>);
    let ch = char::from_u32(ch as u32).unwrap();
    cb.call(entry, ch, cursor) as i32
}

pub fn newt_set_callback<'a, FN: 'a, T: 'a>(co: newtComponent,
                                            cb: &Callback<'a, FN, T>)
where FN: FnMut(Option<&Component>, Option<&T>)
{
    unsafe {
        let c_ptr = cb as *const _ as *mut c_void;
        newtComponentAddCallback(co, Some(callback::<FN, T>), c_ptr);
    }
}

pub fn newt_unset_callback(co: &Component)
{
    unsafe {
        newtComponentAddCallback(co.co(), None, ptr::null_mut());
    }
}

pub fn newt_init_help_callback<'a, FN: 'a, T: 'a>(_cb: &Callback<'a, FN, T>)
where FN: FnMut(Option<&Component>, Option<&T>)
{
    unsafe { newtSetHelpCallback(Some(help_callback::<FN, T>)); }
}

pub fn newt_set_suspend_callback<'a, FN: 'a, T: 'a>(cb: &Callback<'a, FN, T>)
where FN: FnMut(Option<&Component>, Option<&T>)
{
    unsafe {
        let c_ptr = cb as *const _ as *mut c_void;
        newtSetSuspendCallback(Some(suspend_callback::<FN, T>), c_ptr);
    }
}

pub fn newt_unset_suspend_callback()
{
    unsafe { newtSetSuspendCallback(None, ptr::null_mut()); }
}

pub fn newt_entry_set_filter<'a, FN: 'a, T: 'a>(co: newtComponent,
                                                cb: &EntryFilter<'a, FN, T>)
where FN: FnMut(&Entry, Option<&T>, char, i32) -> char
{
    unsafe {
        let c_ptr = cb as *const _ as *mut c_void;
        newtEntrySetFilter(co, Some(entry_filter::<FN, T>), c_ptr)
    }
}

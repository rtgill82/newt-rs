use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

use newt_sys::*;
use crate::components::Component;
use crate::components::Form;
use crate::Callback;

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

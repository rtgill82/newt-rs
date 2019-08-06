use std::cell::Cell;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw::{c_char, c_void};
use std::ptr;

use newt_sys::*;
use crate::constants::FlagsSense;
use crate::intern::data::Data;

///
/// A widget for displaying a list of selectable items.
///
#[derive(Component)]
pub struct Listbox<D: Data = isize> {
    co: newtComponent,
    added_to_parent: Cell<bool>,
    data: PhantomData<D>
}

impl<D: Data> Listbox<D> {
    pub fn new(left: i32, top: i32, height: i32, flags: i32) -> Listbox<D> {
        Listbox {
            co: unsafe { newtListbox(left, top, height, flags) },
            added_to_parent: Cell::new(false),
            data: PhantomData
        }
    }

    pub fn set_width(&mut self, width: i32) {
        unsafe { newtListboxSetWidth(self.co, width); }
    }

    pub fn item_count(&self) -> i32 {
        unsafe { newtListboxItemCount(self.co) }
    }

    pub fn append_entry(&mut self, text: &str, data: D) -> Result<(), ()> {
        let c_str = CString::new(text).unwrap();
        let rv = unsafe {
            newtListboxAppendEntry(self.co, c_str.as_ptr(), data.newt_to_ptr())
        };
        if rv == 0 { Ok(()) } else { Err(()) }
    }

    pub fn insert_entry(&mut self, text: &str, data: D, key: D)
          -> Result<(), ()> {
        let c_str = CString::new(text).unwrap();
        let rv = unsafe {
            newtListboxInsertEntry(self.co, c_str.as_ptr(), data.newt_to_ptr(),
                                   key.newt_to_ptr() as *mut c_void)
        };
        if rv == 0 { Ok(()) } else { Err(()) }
    }

    pub fn get_current(&self) -> Option<D> {
        let c_data = unsafe { newtListboxGetCurrent(self.co) };
        if c_data.is_null() { return None; }
        Some(D::newt_from_ptr(c_data))
    }

    pub fn set_current(&mut self, num: i32) {
        unsafe { newtListboxSetCurrent(self.co, num); }
    }

    pub fn set_current_by_key(&mut self, key: D) {
        unsafe {
            newtListboxSetCurrentByKey(self.co, key.newt_to_ptr() as *mut c_void);
        }
    }

    pub fn get_entry(&self, num: i32) -> (&str, D) {
        let mut c_str: *mut c_char = ptr::null_mut();
        let mut c_data: *mut c_void = ptr::null_mut();
        unsafe { newtListboxGetEntry(self.co, num, &mut c_str, &mut c_data); }
        let c_str = unsafe { CStr::from_ptr(c_str) };
        (c_str.to_str().unwrap(), D::newt_from_ptr(c_data))
    }

    pub fn set_entry(&mut self, num: i32, text: &str) {
        let c_str = CString::new(text).unwrap();
        unsafe { newtListboxSetEntry(self.co, num, c_str.as_ptr()); }
    }

    pub fn set_data(&mut self, num: i32, data: D) {
        unsafe {
            newtListboxSetData(self.co, num, data.newt_to_ptr() as *mut c_void);
        }
    }

    pub fn delete_entry(&mut self, data: D) -> i32 {
        unsafe {
            newtListboxDeleteEntry(self.co, data.newt_to_ptr() as *mut c_void)
        }
    }

    pub fn clear(&mut self) {
        unsafe { newtListboxClear(self.co); }
    }

    pub fn get_selection(&self) -> Box<[D]> {
        let mut numitems: i32 = 0;
        let ptr = unsafe { newtListboxGetSelection(self.co, &mut numitems) };
        c_ptr_array_to_boxed_slice!(ptr[D], numitems)
    }

    pub fn select_item(&mut self, key: D, sense: FlagsSense) {
        unsafe { newtListboxSelectItem(self.co, key.newt_to_ptr(), sense as u32) };
    }

    pub fn clear_selection(&mut self) {
        unsafe { newtListboxClearSelection(self.co) };
    }
}

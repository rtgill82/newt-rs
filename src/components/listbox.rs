extern crate std;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw::{c_char, c_void};
use ptr;

use components::c_component;
use components::Component;
use constants::FlagsSense;
use intern::ffi::newt::listbox::*;
use intern::ffi::newt::component::newtComponentDestroy;

newt_component!(Listbox<D>);
pub struct Listbox<D> {
    co: c_component,
    attached_to_form: bool,
    data: PhantomData<D>
}

impl<D> Listbox<D> {
    pub fn new(left: i32, top: i32, height: i32, flags: i32) -> Listbox<D> {
        Listbox {
            attached_to_form: false,
            data: PhantomData,
            co: unsafe {
                newtListbox(left, top, height, flags)
            }
        }
    }

    pub fn set_width(&self, width: i32) {
        unsafe { newtListboxSetWidth(self.co, width); }
    }

    pub fn item_count(&self) -> i32 {
        unsafe { newtListboxItemCount(self.co) }
    }

    pub fn append_entry(&self, text: &str, data: &D) -> i32 {
        let c_str = CString::new(text).unwrap();
        let c_data: *const c_void = data as *const _ as *const c_void;
        unsafe { newtListboxAppendEntry(self.co, c_str.as_ptr(), c_data) }
    }

    pub fn insert_entry(&self, text: &str, data: &D, key: &D) -> i32 {
        let c_str = CString::new(text).unwrap();
        let c_data: *const c_void = data as *const _ as *const c_void;
        let c_key: *const c_void = key as *const _ as *const c_void;
        unsafe {
            newtListboxInsertEntry(self.co, c_str.as_ptr(), c_data, c_key)
        }
    }

    pub fn get_current(&self) -> Option<&D> {
        let c_data = unsafe { newtListboxGetCurrent(self.co) };
        if c_data == ptr::null() { return None; }
        Some(unsafe { &*(c_data as *const D) })
    }

    pub fn set_current(&self, num: i32) {
        unsafe { newtListboxSetCurrent(self.co, num); }
    }

    pub fn set_current_by_key(&self, key: &D) {
        let c_key: *const c_void = key as *const _ as *const c_void;
        unsafe { newtListboxSetCurrentByKey(self.co, c_key); }
    }

    pub fn get_entry(&self, num: i32) -> (&str, &D) {
        let c_str: *mut c_char = ptr::null_mut();
        let c_data: *mut c_void = ptr::null_mut();
        unsafe { newtListboxGetEntry(self.co, num, &c_str, &c_data); }
        let c_str = unsafe { CStr::from_ptr(c_str) };
        let c_data = unsafe { &*(c_data as *const D) };
        (c_str.to_str().unwrap(), c_data)
    }

    pub fn set_entry(&self, num: i32, text: &str) {
        let c_str = CString::new(text).unwrap();
        unsafe { newtListboxSetEntry(self.co, num, c_str.as_ptr()); }
    }

    pub fn set_data(&self, num: i32, data: &D) {
        let c_data: *const c_void = data as *const _ as *const c_void;
        unsafe { newtListboxSetData(self.co, num, c_data); }
    }

    pub fn delete_entry(&self, data: &D) -> i32 {
        let c_data: *const c_void = data as *const _ as *const c_void;
        unsafe { newtListboxDeleteEntry(self.co, c_data) }
    }

    pub fn clear(&self) {
        unsafe { newtListboxClear(self.co); }
    }

    pub fn get_selection(&self) -> Box<[&D]> {
        let mut numitems: i32 = 0;
        let ptr: *const c_void = unsafe {
            newtListboxGetSelection(self.co, &mut numitems)
        };
        c_ptr_array_to_boxed_slice!(ptr[D], numitems)
    }

    pub fn select_item(&self, key: &D, sense: FlagsSense) {
        let c_key: *const c_void = key as *const _ as *const c_void;
        unsafe { newtListboxSelectItem(self.co, c_key, sense) };
    }

    pub fn clear_selection(&self) {
        unsafe { newtListboxClearSelection(self.co) };
    }
}

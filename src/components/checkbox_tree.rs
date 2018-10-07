extern crate std;
extern crate libc;

use self::libc::free;
use std::ffi::CString;
use std::marker::PhantomData;
use std::os::raw::{c_char, c_void};
use ptr;

use components::c_component;
use components::Component;
use components::data::Data;
use components::form::ExitReason;
use intern::ffi::newt::checkbox_tree::*;
use intern::ffi::newt::component::newtComponentDestroy;
use intern::funcs::char_slice_to_cstring;
use constants;

pub struct CheckboxTree<D: Data> {
    co: c_component,
    attached_to_form: bool,
    data: PhantomData<D>
}

impl<D: Data> CheckboxTree<D> {
    pub fn new(left: i32, top: i32, height: i32, seq: Option<&[char]>, flags: i32)
            -> CheckboxTree<D> {
        let component: c_component = match seq {
            Some(seq) => {
                let c_seq = char_slice_to_cstring(&seq);
                unsafe {
                    newtCheckboxTreeMulti(left, top, height,
                                          c_seq.as_ptr(), flags)
                }
            },

            None => unsafe { newtCheckboxTree(left, top, height, flags) }
        };

        CheckboxTree {
            attached_to_form: false,
            data: PhantomData,
            co: component
        }
    }

    pub fn set_width(&mut self, width: i32) {
        unsafe { newtCheckboxTreeSetWidth(self.co, width); }
    }

    pub fn add_item(&mut self, text: &str, data: D, flags: i32,
                    indexes: &[i32]) -> i32 {
        let mut i = 0;
        let mut c_array: Vec<i32> = Vec::with_capacity(indexes.len() + 1);
        while i < indexes.len() {
            c_array.push(indexes[i]);
            i = i + 1;
        }
        c_array.push(constants::ARG_LAST);

        let c_str = CString::new(text).unwrap();
        unsafe {
            newtCheckboxTreeAddArray(self.co, c_str.as_ptr(),
                                     data.newt_to_ptr(), flags,
                                     c_array.as_ptr())
        }
    }

    pub fn get_current(&self) -> Option<D> {
        let c_data = unsafe { newtCheckboxTreeGetCurrent(self.co) };
        if c_data == ptr::null() { return None; }
        Some(D::newt_from_ptr(c_data))
    }

    pub fn set_current(&mut self, data: D) {
        unsafe { newtCheckboxTreeSetCurrent(self.co, data.newt_to_ptr()); }
    }

    pub fn get_selection(&self) -> Box<[D]> {
        let mut numitems: i32 = 0;
        let ptr = unsafe {
            newtCheckboxTreeGetSelection(self.co, &mut numitems)
        };
        c_ptr_array_to_boxed_slice!(ptr[D], numitems)
    }

    pub fn get_multi_selection(&self, seqval: char) -> Box<[D]> {
        let mut numitems: i32 = 0;
        let ptr = unsafe {
            newtCheckboxTreeGetMultiSelection(self.co, &mut numitems,
                                              seqval as i8)
        };
        c_ptr_array_to_boxed_slice!(ptr[D], numitems)
    }

    pub fn find_item(&self, data: D) -> Box<[i32]> {
        let mut vec: Vec<i32> = Vec::new();
        unsafe {
            let rv = newtCheckboxTreeFindItem(self.co, data.newt_to_ptr());
            let mut p = rv;
            let mut value: i32 = *p as i32;
            while value != constants::ARG_LAST {
                vec.push(value);
                p = p.offset(1);
                value = *p as i32;
            }
            free(rv as *mut libc::c_void);
        }
        vec.into_boxed_slice()
    }

    pub fn set_entry(&mut self, data: D, text: &str) {
        let c_str = CString::new(text).unwrap();
        unsafe {
            newtCheckboxTreeSetEntry(self.co, data.newt_to_ptr(),
                                     c_str.as_ptr());
        }
    }

    pub fn get_entry_value(&self, data: D) -> char {
        unsafe {
            newtCheckboxTreeGetEntryValue(self.co, data.newt_to_ptr())
                as u8 as char
        }
    }

    pub fn set_entry_value(&mut self, data: D, value: char) {
        unsafe {
            newtCheckboxTreeSetEntryValue(self.co, data.newt_to_ptr(),
                                          value as c_char);
        }
    }
}

impl<D: Data> Component for CheckboxTree<D> {
    fn co(&self) -> c_component {
        self.co
    }

    fn attach_to_form(&mut self) {
        self.attached_to_form = true;
    }

    fn attached_to_form(&self) -> bool {
        self.attached_to_form
    }
}

impl<D: Data> Drop for CheckboxTree<D> {
    fn drop(&mut self) {
        if !self.attached_to_form() {
            unsafe { newtComponentDestroy(self.co()); }
        }
    }
}

impl<D: Data, Rhs: Component> PartialEq<Rhs> for CheckboxTree<D> {
    fn eq(&self, other: &Rhs) -> bool {
        self.co == other.co()
    }
}

impl<D: Data> PartialEq<Box<dyn (Component)>> for CheckboxTree<D> {
    fn eq(&self, other: &Box<dyn (Component)>) -> bool {
        self.co == other.co()
    }
}

impl<D: Data> PartialEq<ExitReason> for CheckboxTree<D> {
    fn eq(&self, other: &ExitReason) -> bool {
        other == self
    }
}

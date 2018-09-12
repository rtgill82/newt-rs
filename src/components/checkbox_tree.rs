extern crate std;
extern crate libc;

use self::libc::free;
use std::ffi::CString;
use std::marker::PhantomData;
use std::os::raw::{c_char, c_void};
use std::mem;

use components::c_component;
use components::Component;
use components::form::ExitReason;
use intern::ffi::newt::checkbox_tree::*;
use intern::ffi::newt::component::newtComponentDestroy;
use intern::funcs::char_slice_to_cstring;
use constants;

newt_component!(CheckboxTree<D>);
pub struct CheckboxTree<D> {
    co: c_component,
    attached_to_form: bool,
    data: PhantomData<D>
}

impl<D> CheckboxTree<D> {
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

    pub fn add_item(&mut self, text: &str, data: &D, flags: i32,
                    indexes: &[i32]) -> i32 {
        let c_str = CString::new(text).unwrap();
        let c_data: *const c_void = data as *const _ as *const c_void;

        let mut i = 0;
        let mut c_array: Vec<i32> = Vec::with_capacity(indexes.len() + 1);
        while i < indexes.len() {
            c_array.push(indexes[i]);
            i = i + 1;
        }
        c_array.push(constants::ARG_LAST);

        unsafe {
            newtCheckboxTreeAddArray(self.co, c_str.as_ptr(), c_data, flags,
                                     c_array.as_ptr())
        }
    }

    pub fn get_current(&self) -> &D {
        let c_data = unsafe { newtCheckboxTreeGetCurrent(self.co) };
        unsafe { &*(c_data as *const D) }
    }

    pub fn set_current(&mut self, data: &D) {
        let c_data: *const c_void = data as *const _ as *const c_void;
        unsafe { newtCheckboxTreeSetCurrent(self.co, c_data); }
    }

    pub fn get_selection(&self) -> Box<[&D]> {
        let mut numitems: i32 = 0;
        let ptr: *const c_void = unsafe {
            newtCheckboxTreeGetSelection(self.co, &mut numitems)
        };
        c_ptr_array_to_boxed_slice!(ptr[D], numitems)
    }

    pub fn get_multi_selection(&self, seqval: char) -> Box<[&D]> {
        let mut numitems: i32 = 0;
        let ptr: *const c_void = unsafe {
            newtCheckboxTreeGetMultiSelection(self.co, &mut numitems,
                                              seqval as i8)
        };
        c_ptr_array_to_boxed_slice!(ptr[D], numitems)
    }

    pub fn find_item(&self, data: &D) -> Box<[i32]> {
        let mut vec: Vec<i32> = Vec::new();
        let c_data: *const c_void = data as *const _ as *const c_void;
        unsafe {
            let rv = newtCheckboxTreeFindItem(self.co, c_data);
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

    pub fn set_entry(&mut self, data: &D, text: &str) {
        let c_data: *const c_void = data as *const _ as *const c_void;
        let c_str = CString::new(text).unwrap();
        unsafe { newtCheckboxTreeSetEntry(self.co, c_data, c_str.as_ptr()); }
    }

    pub fn get_entry_value(&self, data: &D) -> char {
        let c_data: *const c_void = data as *const _ as *const c_void;
        unsafe {
            newtCheckboxTreeGetEntryValue(self.co, c_data) as u8  as char
        }
    }

    pub fn set_entry_value(&mut self, data: &D, value: char) {
        let c_data: *const c_void = data as *const _ as *const c_void;
        unsafe {
            newtCheckboxTreeSetEntryValue(self.co, c_data, value as c_char);
        }
    }
}

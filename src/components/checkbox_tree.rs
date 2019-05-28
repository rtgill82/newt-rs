extern crate std;
extern crate libc;
extern crate newt_sys;

use self::libc::free;
use std::ffi::CString;
use std::marker::PhantomData;
use std::os::raw::{c_char, c_void};

use newt_sys::*;
use crate::intern::data::Data;
use crate::intern::funcs::char_slice_to_cstring;
use crate::constants;

#[derive(Component)]
pub struct CheckboxTree<D: Data = isize> {
    co: newtComponent,
    added_to_parent: bool,
    data: PhantomData<D>
}

impl<D: Data> CheckboxTree<D> {
    pub fn new(left: i32, top: i32, height: i32, seq: Option<&[char]>, flags: i32)
            -> CheckboxTree<D> {
        let component: newtComponent = match seq {
            Some(seq) => {
                let c_seq = char_slice_to_cstring(&seq);
                unsafe {
                    newtCheckboxTreeMulti(left, top, height,
                                          c_seq.as_ptr() as *mut i8,
                                          flags)
                }
            },

            None => unsafe { newtCheckboxTree(left, top, height, flags) }
        };

        CheckboxTree {
            co: component,
            added_to_parent: false,
            data: PhantomData
        }
    }

    pub fn set_width(&mut self, width: i32) {
        unsafe { newtCheckboxTreeSetWidth(self.co, width); }
    }

    pub fn add_item(&mut self, text: &str, data: D, flags: i32,
                    indexes: Option<&[i32]>) -> i32 {
        let mut i = 0;
        let mut c_array: Vec<i32>;
        if let Some(indexes) = indexes {
            c_array = Vec::with_capacity(indexes.len() + 1);
            while i < indexes.len() {
                c_array.push(indexes[i]);
                i += 1;
            }
            c_array.push(constants::ARG_LAST);
        } else {
            c_array = Vec::with_capacity(2);
            c_array.push(constants::ARG_APPEND);
            c_array.push(constants::ARG_LAST);
        }

        let c_str = CString::new(text).unwrap();
        unsafe {
            newtCheckboxTreeAddArray(self.co, c_str.as_ptr(),
                                     data.newt_to_ptr(), flags,
                                     c_array.as_ptr() as *mut i32)
        }
    }

    pub fn get_current(&self) -> Option<D> {
        let c_data = unsafe { newtCheckboxTreeGetCurrent(self.co) };
        if c_data.is_null() { return None; }
        Some(D::newt_from_ptr(c_data))
    }

    pub fn set_current(&mut self, data: D) {
        unsafe {
            newtCheckboxTreeSetCurrent(self.co, data.newt_to_ptr() as *mut c_void);
        }
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
            let rv = newtCheckboxTreeFindItem(self.co, data.newt_to_ptr() as *mut c_void);
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

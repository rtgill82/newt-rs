extern crate std;
extern crate newt_sys;
use std::ffi::CString;
use crate::ptr;

use newt_sys::*;
use crate::constants::FlagsSense;
use crate::intern::funcs::char_slice_to_cstring;

#[derive(Component)]
pub struct Checkbox {
    co: newtComponent,
    added_to_parent: bool
}

impl Checkbox {
    pub fn new(left: i32, top: i32, text: &str, def_value: Option<char>,
               seq: Option<&[char]>)
            -> Checkbox {
        let c_text = CString::new(text).unwrap();
        let default: i8 = match def_value {
            Some(value) => value as i8,
            None => 0 as i8
        };

        let cstr: CString;
        let c_seq = match seq {
            Some(seq) => {
                cstr = char_slice_to_cstring(&seq);
                cstr.as_ptr()
            },
            None => ptr::null()
        };

        Checkbox {
            co: unsafe {
                newtCheckbox(left, top, c_text.as_ptr(), default, c_seq,
                             ptr::null_mut())
            },
            added_to_parent: false
        }
    }

    pub fn get_value(&self) -> char {
        unsafe { newtCheckboxGetValue(self.co) as u8 as char }
    }

    pub fn set_value(&mut self, value: char) {
        unsafe { newtCheckboxSetValue(self.co, value as i8); }
    }

    pub fn set_flags(&mut self, flags: i32, sense: FlagsSense) {
        unsafe { newtCheckboxSetFlags(self.co, flags, sense as u32); }
    }
}

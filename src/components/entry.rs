extern crate std;
use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use ptr;


use components::c_component;
use components::Component;
use constants::FlagsSense;
use intern::ffi::newt::entry::*;

newt_component!(Entry);
pub struct Entry {
    co: c_component,
    attached_to_form: bool
}

impl Entry  {
    pub fn new(left: i32, top: i32, initial_value: Option<&str>, width: i32,
               flags: i32) -> Entry {
        let c_str: CString;
        let ptr = match initial_value {
            Some(text) => {
                c_str = CString::new(text).unwrap();
                c_str.as_ptr()
            },
            None => ptr::null()
        };

        Entry {
            co: unsafe {
                newtEntry(left, top, ptr, width, ptr::null(), flags)
            },
            attached_to_form: false
        }
    }

    pub fn get_text(&self) -> String {
        unsafe { CStr::from_ptr(newtEntryGetValue(self.co)) }
            .to_string_lossy()
            .into_owned()
    }

    pub fn set_text(&mut self, text: &str, cursor_at_end: bool) {
        let c_str = CString::new(text).unwrap();
        unsafe {
            newtEntrySet(self.co, c_str.as_ptr(), cursor_at_end as c_int);
        }
    }

    pub fn set_flags(&mut self, flags: i32, sense: FlagsSense) {
        unsafe { newtEntrySetFlags(self.co, flags, sense); }
    }

    pub fn set_colors(&mut self, normal: i32, disabled: i32) {
        unsafe { newtEntrySetColors(self.co, normal, disabled); }
    }
}

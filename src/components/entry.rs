extern crate std;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use ptr;

use FlagsSense;

use components::c_component;
use components::Component;
use components::form::ExitReason;

newt_component!(Entry);
pub struct Entry {
    co: c_component,
    attached_to_form: bool
}

impl Entry  {
    pub fn new(left: i32, top: i32, initial_value: Option<&str>, width: i32,
               flags: i32) -> Entry {
        #[link(name="newt")]
        extern "C" {
            fn newtEntry(left: c_int, top: c_int, initialValue: *const c_char,
                         width: c_int, resultPtr: *const c_char, flags: c_int)
                -> c_component;
        }

        let c_str: CString;
        let ptr = match initial_value {
            Some(text) => {
                c_str = CString::new(text).unwrap();
                c_str.as_ptr()
            },
            None => ptr::null()
        };

        Entry {
            attached_to_form: false,
            co: unsafe {
                newtEntry(left, top, ptr, width, ptr::null(), flags)
            }
        }
    }

    pub fn get_text(&self) -> String {
        #[link(name="newt")]
        extern "C" {
            fn newtEntryGetValue(co: c_component) -> *mut c_char;
        }

        unsafe { CStr::from_ptr(newtEntryGetValue(self.co)) }
            .to_string_lossy()
            .into_owned()
    }

    pub fn set_text(&mut self, text: &str, cursor_at_end: bool) {
        #[link(name="newt")]
        extern "C" {
            fn newtEntrySet(co: c_component, value: *const c_char,
                            cursorAtEnd: c_int);
        }

        let c_str = CString::new(text).unwrap();
        unsafe {
            newtEntrySet(self.co, c_str.as_ptr(), cursor_at_end as c_int);
        }
    }

    pub fn set_flags(&mut self, flags: i32, sense: FlagsSense) {
        #[link(name="newt")]
        extern "C" {
            fn newtEntrySetFlags(co: c_component, flags: c_int,
                                 sense: FlagsSense);
        }

        unsafe { newtEntrySetFlags(self.co, flags, sense); }
    }

    pub fn set_colors(&mut self, normal: i32, disabled: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtEntrySetColors(co: c_component, normal: c_int,
                                  disabled: c_int);
        }

        unsafe { newtEntrySetColors(self.co, normal, disabled); }
    }
}

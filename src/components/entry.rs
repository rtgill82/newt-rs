extern crate std;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;
use ptr;

use components::NewtComponent;
use components::NewtComponentPtr;
use FlagsSense;

newt_component!(Entry);
pub struct Entry {
    co: NewtComponentPtr
}

impl Entry  {
    pub fn new(left: i32, top: i32, initial_value: Option<&str>, width: i32,
               flags: i32) -> Entry {
        #[link(name="newt")]
        extern "C" {
            fn newtEntry(left: c_int, top: c_int, initialValue: *const c_char,
                         width: c_int, resultPtr: *const c_char, flags: c_int)
                -> NewtComponentPtr;
        }

        let ptr = match initial_value {
            Some(text) => CString::new(text).unwrap().as_ptr(),
            None => ptr::null()
        };

        Entry {
            co: unsafe {
                newtEntry(left, top, ptr, width, ptr::null(), flags)
            }
        }
    }

    pub fn set_text(&self, text: &str, cursor_at_end: bool) {
        #[link(name="newt")]
        extern "C" {
            fn newtEntrySet(co: NewtComponentPtr, value: *const c_char,
                            cursorAtEnd: c_int);
        }

        let c_str = CString::new(text).unwrap();
        unsafe {
            newtEntrySet(self.co, c_str.as_ptr(), cursor_at_end as c_int);
        }
    }

    pub fn get_value(&self) -> String {
        #[link(name="newt")]
        extern "C" {
            fn newtEntryGetValue(co: NewtComponentPtr) -> *mut c_char;
        }

        unsafe { CStr::from_ptr(newtEntryGetValue(self.co)) }
            .to_string_lossy()
            .into_owned()
    }

    pub fn set_flags(&self, flags: i32, sense: FlagsSense) {
        #[link(name="newt")]
        extern "C" {
            fn newtEntrySetFlags(co: NewtComponentPtr, flags: c_int,
                                 sense: FlagsSense);
        }

        unsafe { newtEntrySetFlags(self.co, flags, sense); }
    }

    pub fn set_colors(&self, normal: i32, disabled: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtEntrySetColors(co: NewtComponentPtr, normal: c_int,
                                  disabled: c_int);
        }

        unsafe { newtEntrySetColors(self.co, normal, disabled); }
    }
}

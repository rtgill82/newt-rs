extern crate std;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;
use ptr;

use FlagsSense;
use components::Component;
use components::NewtComponentPtr;

newt_component!(Checkbox);
pub struct Checkbox {
    co: NewtComponentPtr
}

impl Checkbox {
    pub fn new(left: i32, top: i32, text: &str, def_value: i8, seq: &[u8])
            -> Checkbox {
        #[link(name="newt")]
        extern "C" {
            fn newtCheckbox(left: c_int, top: c_int, text: *const c_char,
                            defValue: c_char, seq: *const c_char,
                            result: *mut c_char) -> NewtComponentPtr;
        }
        let c_text = CString::new(text).unwrap();
        let s_seq = String::from_utf8_lossy(seq);
        let c_seq = CString::new(s_seq.into_owned()).unwrap();
        Checkbox {
            co: unsafe {
                newtCheckbox(left, top, c_text.as_ptr(), def_value,
                             c_seq.as_ptr(), ptr::null_mut())
            }
        }
    }

    pub fn get_value(&self) -> i8 {
        #[link(name="newt")]
        extern "C" { fn newtCheckboxGetValue(co: NewtComponentPtr) -> c_char; }
        unsafe { newtCheckboxGetValue(self.co) }
    }

    pub fn set_value(&self, value: i8) {
        #[link(name="newt")]
        extern "C" {
            fn newtCheckboxSetValue(co: NewtComponentPtr, value: c_char);
        }
        unsafe { newtCheckboxSetValue(self.co, value); }
    }

    pub fn set_flags(&self, flags: i32, sense: FlagsSense) {
        #[link(name="newt")]
        extern "C" {
            fn newtCheckboxSetFlags(co: NewtComponentPtr, flags: c_int,
                                    sense: FlagsSense);
        }
        unsafe { newtCheckboxSetFlags(self.co, flags, sense); }
    }
}

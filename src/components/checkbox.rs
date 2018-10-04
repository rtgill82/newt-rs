extern crate std;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use ptr;

use FlagsSense;

use components::c_component;
use components::Component;
use components::form::ExitReason;
use intern::funcs::char_slice_to_cstring;

newt_component!(Checkbox);
pub struct Checkbox {
    co: c_component,
    attached_to_form: bool
}

impl Checkbox {
    pub fn new(left: i32, top: i32, text: &str, def_value: Option<char>,
               seq: Option<&[char]>)
            -> Checkbox {
        #[link(name="newt")]
        extern "C" {
            fn newtCheckbox(left: c_int, top: c_int, text: *const c_char,
                            defValue: c_char, seq: *const c_char,
                            result: *mut c_char) -> c_component;
        }

        let c_text = CString::new(text).unwrap();
        let default: i8 = match def_value {
            Some(value) => value as i8,
            None => 0 as i8
        };
        let c_seq = match seq {
            Some(seq) => char_slice_to_cstring(&seq).as_ptr(),
            None => ptr::null()
        };

        Checkbox {
            attached_to_form: false,
            co: unsafe {
                newtCheckbox(left, top, c_text.as_ptr(), default, c_seq,
                             ptr::null_mut())
            }
        }
    }

    pub fn get_value(&self) -> char {
        #[link(name="newt")]
        extern "C" { fn newtCheckboxGetValue(co: c_component) -> c_char; }

        unsafe { newtCheckboxGetValue(self.co) as u8 as char }
    }

    pub fn set_value(&mut self, value: char) {
        #[link(name="newt")]
        extern "C" {
            fn newtCheckboxSetValue(co: c_component, value: c_char);
        }

        unsafe { newtCheckboxSetValue(self.co, value as i8); }
    }

    pub fn set_flags(&mut self, flags: i32, sense: FlagsSense) {
        #[link(name="newt")]
        extern "C" {
            fn newtCheckboxSetFlags(co: c_component, flags: c_int,
                                    sense: FlagsSense);
        }

        unsafe { newtCheckboxSetFlags(self.co, flags, sense); }
    }
}

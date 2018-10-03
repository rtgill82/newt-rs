extern crate std;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use ptr;

use FlagsSense;

use components::c_component;
use components::Component;
use components::form::ExitReason;

newt_component!(Checkbox);
pub struct Checkbox {
    co: c_component,
    attached_to_form: bool
}

impl Checkbox {
    pub fn new(left: i32, top: i32, text: &str, def_value: u8, seq: &[u8])
            -> Checkbox {
        #[link(name="newt")]
        extern "C" {
            fn newtCheckbox(left: c_int, top: c_int, text: *const c_char,
                            defValue: c_char, seq: *const c_char,
                            result: *mut c_char) -> c_component;
        }

        let c_text = CString::new(text).unwrap();
        let s_seq = String::from_utf8_lossy(seq);
        let c_seq = CString::new(s_seq.into_owned()).unwrap();
        Checkbox {
            attached_to_form: false,
            co: unsafe {
                newtCheckbox(left, top, c_text.as_ptr(), def_value as i8,
                             c_seq.as_ptr(), ptr::null_mut())
            }
        }
    }

    pub fn get_value(&self) -> u8 {
        #[link(name="newt")]
        extern "C" { fn newtCheckboxGetValue(co: c_component) -> c_char; }

        unsafe { newtCheckboxGetValue(self.co) as u8 }
    }

    pub fn set_value(&mut self, value: u8) {
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

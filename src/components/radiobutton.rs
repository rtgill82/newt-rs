extern crate std;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use ptr;

use components::c_component;
use components::Component;
use components::form::ExitReason;

newt_component!(Radiobutton);
pub struct Radiobutton {
    co: c_component,
    attached_to_form: bool
}

impl Radiobutton {
    pub fn new(left: i32, top: i32, text: &str, is_default: bool,
               prev_button: Option<&mut Radiobutton>) -> Radiobutton {
        #[link(name="newt")]
        extern "C" {
            fn newtRadiobutton(left: c_int, top: c_int, text: *const c_char,
                               isDefault: c_int, prevButton: c_component)
                -> c_component;
        }

        let c_text = CString::new(text).unwrap();
        let ptr = match prev_button {
            Some(radio_button) => radio_button.co,
            None => ptr::null()
        };

        Radiobutton {
            attached_to_form: false,
            co: unsafe {
                newtRadiobutton(left, top, c_text.as_ptr(),
                                is_default as i32, ptr)
            }
        }
    }

    pub fn get_current(&self) -> Radiobutton {
        #[link(name="newt")]
        extern "C" {
            fn newtRadioGetCurrent(setMember: c_component) -> c_component;
        }

        Radiobutton {
            attached_to_form: true,
            co: unsafe { newtRadioGetCurrent(self.co) }
        }
    }

    pub fn set_current(&mut self) {
        #[link(name="newt")]
        extern "C" {
            fn newtRadioSetCurrent(setMember: c_component);
        }

        unsafe { newtRadioSetCurrent(self.co) }
    }
}

extern crate std;
use std::ffi::CString;
use ptr;

use components::c_component;
use components::Component;
use intern::ffi::newt::radiobutton::*;
use intern::ffi::newt::component::newtComponentDestroy;

newt_component!(Radiobutton);
pub struct Radiobutton {
    co: c_component,
    attached_to_form: bool
}

impl Radiobutton {
    pub fn new(left: i32, top: i32, text: &str, is_default: bool,
               prev_button: Option<&Radiobutton>) -> Radiobutton {
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
        Radiobutton {
            attached_to_form: true,
            co: unsafe { newtRadioGetCurrent(self.co) }
        }
    }

    pub fn set_current(&mut self) {
        unsafe { newtRadioSetCurrent(self.co) }
    }
}

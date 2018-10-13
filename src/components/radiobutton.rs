extern crate std;
extern crate newt_sys;
use std::ffi::CString;
use ptr;

use newt_sys::*;
use components::Component;

newt_component!(Radiobutton);
pub struct Radiobutton {
    co: newtComponent,
    attached_to_form: bool
}

impl Radiobutton {
    pub fn new(left: i32, top: i32, text: &str, is_default: bool,
               prev_button: Option<&Radiobutton>) -> Radiobutton {
        let c_text = CString::new(text).unwrap();
        let ptr = match prev_button {
            Some(radio_button) => radio_button.co,
            None => ptr::null_mut()
        };

        Radiobutton {
            co: unsafe {
                newtRadiobutton(left, top, c_text.as_ptr(),
                                is_default as i32, ptr)
            },
            attached_to_form: false
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

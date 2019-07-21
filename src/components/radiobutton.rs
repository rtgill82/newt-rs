extern crate std;
extern crate newt_sys;
use std::ffi::CString;
use crate::ptr;

use newt_sys::*;

///
/// A Radiobutton widget.
///
#[derive(Component, ComponentFuncs)]
pub struct Radiobutton {
    co: newtComponent,
    added_to_parent: bool
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
            added_to_parent: false
        }
    }

    pub fn get_current(&self) -> Radiobutton {
        Radiobutton {
            added_to_parent: true,
            co: unsafe { newtRadioGetCurrent(self.co) }
        }
    }

    pub fn set_current(&mut self) {
        unsafe { newtRadioSetCurrent(self.co) }
    }
}

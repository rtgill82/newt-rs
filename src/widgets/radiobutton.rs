use std::cell::Cell;
use std::ffi::CString;
use std::ptr;

use newt_sys::*;
use crate::component::Component;

///
/// A Radiobutton widget.
///
#[derive(Component)]
pub struct Radiobutton {
    co: Cell<newtComponent>,
    added_to_parent: Cell<bool>
}

impl Radiobutton {
    pub fn new(left: i32, top: i32, text: &str, is_default: bool,
               prev_button: Option<&Radiobutton>) -> Radiobutton {
        let c_text = CString::new(text).unwrap();
        let ptr = match prev_button {
            Some(radio_button) => radio_button.co(),
            None => ptr::null_mut()
        };

        Radiobutton {
            co: unsafe {
                let co = newtRadiobutton(left, top, c_text.as_ptr(),
                                         is_default as i32, ptr);
                Cell::new(co)
            },
            added_to_parent: Cell::new(false)
        }
    }

    pub fn get_current(&self) -> Radiobutton {
        Radiobutton {
            co: unsafe { Cell::new(newtRadioGetCurrent(self.co())) },
            added_to_parent: Cell::new(true)
        }
    }

    pub fn set_current(&self) {
        unsafe { newtRadioSetCurrent(self.co()) }
    }
}

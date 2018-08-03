extern crate std;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;
use ptr;

use components::NewtComponent;
use components::NewtComponentPtr;

newt_component!(Radiobutton);
pub struct Radiobutton {
    co: NewtComponentPtr
}

impl Radiobutton {
    pub fn new(left: i32, top: i32, text: &str, is_default: i32,
               prev_button: Option<Radiobutton>) -> Radiobutton {
        #[link(name="newt")]
        extern "C" {
            fn newtRadiobutton(left: c_int, top: c_int, text: *const c_char,
                               isDefault: c_int, prevButton: NewtComponentPtr)
                -> NewtComponentPtr;
        }

        let c_text = CString::new(text).unwrap();
        let ptr = match prev_button {
            Some(radio_button) => radio_button.co,
            None => ptr::null()
        };

        Radiobutton {
            co: unsafe {
                newtRadiobutton(left, top, c_text.as_ptr(), is_default, ptr)
            }
        }
    }

    pub fn get_current(&self) -> Radiobutton {
        #[link(name="newt")]
        extern "C" {
            fn newtRadioGetCurrent(setMember: NewtComponentPtr)
                -> NewtComponentPtr;
        }

        Radiobutton {
            co: unsafe { newtRadioGetCurrent(self.co) }
        }
    }

    pub fn set_current(&self) {
        #[link(name="newt")]
        extern "C" {
            fn newtRadioSetCurrent(setMember: NewtComponentPtr);
        }

        unsafe { newtRadioSetCurrent(self.co) }
    }
}

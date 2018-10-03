extern crate std;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;

use components::Component;
use components::NewtComponentPtr;

newt_component!(Label);
pub struct Label {
    co: NewtComponentPtr
}

impl Label  {
    pub fn new(left: i32, top: i32, text: &str) -> Label {
        #[link(name="newt")]
        extern "C" {
            fn newtLabel(left: c_int, top: c_int, text: *const c_char)
                -> NewtComponentPtr;
        }

        let c_text = CString::new(text).unwrap();
        Label {
            co: unsafe { newtLabel(left, top, c_text.as_ptr()) }
        }
    }

    pub fn set_text(&self, text: &str) {
        #[link(name="newt")]
        extern "C" {
            fn newtLabelSetText(co: NewtComponentPtr, text: *const c_char);
        }

        let c_text = CString::new(text).unwrap();
        unsafe { newtLabelSetText(self.co, c_text.as_ptr()); }
    }

    pub fn set_colors(&self, colorset: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtLabelSetColors(co: NewtComponentPtr, colorset: c_int);
        }

        unsafe { newtLabelSetColors(self.co, colorset); }
    }
}

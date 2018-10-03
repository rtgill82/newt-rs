extern crate std;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

use components::c_component;
use components::Component;
use components::form::ExitReason;

newt_component!(Label);
pub struct Label {
    co: c_component,
    attached_to_form: bool
}

impl Label  {
    pub fn new(left: i32, top: i32, text: &str) -> Label {
        #[link(name="newt")]
        extern "C" {
            fn newtLabel(left: c_int, top: c_int, text: *const c_char)
                -> c_component;
        }

        let c_text = CString::new(text).unwrap();
        Label {
            attached_to_form: false,
            co: unsafe { newtLabel(left, top, c_text.as_ptr()) }
        }
    }

    pub fn set_text(&mut self, text: &str) {
        #[link(name="newt")]
        extern "C" {
            fn newtLabelSetText(co: c_component, text: *const c_char);
        }

        let c_text = CString::new(text).unwrap();
        unsafe { newtLabelSetText(self.co, c_text.as_ptr()); }
    }

    pub fn set_colors(&mut self, colorset: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtLabelSetColors(co: c_component, colorset: c_int);
        }

        unsafe { newtLabelSetColors(self.co, colorset); }
    }
}

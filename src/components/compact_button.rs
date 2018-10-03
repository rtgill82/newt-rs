extern crate std;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

use components::c_component;
use components::Component;

newt_component!(CompactButton);
pub struct CompactButton {
    co: c_component,
    attached_to_form: bool
}

impl CompactButton {
    pub fn new(left: i32, top: i32, text: &str) -> CompactButton {
        #[link(name="newt")]
        extern "C" {
            fn newtCompactButton(left: c_int, top: c_int, text: *const c_char)
                -> c_component;
        }

        let c_str = CString::new(text).unwrap();
        CompactButton {
            attached_to_form: false,
            co: unsafe { newtCompactButton(left, top, c_str.as_ptr()) }
        }
    }
}

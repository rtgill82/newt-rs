extern crate std;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;

use components::c_component;
use components::Component;

newt_component!(Button);
pub struct Button {
    co: c_component
}

impl Button {
    pub fn new(left: i32, top: i32, text: &str) -> Button {
        #[link(name="newt")]
        extern "C" {
            fn newtButton(left: c_int, top: c_int, text: *const c_char)
                -> c_component;
        }

        let c_str = CString::new(text).unwrap();
        Button {
            co: unsafe { newtButton(left, top, c_str.as_ptr()) }
        }
    }
}

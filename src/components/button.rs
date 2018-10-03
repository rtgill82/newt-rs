extern crate std;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;

use components::Component;
use components::NewtComponentPtr;

newt_component!(Button);
pub struct Button {
    co: NewtComponentPtr
}

impl Button {
    pub fn new(left: i32, top: i32, text: &str) -> Button {
        #[link(name="newt")]
        extern "C" {
            fn newtButton(left: c_int, top: c_int, text: *const c_char)
                -> NewtComponentPtr;
        }

        let c_str = CString::new(text).unwrap();
        Button {
            co: unsafe { newtButton(left, top, c_str.as_ptr()) }
        }
    }
}

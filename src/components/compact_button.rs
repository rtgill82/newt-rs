extern crate std;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;

use components::NewtComponent;
use components::NewtComponentPtr;

newt_component!(CompactButton);
pub struct CompactButton {
    co: NewtComponentPtr
}

impl CompactButton {
    pub fn new(left: i32, top: i32, text: &str) -> CompactButton {
        #[link(name="newt")]
        extern "C" {
            fn newtCompactButton(left: c_int, top: c_int, text: *const c_char)
                -> NewtComponentPtr;
        }

        let c_str = CString::new(text).unwrap();
        CompactButton {
            co: unsafe { newtCompactButton(left, top, c_str.as_ptr()) }
        }
    }
}

extern crate std;
use std::os::raw::{c_char, c_int};
use components::c_component;

extern "C" {
    pub fn newtCompactButton(left: c_int, top: c_int, text: *const c_char)
        -> c_component;
}

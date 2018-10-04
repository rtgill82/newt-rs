extern crate std;
use std::os::raw::{c_char, c_int};
use components::c_component;

extern "C" {
    pub fn newtLabel(left: c_int, top: c_int, text: *const c_char)
        -> c_component;
    pub fn newtLabelSetText(co: c_component, text: *const c_char);
    pub fn newtLabelSetColors(co: c_component, colorset: c_int);
}

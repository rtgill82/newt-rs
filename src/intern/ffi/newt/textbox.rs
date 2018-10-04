extern crate std;
use std::os::raw::{c_char, c_int};
use components::c_component;

extern "C" {
    pub fn newtTextbox(left: c_int, top: c_int, width: c_int,
                       height: c_int, flags: c_int) -> c_component;
    pub fn newtTextboxReflowed(left: c_int, top: c_int, text: *const c_char,
                               width: c_int, flexDown: c_int, flexUp: c_int,
                               flags: c_int) -> c_component;
    pub fn newtTextboxSetText(co: c_component, text: *const c_char);
    pub fn newtTextboxSetHeight(co: c_component, height: c_int);
    pub fn newtTextboxGetNumLines(co: c_component) -> c_int;
    pub fn newtTextboxSetColors(co: c_component, normal: c_int, active: c_int);
}

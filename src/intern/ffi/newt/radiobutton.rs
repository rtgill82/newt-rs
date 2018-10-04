extern crate std;
use std::os::raw::{c_char, c_int};
use components::c_component;

extern "C" {
    pub fn newtRadiobutton(left: c_int, top: c_int, text: *const c_char,
                           isDefault: c_int, prevButton: c_component)
        -> c_component;
    pub fn newtRadioGetCurrent(setMember: c_component) -> c_component;
    pub fn newtRadioSetCurrent(setMember: c_component);
}

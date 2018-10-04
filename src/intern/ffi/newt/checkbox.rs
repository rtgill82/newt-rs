extern crate std;
use std::os::raw::{c_char, c_int};
use components::c_component;
use FlagsSense;

extern "C" {
    pub fn newtCheckbox(left: c_int, top: c_int, text: *const c_char,
                        defValue: c_char, seq: *const c_char,
                        result: *mut c_char) -> c_component;
    pub fn newtCheckboxGetValue(co: c_component) -> c_char;
    pub fn newtCheckboxSetValue(co: c_component, value: c_char);
    pub fn newtCheckboxSetFlags(co: c_component, flags: c_int,
                                sense: FlagsSense);
}

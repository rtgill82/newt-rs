extern crate std;
use std::os::raw::{c_char, c_int};
use components::c_component;
use constants::FlagsSense;

extern "C" {
    pub fn newtEntry(left: c_int, top: c_int, initialValue: *const c_char,
                     width: c_int, resultPtr: *const c_char, flags: c_int)
        -> c_component;
    pub fn newtEntryGetValue(co: c_component) -> *mut c_char;
    pub fn newtEntrySet(co: c_component, value: *const c_char,
                        cursorAtEnd: c_int);
    pub fn newtEntrySetFlags(co: c_component, flags: c_int, sense: FlagsSense);
    pub fn newtEntrySetColors(co: c_component, normal: c_int, disabled: c_int);
}

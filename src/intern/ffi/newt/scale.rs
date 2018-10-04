extern crate std;
use std::os::raw::{c_int, c_longlong, c_ulonglong};
use components::c_component;

extern "C" {
    pub fn newtScale(left: c_int, top: c_int, width: c_int,
                     fullValue: c_longlong) -> c_component;
    pub fn newtScaleSet(co: c_component, amount: c_ulonglong);
    pub fn newtScaleSetColors(co: c_component, empty: c_int, full: c_int);
}

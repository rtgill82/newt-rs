extern crate std;
use std::os::raw::c_int;
use components::c_component;

extern "C" {
    pub fn newtComponentTakesFocus(co: c_component, val: c_int);
    pub fn newtComponentDestroy(co: c_component);
}

extern crate std;
use std::os::raw::{c_char, c_int};
use components::c_component;
use intern::structs::ExitStruct;

extern "C" {
    pub fn newtForm(vert_bar: c_component, help: *const c_char,
                    flags: c_int) -> c_component;
    pub fn newtFormAddComponent(form: c_component, co: c_component);
    pub fn newtFormSetHeight(co: c_component, height: c_int);
    pub fn newtFormSetWidth(co: c_component, width: c_int);
    pub fn newtFormAddHotKey(co: c_component, key: c_int);
    pub fn newtFormSetTimer(form: c_component, millisecs: c_int);
    pub fn newtFormRun(form: c_component, es: *mut ExitStruct);
    pub fn newtDrawForm(co: c_component);
    pub fn newtFormDestroy(form: c_component);
}

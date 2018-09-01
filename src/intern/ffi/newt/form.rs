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
    pub fn newtFormSetSize(co: c_component);
    pub fn newtFormAddHotKey(co: c_component, key: c_int);
    pub fn newtFormSetTimer(form: c_component, millisecs: c_int);
    pub fn newtFormGetCurrent(form: c_component) -> c_component;
    pub fn newtFormSetCurrent(form: c_component, co: c_component);
    pub fn newtFormSetBackground(form: c_component, color: i32);
    pub fn newtFormGetScrollPosition(form: c_component) -> c_int;
    pub fn newtFormSetScrollPosition(form: c_component, position: i32);
    pub fn newtFormRun(form: c_component, es: *mut ExitStruct);
    pub fn newtDrawForm(co: c_component);
    pub fn newtFormDestroy(form: c_component);
}

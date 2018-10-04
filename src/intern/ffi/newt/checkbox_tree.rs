extern crate std;
use std::os::raw::{c_char, c_int, c_void};
use components::c_component;

extern "C" {
    pub fn newtCheckboxTree(left: c_int, top: c_int, height: c_int,
                            flags: c_int) -> c_component;
    pub fn newtCheckboxTreeMulti(left: c_int, top: c_int, height: c_int,
                                 seq: *const c_char, flags: c_int)
        -> c_component;
    pub fn newtCheckboxTreeSetWidth(co: c_component, width: c_int);
    pub fn newtCheckboxTreeAddArray(co: c_component,
                                    text: *const c_char,
                                    data: *const c_void,
                                    flags: c_int,
                                    indexes: *const c_int) -> c_int;
    pub fn newtCheckboxTreeGetCurrent(co: c_component) -> *const c_void;
    pub fn newtCheckboxTreeSetCurrent(co: c_component, item: *const c_void);
    pub fn newtCheckboxTreeGetSelection(co: c_component, numitems: *mut c_int)
        -> *const c_void;
    pub fn newtCheckboxTreeGetMultiSelection(co: c_component,
                                             numitems: &mut c_int,
                                             seqnum: c_char)
        -> *const c_void;
    pub fn newtCheckboxTreeFindItem(co: c_component, data: *const c_void)
        -> *mut c_int;
    pub fn newtCheckboxTreeSetEntry(co: c_component,
                                    data: *const c_void,
                                    text: *const c_char);
    pub fn newtCheckboxTreeGetEntryValue(co: c_component,
                                         data: *const c_void) -> c_char;
    pub fn newtCheckboxTreeSetEntryValue(co: c_component,
                                         data: *const c_void,
                                         value: c_char);
}

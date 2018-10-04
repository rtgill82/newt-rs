extern crate std;
use std::os::raw::{c_char, c_int, c_void};
use components::c_component;
use FlagsSense;

extern "C" {
    pub fn newtListbox(left: c_int, top: c_int, height: c_int,
                       flags: c_int) -> c_component;
    pub fn newtListboxSetWidth(co: c_component, width: c_int);
    pub fn newtListboxItemCount(co: c_component) -> c_int;
    pub fn newtListboxAppendEntry(co: c_component, text: *const c_char,
                                  data: *const c_void) -> c_int;
    pub fn newtListboxInsertEntry(co: c_component, text: *const c_char,
                                  data: *const c_void, key: *const c_void)
        -> c_int;
    pub fn newtListboxGetCurrent(co: c_component) -> *const c_void;
    pub fn newtListboxSetCurrent(co: c_component, num: c_int);
    pub fn newtListboxSetCurrentByKey(co: c_component, key: *const c_void);
    pub fn newtListboxGetEntry(co: c_component, num: c_int,
                               text: *const *mut c_char,
                               data: *const *mut c_void);
    pub fn newtListboxSetEntry(co: c_component, num: c_int,
                               text: *const c_char);
    pub fn newtListboxSetData(co: c_component, num: c_int,
                              data: *const c_void);
    pub fn newtListboxDeleteEntry(co: c_component, data: *const c_void)
        -> c_int;
    pub fn newtListboxClear(co: c_component);
    pub fn newtListboxGetSelection(co: c_component, numitems: *mut c_int)
        -> *const c_void;
    pub fn newtListboxSelectItem(co: c_component, key: *const c_void,
                                 sense: FlagsSense);
    pub fn newtListboxClearSelection(co: c_component);
}

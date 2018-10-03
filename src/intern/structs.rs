use std::os::raw::c_char;
use std::os::raw::c_int;
use components::c_component;

#[repr(C)]
#[allow(dead_code)]
pub enum ExitStructEnum {
    HotKey,
    Component,
    FDReady,
    Timer,
    Error
}

#[repr(C)]
pub union ExitStructUnion {
    pub watch: c_int,
    pub key: c_int,
    pub co: c_component
}

#[repr(C)]
pub struct ExitStruct {
    pub reason: ExitStructEnum,
    pub u: ExitStructUnion
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct NewtColors {
    pub rootFg: *const c_char,          pub rootBg: *const c_char,
    pub borderFg: *const c_char,        pub borderBg: *const c_char,
    pub windowFg: *const c_char,        pub windowBg: *const c_char,
    pub shadowFg: *const c_char,        pub shadowBg: *const c_char,
    pub titleFg: *const c_char,         pub titleBg: *const c_char,
    pub buttonFg: *const c_char,        pub buttonBg: *const c_char,
    pub actButtonFg: *const c_char,     pub actButtonBg: *const c_char,
    pub checkboxFg: *const c_char,      pub checkboxBg: *const c_char,
    pub actCheckboxFg: *const c_char,   pub actCheckboxBg: *const c_char,
    pub entryFg: *const c_char,         pub entryBg: *const c_char,
    pub labelFg: *const c_char,         pub labelBg: *const c_char,
    pub listboxFg: *const c_char,       pub listboxBg: *const c_char,
    pub actListboxFg: *const c_char,    pub actListboxBg: *const c_char,
    pub textboxFg: *const c_char,       pub textboxBg: *const c_char,
    pub actTextboxFg: *const c_char,    pub actTextboxBg: *const c_char,
    pub helpLineFg: *const c_char,      pub helpLineBg: *const c_char,
    pub rootTextFg: *const c_char,      pub rootTextBg: *const c_char,
    pub emptyScale: *const c_char,      pub fullScale: *const c_char,
    pub disabledEntryFg: *const c_char, pub disabledEntryBg: *const c_char,
    pub compactButtonFg: *const c_char, pub compactButtonBg: *const c_char,
    pub actSelListboxFg: *const c_char, pub actSelListboxBg: *const c_char,
    pub selListboxFg: *const c_char,    pub selListboxBg: *const c_char
}

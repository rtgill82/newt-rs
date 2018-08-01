use std::os::raw::c_int;
use NewtComponentPtr;

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
    pub co: NewtComponentPtr
}

#[repr(C)]
pub struct ExitStruct {
    pub reason: ExitStructEnum,
    pub u: ExitStructUnion
}

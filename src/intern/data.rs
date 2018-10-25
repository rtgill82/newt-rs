use std::os::raw::c_void;
use ptr;

pub trait Data {
    fn newt_to_ptr(&self) -> *const c_void;
    fn newt_from_ptr(ptr: *const c_void) -> Self;
}

impl Data for () {
    fn newt_to_ptr(&self) -> *const c_void {
        ptr::null()
    }

    fn newt_from_ptr(_ptr: *const c_void) -> Self {
        ()
    }
}

impl Data for char {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as u8 as char
    }
}

impl Data for i8 {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as i8
    }
}

impl Data for i32 {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as i32
    }
}

impl Data for isize {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as isize
    }
}

impl Data for u8 {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as u8
    }
}

impl Data for u32 {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as usize as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize as u32
    }
}

impl Data for usize {
    fn newt_to_ptr(&self) -> *const c_void {
        *self as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        ptr as usize
    }
}

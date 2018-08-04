extern crate std;
use std::ffi::CString;
use std::marker::PhantomData;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_void;

use components::NewtComponent;
use components::NewtComponentPtr;

pub struct Listitem<T> {
    co: NewtComponentPtr,
    data: PhantomData<T>
}

impl<T> NewtComponent for Listitem<T> {
    fn co(&self) -> NewtComponentPtr {
        self.co
    }

    fn takes_focus(&self, value: bool) {
        #[link(name="newt")]
        extern "C" {
            fn newtComponentTakesFocus(co: NewtComponentPtr,
                                       val: c_int);
        }

        unsafe { newtComponentTakesFocus(self.co, value as c_int); }
    }
}

impl<T, Rhs: NewtComponent> std::cmp::PartialEq<Rhs> for Listitem<T> {
    fn eq(&self, other: &Rhs) -> bool {
        self.co == other.co()
    }
}

impl<T> std::ops::Deref for Listitem<T> {
    type Target = NewtComponentPtr;
    fn deref(&self) -> &Self::Target {
        &self.co
    }
}

impl<T> Listitem<T> {
    pub fn new(left: i32, top: i32, text: &str, is_default: i32,
               prev_item: Listitem<T>, data: &T, flags: i32)
            -> Listitem<T> {
        extern "C" {
            fn newtListItem(left: c_int, top: c_int, text: *const c_char,
                            isDefault: c_int, prevItem: NewtComponentPtr,
                            data: *const c_void, flags: c_int)
                -> NewtComponentPtr;
        }

        let c_str = CString::new(text).unwrap();
        let c_data: *const c_void  = data as *const _ as *const c_void;
        Listitem {
            data: PhantomData,
            co: unsafe {
                newtListItem(left, top, c_str.as_ptr(), is_default,
                             prev_item.co, c_data, flags)
            }
        }
    }

    pub fn set(&self, text: &str) {
        extern "C" {
            fn newtListitemSet(co: NewtComponentPtr, text: *const c_char);
        }

        let c_str = CString::new(text).unwrap();
        unsafe { newtListitemSet(self.co, c_str.as_ptr()); }
    }

    pub fn get_data(&self) -> &T {
        extern "C" {
            fn newtListitemGetData(co: NewtComponentPtr) -> *const c_void;
        }

        let c_data = unsafe { newtListitemGetData(self.co) };
        unsafe { &*(c_data as *const T) }
    }
}

extern crate std;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw::{c_char, c_int, c_void};
use ptr;

use components::Component;
use components::NewtComponentPtr;
use FlagsSense;

newt_component!(Listbox, K, D);
pub struct Listbox<K, D> {
    co: NewtComponentPtr,
    key: PhantomData<K>,
    data: PhantomData<D>
}

impl<K, D> Listbox<K, D> {
    pub fn new(left: i32, top: i32, height: i32, flags: i32) -> Listbox<K, D> {
        #[link(name="newt")]
        extern "C" {
            fn newtListbox(left: c_int, top: c_int, height: c_int,
                           flags: c_int) -> NewtComponentPtr;
        }

        Listbox {
            key: PhantomData,
            data: PhantomData,
            co: unsafe {
                newtListbox(left, top, height, flags)
            }
        }
    }

    pub fn get_current(&self) -> &D {
        #[link(name="newt")]
        extern "C" {
            fn newtListboxGetCurrent(co: NewtComponentPtr) -> *const c_void;
        }

        let c_data = unsafe { newtListboxGetCurrent(self.co) };
        unsafe { &*(c_data as *const D) }
    }

    pub fn set_current(&self, num: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtListboxSetCurrent(co: NewtComponentPtr, num: c_int);
        }

        unsafe { newtListboxSetCurrent(self.co, num); }
    }

    pub fn set_current_by_key(&self, key: &K) {
        #[link(name="newt")]
        extern "C" {
            fn newtListboxSetCurrentByKey(co: NewtComponentPtr, key: *const c_void);
        }

        let c_key: *const c_void = key as *const _ as *const c_void;
        unsafe { newtListboxSetCurrentByKey(self.co, c_key); }
    }

    pub fn set_entry(&self, num: i32, text: &str) {
        #[link(name="newt")]
        extern "C" {
            fn newtListboxSetEntry(co: NewtComponentPtr, num: c_int,
                                   text: *const c_char);
        }

        let c_str = CString::new(text).unwrap();
        unsafe { newtListboxSetEntry(self.co, num, c_str.as_ptr()); }
    }

    pub fn set_width(&self, width: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtListboxSetWidth(co: NewtComponentPtr, width: c_int);
        }

        unsafe { newtListboxSetWidth(self.co, width); }
    }

    pub fn set_data(&self, num: i32, data: &D) {
        #[link(name="newt")]
        extern "C" {
            fn newtListboxSetData(co: NewtComponentPtr, num: c_int,
                                  data: *const c_void);
        }

        let c_data: *const c_void = data as *const _ as *const c_void;
        unsafe { newtListboxSetData(self.co, num, c_data); }
    }

    pub fn append_entry(&self, text: &str, data: &D) -> i32 {
        #[link(name="newt")]
        extern "C" {
            fn newtListboxAppendEntry(co: NewtComponentPtr,
                                      text: *const c_char,
                                      data: *const c_void) -> c_int;
        }

        let c_str = CString::new(text).unwrap();
        let c_data: *const c_void = data as *const _ as *const c_void;
        unsafe { newtListboxAppendEntry(self.co, c_str.as_ptr(), c_data) }
    }

    pub fn insert_entry(&self, text: &str, data: &D, key: &K) -> i32 {
        #[link(name="newt")]
        extern "C" {
            fn newtListboxInsertEntry(co: NewtComponentPtr,
                                      text: *const c_char,
                                      data: *const c_void,
                                      key: *const c_void) -> c_int;
        }

        let c_str = CString::new(text).unwrap();
        let c_data: *const c_void = data as *const _ as *const c_void;
        let c_key: *const c_void = key as *const _ as *const c_void;
        unsafe {
            newtListboxInsertEntry(self.co, c_str.as_ptr(), c_data, c_key)
        }
    }

    pub fn delete_entry(&self, data: &D) -> i32 {
        #[link(name="newt")]
        extern "C" {
            fn newtListboxDeleteEntry(co: NewtComponentPtr,
                                      data: *const c_void) -> c_int;
        }

        let c_data: *const c_void = data as *const _ as *const c_void;
        unsafe { newtListboxDeleteEntry(self.co, c_data) }
    }

    pub fn clear(&self) {
        #[link(name="newt")]
        extern "C" {
            fn newtListboxClear(co: NewtComponentPtr);
        }

        unsafe { newtListboxClear(self.co); }
    }

    pub fn get_entry(&self, num: i32) -> (&str, &D) {
        #[link(name="newt")]
        extern "C" {
            fn newtListboxGetEntry(co: NewtComponentPtr, num: c_int,
                                   text: *mut c_char, data: *mut c_void);
        }

        let c_str: *mut c_char = ptr::null_mut();
        let c_data: *mut c_void = ptr::null_mut();
        unsafe { newtListboxGetEntry(self.co, num, c_str, c_data); }
        let c_str = unsafe { CStr::from_ptr(c_str) };
        let c_data = unsafe { &*(c_data as *const D) };
        (c_str.to_str().unwrap(), c_data)
    }

    pub fn get_selection(&self, numitems: i32) -> (Box<[&D]>) {
        #[link(name="newt")]
        extern "C" {
            fn newtListboxGetSelection(co: NewtComponentPtr,
                                       numitems: *mut c_int)
                -> *const c_void;
        }

        let mut nitems: i32 = numitems;
        let ptr: *const c_void = unsafe {
            newtListboxGetSelection(self.co, &mut nitems)
        };

        let mut vec: Vec<&D> = Vec::new();
        if nitems > 0 {
            let mut cnt = 0;
            let mut p = ptr;
            unsafe {
                while cnt < nitems {
                    vec.push(&*(p as *const D));
                    p = p.offset(1);
                    cnt += 1;
                }
            }
        }
        vec.into_boxed_slice()
    }

    pub fn clear_selection(&self) {
        #[link(name="newt")]
        extern "C" {
            fn newtListboxClearSelection(co: NewtComponentPtr);
        }

        unsafe { newtListboxClearSelection(self.co) };
    }

    pub fn select_item(&self, key: &K, sense: FlagsSense) {
        #[link(name="newt")]
        extern "C" {
            fn newtListboxSelectItem(co: NewtComponentPtr, key: *const c_void,
                                     sense: FlagsSense);
        }

        let c_key: *const c_void = key as *const _ as *const c_void;
        unsafe { newtListboxSelectItem(self.co, c_key, sense) };
    }

    pub fn item_count(&self) {
        #[link(name="newt")]
        extern "C" {
            fn newtListboxSelectItem(co: NewtComponentPtr);
        }

        unsafe { newtListboxSelectItem(self.co) };
    }
}

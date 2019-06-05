extern crate std;
extern crate newt_sys;

use std::ops::Drop;
use std::os::unix::io::RawFd;
use std::ptr;

use newt_sys::*;
use crate::callbacks::HelpCallback;
use crate::components::Component;
use crate::components::VerticalScrollbar;

mod exit_reason;
pub use self::exit_reason::ExitReason;

#[allow(non_camel_case_types)]
type newtExitReason = newtExitStruct__bindgen_ty_1;
const NEWT_EXIT_HOTKEY: newtExitReason    = newtExitStruct_NEWT_EXIT_HOTKEY;
const NEWT_EXIT_COMPONENT: newtExitReason = newtExitStruct_NEWT_EXIT_COMPONENT;
const NEWT_EXIT_FDREADY: newtExitReason   = newtExitStruct_NEWT_EXIT_FDREADY;
const NEWT_EXIT_TIMER: newtExitReason     = newtExitStruct_NEWT_EXIT_TIMER;
const NEWT_EXIT_ERROR: newtExitReason     = newtExitStruct_NEWT_EXIT_ERROR;

#[allow(non_camel_case_types)]
type newtExitStructUnion = newtExitStruct__bindgen_ty_2;

#[repr(C)]
pub enum FDFlags {
    Read   = NEWT_FD_READ as isize,
    Write  = NEWT_FD_WRITE as isize,
    Except = NEWT_FD_EXCEPT as isize
}

#[derive(Component, ComponentFuncs)]
struct BaseComponent {
    co: newtComponent,
    added_to_parent: bool
}

#[derive(Component, ComponentFuncs)]
pub struct Form
{
    co: newtComponent,
    added_to_parent: bool
}

impl Drop for Form
{
    fn drop(&mut self) {
        if !self.added_to_parent {
            unsafe { newtFormDestroy(self.co); }
        }
    }
}

impl Form
{
    pub fn new(_scrollbar: Option<&VerticalScrollbar>, flags: i32) -> Form {
        Form {
            co: unsafe { newtForm(ptr::null_mut(), ptr::null_mut(), flags) },
            added_to_parent: false
        }
    }

    ///
    /// Creates a new `Form` with an associated help `HelpCallback`. See
    /// [HelpCallback][help_cb] for additional information.
    ///
    /// [help_cb]: ../callbacks/struct.HelpCallback.html#method.new
    ///
    pub fn new_with_help_callback<FN, T>
      (_scrollbar: Option<&VerticalScrollbar>, flags: i32, function: FN, data: Option<T>)
        -> (Form, Box<HelpCallback<FN, T>>)
        where FN: Fn(&Form, Option<&T>)
    {
        HelpCallback::new(_scrollbar, flags, function, data)
    }

    pub(crate) fn new_co(co: newtComponent) -> Form {
        Form { co, added_to_parent: false }
    }

    pub fn add_component(&mut self, component: &mut dyn Component)
      -> Result<(), &'static str> {
        component.add_to_parent(false)?;
        unsafe { newtFormAddComponent(self.co, component.co()); }
        Ok(())
    }

    pub fn add_components(&mut self, components: &mut [&mut dyn Component])
            -> Result<(), &'static str> {
        for component in components.iter_mut() {
            self.add_component(*component)?;
        }
        Ok(())
    }

    pub fn set_height(&mut self, height: i32) {
        unsafe { newtFormSetHeight(self.co, height); }
    }

    pub fn set_width(&mut self, width: i32) {
        unsafe { newtFormSetWidth(self.co, width); }
    }

    pub fn set_size(&mut self) {
        unsafe { newtFormSetSize(self.co); }
    }

    pub fn add_hot_key(&mut self, key: i32) {
        unsafe { newtFormAddHotKey(self.co, key); }
    }

    pub fn set_timer(&mut self, millisecs: i32) {
        unsafe { newtFormSetTimer(self.co, millisecs); }
    }

    pub fn watch_fd(&mut self, fd: RawFd, flags: FDFlags) {
        unsafe { newtFormWatchFd(self.co, fd, flags as i32); }
    }

    pub fn get_current(&self) -> Box<dyn Component> {
        Box::new(BaseComponent {
            co: unsafe { newtFormGetCurrent(self.co) },
            added_to_parent: true
        })
    }

    pub fn set_current(&mut self, subcomponent: &dyn Component) {
        unsafe { newtFormSetCurrent(self.co, subcomponent.co()); }
    }

    pub fn set_background(&mut self, color: i32) {
        unsafe { newtFormSetBackground(self.co, color); }
    }

    pub fn get_scroll_position(&self) -> i32 {
        unsafe { newtFormGetScrollPosition(self.co) }
    }

    pub fn set_scroll_position(&mut self, position: i32) {
        unsafe { newtFormSetScrollPosition(self.co, position); }
    }

    pub fn run(&self) -> Result<ExitReason, ()> {
        use self::ExitReason::{HotKey,Component,FDReady,Timer};

        let mut es = newtExitStruct {
            reason: NEWT_EXIT_HOTKEY,
            u: newtExitStructUnion { watch: 0 }
        };

        unsafe {
            newtFormRun(self.co, &mut es);
            match es.reason {
                NEWT_EXIT_HOTKEY => Ok(HotKey(es.u.key)),
                NEWT_EXIT_COMPONENT => Ok(
                    Component(Box::new(BaseComponent {
                                       co: es.u.co,
                                       added_to_parent: true
                    }))
                ),
                NEWT_EXIT_FDREADY => Ok(FDReady(es.u.watch)),
                NEWT_EXIT_TIMER => Ok(Timer),
                NEWT_EXIT_ERROR => Err(()),
                _ => panic!("Unexpected newt exit reason.")
            }
        }
    }

    pub fn draw(&self) {
        unsafe { newtDrawForm(self.co); }
    }
}

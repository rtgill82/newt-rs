use std::cell::Cell;
use std::ops::Drop;
use std::os::unix::io::RawFd;
use std::ptr;

use newt_sys::*;
use crate::component::Component;
use crate::callbacks::HelpCallback;
use crate::widgets::VerticalScrollbar;

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

///
/// File descriptor flags for the [`Form.watch_fd()`][watch_fd] function.
///
/// [watch_fd]: ../form/struct.Form.html#method.watch_fd
///
#[repr(C)]
pub enum FDFlags {
    /// Exit when the file descriptor is ready for reading.
    Read   = NEWT_FD_READ as isize,
    /// Exit when the file descriptor is ready for writing.
    Write  = NEWT_FD_WRITE as isize,
    /// Exit when an exception has occurred on the file descriptor.
    Except = NEWT_FD_EXCEPT as isize
}

#[derive(Component)]
struct BaseComponent {
    co: newtComponent,
    added_to_parent: Cell<bool>
}

///
/// Displays `Component`s and accepts user input.
///
#[derive(Component)]
pub struct Form
{
    co: newtComponent,
    added_to_parent: Cell<bool>
}

impl Drop for Form
{
    fn drop(&mut self) {
        if !self.added_to_parent.get() {
            unsafe { newtFormDestroy(self.co); }
        }
    }
}

impl Form
{
    ///
    /// Creates a new `Form`.
    ///
    pub fn new(scrollbar: Option<&VerticalScrollbar>, flags: i32) -> Form {
        let scrollbar = if let Some(scrollbar) = scrollbar {
            scrollbar.co()
        } else {
            ptr::null_mut()
        };

        Form {
            co: unsafe { newtForm(scrollbar, ptr::null_mut(), flags) },
            added_to_parent: Cell::new(false)
        }
    }

    ///
    /// Creates a new `Form` with an associated help `HelpCallback`. See
    /// [HelpCallback][help_cb] for additional information.
    ///
    /// [help_cb]: ../../callbacks/struct.HelpCallback.html#method.new
    ///
    pub fn new_with_help_callback<FN, T>
      (scrollbar: Option<&VerticalScrollbar>, flags: i32, function: FN, data: Option<T>)
        -> (Form, Box<HelpCallback<FN, T>>)
        where FN: Fn(&Form, Option<&T>)
    {
        HelpCallback::new(scrollbar, flags, function, data)
    }

    pub(crate) fn new_co(co: newtComponent) -> Form {
        Form { co, added_to_parent: Cell::new(false) }
    }

    ///
    /// Add a `Component` to the `Form` to be displayed when the `Form` is run.
    ///
    pub fn add_component(&mut self, component: &dyn Component)
      -> Result<(), &'static str> {
        component.add_to_parent()?;
        unsafe { newtFormAddComponent(self.co, component.co()); }
        Ok(())
    }

    ///
    /// Add multiple `Component`s to the `Form`.
    ///
    pub fn add_components(&mut self, components: &[&dyn Component])
            -> Result<(), &'static str> {
        for component in components.iter() {
            self.add_component(*component)?;
        }
        Ok(())
    }

    ///
    /// Set the height of the `Form`.
    ///
    pub fn set_height(&self, height: i32) {
        unsafe { newtFormSetHeight(self.co, height); }
    }

    ///
    /// Set the width of the `Form`.
    ///
    pub fn set_width(&self, width: i32) {
        unsafe { newtFormSetWidth(self.co, width); }
    }

    ///
    /// Tell the `Form` to resize itself.
    ///
    pub fn set_size(&self) {
        unsafe { newtFormSetSize(self.co); }
    }

    ///
    /// Add an exit hot key to the `Form`. The `Form` will stop running
    /// when the key is pressed.
    ///
    pub fn add_hot_key(&self, key: i32) {
        unsafe { newtFormAddHotKey(self.co, key); }
    }

    ///
    /// Add an exit timer to the `Form`. The `Form` will stop running
    /// when the timer times out.
    ///
    /// * `millisecs` - The timer countdown in milliseconds.
    ///
    pub fn set_timer(&self, millisecs: i32) {
        unsafe { newtFormSetTimer(self.co, millisecs); }
    }

    ///
    /// Add a file descriptor for the `Form` to watch. The `Form` will stop
    /// running when the specified activity occurs on the file descriptor.
    ///
    /// * `fd` - The file descriptor to watch.
    /// * `flags` - Flags specifying the activity to watch for.
    ///
    pub fn watch_fd(&self, fd: RawFd, flags: FDFlags) {
        unsafe { newtFormWatchFd(self.co, fd, flags as i32); }
    }

    ///
    /// Get the `Form`'s currently focused `Component`.
    ///
    pub fn get_current(&self) -> Box<dyn Component> {
        Box::new(BaseComponent {
            co: unsafe { newtFormGetCurrent(self.co) },
            added_to_parent: Cell::new(true)
        })
    }

    ///
    /// Set the `Form`'s currently focused `Component`.
    ///
    pub fn set_current(&self, subcomponent: &dyn Component) {
        unsafe { newtFormSetCurrent(self.co, subcomponent.co()); }
    }

    ///
    /// Set the `Form`'s background color.
    ///
    pub fn set_background(&self, color: i32) {
        unsafe { newtFormSetBackground(self.co, color); }
    }

    pub fn get_scroll_position(&self) -> i32 {
        unsafe { newtFormGetScrollPosition(self.co) }
    }

    pub fn set_scroll_position(&self, position: i32) {
        unsafe { newtFormSetScrollPosition(self.co, position); }
    }

    ///
    /// Run the form displaying all added components and accepting
    /// input from the user.
    ///
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
                                       added_to_parent: Cell::new(true)
                    }))
                ),
                NEWT_EXIT_FDREADY => Ok(FDReady(es.u.watch)),
                NEWT_EXIT_TIMER => Ok(Timer),
                NEWT_EXIT_ERROR => Err(()),
                _ => panic!("Unexpected newt exit reason.")
            }
        }
    }

    ///
    /// Redraw the `Form`.
    ///
    pub fn draw(&self) {
        unsafe { newtDrawForm(self.co); }
    }
}

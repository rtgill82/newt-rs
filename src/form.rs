//
// Copyright (C) 2019,2020 Robert Gill <rtgill82@gmail.com>
//
// This file is a part of newt-rs.
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License version 2.1 as published by the Free Software Foundation.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

//!
//! Displays `Component`s and accepts user input.
//!
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
/// [watch_fd]: crate::form::Form::watch_fd
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
    co: Cell<newtComponent>,
    added_to_parent: Cell<bool>
}

///
/// Displays `Component`s and accepts user input.
///
pub struct Form<'a>
{
    pub(crate) co: newtComponent,
    components: Vec<&'a dyn Component>
}

impl<'a> Drop for Form<'a>
{
    fn drop(&mut self) {
        unsafe { newtFormDestroy(self.co); }
        self.co = ptr::null_mut();

        for component in self.components.iter() {
            component.nullify();
        }
    }
}

impl<'a> Form<'a>
{
    ///
    /// Creates a new `Form`.
    ///
    pub fn new(scrollbar: Option<&VerticalScrollbar>, flags: i32) -> Form<'a> {
        let scrollbar = if let Some(scrollbar) = scrollbar {
            scrollbar.co()
        } else {
            ptr::null_mut()
        };

        Form {
            co: unsafe { newtForm(scrollbar, ptr::null_mut(), flags) },
            components: Vec::new()
        }
    }

    ///
    /// Creates a new `Form` with an associated help `HelpCallback`. See
    /// [`HelpCallback`] for additional information.
    ///
    pub fn new_with_help_callback<FN, T>
      (scrollbar: Option<&VerticalScrollbar>, flags: i32,
       function: FN, data: Option<T>)
      -> (Form<'a>, Box<HelpCallback<'a, FN, T>>)
        where FN: Fn(&Form, Option<&T>)
    {
        HelpCallback::new(scrollbar, flags, data, function)
    }

    pub(crate) fn new_co(co: newtComponent) -> Form<'a> {
        Form {
            co: co,
            components: Vec::new()
        }
    }

    #[cfg(feature = "asm")]
    pub(crate) fn add_refs(&mut self, components: Vec<&'a dyn Component>) {
        for co in components.iter() {
            self.components.push(*co);
        }
    }

    ///
    /// Add a `Component` to the `Form` to be displayed when the `Form` is run.
    ///
    pub fn add_component(&mut self, component: &'a dyn Component)
        -> Result<(), &'static str>
    {
        component.add_to_parent()?;
        self.components.push(component);
        unsafe { newtFormAddComponent(self.co, component.co()); }
        Ok(())
    }

    ///
    /// Add multiple `Component`s to the `Form`.
    ///
    pub fn add_components<'t>(&mut self, components: &'t [&'a dyn Component])
        -> Result<(), &'static str>
    {
        for component in components.iter() {
            self.add_component(*component)?;
        }
        Ok(())
    }

    ///
    /// Add a `Component` to the `Form`, taking ownership.
    ///
    pub fn take_component<T>(&mut self, component: T)
        -> Result<(), &'static str>
        where T: Component
    {
        component.add_to_parent()?;
        unsafe { newtFormAddComponent(self.co, component.co()); }
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
    /// when the key is pressed. Defaults to `F12`. Use the
    /// [FORM_NOF12][form_nof12] flag to disable the default.
    ///
    /// [form_nof12]: crate::constants::form::FORM_NOF12
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
    pub fn get_current(&self) -> Option<Box<dyn Component>> {
        unsafe {
            let co = newtFormGetCurrent(self.co);
            if co == ptr::null_mut() {
                return None;
            }

            let component = Box::new(BaseComponent {
                co: Cell::new(co),
                added_to_parent: Cell::new(true)
            });

            Some(component)
        }
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
    /// Run the form displaying all added components and accept input from
    /// the user.
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
                                       co: Cell::new(es.u.co),
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

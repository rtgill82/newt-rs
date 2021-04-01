//
// Copyright (C) 2019 Robert Gill <rtgill82@gmail.com>
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

use std::os::unix::io::RawFd;
use crate::component::Component;

///
/// The `Form` exit reason.
/// Returned by [`Form.run()`][form_run].
///
/// [form_run]: ../form/struct.Form.html#method.run
///
#[derive(Debug)]
pub enum ExitReason {
    /// The `Form` exited due to a hot key press. Contains the key pressed.
    HotKey(i32),

    /// The `Form` exited because a `Component` was activated.
    /// Contains the component.
    Component(Box<dyn Component>),

    /// The `Form` exited due to activity on a file descriptor.
    /// Contains the file descriptor.
    FDReady(RawFd),

    /// The `Form` exited because a timer timed out.
    Timer
}

impl PartialEq<i32> for ExitReason {
    fn eq(&self, other: &i32) -> bool {
        if let ExitReason::HotKey(ref hotkey) = self {
            return hotkey == other
        }
        false
    }
}

impl<Rhs: Component> PartialEq<Rhs> for ExitReason {
    fn eq(&self, other: &Rhs) -> bool {
        if let ExitReason::Component(ref component) = self {
            return component.co() == other.co();
        }
        false
    }
}

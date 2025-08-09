//
// Copyright (C) 2025 Robert Gill <rtgill82@gmail.com>
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

//! An enum representing errors raised by `libnewt`.

use std::error;
use std::fmt;
use std::result;

/// Result type for `newt-rs` errors.
pub type Result<T> = result::Result<T, Error>;

/// An enum representing errors raised by `libnewt`.
#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum Error {
    /// Error initializing libnewt.
    Init,
    /// Error adding a component to a [`Form`][crate::widgets::form::Form].
    FormAdd,
    /// Error running [`Form`][crate::widgets::form::Form].
    FormRun,
    /// Error adding a [`Grid`][crate::grid::Grid] to a parent.
    GridAdd,
    /// Error adding an item to a list widget.
    ItemAdd,
    /// Error opening window.
    WindowOpen
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Init => write!(f, "Error initializing libnewt."),
            Error::FormAdd => write!(f, "Component already belongs to a Form."),
            Error::FormRun => write!(f, "Error running Form."),
            Error::GridAdd => write!(f, "Grid already belongs to a parent."),
            Error::ItemAdd => write!(f, "Error adding an item to a list widget."),
            Error::WindowOpen => write!(f, "Error opening window.")
        }
    }
}

impl error::Error for Error { }

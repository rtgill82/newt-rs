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

use newt_sys::*;

#[allow(non_camel_case_types)]
pub type newtExitReason = newtExitStruct__bindgen_ty_1;
pub const NEWT_EXIT_HOTKEY: newtExitReason    = newtExitStruct_NEWT_EXIT_HOTKEY;
pub const NEWT_EXIT_COMPONENT: newtExitReason = newtExitStruct_NEWT_EXIT_COMPONENT;
pub const NEWT_EXIT_FDREADY: newtExitReason   = newtExitStruct_NEWT_EXIT_FDREADY;
pub const NEWT_EXIT_TIMER: newtExitReason     = newtExitStruct_NEWT_EXIT_TIMER;
pub const NEWT_EXIT_ERROR: newtExitReason     = newtExitStruct_NEWT_EXIT_ERROR;

#[allow(non_camel_case_types)]
pub type newtExitStructUnion = newtExitStruct__bindgen_ty_2;

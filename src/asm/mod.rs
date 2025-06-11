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

#![doc(hidden)]
#[cfg(feature = "asm")]
use crate::Component;
#[cfg(feature = "asm")]
use crate::grid::traits::Grid;

mod funcs;

#[cfg(all(feature = "asm", target_arch = "arm"))]
mod arm;
#[cfg(all(feature = "asm", target_arch = "arm"))]
pub use self::arm::*;

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
mod aarch64;
#[cfg(all(feature = "asm", target_arch = "aarch64"))]
pub use self::aarch64::*;

#[cfg(all(feature = "asm", target_arch = "riscv32"))]
mod riscv32;
#[cfg(all(feature = "asm", target_arch = "riscv32"))]
pub use self::riscv32::*;

#[cfg(all(feature = "asm", target_arch = "riscv64"))]
mod riscv64;
#[cfg(all(feature = "asm", target_arch = "riscv64"))]
pub use self::riscv64::*;

#[cfg(all(feature = "asm", target_arch = "x86"))]
mod x86;
#[cfg(all(feature = "asm", target_arch = "x86"))]
pub use self::x86::*;

#[cfg(all(feature = "asm", target_arch = "x86_64"))]
mod x86_64;
#[cfg(all(feature = "asm", target_arch = "x86_64"))]
pub use self::x86_64::*;

#[cfg(feature = "asm")]
pub mod grid;

#[cfg(feature = "asm")]
pub mod windows;

#[cfg(feature = "asm")]
pub trait AsComponent {
    fn as_component(&self) -> Option<&dyn Component>;
}

#[cfg(not(feature = "asm"))]
pub trait AsComponent { }

#[cfg(feature = "asm")]
pub trait AsGrid {
    fn as_grid(&self) -> Option<&dyn Grid>;
}

#[cfg(not(feature = "asm"))]
pub trait AsGrid { }

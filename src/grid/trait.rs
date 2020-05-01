//
// Copyright (C) 2019 Robert Gill <locke@sdf.org>
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
use crate::component::Component;
use crate::widgets::{Form,WidgetFns};
use crate::intern::{AsComponent,Child,ComponentPtr,Nullify,Parent};

pub trait Grid: Component + GridFns { }

impl <T: Grid> WidgetFns for T {
    fn takes_focus(&self, _value: bool) {
        panic!("`Grid` is not a `Widget`");
    }

    fn get_position(&self) -> (i32, i32) {
        panic!("`Grid` is not a `Widget`");
    }

    fn get_size(&self) -> (i32, i32) {
        panic!("`Grid` is not a `Widget`");
    }
}

impl <T: Grid> Nullify for T {
    fn nullify(&self) { }
}

///
/// Implements functions shared by `Grid`s.
///
pub trait GridFns: AsComponent + Child + ComponentPtr + Parent {
    fn add_to_form<'a>(&'a mut self, form: &mut Form<'a>)
      -> Result<(), &'static str> {
        self.add_to_parent()?;
        unsafe { newtGridAddComponentsToForm(self.grid_ptr(), form.co(), 1); }
        form.add_refs(self.children());
        Ok(())
    }

    fn get_size(&self) -> (i32, i32) {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        unsafe {
            newtGridGetSize(self.grid_ptr(), &mut width, &mut height);
        }
        (width, height)
    }

    fn place(&self, left: i32, top: i32) {
        unsafe {
            newtGridPlace(self.grid_ptr(), left, top);
        }
    }
}

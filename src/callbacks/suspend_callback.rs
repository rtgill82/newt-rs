//
// Copyright (C) 2019  Robert Gill <locke@sdf.lonestar.org>
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

use crate::intern::funcs::*;

///
/// A callback called when `Ctrl-Z` is pressed.
///
pub struct SuspendCallback<FN, T>
where FN: FnMut(Option<&T>)
{
    function: FN,
    data: Option<T>
}

impl<FN, T> SuspendCallback<FN, T>
where FN: FnMut(Option<&T>)
{
    ///
    /// Create a new `SuspendCallback` to be called when a suspend (`Ctrl-Z`)
    /// event occurs.
    ///
    /// * `function` - The function or closure to be called when a suspend
    ///                event occurs.
    /// * `data` - The optional user data to pass to the function.
    ///
    pub fn new(function: FN, data: Option<T>)
      -> Box<SuspendCallback<FN, T>> {
        let cb = Box::new(SuspendCallback { function, data });
        newt_set_suspend_callback(cb.as_ref());
        cb
    }

    pub(crate) fn call(&mut self) {
        (self.function)(self.data.as_ref())
    }
}

impl<FN, T> Drop for SuspendCallback<FN, T>
where FN: FnMut(Option<&T>)
{
    fn drop(&mut self) {
        newt_unset_suspend_callback();
    }
}

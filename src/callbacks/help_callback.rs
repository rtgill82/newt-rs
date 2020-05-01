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

use std::os::raw::c_void;
use std::marker::PhantomData;
use std::ptr;

use crate::component::Component;
use crate::widgets::{Form,VerticalScrollbar};
use crate::intern::funcs::*;
use newt_sys::*;

///
/// A callback called when `F1` is pressed while a `Form` is running.
///
/// A new `Form` is initalized along with the callback associating the two
/// together. The [`Form::new_with_help_callback()`][form] could also be used
/// in lieu of `HelpCallback::new()`.
///
/// [form]: ../widgets/form/struct.Form.html#method.new_with_help_callback
///
/// ## Example
/// ```rust no_run
/// extern crate newt;
/// use newt::prelude::*;
///
/// pub fn main() {
///     newt::init().unwrap();
///     newt::cls();
///     newt::centered_window(20, 6, Some("Help Test")).unwrap();
///
///     // Closure that will display a new window when `F1` is pressed.
///     let f = |_form: &Form, data: Option<&&str>| {
///         let string = data.unwrap_or(&"None");
///         let len = string.len();
///
///         let width = (len + 18) as u32;
///         newt::centered_window(width, 5, Some("Help")).unwrap();
///
///         let text = format!("Help Text Data: {}", string);
///         let label = Label::new(1, 1, &text);
///
///         let pos = (width / 2 - 3) as i32;
///         let ok = CompactButton::new(pos, 3, "Ok");
///
///         let mut form = Form::new(None, 0);
///         form.add_component(&label).unwrap();
///         form.add_component(&ok).unwrap();
///         form.run().unwrap();
///         newt::pop_window();
///     };
///
///     // `Form` is allocated with the callback and both are associated.
///     let label = Label::new(1, 1, "Press F1 for help!");
///     let ok = CompactButton::new(7, 4, "Ok");
///
///     let (mut form, _cb) =
///         Form::new_with_help_callback(None, 0, f, Some("This is help text."));
///     form.add_components(&[&label, &ok]).unwrap();
///
///     form.run().unwrap();
///     newt::finished();
/// }
/// ```
pub struct HelpCallback<'a, FN, T>
where FN: FnMut(&Form, Option<&T>)
{
    function: FN,
    data: Option<T>,
    form: PhantomData<Form<'a>>
}

impl<'a, FN, T> HelpCallback<'a, FN, T>
where FN: FnMut(&Form, Option<&T>)
{
    ///
    /// Initialize a new `Form` and associate the function or closure
    /// `function` as a HelpCallback. The callback will be executed
    /// when `F1` is pressed while the `Form` is running.  The new `Form`
    /// and `HelpCallback` are returned as a tuple pair.
    ///
    /// * `scrollbar` - A `VerticalScrollbar` to be attached to the created
    ///                 form.
    /// * `form_flags` - The flags the form is to be initialized with.
    /// * `function` - The function or closure to associate with the `Form`.
    /// * `data` - The optional user data to pass to the function.
    ///
    pub fn new(scrollbar: Option<&VerticalScrollbar>,
               form_flags: i32, function: FN, data: Option<T>)
      -> (Form<'a>, Box<HelpCallback<'a, FN, T>>) {

        let cb = Box::new(HelpCallback { function, data, form: PhantomData });
        newt_init_help_callback(cb.as_ref());

        let scrollbar = if let Some(scrollbar) = scrollbar {
            scrollbar.co()
        } else {
            ptr::null_mut()
        };

        let c_ptr = cb.as_ref() as *const _ as *mut c_void;
        let co = unsafe { newtForm(scrollbar, c_ptr, form_flags) };
        let form = Form::new_co(co);
        (form, cb)
    }

    pub(crate) fn call(&mut self, form: &Form<'a>) {
        (self.function)(form, self.data.as_ref())
    }
}

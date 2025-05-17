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

use crate::component::Component;
use crate::intern::funcs::*;
use newt_sys::*;

///
/// A callback called when a newt `Component` is destroyed.
///
/// ## Example
/// ```rust no_run
/// extern crate newt;
/// use newt::callbacks::DestroyCallback;
/// use newt::prelude::*;
///
/// pub fn main() {
///     newt::init().unwrap();
///     newt::cls();
///     newt::centered_window(20, 6, Some("DestroyCallback Test")).unwrap();
///
///     let cb1 = Checkbox::new(3, 1, "Check 1", None, None);
///     let cb2 = Checkbox::new(3, 2, "Check 2", None, None);
///     let ok = CompactButton::new(7, 4, "Ok");
///
///     let mut value: i32 = 0;
///
///     // Closure `f` borrows `value` as mutable so create a new subscope here
///     // allowing `value` to be borrowed immutably when printing the result later.
///     {
///         // Create closure to be called by DestroyCallback
///         let mut f = |_c: &dyn Component, data: Option<&i32>| {
///             value = *data.unwrap();
///         };
///
///         // Create DestroyCallback with first Ok button using `5` as data.
///         let mut callback = DestroyCallback::new(&ok, Some(5), &mut f);
///         callback.add_component(&cb1, Some(10));
///         callback.add_component(&cb2, Some(15));
///
///         //
///         // Create a subscope so that Form and the components added to it are dropped when it ends.
///         //
///         {
///             let mut form = Form::new(None, 0);
///             form.add_components(&[&cb1, &cb2, &ok]).unwrap();
///             form.run().unwrap();
///             newt::finished();
///         }
///     }
///
///     // `value` should be 5 because `ok` button was the last added to the form
///     println!("value = {}", value);
/// }
///
pub struct DestroyCallback<'a, FN: 'a, T: 'a>
where FN: FnMut(&dyn Component, Option<&T>)
{
    function: FN,
    components: Vec<(&'a dyn Component, Option<T>)>
}

impl<'a, FN: 'a, T: 'a> DestroyCallback<'a, FN, T>
where FN: FnMut(&dyn Component, Option<&T>)
{
    ///
    /// Create a new `DestroyCallback` using the function or closure `function` and
    /// associate it with `component`.
    ///
    /// * `component` - The `Component` associated with the callback.
    /// * `data` - Optional user data to pass to the function.
    /// * `function` - The function or closure to call when the
    ///                `Component` is activated.
    ///
    pub fn new(component: &'a dyn Component, data: Option<T>, function: FN)
      -> Box<DestroyCallback<'a, FN, T>> {

        unsafe {
            let cb = Box::new(DestroyCallback {
                function,
                components: vec![(component, data)]
            });
            newt_set_destroy_callback(component.co(), cb.as_ref());
            cb
        }
    }

    ///
    /// Associate another component with the `DestroyCallback`.
    ///
    /// * `component` - Another `Component` to associate with the callback.
    /// * `data` - Optional user data to pass to the function.
    ///
    pub fn add_component(&mut self, component: &'a dyn Component,
                         data: Option<T>)
    {
        unsafe {
            self.components.push((component, data));
            newt_set_destroy_callback(component.co(), &self);
        }
    }

    pub(crate) fn call(&mut self, co: newtComponent) {
        let function = &mut self.function;
        self.components.retain(|(component, data)| {
            if !component.is_null() && component.co() == co  {
                (*function)(*component, data.as_ref());
                return false;
            }

            true
        });
    }
}

impl<'a, FN: 'a, T: 'a> Drop for DestroyCallback<'a, FN, T>
where FN: FnMut(&dyn Component, Option<&T>)
{
    fn drop(&mut self) {
        unsafe {
            for (component, _data) in self.components.iter() {
                if !component.is_null() {
                    newt_unset_destroy_callback(component.co())
                }
            }
        }
    }
}

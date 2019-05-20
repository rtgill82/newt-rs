use std::os::raw::c_void;
use std::ptr;

use crate::components::Form;
use crate::components::VerticalScrollbar;
use crate::intern::funcs::*;
use newt_sys::*;

///
/// A callback called when `F1` is pressed while a `Form` is running.
///
pub struct HelpCallback<FN, T>
where FN: FnMut(&Form, Option<&T>)
{
    function: FN,
    data: Option<T>
}

impl<FN, T> HelpCallback<FN, T>
where FN: FnMut(&Form, Option<&T>)
{
    ///
    /// Initialize a new `Form` and associate the function or closure
    /// `function` as a HelpCallback. The callback will be executed
    /// when `F1` is pressed while the `Form` is running.  The new `Form`
    /// and `HelpCallback` are returned as a tuple pair.
    ///
    /// * `_scrollbar` - A `VerticalScrollbar` to be attached to the created
    ///                  form _unused_.
    /// * `form_flags` - The flags the form is to be initialized with.
    /// * `function` - The function or closure to associate with the `Form`.
    /// * `data` - The optional user data to pass to the function.
    ///
    pub fn new(_scrollbar: Option<&VerticalScrollbar>,
               form_flags: i32, function: FN, data: Option<T>)
      -> (Form, Box<HelpCallback<FN, T>>) {

        let cb = Box::new(HelpCallback { function, data });
        newt_init_help_callback(cb.as_ref());

        let c_ptr = cb.as_ref() as *const _ as *mut c_void;
        let co = unsafe { newtForm(ptr::null_mut(), c_ptr, form_flags) };
        let form = Form::new_co(co);
        (form, cb)
    }

    pub(crate) fn call(&mut self, form: &Form) {
        (self.function)(form, self.data.as_ref())
    }
}

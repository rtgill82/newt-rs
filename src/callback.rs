use std::os::raw::c_void;
use std::ptr;

use crate::components::Component;
use crate::components::Form;
use crate::components::VerticalScrollbar;
use crate::intern::funcs::*;
use newt_sys::*;

pub struct Callback<'a, FN: 'a, T: 'a>(CallbackType<'a, FN, T>)
where FN: FnMut(Option<&Component>, Option<&T>);

impl<'a, FN: 'a, T: 'a> Callback<'a, FN, T>
where FN: FnMut(Option<&Component>, Option<&T>)
{
    pub(crate) fn assert_help_callback(&self) {
        use CallbackType::Help;

        match self.0 {
            Help { func: _, data: _ } => (),
            _ => panic!("Help callback expected!")
        }
    }
}

#[derive(Clone)]
enum CallbackType<'a, FN: 'a, T: 'a>
where FN: FnMut(Option<&Component>, Option<&T>)
{
    Component {
        func: FN,
        components: Vec<(&'a Component, Option<T>)>
    },

    Help {
        func: FN,
        data: Option<T>
    },

    Suspend{
        func: FN,
        data: Option<T>
    }
}

impl<'a, FN: 'a, T: 'a> Callback<'a, FN, T>
where FN: FnMut(Option<&Component>, Option<&T>)
{
    ///
    /// Create a new `Callback` using the function or closure `function` and
    /// associate it with `component`.
    ///
    /// * `component` - the `Component` to which the callback will be attached
    /// * `function`  - the function or closure to attach to the `Component`
    /// * `data`      - optional data to pass to the function
    ///
    pub fn new(component: &'a Component, function: FN, data: Option<T>)
      -> Box<Callback<'a, FN, T>> {
        use CallbackType::Component;

        let cb = Box::new(Callback(Component{
            func: function,
            components: vec![(component, data)]
        }));
        newt_set_callback(component.co(), cb.as_ref());
        return cb;
    }

    ///
    /// Initialize a new `Form` and associate the function or closure
    /// `function` as a help callback. The callback will be executed
    /// when `F1` is pressed while the `Form` is running.  The new `Form`
    /// and `Callback` are returned as a tuple pair.
    ///
    /// * `_scrollbar` - a `VerticalScrollbar` to be attached to the created
    ///                  form _unused_
    /// * `form_flags` - the flags the form is to be initialized with
    /// * `function`   - the function or closure to attach to the `Form`
    /// * `data`       - optional data to pass to the function
    ///
    pub fn new_help_callback(_scrollbar: Option<&VerticalScrollbar>,
                             form_flags: i32, function: FN, data: Option<T>)
      -> (Form, Box<Callback<'a, FN, T>>) {
        use CallbackType::Help;

        let cb = Box::new(Callback(Help{ func: function, data: data }));
        newt_init_help_callback(cb.as_ref());

        let c_ptr = cb.as_ref() as *const _ as *mut c_void;
        let co = unsafe { newtForm(ptr::null_mut(), c_ptr, form_flags) };
        let form = Form::new_co(co);
        return (form, cb);
    }

    ///
    /// Create a new `Callback` to be called when a suspend (Ctrl-Z) event
    /// occurs.
    ///
    /// * `function` - function or closure to be called when a suspend
    ///                event occurs
    /// * `data`     - optional data to pass to the function
    ///
    pub fn new_suspend_callback(function: FN, data: Option<T>)
      -> Box<Callback<'a, FN, T>> {
        use CallbackType::Suspend;

        let cb = Box::new(Callback(Suspend{ func: function, data: data }));
        newt_set_suspend_callback(cb.as_ref());
        return cb;
    }

    pub fn add_component(&mut self, component: &'a Component, data: Option<T>)
    {
        use CallbackType::Component;

        match &mut self.0 {
            Component { func: _, components } => {
                components.push((component, data));
            },
            _ => panic!("Callback must be a Component callback.")
        }
        newt_set_callback(component.co(), &self);
    }

    pub(crate) fn call(&mut self, co: newtComponent, form: Option<&Component>) {
        use CallbackType::{Component,Help,Suspend};
        match &mut self.0 {
            Component { func, components } => {
                for (component, data) in components.iter() {
                    if component.co() == co {
                        (func)(Some(*component), data.as_ref());
                    }
                }
            },
            Help { func, data } => (func)(form, data.as_ref()),
            Suspend { func, data } => (func)(None, data.as_ref())
        }
    }
}

impl<'a, FN: 'a, T: 'a> Drop for Callback<'a, FN, T>
where FN: FnMut(Option<&Component>, Option<&T>)
{
    fn drop(&mut self) {
        use CallbackType::{Component,Help,Suspend};

        match &self.0 {
            Component { func: _, components } => {
                for (component, _data) in components {
                    newt_unset_callback(*component)
                }
            },
            Help { func: _, data: _ } => (),
            Suspend { func: _, data: _ } => newt_unset_suspend_callback(),
        }
    }
}

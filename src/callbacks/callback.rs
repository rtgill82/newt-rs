use crate::component::Component;
use crate::intern::funcs::*;
use newt_sys::*;

///
/// A callback called when a newt `Component` is activated.
///
/// ## Example
/// ```rust no_run
/// extern crate newt;
/// use newt::Callback;
/// use newt::prelude::*;
///
/// pub fn main() {
///     newt::init().unwrap();
///     newt::cls();
///     newt::centered_window(20, 6, Some("Callback Test")).unwrap();
///
///     let cb1 = Checkbox::new(3, 1, "Check 1", None, None);
///     let cb2 = Checkbox::new(3, 2, "Check 2", None, None);
///     let ok = CompactButton::new(7, 4, "Ok");
///
///     let mut form = Form::new(None, 0);
///     form.add_components(&[&cb1, &cb2, &ok]).unwrap();
///
///     let mut value: i32 = 0;
///     // Closure `f` borrows `value` as mutable so create a new subscope here
///     // allowing `value` to be borrowed immutably when printing the result later.
///     {
///         // Create closure to be called by Callback
///         let mut f = |_c: Option<&dyn Component>, data: Option<&i32>| {
///             value = *data.unwrap();
///         };
///
///         // Create Callback with first Checkbox using `5` as data.
///         let mut callback = Callback::new(&cb1, &mut f, Some(5));
///         // Add second Checkbox using `10` as data.
///         callback.add_component(&cb2, Some(10));
///
///         form.run().unwrap();
///         newt::finished();
///     }
///
///     // `value` will be 0, 5, or 10 depending on the last Checkbox "clicked".
///     println!("value = {}", value);
/// }
/// ```
///
pub struct Callback<'a, FN: 'a, T: 'a>
where FN: FnMut(Option<&dyn Component>, Option<&T>)
{
    function: FN,
    components: Vec<(&'a dyn Component, Option<T>)>
}

impl<'a, FN: 'a, T: 'a> Callback<'a, FN, T>
where FN: FnMut(Option<&dyn Component>, Option<&T>)
{
    ///
    /// Create a new `Callback` using the function or closure `function` and
    /// associate it with `component`.
    ///
    /// * `component` - The `Component` associated with the callback.
    /// * `function` - The function or closure to call when the
    ///                `Component` is activated.
    /// * `data` - Optional user data to pass to the function.
    ///
    pub fn new(component: &'a dyn Component, function: FN, data: Option<T>)
      -> Box<Callback<'a, FN, T>> {

        let cb = Box::new(Callback {
            function,
            components: vec![(component, data)]
        });
        newt_set_callback(component.co(), cb.as_ref());
        cb
    }

    ///
    /// Associate another component with the `Callback`.
    ///
    /// * `component` - Another `Component` to associate with the callback.
    /// * `data` - Optional user data to pass to the function.
    ///
    pub fn add_component(&mut self, component: &'a dyn Component, data: Option<T>)
    {
        self.components.push((component, data));
        newt_set_callback(component.co(), &self);
    }

    pub(crate) fn call(&mut self, co: newtComponent) {
        for (component, data) in self.components.iter() {
            if component.co() == co {
                (self.function)(Some(*component), data.as_ref());
            }
        }
    }
}

impl<'a, FN: 'a, T: 'a> Drop for Callback<'a, FN, T>
where FN: FnMut(Option<&dyn Component>, Option<&T>)
{
    fn drop(&mut self) {
        for (component, _data) in self.components.iter() {
            newt_unset_callback(*component)
        }
    }
}

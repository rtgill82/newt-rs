use crate::intern::funcs::*;

///
/// A callback called when Ctrl-Z is pressed.
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
    /// Create a new `SuspendCallback` to be called when a suspend (Ctrl-Z)
    /// event occurs.
    ///
    /// * `function` - function or closure to be called when a suspend
    ///                event occurs
    /// * `data` - optional user data to pass to the function
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

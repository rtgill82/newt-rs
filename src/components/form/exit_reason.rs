use std::os::unix::io::RawFd;

use crate::components::Component;

///
/// The `Form` exit reason.
/// Returned by [`Form.run()`][form_run].
///
/// [form_run]: ../form/struct.Form.html#method.run
///
#[derive(Debug)]
pub enum ExitReason {
    /// The `Form` exited due to a hot key press. Contains the key pressed.
    HotKey(i32),

    /// The `Form` exited because a `Component` was activated.
    /// Contains the component.
    Component(Box<dyn Component>),

    /// The `Form` exited due to activity on a file descriptor.
    /// Contains the file descriptor.
    FDReady(RawFd),

    /// The `Form` exited because a timer timed out.
    Timer
}

impl PartialEq<i32> for ExitReason {
    fn eq(&self, other: &i32) -> bool {
        if let ExitReason::HotKey(ref hotkey) = self {
            return hotkey == other
        }
        false
    }
}

impl<Rhs: Component> PartialEq<Rhs> for ExitReason {
    fn eq(&self, other: &Rhs) -> bool {
        if let ExitReason::Component(ref component) = self {
            return component.co() == other.co();
        }
        false
    }
}

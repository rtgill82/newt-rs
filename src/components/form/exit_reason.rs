use std::os::unix::io::RawFd;

use crate::components::Component;

#[derive(Debug)]
pub enum ExitReason {
    HotKey(i32),
    Component(Box<dyn Component>),
    FDReady(RawFd),
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

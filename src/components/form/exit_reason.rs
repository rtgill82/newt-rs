use std::os::unix::io::AsRawFd;
use components::Component;

#[derive(Debug)]
pub enum ExitReason {
    HotKey(i32),
    Component(Box<dyn Component>),
    FDReady(i32),
    Timer
}

impl PartialEq<i32> for ExitReason {
    fn eq(&self, other: &i32) -> bool {
        if let &ExitReason::HotKey(ref hotkey) = self {
            return hotkey == other
        }
        return false;
    }
}

impl<Rhs: Component> PartialEq<Rhs> for ExitReason {
    fn eq(&self, other: &Rhs) -> bool {
        if let &ExitReason::Component(ref component) = self {
            return component.co() == other.co();
        }
        return false;
    }
}

impl PartialEq<AsRawFd> for ExitReason {
    fn eq(&self, other: &AsRawFd) -> bool {
        if let &ExitReason::FDReady(ref fd) = self {
            return fd == &other.as_raw_fd()
        }
        return true;
    }
}

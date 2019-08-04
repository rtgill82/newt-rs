//!
//! Deprecated module re-exported for backwards compatibility.
//!
#![deprecated(since = "0.5.4", note = "Please use the `newt::widgets` module instead")]
pub use crate::widgets::WidgetFns as ComponentFuncs;
pub use crate::component::Component;
pub use crate::widgets::*;

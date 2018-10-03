extern crate std;
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::os::raw::c_int;

#[allow(non_camel_case_types)]
pub enum component_enum {}
#[allow(non_camel_case_types)]
pub type c_component = *const component_enum;

pub trait Component {
    fn co(&self) -> c_component;
    fn attach_to_form(&mut self);
    fn attached_to_form(&self) -> bool;

    fn takes_focus(&self, value: bool) {
        #[link(name="newt")]
        extern "C" {
            fn newtComponentTakesFocus(co: c_component, val: c_int);
        }

        unsafe { newtComponentTakesFocus(self.co(), value as c_int); }
    }
}

impl Debug for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Component {{ {:p} }}", self.co())
    }
}

impl<Rhs: Component> PartialEq<Rhs> for Component {
    fn eq(&self, other: &Rhs) -> bool {
        self.co() == other.co()
    }
}

impl<Rhs: Component> PartialEq<Rhs> for Box<Component> {
    fn eq(&self, other: &Rhs) -> bool {
        self.co() == other.co()
    }
}

pub mod form;
pub use self::form::Form;

mod compact_button;
pub use self::compact_button::CompactButton;
mod button;
pub use self::button::Button;
mod checkbox;
pub use self::checkbox::Checkbox;
mod radiobutton;
pub use self::radiobutton::Radiobutton;
mod listitem;
pub use self::listitem::Listitem;
mod label;
pub use self::label::Label;
mod listbox;
pub use self::listbox::Listbox;
mod checkbox_tree;
pub use self::checkbox_tree::CheckboxTree;
mod textbox;
pub use self::textbox::Textbox;
mod entry;
pub use self::entry::Entry;
mod scale;
pub use self::scale::Scale;

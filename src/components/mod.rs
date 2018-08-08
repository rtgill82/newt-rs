extern crate std;
pub enum NewtComponentStruct {}
pub type NewtComponentPtr = *const NewtComponentStruct;

pub trait NewtComponent {
    fn co(&self) -> NewtComponentPtr;
    fn takes_focus(&self, value: bool);
}

impl std::fmt::Debug for NewtComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "NewtComponent {{ {:p} }}", self.co())
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
mod textbox;
pub use self::textbox::Textbox;
mod entry;
pub use self::entry::Entry;
mod scale;
pub use self::scale::Scale;

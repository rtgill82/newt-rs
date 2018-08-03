pub enum NewtComponentStruct {}
pub type NewtComponentPtr = *const NewtComponentStruct;

pub trait NewtComponent {
    fn co(&self) -> NewtComponentPtr;
}

mod compact_button;
pub use self::compact_button::CompactButton;
mod button;
pub use self::button::Button;
mod checkbox;
pub use self::checkbox::Checkbox;
mod radio_button;
pub use self::radio_button::Radiobutton;
mod listitem;
pub use self::listitem::Listitem;
mod label;
pub use self::label::Label;
mod textbox;
pub use self::textbox::Textbox;
mod form;
pub use self::form::Form;

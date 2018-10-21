extern crate newt;
use newt::components::Component;
use newt::components::CompactButton;
use newt::components::Form;
use std::ptr;

#[test]
fn compact_button_create() {
    let button = CompactButton::new(0, 0, "Ok");
    assert!(button.co() != ptr::null_mut());
}

#[test]
fn compact_button_partial_eq_true() {
    let button = CompactButton::new(0, 0, "Ok");
    assert!(button == button);
}

#[test]
fn compact_button_partial_eq_false() {
    let button = CompactButton::new(0, 0, "Ok");
    let form: Form<()> = Form::new(None, None, 0);
    assert!(button != form);
}

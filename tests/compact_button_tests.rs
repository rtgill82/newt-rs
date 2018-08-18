extern crate newt;
use newt::components::CompactButton;
use newt::components::Form;

#[test]
fn compact_button_create() {
    let _button = CompactButton::new(0, 0, "Ok");
}

#[test]
fn compact_button_partial_eq_true() {
    let button = CompactButton::new(0, 0, "Ok");
    assert!(button == button);
}

#[test]
fn compact_button_partial_eq_false() {
    let button = CompactButton::new(0, 0, "Ok");
    let form = Form::new(0);
    assert!(button != form);
}

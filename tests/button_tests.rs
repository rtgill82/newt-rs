extern crate newt;
use newt::components::Component;
use newt::components::Button;
use newt::components::Form;
use std::ptr;

#[test]
fn button_create() {
    let button = Button::new(0, 0, "Ok");
    assert!(button.co() != ptr::null_mut());
}

#[test]
fn button_partial_eq_true() {
    let button = Button::new(0, 0, "Ok");
    assert!(button == button);
}

#[test]
fn button_partial_eq_false() {
    let button = Button::new(0, 0, "Ok");
    let form = Form::new(None, 0);
    assert!(button != form);
}

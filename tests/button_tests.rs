extern crate newt;
use newt::components::Button;
use newt::components::Form;

#[test]
fn button_create() {
    let _button = Button::new(0, 0, "Ok");
}

#[test]
fn button_partial_eq_true() {
    let button = Button::new(0, 0, "Ok");
    assert!(button == button);
}

#[test]
fn button_partial_eq_false() {
    let button = Button::new(0, 0, "Ok");
    let form = Form::new(0);
    assert!(button != form);
}

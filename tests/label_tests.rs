extern crate newt;
use newt::components::Component;
use newt::components::Label;
use newt::components::Form;
use std::ptr;

use newt::constants::COLORSET_LABEL;

#[test]
fn label_create() {
    let label = Label::new(0, 0, "Ok");
    assert!(label.co() != ptr::null());
}

#[test]
fn label_partial_eq_true() {
    let label = Label::new(0, 0, "Ok");
    assert!(label == label);
}

#[test]
fn label_partial_eq_false() {
    let label = Label::new(0, 0, "Ok");
    let form = Form::new(0);
    assert!(label != form);
}

#[test]
fn label_set_text() {
    let mut label = Label::new(0, 0, "Ok");
    label.set_text("Not Ok");
}

#[test]
fn label_set_colors() {
    let mut label = Label::new(0, 0, "Ok");
    label.set_colors(COLORSET_LABEL);
}

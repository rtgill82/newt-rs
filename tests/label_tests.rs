extern crate newt;
use newt::Component;
use newt::widgets::{Form,Label};
use std::ptr;

use newt::constants::COLORSET_LABEL;

#[test]
fn label_create() {
    let label = Label::new(0, 0, "Ok");
    assert!(label.co() != ptr::null_mut());
}

#[test]
fn label_partial_eq_true() {
    let label = Label::new(0, 0, "Ok");
    assert!(label == label);
}

#[test]
fn label_partial_eq_false() {
    let label = Label::new(0, 0, "Ok");
    let form = Form::new(None, 0);
    assert!(label != form);
}

#[test]
fn label_set_text() {
    let label = Label::new(0, 0, "Ok");
    label.set_text("Not Ok");
}

#[test]
fn label_set_colors() {
    let label = Label::new(0, 0, "Ok");
    label.set_colors(COLORSET_LABEL);
}

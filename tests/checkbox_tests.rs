extern crate newt;
use newt::Component;
use newt::widgets::{Checkbox,Form};
use std::ptr;

use newt::constants::FlagsSense;

#[test]
fn checkbox_create() {
    let checkbox = Checkbox::new(0, 0, "Ok", None, None);
    assert!(checkbox.co() != ptr::null_mut());
}

#[test]
fn checkbox_partial_eq_true() {
    let checkbox = Checkbox::new(0, 0, "Ok", None, None);
    assert!(checkbox == checkbox);
}

#[test]
fn checkbox_partial_eq_false() {
    let checkbox = Checkbox::new(0, 0, "Ok", None, None);
    let form = Form::new(None, 0);
    assert!(checkbox != form);
}

#[test]
fn checkbox_get_value() {
    let checkbox = Checkbox::new(0, 0, "Ok", None, None);
    assert!(checkbox.get_value() == ' ');
}

#[test]
fn checkbox_set_value() {
    let checkbox = Checkbox::new(0, 0, "Ok", None, None);
    assert!(checkbox.get_value() == ' ');
    checkbox.set_value('X');
    assert!(checkbox.get_value() == 'X');
}

#[test]
fn checkbox_set_flags() {
    let checkbox = Checkbox::new(0, 0, "Ok", None, None);
    checkbox.set_flags(0, FlagsSense::Reset);
}

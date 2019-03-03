extern crate newt;
use newt::components::Component;
use newt::components::Radiobutton;
use newt::components::Form;
use std::ptr;

#[test]
fn radiobutton_create() {
    let radio = Radiobutton::new(0, 0, "Yes", true, None);
    assert!(radio.co() != ptr::null_mut());
}

#[test]
fn radiobutton_partial_eq_true() {
    let radio = Radiobutton::new(0, 0, "Yes", true, None);
    assert!(radio == radio);
}

#[test]
fn radiobutton_partial_eq_false() {
    let radio = Radiobutton::new(0, 0, "Yes", true, None);
    let form = Form::new(None, 0);
    assert!(radio != form);
}

#[test]
fn radiobutton_get_current() {
    let radio = Radiobutton::new(0, 0, "Yes", true, None);
    assert!(radio.get_current() == radio);
}

#[test]
fn radiobutton_set_current() {
    let mut radio1 = Radiobutton::new(0, 0, "Yes", true, None);
    let mut radio2 = Radiobutton::new(0, 0, "No", false, Some(&mut radio1));
    radio2.set_current();
    assert!(radio1.get_current() == radio2);
    assert!(radio2.get_current() == radio2);
    assert!(radio1.get_current() != radio1);
    assert!(radio2.get_current() != radio1);
}

extern crate newt;
use newt::Component;
use newt::widgets::{Button,Form};
use std::ptr;

use newt::constants::KEY_ENTER;

#[test]
fn form_create() {
    let form = Form::new(None, 0);
    assert!(form.co() != ptr::null_mut());
}

#[test]
fn form_partial_eq_true() {
    let form = Form::new(None, 0);
    assert!(form == form);
}

#[test]
fn form_partial_eq_false() {
    let form1 = Form::new(None, 0);
    let form2 = Form::new(None, 0);
    assert!(form1 != form2);
}

#[test]
fn form_add_component() {
    let button = Button::new(0, 0, "Ok");
    let mut form = Form::new(None, 0);
    form.add_component(&button).unwrap();
}

#[test]
fn form_add_component_x2() {
    let button = Button::new(0, 0, "Ok");
    let mut form = Form::new(None, 0);
    form.add_component(&button).unwrap();
    match form.add_component(&button) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true)
    }
}

#[test]
fn form_add_components() {
    let button1 = Button::new(0, 0, "Ok");
    let button2 = Button::new(0, 0, "Cancel");
    let mut form = Form::new(None, 0);
    form.add_components(&[&button1, &button2]).unwrap();
}

#[test]
fn form_add_components_x2() {
    let button1 = Button::new(0, 0, "Ok");
    let button2 = Button::new(0, 0, "Cancel");

    let mut form = Form::new(None, 0);
    form.add_component(&button2).unwrap();
    match form.add_components(&[&button1, &button2]) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true)
    }
}

#[test]
fn form_set_height() {
    let form = Form::new(None, 0);
    form.set_height(10);
}

#[test]
fn form_set_width() {
    let form = Form::new(None, 0);
    form.set_width(10);
}

#[test]
fn form_add_hot_key() {
    let form = Form::new(None, 0);
    form.add_hot_key(KEY_ENTER);
}

#[test]
fn form_set_timer() {
    let form = Form::new(None, 0);
    form.set_timer(100);
}

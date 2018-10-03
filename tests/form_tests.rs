extern crate newt;
use newt::components::Component;
use newt::components::Button;
use newt::components::Form;
use std::ptr;

use newt::constants::KEY_ENTER;

#[test]
fn form_create() {
    let form = Form::new(0);
    assert!(form.co() != ptr::null());
}

#[test]
fn form_partial_eq_true() {
    let form = Form::new(0);
    assert!(form == form);
}

#[test]
fn form_partial_eq_false() {
    let form1 = Form::new(0);
    let form2 = Form::new(0);
    assert!(form1 != form2);
}

#[test]
fn form_add_component() {
    let mut form = Form::new(0);
    let mut button = Button::new(0, 0, "Ok");
    form.add_component(&mut button).unwrap();
}

#[test]
fn form_add_component_x2() {
    let mut form = Form::new(0);
    let mut button = Button::new(0, 0, "Ok");
    form.add_component(&mut button).unwrap();
    match form.add_component(&mut button) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true)
    }
}

#[test]
fn form_add_components() {
    let mut form = Form::new(0);
    let mut button1 = Button::new(0, 0, "Ok");
    let mut button2 = Button::new(0, 0, "Cancel");
    form.add_components(&mut [&mut button1, &mut button2]).unwrap();
}

#[test]
fn form_add_components_x2() {
    let mut form = Form::new(0);
    let mut button1 = Button::new(0, 0, "Ok");
    let mut button2 = Button::new(0, 0, "Cancel");
    form.add_component(&mut button2).unwrap();
    match form.add_components(&mut [&mut button1, &mut button2]) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true)
    }
}

#[test]
fn form_set_height() {
    let mut form = Form::new(0);
    form.set_height(10);
}

#[test]
fn form_set_width() {
    let mut form = Form::new(0);
    form.set_width(10);
}

#[test]
fn form_add_hot_key() {
    let mut form = Form::new(0);
    form.add_hot_key(KEY_ENTER);
}

#[test]
fn form_set_timer() {
    let mut form = Form::new(0);
    form.set_timer(100);
}

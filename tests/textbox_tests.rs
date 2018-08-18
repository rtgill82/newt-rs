extern crate newt;
use newt::components::Component;
use newt::components::Textbox;
use newt::components::Form;
use std::ptr;

use newt::constants::COLORSET_TEXTBOX;
use newt::constants::COLORSET_ACTTEXTBOX;

#[test]
fn textbox_create() {
    let textbox = Textbox::new(0, 0, 10, 10, 0);
    assert!(textbox.co() != ptr::null());
}

#[test]
fn textbox_create_reflowed() {
    let textbox = Textbox::new_reflowed(0, 0, "Hello world!", 20, 15, 20, 0);
    assert!(textbox.co() != ptr::null());
}

#[test]
fn textbox_partial_eq_true() {
    let textbox = Textbox::new(0, 0, 10, 10, 0);
    assert!(textbox == textbox);
}

#[test]
fn textbox_partial_eq_false() {
    let textbox = Textbox::new(0, 0, 10, 10, 0);
    let form = Form::new(0);
    assert!(textbox != form);
}

#[test]
fn textbox_set_text() {
    let textbox = Textbox::new(0, 0, 10, 10, 0);
    textbox.set_text("Hello world!");
}

#[test]
fn textbox_set_height() {
    let textbox = Textbox::new(0, 0, 10, 10, 0);
    textbox.set_height(20);
}

#[test]
fn textbox_get_num_lines() {
    let textbox = Textbox::new(0, 0, 10, 10, 0);
    textbox.set_text("Hello\nworld!");
    assert!(textbox.get_num_lines() == 2);
}

#[test]
fn textbox_set_colors() {
    let textbox = Textbox::new(0, 0, 10, 10, 0);
    textbox.set_colors(COLORSET_TEXTBOX, COLORSET_ACTTEXTBOX);
}

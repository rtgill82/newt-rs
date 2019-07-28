extern crate newt;
use newt::components::Component;
use newt::components::VerticalScrollbar;
use newt::components::Form;
use std::ptr;

#[test]
fn vertical_scrollbar_create() {
    let scrollbar = VerticalScrollbar::new(0, 0, 10, 0, 0);
    assert!(scrollbar.co() != ptr::null_mut());
}

#[test]
fn vertical_scrollbar_partial_eq_false() {
    let scrollbar = VerticalScrollbar::new(0, 0, 10, 0, 0);
    let form = Form::new(Some(&scrollbar), 0);
    assert!(scrollbar == scrollbar);
    assert!(scrollbar != form);
}

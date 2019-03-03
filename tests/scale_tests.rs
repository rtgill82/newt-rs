extern crate newt;
use newt::components::Component;
use newt::components::Scale;
use newt::components::Form;
use std::ptr;

use newt::constants::COLORSET_EMPTYSCALE;
use newt::constants::COLORSET_FULLSCALE;

#[test]
fn scale_create() {
    let scale = Scale::new(0, 0, 10, 100);
    assert!(scale.co() != ptr::null_mut());
}

#[test]
fn scale_partial_eq_true() {
    let scale = Scale::new(0, 0, 10, 100);
    assert!(scale == scale);
}

#[test]
fn scale_partial_eq_false() {
    let scale = Scale::new(0, 0, 10, 100);
    let form = Form::new(None, 0);
    assert!(scale != form);
}

#[test]
fn scale_set() {
    let mut scale = Scale::new(0, 0, 10, 100);
    scale.set(50);
}

#[test]
fn scale_set_colors() {
    let mut scale = Scale::new(0, 0, 10, 100);
    scale.set_colors(COLORSET_EMPTYSCALE, COLORSET_FULLSCALE);
}

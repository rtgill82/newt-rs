extern crate newt;
use newt::components::Component;
use newt::components::CheckboxTree;
use newt::components::Form;
use std::ptr;

#[test]
fn checkbox_tree_create() {
    let checkbox_tree: CheckboxTree<usize> =
        CheckboxTree::new(0, 0, 10, None, 0);
    assert!(checkbox_tree.co() != ptr::null());
}

#[test]
fn checkbox_tree_partial_eq_true() {
    let checkbox_tree: CheckboxTree<usize> =
        CheckboxTree::new(0, 0, 10, None, 0);
    assert!(checkbox_tree == checkbox_tree);
}

#[test]
fn checkbox_tree_partial_eq_false() {
    let checkbox_tree: CheckboxTree<usize> =
        CheckboxTree::new(0, 0, 10, None, 0);
    let form = Form::new(0);
    assert!(checkbox_tree != form);
}

#[test]
fn checkbox_tree_set_width() {
    let mut checkbox_tree: CheckboxTree<usize> =
        CheckboxTree::new(0, 0, 10, None, 0);
    checkbox_tree.set_width(20);
}

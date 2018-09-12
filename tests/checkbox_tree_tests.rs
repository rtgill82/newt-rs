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

#[test]
fn checkbox_tree_add_item() {
    let mut checkbox_tree: CheckboxTree<usize> =
        CheckboxTree::new(0, 0, 10, None, 0);
    checkbox_tree.add_item("item 1", &5, 0, &[0]);
}

#[test]
fn checkbox_tree_get_current() {
    let mut checkbox_tree: CheckboxTree<usize> =
        CheckboxTree::new(0, 0, 10, None, 0);
    checkbox_tree.add_item("item 1", &5, 0, &[0]);
    assert!(checkbox_tree.get_current() == &5);
}

#[test]
fn checkbox_tree_set_current() {
    let mut checkbox_tree: CheckboxTree<usize> =
        CheckboxTree::new(0, 0, 10, None, 0);
    checkbox_tree.add_item("item 1", &5, 0, &[0]);
    checkbox_tree.add_item("item 2", &10, 0, &[0]);
    checkbox_tree.set_current(&10);
    assert!(checkbox_tree.get_current() == &10);
}

#[test]
fn checkbox_tree_find_item() {
    let mut checkbox_tree: CheckboxTree<usize> =
        CheckboxTree::new(0, 0, 10, None, 0);
    checkbox_tree.add_item("item 1", &5, 0, &[0]);
    checkbox_tree.add_item("item 2", &10, 0, &[1]);
    checkbox_tree.add_item("item 3", &15, 0, &[1, 0]);
    let pos = checkbox_tree.find_item(&15);
    assert!(*pos == [1, 0]);
}

#[test]
fn checkbox_tree_set_entry() {
    let mut checkbox_tree: CheckboxTree<usize> =
        CheckboxTree::new(0, 0, 10, None, 0);
    checkbox_tree.add_item("item 1", &5, 0, &[0]);
    checkbox_tree.set_entry(&5, "new item 1");
}

#[test]
fn checkbox_get_entry_value() {
    let mut checkbox_tree: CheckboxTree<usize> =
        CheckboxTree::new(0, 0, 10, None, 0);
    checkbox_tree.add_item("item 1", &5, 0, &[0]);
    assert!(checkbox_tree.get_entry_value(&5) == ' ');
}

#[test]
fn checkbox_set_entry_value() {
    let mut checkbox_tree: CheckboxTree<usize> =
        CheckboxTree::new(0, 0, 10, None, 0);
    checkbox_tree.add_item("item 1", &5, 0, &[0]);
    checkbox_tree.set_entry_value(&5, '*');
    assert!(checkbox_tree.get_entry_value(&5) == '*');
}

#[test]
fn checkbox_get_selection() {
    let mut checkbox_tree: CheckboxTree<usize> =
        CheckboxTree::new(0, 0, 10, None, 0);
    checkbox_tree.add_item("item 1", &5, 0, &[0]);
    checkbox_tree.add_item("item 2", &10, 0, &[0]);
    checkbox_tree.set_entry_value(&10, '*');
    assert!(*checkbox_tree.get_selection() == [&10]);
}

#[test]
fn checkbox_get_multi_selection() {
    let mut checkbox_tree: CheckboxTree<usize> =
        CheckboxTree::new(0, 0, 10, None, 0);
    checkbox_tree.add_item("item 1", &5, 0, &[0]);
    checkbox_tree.add_item("item 2", &10, 0, &[0]);
    checkbox_tree.set_entry_value(&10, '*');
    assert!(*checkbox_tree.get_multi_selection('*') == [&10]);
}

extern crate newt;
use newt::components::Component;
use newt::components::Form;
use newt::components::Listbox;
use newt::constants::FLAG_MULTIPLE;
use newt::constants::FlagsSense::Set;
use std::ptr;

struct TestStruct<'a> {
    pub v1: usize,
    pub v2: i32,
    pub v3: &'a str
}

#[test]
fn listbox_create() {
    let listbox: Listbox<usize> = Listbox::new(0, 0, 5, 0);
    assert!(listbox.co() != ptr::null());
}

#[test]
fn listbox_partial_eq_true() {
    let listbox: Listbox<usize> = Listbox::new(0, 0, 5, 0);
    assert!(listbox == listbox);
}

#[test]
fn listbox_partial_eq_false() {
    let listbox: Listbox<usize> = Listbox::new(0, 0, 5, 0);
    let form = Form::new(0);
    assert!(listbox != form);
}

#[test]
fn listbox_append_entry() {
    let listbox: Listbox<usize> = Listbox::new(0, 0, 5, 0);
    listbox.append_entry("entry 1", &5);
    assert!(*listbox.get_current() == 5);
}

#[test]
fn listbox_set_current() {
    let listbox: Listbox<usize> = Listbox::new(0, 0, 5, 0);
    listbox.append_entry("entry 1", &5);
    listbox.append_entry("entry 2", &10);
    assert!(*listbox.get_current() == 5);
    listbox.set_current(1);
    assert!(*listbox.get_current() == 10);
}

#[test]
fn listbox_set_current_by_key() {
    let listbox: Listbox<usize> = Listbox::new(0, 0, 5, 0);
    listbox.append_entry("entry 1", &5);
    listbox.append_entry("entry 2", &10);
    assert!(*listbox.get_current() == 5);
    listbox.set_current_by_key(&10);
    assert!(*listbox.get_current() == 10);
}

#[test]
fn listbox_item_count() {
    let listbox: Listbox<usize> = Listbox::new(0, 0, 5, 0);
    listbox.append_entry("entry 1", &5);
    listbox.append_entry("entry 2", &10);
    assert!(listbox.item_count() == 2);
    listbox.append_entry("entry 3", &15);
    assert!(listbox.item_count() == 3);
}

#[test]
fn listbox_delete_entry() {
    let listbox: Listbox<usize> = Listbox::new(0, 0, 5, 0);
    listbox.append_entry("entry 1", &5);
    listbox.append_entry("entry 2", &10);
    listbox.append_entry("entry 3", &15);
    assert!(listbox.item_count() == 3);
    listbox.delete_entry(&10);
    assert!(listbox.item_count() == 2);
}

#[test]
fn listbox_clear() {
    let listbox: Listbox<usize> = Listbox::new(0, 0, 5, 0);
    listbox.append_entry("entry 1", &5);
    listbox.append_entry("entry 2", &10);
    assert!(listbox.item_count() == 2);
    listbox.clear();
    assert!(listbox.item_count() == 0);
}

#[test]
fn listbox_get_entry() {
    let listbox: Listbox<usize> = Listbox::new(0, 0, 5, 0);
    listbox.append_entry("entry 1", &5);
    listbox.append_entry("entry 2", &10);
    let (s, d) = listbox.get_entry(1);
    assert!(s == "entry 2");
    assert!(*d == 10);
}

#[test]
fn listbox_set_entry() {
    let listbox: Listbox<usize> = Listbox::new(0, 0, 5, 0);
    listbox.append_entry("entry 1", &5);
    listbox.append_entry("entry 2", &10);
    listbox.set_entry(1, "entry 3");
    let (s, _d) = listbox.get_entry(1);
    assert!(s == "entry 3");
}

#[test]
fn listbox_set_data() {
    let listbox: Listbox<usize> = Listbox::new(0, 0, 5, 0);
    listbox.append_entry("entry 1", &5);
    let (_s, d) = listbox.get_entry(0);
    assert!(*d == 5);
    listbox.set_data(0, &10);
    let (_s, d) = listbox.get_entry(0);
    assert!(*d == 10);
}

#[test]
fn listbox_get_selection_none() {
    let listbox: Listbox<usize> = Listbox::new(0, 0, 5, FLAG_MULTIPLE);
    listbox.append_entry("entry 1", &5);
    listbox.append_entry("entry 2", &10);
    let result = listbox.get_selection();
    assert!(result.len() == 0);
}

#[test]
fn listbox_select_item() {
    let listbox: Listbox<usize> = Listbox::new(0, 0, 5, FLAG_MULTIPLE);
    listbox.append_entry("entry 1", &5);
    listbox.append_entry("entry 2", &10);
    listbox.select_item(&10, Set);
}

#[test]
fn listbox_get_selection() {
    let listbox: Listbox<usize> = Listbox::new(0, 0, 5, FLAG_MULTIPLE);
    listbox.append_entry("entry 1", &5);
    listbox.append_entry("entry 2", &10);
    listbox.append_entry("entry 3", &15);
    listbox.select_item(&10, Set);
    listbox.select_item(&15, Set);
    let result = listbox.get_selection();
    assert!(result.len() == 2);
    assert!(*result[0] == 10);
    assert!(*result[1] == 15);
}

#[test]
fn listbox_clear_selection() {
    let listbox: Listbox<usize> = Listbox::new(0, 0, 5, FLAG_MULTIPLE);
    listbox.append_entry("entry 1", &5);
    listbox.append_entry("entry 2", &10);
    listbox.append_entry("entry 3", &15);
    listbox.select_item(&10, Set);
    listbox.select_item(&15, Set);
    let result = listbox.get_selection();
    assert!(result.len() == 2);
    listbox.clear_selection();
    let result = listbox.get_selection();
    assert!(result.len() == 0);
}

#[test]
fn listbox_get_selection_struct() {
    let listbox: Listbox<TestStruct> = Listbox::new(0, 0, 5, FLAG_MULTIPLE);
    let st1 = TestStruct { v1: 10, v2: 25, v3: "Foo" };
    let st2 = TestStruct { v1: 11, v2: 26, v3: "Bar" };
    listbox.append_entry("entry1", &st1);
    listbox.append_entry("entry2", &st2);
    listbox.select_item(&st1, Set);
    listbox.select_item(&st2, Set);
    let result = listbox.get_selection();
    assert!(result.len() == 2);
    assert!(result[0].v1 == 10);
    assert!(result[0].v2 == 25);
    assert!(result[0].v3 == "Foo");
    assert!(result[1].v1 == 11);
    assert!(result[1].v2 == 26);
    assert!(result[1].v3 == "Bar");
}

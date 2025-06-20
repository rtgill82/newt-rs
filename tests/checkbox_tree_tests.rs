//
// Copyright (C) 2019 Robert Gill <rtgill82@gmail.com>
//
// This file is a part of newt-rs.
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License version 2.1 as published by the Free Software Foundation.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

extern crate newt;
use std::{i8,i32,isize};
use std::{u8,u32,usize};
use std::ptr;

use newt::Component;
use newt::component::Data;
use newt::widgets::{Button,CheckboxTree};
use newt::constants::ARG_APPEND;

struct TestStruct<'a> {
    pub v1: usize,
    pub v2: i32,
    pub v3: &'a str
}

#[test]
fn checkbox_tree_create() {
    let checkbox_tree: CheckboxTree = CheckboxTree::new(-1, -1, 10, None, 0);
    assert!(checkbox_tree.co() != ptr::null_mut());
}

#[test]
fn checkbox_tree_partial_eq_true() {
    let checkbox_tree: CheckboxTree = CheckboxTree::new(-1, -1, 10, None, 0);
    assert!(checkbox_tree == checkbox_tree);
}

#[test]
fn checkbox_tree_partial_eq_false() {
    let checkbox_tree: CheckboxTree = CheckboxTree::new(-1, -1, 10, None, 0);
    let button = Button::new(-1, -1, "Ok");
    assert!(checkbox_tree != button);
}

#[test]
fn checkbox_tree_set_width() {
    let checkbox_tree: CheckboxTree = CheckboxTree::new(-1, -1, 10, None, 0);
    checkbox_tree.set_width(20);
}

#[test]
fn checkbox_tree_add_item() {
    let checkbox_tree: CheckboxTree = CheckboxTree::new(-1, -1, 10, None, 0);
    checkbox_tree.add_item("item 1", 5, 0, Some(&[ARG_APPEND]));
}

#[test]
#[should_panic(expected = "Indexes must be positive integers.")]
fn checkbox_tree_add_item_with_negative_index() {
    let checkbox_tree: CheckboxTree = CheckboxTree::new(-1, -1, 10, None, 0);
    checkbox_tree.add_item("item 1", 5, 0, Some(&[-100]));
}

#[test]
fn checkbox_tree_get_current() {
    let checkbox_tree: CheckboxTree = CheckboxTree::new(-1, -1, 10, None, 0);
    checkbox_tree.add_item("item 1", 5, 0, None);
    assert!(checkbox_tree.get_current() == Some(5));
}

#[test]
fn checkbox_tree_get_current_no_entries() {
    let checkbox_tree: CheckboxTree = CheckboxTree::new(-1, -1, 10, None, 0);
    assert!(checkbox_tree.get_current() == None);
}

#[test]
fn checkbox_tree_set_current() {
    let checkbox_tree: CheckboxTree = CheckboxTree::new(-1, -1, 10, None, 0);
    checkbox_tree.add_item("item 1", 5, 0, None);
    checkbox_tree.add_item("item 2", 10, 0, None);
    checkbox_tree.set_current(10);
    assert!(checkbox_tree.get_current() == Some(10));
}

#[test]
fn checkbox_tree_find_item() {
    let checkbox_tree: CheckboxTree = CheckboxTree::new(-1, -1, 10, None, 0);
    checkbox_tree.add_item("item 1", 5, 0, Some(&[ARG_APPEND]));
    checkbox_tree.add_item("item 2", 10, 0, Some(&[1]));
    checkbox_tree.add_item("item 3", 15, 0, Some(&[1, ARG_APPEND]));
    let pos = checkbox_tree.find_item(15);
    assert!(*pos == [1, 0]);
}

#[test]
fn checkbox_tree_set_entry() {
    let checkbox_tree: CheckboxTree = CheckboxTree::new(-1, -1, 10, None, 0);
    checkbox_tree.add_item("item 1", 5, 0, None);
    checkbox_tree.set_entry(5, "new item 1");
}

#[test]
fn checkbox_get_entry_value() {
    let checkbox_tree: CheckboxTree = CheckboxTree::new(-1, -1, 10, None, 0);
    checkbox_tree.add_item("item 1", 5, 0, None);
    assert!(checkbox_tree.get_entry_value(5) == ' ');
}

#[test]
fn checkbox_set_entry_value() {
    let checkbox_tree: CheckboxTree = CheckboxTree::new(-1, -1, 10, None, 0);
    checkbox_tree.add_item("item 1", 5, 0, None);
    checkbox_tree.set_entry_value(5, '*');
    assert!(checkbox_tree.get_entry_value(5) == '*');
}

#[test]
fn checkbox_get_selection() {
    let checkbox_tree: CheckboxTree = CheckboxTree::new(-1, -1, 10, None, 0);
    checkbox_tree.add_item("item 1", 5, 0, None);
    checkbox_tree.add_item("item 2", 10, 0, None);
    checkbox_tree.set_entry_value(10, '*');
    assert!(*checkbox_tree.get_selection() == [10]);
}

#[test]
fn checkbox_get_multi_selection() {
    let checkbox_tree: CheckboxTree = CheckboxTree::new(-1, -1, 10, None, 0);
    checkbox_tree.add_item("item 1", 5, 0, None);
    checkbox_tree.add_item("item 2", 10, 0, None);
    checkbox_tree.set_entry_value(10, '*');
    assert!(*checkbox_tree.get_multi_selection('*') == [10]);
}

#[test]
fn checkbox_get_multi_selection_char() {
    let checkbox_tree: CheckboxTree<char> =
        CheckboxTree::new(-1, -1, 10, Some(&[' ', 'X', 'Y']), 0);
    checkbox_tree.add_item("entry1", 'a', 0, None);
    checkbox_tree.add_item("entry2", 'b', 0, None);
    checkbox_tree.add_item("entry3", 'c', 0, None);
    checkbox_tree.set_entry_value('a', 'X');
    checkbox_tree.set_entry_value('b', 'Y');
    checkbox_tree.set_entry_value('c', 'X');
    let result = checkbox_tree.get_multi_selection('X');
    assert!(result.len() == 2);
    assert!(*result == ['a', 'c']);
}

#[test]
fn checkbox_get_multi_selection_i8() {
    let checkbox_tree: CheckboxTree<i8> =
        CheckboxTree::new(-1, -1, 10, Some(&[' ', 'X', 'Y']), 0);
    checkbox_tree.add_item("entry1", i8::MIN, 0, None);
    checkbox_tree.add_item("entry2", 0, 0, None);
    checkbox_tree.add_item("entry3", i8::MAX, 0, None);
    checkbox_tree.set_entry_value(i8::MIN, 'X');
    checkbox_tree.set_entry_value(0, 'Y');
    checkbox_tree.set_entry_value(i8::MAX, 'X');
    let result = checkbox_tree.get_multi_selection('X');
    assert!(result.len() == 2);
    assert!(*result == [i8::MIN, i8::MAX]);
}

#[test]
fn checkbox_get_multi_selection_i32() {
    let checkbox_tree: CheckboxTree<i32> =
        CheckboxTree::new(-1, -1, 10, Some(&[' ', 'X', 'Y']), 0);
    checkbox_tree.add_item("entry1", i32::MIN, 0, None);
    checkbox_tree.add_item("entry2", 0, 0, None);
    checkbox_tree.add_item("entry3", i32::MAX, 0, None);
    checkbox_tree.set_entry_value(i32::MIN, 'X');
    checkbox_tree.set_entry_value(0, 'Y');
    checkbox_tree.set_entry_value(i32::MAX, 'X');
    let result = checkbox_tree.get_multi_selection('X');
    assert!(result.len() == 2);
    assert!(*result == [i32::MIN, i32::MAX]);
}

#[test]
fn checkbox_get_multi_selection_isize() {
    let checkbox_tree: CheckboxTree =
        CheckboxTree::new(-1, -1, 10, Some(&[' ', 'X', 'Y']), 0);
    checkbox_tree.add_item("entry1", isize::MIN, 0, None);
    checkbox_tree.add_item("entry2", 0, 0, None);
    checkbox_tree.add_item("entry3", isize::MAX, 0, None);
    checkbox_tree.set_entry_value(isize::MIN, 'X');
    checkbox_tree.set_entry_value(0, 'Y');
    checkbox_tree.set_entry_value(isize::MAX, 'X');
    let result = checkbox_tree.get_multi_selection('X');
    assert!(result.len() == 2);
    assert!(*result == [isize::MIN, isize::MAX]);
}

#[test]
fn checkbox_get_multi_selection_u8() {
    let checkbox_tree: CheckboxTree<u8> =
        CheckboxTree::new(-1, -1, 10, Some(&[' ', 'X', 'Y']), 0);
    let mid = u8::MAX / 2;
    checkbox_tree.add_item("entry1", u8::MIN, 0, None);
    checkbox_tree.add_item("entry2", mid, 0, None);
    checkbox_tree.add_item("entry3", u8::MAX, 0, None);
    checkbox_tree.set_entry_value(u8::MIN, 'X');
    checkbox_tree.set_entry_value(mid, 'Y');
    checkbox_tree.set_entry_value(u8::MAX, 'X');
    let result = checkbox_tree.get_multi_selection('X');
    assert!(result.len() == 2);
    assert!(*result == [u8::MIN, u8::MAX]);
}

#[test]
fn checkbox_get_multi_selection_u32() {
    let checkbox_tree: CheckboxTree<u32> =
        CheckboxTree::new(-1, -1, 10, Some(&[' ', 'X', 'Y']), 0);
    let mid = u32::MAX / 2;
    checkbox_tree.add_item("entry1", u32::MIN, 0, None);
    checkbox_tree.add_item("entry2", mid, 0, None);
    checkbox_tree.add_item("entry3", u32::MAX, 0, None);
    checkbox_tree.set_entry_value(u32::MIN, 'X');
    checkbox_tree.set_entry_value(mid, 'Y');
    checkbox_tree.set_entry_value(u32::MAX, 'X');
    let result = checkbox_tree.get_multi_selection('X');
    assert!(result.len() == 2);
    assert!(*result == [u32::MIN, u32::MAX]);
}

#[test]
fn checkbox_get_multi_selection_usize() {
    let checkbox_tree: CheckboxTree<usize> =
        CheckboxTree::new(-1, -1, 10, Some(&[' ', 'X', 'Y']), 0);
    let mid = usize::MAX / 2;
    checkbox_tree.add_item("entry1", usize::MIN, 0, None);
    checkbox_tree.add_item("entry2", mid, 0, None);
    checkbox_tree.add_item("entry3", usize::MAX, 0, None);
    checkbox_tree.set_entry_value(usize::MIN, 'X');
    checkbox_tree.set_entry_value(mid, 'Y');
    checkbox_tree.set_entry_value(usize::MAX, 'X');
    let result = checkbox_tree.get_multi_selection('X');
    assert!(result.len() == 2);
    assert!(*result == [usize::MIN, usize::MAX]);
}

#[test]
fn checkbox_tree_get_multi_selection_struct() {
    let st1 = TestStruct { v1: 10, v2: 25, v3: "Foo" };
    let st2 = TestStruct { v1: 11, v2: 26, v3: "Bar" };
    let st3 = TestStruct { v1: 12, v2: 27, v3: "Baz" };
    let checkbox_tree: CheckboxTree<Data<TestStruct>> =
        CheckboxTree::new(-1, -1, 10, Some(&[' ', 'X', 'Y']), 0);

    checkbox_tree.add_item("entry1", Data(&st1), 0, None);
    checkbox_tree.add_item("entry2", Data(&st2), 0, None);
    checkbox_tree.add_item("entry3", Data(&st3), 0, None);
    checkbox_tree.set_entry_value(Data(&st1), 'X');
    checkbox_tree.set_entry_value(Data(&st2), 'Y');
    checkbox_tree.set_entry_value(Data(&st3), 'X');

    let result = checkbox_tree.get_multi_selection('X');
    assert!(result.len() == 2);
    assert!(result[0].v1 == 10);
    assert!(result[0].v2 == 25);
    assert!(result[0].v3 == "Foo");
    assert!(result[1].v1 == 12);
    assert!(result[1].v2 == 27);
    assert!(result[1].v3 == "Baz");
}

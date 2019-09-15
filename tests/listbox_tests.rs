//
// Copyright (C) 2019  Robert Gill <locke@sdf.lonestar.org>
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
use newt::widgets::{Form,Listbox};
use newt::constants::FLAG_MULTIPLE;
use newt::constants::FlagsSense::Set;

struct TestStruct<'a> {
    pub v1: usize,
    pub v2: i32,
    pub v3: &'a str
}

#[test]
fn listbox_create() {
    let listbox: Listbox = Listbox::new(0, 0, 5, 0);
    assert!(listbox.co() != ptr::null_mut());
}

#[test]
fn listbox_partial_eq_true() {
    let listbox: Listbox = Listbox::new(0, 0, 5, 0);
    assert!(listbox == listbox);
}

#[test]
fn listbox_partial_eq_false() {
    let listbox: Listbox = Listbox::new(0, 0, 5, 0);
    let form = Form::new(None, 0);
    assert!(listbox != form);
}

#[test]
fn listbox_append_entry() {
    let listbox: Listbox = Listbox::new(0, 0, 5, 0);
    listbox.append_entry("entry 1", 5).unwrap();
    assert!(listbox.get_current() == Some(5));
}

#[test]
fn listbox_get_current_no_entries() {
    let listbox: Listbox = Listbox::new(0, 0, 5, 0);
    assert!(listbox.get_current() == None);
}

#[test]
fn listbox_set_current() {
    let listbox: Listbox = Listbox::new(0, 0, 5, 0);
    listbox.append_entry("entry 1", 5).unwrap();
    listbox.append_entry("entry 2", 10).unwrap();
    assert!(listbox.get_current() == Some(5));
    listbox.set_current(1);
    assert!(listbox.get_current() == Some(10));
}

#[test]
fn listbox_set_current_by_key() {
    let listbox: Listbox = Listbox::new(0, 0, 5, 0);
    listbox.append_entry("entry 1", 5).unwrap();
    listbox.append_entry("entry 2", 10).unwrap();
    assert!(listbox.get_current() == Some(5));
    listbox.set_current_by_key(10);
    assert!(listbox.get_current() == Some(10));
}

#[test]
fn listbox_item_count() {
    let listbox: Listbox<()> = Listbox::new(0, 0, 5, 0);
    listbox.append_entry("entry 1", ()).unwrap();
    listbox.append_entry("entry 2", ()).unwrap();
    assert!(listbox.item_count() == 2);
    listbox.append_entry("entry 3", ()).unwrap();
    assert!(listbox.item_count() == 3);
}

#[test]
fn listbox_delete_entry() {
    let listbox: Listbox = Listbox::new(0, 0, 5, 0);
    listbox.append_entry("entry 1", 5).unwrap();
    listbox.append_entry("entry 2", 10).unwrap();
    listbox.append_entry("entry 3", 15).unwrap();
    assert!(listbox.item_count() == 3);
    listbox.delete_entry(10);
    assert!(listbox.item_count() == 2);
}

#[test]
fn listbox_clear() {
    let listbox: Listbox<()> = Listbox::new(0, 0, 5, 0);
    listbox.append_entry("entry 1", ()).unwrap();
    listbox.append_entry("entry 2", ()).unwrap();
    assert!(listbox.item_count() == 2);
    listbox.clear();
    assert!(listbox.item_count() == 0);
}

#[test]
fn listbox_get_entry() {
    let listbox: Listbox = Listbox::new(0, 0, 5, 0);
    listbox.append_entry("entry 1", 5).unwrap();
    listbox.append_entry("entry 2", 10).unwrap();
    let (s, d) = listbox.get_entry(1);
    assert!(s == "entry 2");
    assert!(d == 10);
}

#[test]
fn listbox_set_entry() {
    let listbox: Listbox<()> = Listbox::new(0, 0, 5, 0);
    listbox.append_entry("entry 1", ()).unwrap();
    listbox.append_entry("entry 2", ()).unwrap();
    listbox.set_entry(1, "entry 3");
    let (s, _d) = listbox.get_entry(1);
    assert!(s == "entry 3");
}

#[test]
fn listbox_set_data() {
    let listbox: Listbox = Listbox::new(0, 0, 5, 0);
    listbox.append_entry("entry 1", 5).unwrap();
    { let (_s, d) = listbox.get_entry(0); assert!(d == 5); }
    listbox.set_data(0, 10);
    let (_s, d) = listbox.get_entry(0);
    assert!(d == 10);
}

#[test]
fn listbox_get_selection_none() {
    let listbox: Listbox<()> = Listbox::new(0, 0, 5, FLAG_MULTIPLE);
    listbox.append_entry("entry 1", ()).unwrap();
    listbox.append_entry("entry 2", ()).unwrap();
    let result = listbox.get_selection();
    assert!(result.len() == 0);
}

#[test]
fn listbox_select_item() {
    let listbox: Listbox = Listbox::new(0, 0, 5, FLAG_MULTIPLE);
    listbox.append_entry("entry 1", 5).unwrap();
    listbox.append_entry("entry 2", 10).unwrap();
    listbox.select_item(10, Set);
}

#[test]
fn listbox_get_selection() {
    let listbox: Listbox = Listbox::new(0, 0, 5, FLAG_MULTIPLE);
    listbox.append_entry("entry 1", 5).unwrap();
    listbox.append_entry("entry 2", 10).unwrap();
    listbox.append_entry("entry 3", 15).unwrap();
    listbox.select_item(10, Set);
    listbox.select_item(15, Set);
    let result = listbox.get_selection();
    assert!(result.len() == 2);
    assert!(result[0] == 10);
    assert!(result[1] == 15);
}

#[test]
fn listbox_clear_selection() {
    let listbox: Listbox = Listbox::new(0, 0, 5, FLAG_MULTIPLE);
    listbox.append_entry("entry 1", 5).unwrap();
    listbox.append_entry("entry 2", 10).unwrap();
    listbox.append_entry("entry 3", 15).unwrap();
    listbox.select_item(10, Set);
    listbox.select_item(15, Set);
    let result = listbox.get_selection();
    assert!(result.len() == 2);
    listbox.clear_selection();
    let result = listbox.get_selection();
    assert!(result.len() == 0);
}

#[test]
fn listbox_get_selection_char() {
    let listbox: Listbox<char> = Listbox::new(0, 0, 5, FLAG_MULTIPLE);
    listbox.append_entry("entry1", 'a').unwrap();
    listbox.append_entry("entry2", 'b').unwrap();
    listbox.append_entry("entry3", 'c').unwrap();
    listbox.select_item('a', Set);
    listbox.select_item('c', Set);
    let result = listbox.get_selection();
    assert!(result.len() == 2);
    assert!(*result == ['a', 'c']);
}

#[test]
fn listbox_get_selection_i8() {
    let listbox: Listbox<i8> = Listbox::new(0, 0, 5, FLAG_MULTIPLE);
    listbox.append_entry("entry1", i8::MAX).unwrap();
    listbox.append_entry("entry2", 0).unwrap();
    listbox.append_entry("entry3", i8::MIN).unwrap();
    listbox.select_item(i8::MAX, Set);
    listbox.select_item(i8::MIN, Set);
    let result = listbox.get_selection();
    assert!(result.len() == 2);
    assert!(*result == [i8::MAX, i8::MIN]);
}

#[test]
fn listbox_get_selection_i32() {
    let listbox: Listbox<i32> = Listbox::new(0, 0, 5, FLAG_MULTIPLE);
    listbox.append_entry("entry1", i32::MAX).unwrap();
    listbox.append_entry("entry2", 0).unwrap();
    listbox.append_entry("entry3", i32::MIN).unwrap();
    listbox.select_item(i32::MAX, Set);
    listbox.select_item(i32::MIN, Set);
    let result = listbox.get_selection();
    assert!(result.len() == 2);
    assert!(*result == [i32::MAX, i32::MIN]);
}

#[test]
fn listbox_get_selection_isize() {
    let listbox: Listbox = Listbox::new(0, 0, 5, FLAG_MULTIPLE);
    listbox.append_entry("entry1", isize::MAX).unwrap();
    listbox.append_entry("entry2", 0).unwrap();
    listbox.append_entry("entry3", isize::MIN).unwrap();
    listbox.select_item(isize::MAX, Set);
    listbox.select_item(isize::MIN, Set);
    let result = listbox.get_selection();
    assert!(result.len() == 2);
    assert!(*result == [isize::MAX, isize::MIN]);
}

#[test]
fn listbox_get_selection_u8() {
    let listbox: Listbox<u8> = Listbox::new(0, 0, 5, FLAG_MULTIPLE);
    listbox.append_entry("entry1", u8::MAX).unwrap();
    listbox.append_entry("entry2", 0).unwrap();
    listbox.append_entry("entry3", u8::MIN).unwrap();
    listbox.select_item(u8::MAX, Set);
    listbox.select_item(u8::MIN, Set);
    let result = listbox.get_selection();
    assert!(result.len() == 2);
    assert!(*result == [u8::MAX, u8::MIN]);
}

#[test]
fn listbox_get_selection_u32() {
    let listbox: Listbox<u32> = Listbox::new(0, 0, 5, FLAG_MULTIPLE);
    listbox.append_entry("entry1", u32::MAX).unwrap();
    listbox.append_entry("entry2", 0).unwrap();
    listbox.append_entry("entry3", u32::MIN).unwrap();
    listbox.select_item(u32::MAX, Set);
    listbox.select_item(u32::MIN, Set);
    let result = listbox.get_selection();
    assert!(result.len() == 2);
    assert!(*result == [u32::MAX, u32::MIN]);
}

#[test]
fn listbox_get_selection_usize() {
    let listbox: Listbox<usize> = Listbox::new(0, 0, 5, FLAG_MULTIPLE);
    listbox.append_entry("entry1", usize::MAX).unwrap();
    listbox.append_entry("entry2", 0).unwrap();
    listbox.append_entry("entry3", usize::MIN).unwrap();
    listbox.select_item(usize::MAX, Set);
    listbox.select_item(usize::MIN, Set);
    let result = listbox.get_selection();
    assert!(result.len() == 2);
    assert!(*result == [usize::MAX, usize::MIN]);
}

#[test]
fn listbox_get_selection_struct() {
    let st1 = TestStruct { v1: 10, v2: 25, v3: "Foo" };
    let st2 = TestStruct { v1: 11, v2: 26, v3: "Bar" };
    let listbox: Listbox<Data<TestStruct>> =
        Listbox::new(0, 0, 5, FLAG_MULTIPLE);

    listbox.append_entry("entry1", Data(&st1)).unwrap();
    listbox.append_entry("entry2", Data(&st2)).unwrap();
    listbox.select_item(Data(&st1), Set);
    listbox.select_item(Data(&st2), Set);

    let result = listbox.get_selection();
    assert!(result.len() == 2);
    assert!(result[0].v1 == 10);
    assert!(result[0].v2 == 25);
    assert!(result[0].v3 == "Foo");
    assert!(result[1].v1 == 11);
    assert!(result[1].v2 == 26);
    assert!(result[1].v3 == "Bar");
}

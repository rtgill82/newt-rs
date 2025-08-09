//
// Copyright (C) 2025 Robert Gill <rtgill82@gmail.com>
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

use std::cell::Cell;
use std::collections::HashMap;
use std::hash::{Hash,Hasher};
use std::marker::PhantomData;
use std::sync::{Arc,LazyLock,Mutex,Weak};

use std::os::raw::c_void;
use std::{fmt,mem};

use rand::Rng;

use newt::data::Data;
use newt::constants::FlagsSense;
use newt::constants::listbox::LISTBOX_MULTIPLE;
use newt::widgets::{CompactButton,Form,Listbox};

static STRINGS: LazyLock<Mutex<HashMap<usize, OwnedString>>> =
    LazyLock::new(|| { Mutex::new(HashMap::new()) });

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(15, 6, Some("Options")).unwrap();

    let listbox: Listbox<MyString> = Listbox::new(1, 1, 3, LISTBOX_MULTIPLE);
    let ok = CompactButton::new(1, 5, "Ok");
    let clear = CompactButton::new(6, 5, "Clear");

    for x in 1..=3 {
        let s = format!("Entry {}", x);
        let ms = MyString::from(&s);
        listbox.append_entry(&s, ms.clone()).unwrap();

        if x == 2 { listbox.select_item(&ms, FlagsSense::Set); }
    }

    let mut form = Form::new(None, 0);
    form.add_components(&[&listbox, &ok, &clear]).unwrap();

    while form.run().unwrap() == clear { listbox.clear(); }
    newt::finished();

    let current = listbox.get_current().unwrap();
    let selected = listbox.get_selection();
    println!("current = \"{}\"", current);
    println!("selected = {:?}", selected);
}

#[derive(Debug)]
struct MyString<'a> {
    string: Weak<String>,
    id: Id,
    lifetime: PhantomData<&'a ()>
}

#[derive(Debug)]
struct OwnedString {
    string: Arc<String>
}

impl<'a> Eq for MyString<'a> { }

impl<'a> MyString<'a> {
    pub fn from<T>(value: T) -> MyString<'a>
        where T: Into<String>
    {
        let owned = OwnedString {
            string: Arc::new(value.into())
        };

        let this = MyString {
            string: Arc::downgrade(&owned.string),
            id: Id::gen(),
            lifetime: PhantomData
        };

        let mut map = STRINGS.lock().unwrap();
        while map.contains_key(&this.id()) {
            this.id.update();
        }

        map.insert(this.id(), owned);
        this
    }

    pub fn id(&self) -> usize {
        self.id.0.get()
    }
}

impl<'a> Clone for MyString<'a> {
    fn clone(&self) -> Self {
        MyString {
            string: Weak::clone(&self.string),
            id: self.id.clone(),
            lifetime: PhantomData
        }
    }
}

unsafe impl<'a> Data for MyString<'a> {
    fn newt_to_ptr(&self) -> *const c_void {
        self.id() as *const c_void
    }

    fn newt_from_ptr(ptr: *const c_void) -> Self {
        let id = Id::from(ptr);
        let map = STRINGS.lock().unwrap();
        let owned = map.get(&id.0.get())
            .expect("item not found");

        MyString {
            string: Arc::downgrade(&owned.string),
            id: id,
            lifetime: PhantomData
        }
    }
}

impl<'a> fmt::Display for MyString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = self.string.upgrade()
            .expect("Owned value dropped");
        write!(f, "{}", string)
    }
}

impl<'a> Drop for MyString<'a> {
    fn drop(&mut self) {
        let count = Weak::weak_count(&self.string);
        if count == 0 {
            let mut map = STRINGS.lock().unwrap();
            map.remove(&self.id());
        }
    }
}

impl<'a> Hash for MyString<'a> {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.id.hash(state);
    }
}

impl<'a> PartialEq for MyString<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone,Debug)]
struct Id (Cell<usize>);

impl Id {
    pub fn gen() -> Id {
        let value = Self::gen_usize();
        Id (Cell::new(value))
    }

    pub fn update(&self) {
        let value = Self::gen_usize();
        self.0.set(value);
    }

    fn gen_usize() -> usize {
        let mut rng = rand::rng();
         match mem::size_of::<usize>() {
            2 => rng.random::<u16>() as usize,
            4 => rng.random::<u32>() as usize,
            8 => rng.random::<u64>() as usize,
            other => unimplemented!("unsupported pointer size: {}", other)
        }
    }
}

impl Eq for Id { }

impl From<*const c_void> for Id {
    fn from(value: *const c_void) -> Self {
        Id(Cell::new(value as usize))
    }
}

impl Hash for Id {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.0.get().hash(state);
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

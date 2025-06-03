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
use newt::callbacks::DestroyCallback;
use newt::prelude::*;

pub fn main() {
    // Receives the new value when the DestroyCallback is activated.
    let mut value: i32 = 0;

    newt::init().unwrap();
    newt::cls();
    newt::centered_window(20, 6, Some("DestroyCallback Test")).unwrap();

    let cb1 = Checkbox::new(3, 1, "Check 1", None, None);
    let cb2 = Checkbox::new(3, 2, "Check 2", None, None);
    let ok = CompactButton::new(7, 4, "Ok");

    // Closure `f` borrows `value` as mutable so create a new subscope here
    // allowing `value` to be borrowed immutably when printing the result
    // later.
    {
        // Create closure to be called by DestroyCallback
        let mut f = |_c: &dyn Component, data: Option<&i32>| {
            value = *data.unwrap();
        };

        // Create DestroyCallback with first Ok button using `5` as data.
        let mut callback = DestroyCallback::new(&ok, Some(5), &mut f);
        callback.add_component(&cb1, Some(10));
        callback.add_component(&cb2, Some(15));

        // Create another subscope so that Form and the components added to it
        // are dropped when it ends.
        {
            let mut form = Form::new(None, 0);
            form.add_components(&[&cb1, &cb2, &ok]).unwrap();
            form.run().unwrap();
            newt::finished();
        }
    }

    // `value` should be 5 because `ok` button was the last added to the form
    println!("value = {}", value);
}

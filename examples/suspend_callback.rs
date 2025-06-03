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
use newt::callbacks::SuspendCallback;
use newt::prelude::*;

pub fn main() {
    // Receives the new value when the SuspendCallback is activated.
    let mut value: i32 = 0;

    newt::init().unwrap();
    newt::cls();
    newt::centered_window(20, 5, Some("Suspend Callback Test")).unwrap();

    let label = Label::new(4, 1, "Press Ctrl-Z");
    let ok = CompactButton::new(7, 4, "Ok");

    let mut form = Form::new(None, 0);
    form.add_components(&[&label, &ok]).unwrap();

    // Closure `f` borrows `value` as mutable so create a new subscope here
    // allowing `value` to be borrowed immutably when printing the result
    // later.
    {
        // Create closure to be called by SuspendCallback
        let mut f = |data: Option<&i32>| {
            value = *data.unwrap();
        };
        // Create SuspendCallback using `10` as data.
        let _callback = SuspendCallback::new(Some(10), &mut f);

        form.run().unwrap();
        newt::finished();
    }

    // `value` will be `10` if Ctrl-Z was pressed.
    println!("value = {}", value);
}

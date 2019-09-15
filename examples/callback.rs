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
use newt::Callback;
use newt::prelude::*;

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(20, 6, Some("Callback Test")).unwrap();

    let cb1 = Checkbox::new(3, 1, "Check 1", None, None);
    let cb2 = Checkbox::new(3, 2, "Check 2", None, None);
    let ok = CompactButton::new(7, 4, "Ok");

    let mut form = Form::new(None, 0);
    form.add_components(&[&cb1, &cb2, &ok]).unwrap();

    let mut value: i32 = 0;
    // Closure `f` borrows `value` as mutable so create a new subscope here
    // allowing `value` to be borrowed immutably when printing the result later.
    {
        // Create closure to be called by Callback
        let mut f = |_c: &dyn Component, data: Option<&i32>| {
            value = *data.unwrap();
        };

        // Create Callback with first Checkbox using `5` as data.
        let mut callback = Callback::new(&cb1, &mut f, Some(5));
        // Add second Checkbox using `10` as data.
        callback.add_component(&cb2, Some(10));

        form.run().unwrap();
        newt::finished();
    }

    // `value` will be 0, 5, or 10 depending on the last Checkbox "clicked".
    println!("value = {}", value);
}

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
use newt::prelude::*;

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(20, 5, Some("Greetings")).unwrap();

    let text = Textbox::new(4, 1, 12, 1, 0);
    text.set_text("Hello World!");
    let ok = CompactButton::new(7, 3, "Ok");

    let mut form = Form::new(None, 0);
    form.add_components(&[&text, &ok]).unwrap();
    let reason = form.run().unwrap();
    newt::finished();

    match reason {
        ExitReason::HotKey(key) => // F12 is the default HotKey
            println!("Execution stopped due to HotKey: {}", key),
        ExitReason::Component(co) =>
            println!("Execution stopped due to Component: {:?}", co),
        _ =>
            println!("Execution stopped due to other reason...")
    }
}

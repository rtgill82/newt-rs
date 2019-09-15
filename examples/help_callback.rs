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
use newt::prelude::*;

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(20, 6, Some("Help Test")).unwrap();

    let f = |_form: &Form, data: Option<&&str>| {
        let string = data.unwrap_or(&"None");
        let len = string.len();

        let width = (len + 18) as u32;
        newt::centered_window(width, 5, Some("Help")).unwrap();

        let text = format!("Help Text Data: {}", string);
        let label = Label::new(1, 1, &text);

        let pos = (width / 2 - 3) as i32;
        let ok = CompactButton::new(pos, 3, "Ok");

        let mut form = Form::new(None, 0);
        form.add_component(&label).unwrap();
        form.add_component(&ok).unwrap();
        form.run().unwrap();

        newt::pop_window();
    };

    let label = Label::new(1, 1, "Press F1 for help!");
    let ok = CompactButton::new(7, 4, "Ok");

    let (mut form, _cb) =
        Form::new_with_help_callback(None, 0, f, Some("This is help text."));
    form.add_components(&[&label, &ok]).unwrap();

    form.run().unwrap();
    newt::finished();
}

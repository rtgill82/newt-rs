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

extern crate chrono;
use chrono::prelude::*;
use newt::prelude::*;

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(26, 7, Some("Timer Exit Test")).unwrap();

    let l1 = Label::new(1,  1, "Timer: 0ms");
    let l2 = Label::new(1,  2, "Last Event: None");
    let l3 = Label::new(1,  3, "Timer Value:");
    let e  = Entry::new(14, 3, Some("0"), 8, 0);

    let b1 = CompactButton::new(2, 5, "Set");
    let b2 = CompactButton::new(8, 5, "Unset");
    let b3 = CompactButton::new(16, 5, "Exit");

    let components: &[&dyn Component] = &[&l1, &l2, &l3, &e, &b1, &b2, &b3];
    let mut form = Form::new(None, 0);
    form.add_components(components).unwrap();

    loop {
        let r = form.run().unwrap();
        match r {
            ExitReason::Component(co) => {
                if co == b1 {
                    let t: i32 = e.get_text().parse().unwrap();
                    l1.set_text(&format!("Timer: {}ms", t));
                    newt::refresh();
                    form.set_timer(t);
                };
                if co == b2 {
                    l1.set_text("Timer: 0ms");
                    newt::refresh();
                    form.set_timer(0);
                };
                if co == b3 { break; };
            },

            ExitReason::Timer => {
                let now = Local::now();
                let timestr = now.format("%H:%M:%S%.3f").to_string();
                let msg = format!("Last Event: {}", timestr);
                l2.set_text(&msg);
            },

            _ => ()
        }
    }
    newt::finished();
}

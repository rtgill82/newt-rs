extern crate chrono;

use chrono::prelude::*;
use newt::components::CompactButton;
use newt::components::Entry;
use newt::components::Form;
use newt::components::Label;
use newt::components::form::ExitReason::*;

pub fn main() {
    newt::init();
    newt::cls();
    newt::centered_window(26, 7, Some("Timer Exit Test"));

    let mut l1 = Label::new(1,  1, "Timer: 0ms");
    let mut l2 = Label::new(1,  2, "Last Event: None");
    let mut l3 = Label::new(1,  3, "Timer Value:");
    let mut e  = Entry::new(14, 3, Some("0"), 8, 0);

    let mut b1 = CompactButton::new(2, 5, "Set");
    let mut b2 = CompactButton::new(8, 5, "Unset");
    let mut b3 = CompactButton::new(16, 5, "Exit");

    let mut form = Form::new(None, 0);
    form.add_components(&mut [&mut l1, &mut l2, &mut l3, &mut e,
                              &mut b1, &mut b2, &mut b3])
        .unwrap();

    loop {
        let r = form.run().unwrap();
        match r {
            Component(co) => {
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

            Timer => {
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

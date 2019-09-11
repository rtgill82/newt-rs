extern crate newt;
use newt::callbacks::SuspendCallback;
use newt::prelude::*;

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(20, 5, Some("Suspend Callback Test")).unwrap();

    let label = Label::new(4, 1, "Press Ctrl-Z");
    let ok = CompactButton::new(7, 4, "Ok");

    let mut form = Form::new(None, 0);
    form.add_components(&[&label, &ok]).unwrap();

    let mut value: i32 = 0;
    {
        let mut f = |data: Option<&i32>| {
            value = *data.unwrap();
        };
        let _callback = SuspendCallback::new(&mut f, Some(10));

        form.run().unwrap();
        newt::finished();
    }

    println!("value = {}", value);
}

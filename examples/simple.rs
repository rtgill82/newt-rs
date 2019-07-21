extern crate newt;
use newt::prelude::*;

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(20, 5, Some("Greetings")).unwrap();

    let mut form = Form::new(None, 0);
    let mut text = Textbox::new(4, 1, 12, 1, 0);
    let mut ok = CompactButton::new(7, 3, "Ok");

    text.set_text("Hello World!");
    form.add_components(&mut [&mut text, &mut ok]).unwrap();
    let reason = form.run().unwrap();
    newt::finished();

    assert!(ok == reason);
    assert!(reason == ok);
}

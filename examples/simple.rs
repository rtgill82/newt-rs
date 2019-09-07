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

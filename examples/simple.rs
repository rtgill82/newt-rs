extern crate newt;
use newt::components::CompactButton;
use newt::components::Form;
use newt::components::Textbox;
use newt::components::form::ExitReason;

pub fn main() {
    newt::init();
    newt::cls();
    newt::centered_window(20, 5, Some("Greetings"));

    let mut form = Form::new(None, 0);
    let mut text = Textbox::new(4, 1, 12, 1, 0);
    let mut ok = CompactButton::new(7, 3, "Ok");

    text.set_text("Hello World!");
    form.add_components(&mut [&mut text, &mut ok]).unwrap();
    let reason: ExitReason = form.run().unwrap();
    newt::finished();

    assert!(ok == reason);
    assert!(reason == ok);
}

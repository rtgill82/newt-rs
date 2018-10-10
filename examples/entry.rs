extern crate newt;
use newt::components::CompactButton;
use newt::components::Form;
use newt::components::Entry;
use newt::components::Label;

pub fn main() {
    newt::init();
    newt::cls();
    newt::centered_window(20, 5, None);

    let mut form = Form::new(0);
    let mut label = Label::new(1, 1, "Entry:");
    let mut entry = Entry::new(8, 1, None, 10, 0);
    let mut ok = CompactButton::new(7, 3, "Ok");

    form.add_components(&mut [&mut label, &mut entry, &mut ok]).unwrap();
    form.run().unwrap();
    newt::finished();

    println!("Entered: {}", entry.get_text());
}

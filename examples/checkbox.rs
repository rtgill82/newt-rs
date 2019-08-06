extern crate newt;
use newt::prelude::*;

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(20, 6, Some("Options")).unwrap();

    let cb1 = Checkbox::new(4, 1, "Option 1", None, None);
    let cb2 = Checkbox::new(4, 2, "Option 2", Some('X'), Some(&[' ', 'X', '*']));
    let ok = CompactButton::new(7, 4, "Ok");

    let mut form = Form::new(None, 0);
    form.add_components(&[&cb1, &cb2, &ok]).unwrap();
    form.run().unwrap();
    newt::finished();

    println!("Option 1: {}", cb1.get_value());
    println!("Option 2: {}", cb2.get_value());
}

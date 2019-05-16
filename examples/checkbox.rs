extern crate newt;
use newt::components::Checkbox;
use newt::components::CompactButton;
use newt::components::Form;

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(20, 6, Some("Options")).unwrap();

    let mut form = Form::new(None, 0);
    let mut cb1 = Checkbox::new(4, 1, "Option 1", None, None);
    let mut cb2 = Checkbox::new(4, 2, "Option 2", Some('X'), Some(&[' ', 'X', '*']));
    let mut ok = CompactButton::new(7, 4, "Ok");

    form.add_components(&mut [&mut cb1, &mut cb2, &mut ok]).unwrap();
    form.run().unwrap();
    newt::finished();

    println!("Option 1: {}", cb1.get_value());
    println!("Option 2: {}", cb2.get_value());
}

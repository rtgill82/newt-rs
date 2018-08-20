extern crate newt;
use newt::components::Checkbox;
use newt::components::CompactButton;
use newt::components::Form;

pub fn main() {
   newt::init();
   newt::cls();
   newt::centered_window(20, 6, "Options");

   let mut form = Form::new(0);
   let mut cb1 = Checkbox::new(4, 1, "Option 1", None, None);
   let mut cb2 = Checkbox::new(4, 2, "Option 2", Some('X'), Some(&[' ', 'X', '*']));
   let mut ok = CompactButton::new(7, 4, "Ok");

   form.add_components(&mut [&mut cb1, &mut cb2, &mut ok]).unwrap();
   form.run().unwrap();
   newt::finished();
}

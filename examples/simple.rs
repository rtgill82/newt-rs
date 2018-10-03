extern crate newt;
use newt::components::Form;
use newt::components::Label;

pub fn main() {
   newt::init();
   newt::cls();
   newt::centered_window(40, 15, "Hello World!");
   let label = Label::new(0, 0, "Label");
   let form = Form::new(0);
   let bval = label == form;
   newt::finished();
   println!("bval = {}", bval);
}

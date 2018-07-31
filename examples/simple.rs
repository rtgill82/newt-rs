extern crate newt;

pub fn main() {
   newt::init();
   newt::cls();
   newt::centered_window(40, 15, "Hello World!");
   let label = newt::Label::new(0, 0, "Label");
   let form = newt::Form::new(0);
   let bval = label == form;
   newt::finished();
   println!("bval = {}", bval);
}

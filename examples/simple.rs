extern crate newt;
use std::process::exit;
use newt::components::CompactButton;
use newt::components::Form;
use newt::components::Textbox;
use newt::components::form::ExitReason;

pub fn main() {
   newt::init();
   newt::cls();
   newt::centered_window(26, 5, "Greetings");

   let form = Form::new(0);
   let mut text = Textbox::new(7, 1, 12, 1, 0);
   let mut ok = CompactButton::new(3, 3, "Ok");
   let mut cancel = CompactButton::new(13, 3, "Cancel");

   text.set_text("Hello World!");
   form.add_component(&mut text).unwrap();
   form.add_component(&mut ok).unwrap();
   form.add_component(&mut cancel).unwrap();

   let reason: ExitReason = form.run().unwrap();
   newt::finished();

   if let ExitReason::Component(component) = reason {
       exit(if component == ok { 0 } else { 1 });
   }
}

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
   let text = Textbox::new(7, 1, 12, 1, 0);
   let ok = CompactButton::new(3, 3, "Ok");
   let cancel = CompactButton::new(13, 3, "Cancel");

   text.set_text("Hello World!");
   form.add_component(&text);
   form.add_component(&ok);
   form.add_component(&cancel);

   let reason: ExitReason = form.run().unwrap();
   newt::finished();

   if let ExitReason::Component(component) = reason {
       exit(if component == ok { 0 } else { 1 });
   }
}

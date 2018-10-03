extern crate newt;
use newt::components::CompactButton;
use newt::components::Form;
use newt::components::Textbox;
use newt::components::form::Result;

pub fn main() {
   newt::init();
   newt::cls();
   newt::centered_window(26, 5, "Hello World!");
   let form = Form::new(0);
   let text = Textbox::new(1, 1, 25, 1, 0);
   let button = CompactButton::new(10, 3, "Ok");
   text.set_text("Press <Ok> to continue.");
   form.add_component(&text);
   form.add_component(&button);

   let result: Result = form.run();
   newt::finished();

   println!("result = {:?}", result);
}

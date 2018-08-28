extern crate newt;
use newt::components::Listbox;
use newt::components::CompactButton;
use newt::components::Form;

pub fn main() {
   newt::init();
   newt::cls();
   newt::centered_window(15, 6, Some("Options"));

   let mut form = Form::new(0);
   let mut listbox: Listbox<usize> = Listbox::new(1, 1, 3, 0);
   let mut ok = CompactButton::new(5, 5, "Ok");

   listbox.append_entry("Entry 1", &5);
   listbox.append_entry("Entry 2", &10);
   listbox.append_entry("Entry 3", &15);

   form.add_components(&mut [&mut listbox, &mut ok]).unwrap();
   form.run().unwrap();

   newt::finished();

   let result = listbox.get_current();
   println!("result = {}", result);
}

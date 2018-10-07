extern crate newt;
use newt::components::Listbox;
use newt::components::CompactButton;
use newt::components::Form;
use newt::constants::FLAG_MULTIPLE;

pub fn main() {
   newt::init();
   newt::cls();
   newt::centered_window(15, 6, Some("Options"));

   let mut form = Form::new(0);
   let mut listbox: Listbox<isize> = Listbox::new(1, 1, 3, FLAG_MULTIPLE);
   let mut ok = CompactButton::new(1, 5, "Ok");
   let mut clear = CompactButton::new(6, 5, "Clear");

   let data: Vec<isize> = (1..10).collect();
   for i in &data {
       let text = format!("Entry {}", i);
       listbox.append_entry(&text, i);
   }

   form.add_components(&mut [&mut listbox, &mut ok, &mut clear]).unwrap();

   while form.run().unwrap() == clear {
       listbox.clear();
   }

   newt::finished();

   let current = listbox.get_current();
   let selected = listbox.get_selection();
   println!("current = {:?}", current);
   println!("selected = {:?}", selected);
}

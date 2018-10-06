extern crate newt;
use newt::components::CheckboxTree;
use newt::components::CompactButton;
use newt::components::Form;
use newt::constants::ARG_APPEND;

pub fn main() {
   newt::init();
   newt::cls();
   newt::centered_window(20, 9, Some("Options"));

   let mut form = Form::new(0);
   let mut tree: CheckboxTree<usize> = CheckboxTree::new(0, 0, 7, Some(&[' ', 'A', 'B']), 0);
   let mut ok = CompactButton::new(7, 8, "Ok");

   tree.add_item("Tree 1", &1, 0, &[0]);
   tree.add_item("Option 1", &2, 0, &[0, ARG_APPEND]);
   tree.add_item("Option 2", &3, 0, &[0, ARG_APPEND]);
   tree.add_item("Tree 2", &4, 0, &[1]);
   tree.add_item("Option 3", &5, 0, &[1, ARG_APPEND]);
   tree.add_item("Option 4", &6, 0, &[1, ARG_APPEND]);
   tree.add_item("Option 5", &7, 0, &[1, ARG_APPEND]);

   form.add_components(&mut [&mut tree, &mut ok]).unwrap();
   form.run().unwrap();
   newt::finished();

   let selection = tree.get_selection();
   println!("selection: {:?}", selection);
   for i in selection.iter() {
       println!("{} is set to {}", i, tree.get_entry_value(&i));
   }
}

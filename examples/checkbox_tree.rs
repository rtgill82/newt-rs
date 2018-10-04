extern crate newt;
use newt::components::CheckboxTree;
use newt::components::CompactButton;
use newt::components::Form;

pub fn main() {
   newt::init();
   newt::cls();
   newt::centered_window(20, 10, Some("Options"));

   let mut form = Form::new(0);
   let mut tree: CheckboxTree<usize> = CheckboxTree::new(0, 0, 5, None, 0);
   let mut ok = CompactButton::new(7, 9, "Ok");

   tree.add_item("Tree 1", &5, 0, &[0]);
   tree.add_item("Option 1", &10, 0, &[0, 1]);
   tree.add_item("Tree 2", &15, 0, &[1]);
   tree.add_item("Option 2", &20, 0, &[1, 1]);

   form.add_components(&mut [&mut tree, &mut ok]).unwrap();
   form.run().unwrap();
   newt::finished();

   println!("selection: {:?}", tree.get_selection());
}

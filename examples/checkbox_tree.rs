extern crate newt;
use newt::prelude::*;

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(20, 9, Some("Options")).unwrap();

    let mut form = Form::new(None, 0);
    let mut tree: CheckboxTree =
        CheckboxTree::new(0, 0, 7, Some(&[' ', 'A', 'B']), 0);
    let mut ok = CompactButton::new(7, 8, "Ok");

    tree.add_item("Tree 1", 1, 0, Some(&[0]));
    tree.add_item("Option 1", 2, 0, Some(&[0, ARG_APPEND]));
    tree.add_item("Option 2", 3, 0, Some(&[0, ARG_APPEND]));
    tree.add_item("Tree 2", 4, 0, Some(&[1]));
    tree.add_item("Option 3", 5, 0, Some(&[1, ARG_APPEND]));
    tree.add_item("Option 4", 6, 0, Some(&[1, ARG_APPEND]));
    tree.add_item("Option 5", 7, 0, Some(&[1, ARG_APPEND]));

    form.add_components(&mut [&mut tree, &mut ok]).unwrap();
    form.run().unwrap();
    newt::finished();

    let selection = tree.get_selection();
    println!("selection: {:?}", selection);
    for i in selection.iter() {
        println!("{} is set to {}", i, tree.get_entry_value(*i));
    }
}

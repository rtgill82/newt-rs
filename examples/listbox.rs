extern crate newt;
use newt::prelude::*;

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(15, 6, Some("Options")).unwrap();

    let mut listbox: Listbox = Listbox::new(1, 1, 3, FLAG_MULTIPLE);
    let mut ok = CompactButton::new(1, 5, "Ok");
    let mut clear = CompactButton::new(6, 5, "Clear");

    for i in 1..10 {
        let text = format!("Entry {}", i);
        listbox.append_entry(&text, i).unwrap();
    }

    let mut form = Form::new(None, 0);
    form.add_components(&mut [&mut listbox, &mut ok, &mut clear]).unwrap();

    while form.run().unwrap() == clear { listbox.clear(); }
    newt::finished();

    let current = listbox.get_current();
    let selected = listbox.get_selection();
    println!("current = {:?}", current);
    println!("selected = {:?}", selected);
}

extern crate newt;
use newt::callbacks::EntryFilter;
use newt::prelude::*;

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(22, 5, None).unwrap();

    let mut g_ch: char = '\0';
    let mut g_cursor: i32 = 0;
    let mut g_data: i32 = 0;

    let mut f = |_e: &Entry, data: Option<&i32>, ch: char, cursor: i32| {
        g_data = *data.unwrap();
        g_ch = ch;
        g_cursor = cursor;
        return ch;
    };

    let l1 = Label::new(1, 1, "Entry 1:");
    let l2 = Label::new(1, 2, "Entry 2:");
    let e1 = Entry::new(10, 1, None, 10, 0);
    let e2 = Entry::new(10, 2, None, 10, 0);
    let ok = CompactButton::new(7, 4, "Ok");

    let components: &[&dyn Component] = &[&l1, &l2, &e1, &e2, &ok];
    let mut form = Form::new(None, 0);
    form.add_components(components).unwrap();

    let mut filter = EntryFilter::new(&e1, &mut f, Some(5));
    filter.add_entry(&e2, Some(10));

    form.run().unwrap();
    newt::finished();

    println!("Entry 1: {}", e1.get_text());
    println!("Entry 2: {}", e2.get_text());

    println!("ch = {}", g_ch);
    println!("cursor = {}", g_cursor);
    println!("data = {}", g_data);
}

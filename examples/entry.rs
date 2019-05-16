extern crate newt;
use newt::components::CompactButton;
use newt::components::Entry;
use newt::components::Form;
use newt::components::Label;
use newt::callbacks::EntryFilter;

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(22, 5, None).unwrap();

    let mut g_ch: char = '\0';
    let mut g_cursor: i32 = 0;
    let mut value: i32 = 0;
    {
        let mut f = |_e: &Entry, data: Option<&i32>, ch: char, cursor: i32| {
            value = *data.unwrap();
            g_ch = ch;
            g_cursor = cursor;
            return ch;
        };

        let mut form = Form::new(None, 0);
        let mut l1 = Label::new(1, 1, "Entry 1:");
        let mut l2 = Label::new(1, 2, "Entry 2:");
        let mut e1 = Entry::new(10, 1, None, 10, 0);
        let mut e2 = Entry::new(10, 2, None, 10, 0);
        let mut ok = CompactButton::new(7, 4, "Ok");

        form.add_components(&mut [&mut l1, &mut l2, &mut e1, &mut e2, &mut ok])
            .unwrap();

        let mut filter = EntryFilter::new(&e1, &mut f, Some(5));
        filter.add_entry(&e2, Some(10));

        form.run().unwrap();
        newt::finished();

        println!("Entry 1: {}", e1.get_text());
        println!("Entry 2: {}", e2.get_text());
    }

    println!("ch = {}", g_ch);
    println!("cursor = {}", g_cursor);
    println!("value = {}", value);
}

extern crate newt;
use newt::windows::*;

#[cfg(not(feature = "asm"))]
fn asm_functions() { }

#[cfg(feature = "asm")]
fn asm_functions() {
    let (rv, item) = win_menu("Test Menu", "Select an item", 50, 5, 5, 3,
                              &["Item1", "Item2"], &["Ok", "Cancel"]);

    win_message("Selection", "Ok",
                &format!("Button selected: {}, Item selected: {}", rv, item));

    let mut entries = Vec::new();
    for n in 1..3 {
        let entry: WinEntry = WinEntry::new(
            &format!("Entry {}", n),
            &format!("Value {}", n),
            0);
        entries.push(entry);
    }

    let rv = win_entries("Test Entries", "Enter some text", 50, 5, 5, 10,
                         entries.as_mut_slice(), &["Ok", "Cancel"]);

    let mut message = String::new();
    for (i, entry) in entries.iter().enumerate() {
        message.push_str(&format!("Entry {}: {}, ", i + 1, entry.value()));
    }

    message.push_str(&format!("Button selected: {}", rv));
    win_message("Entries", "Ok", &message);
}

pub fn main() {
    newt::init().unwrap();
    newt::cls();

    let msg = format!("This message contains a C format string: {}", "%-5d");
    win_message("Safe formatting?", "Ok", &msg);

    let msg = format!("This message also contains a C format string: {}",
                      "%.3f");
    let msg = format!("{}. Is it displayed correctly? Did it crash?", msg);
    let rv = win_choice("Safe formatting?", "Yes", "No", &msg);
    win_message("Selection", "Ok", &format!("Button Selected: {}", rv));


    let msg = "%-5d %.3f %02x %-5s";
    let rv = win_ternary("Ditto", "Yes", "No", "Maybe, so?", &msg);
    win_message("Selection", "Ok", &format!("Button Selected: {}", rv));

    asm_functions();
    newt::finished();
}

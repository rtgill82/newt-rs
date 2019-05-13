extern crate newt;
use newt::windows::*;

#[cfg(not(feature = "asm"))]
fn asm_functions() { }

#[cfg(feature = "asm")]
fn asm_functions() {
    let (rv, item) = win_menu("Test Menu", "Message", 50, 5, 5, 3,
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

    let rv = win_entries("Test Entries", "Message", 50, 5, 5, 10,
                         entries.as_mut_slice(), &["Ok", "Cancel"]);

    let mut message = String::new();
    for (i, entry) in entries.iter().enumerate() {
        message.push_str(&format!("Entry {}: {}, ", i + 1, entry.value()));
    }

    message.push_str(&format!("Button selected: {}", rv));
    win_message("Entries", "Ok", &message);
}

pub fn main() {
    newt::init();
    newt::cls();

    win_message("Message Window", "Ok", "This is a message window.");

    let rv = win_choice("Choice Window", "Ok", "Cancel",
                        "Choose 'Ok' or 'Cancel'");
    win_message("Selection", "Ok", &format!("Button Selected: {}", rv));

    let rv = win_ternary("Ternary Window", "Yes", "No", "Maybe",
                         "Choose a button");
    win_message("Selection", "Ok", &format!("Button Selected: {}", rv));

    asm_functions();
    newt::finished();
}

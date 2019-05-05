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

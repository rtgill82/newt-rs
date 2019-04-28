extern crate newt;
use newt::windows::*;

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

    newt::finished();
}

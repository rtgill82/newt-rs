//
// Copyright (C) 2019  Robert Gill <locke@sdf.lonestar.org>
//
// This file is a part of newt-rs.
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License version 2.1 as published by the Free Software Foundation.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

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

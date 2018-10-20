extern crate newt;

pub fn main() {
    newt::init();
    newt::cls();

    let msg = format!("This message contains a C format string: {}", "%-5d");
    newt::win_message("Safe formatting?", "Ok", &msg);

    let msg = format!("This message also contains a C format string: {}",
                      "%.3f");
    let msg = format!("{}. Is it displayed correctly? Did it crash?", msg);
    let result1 = newt::win_choice("Safe formatting?", "Yes", "No", &msg);

    let msg = "%-5d %.3f %02x %-5s";
    let result2 = newt::win_ternary("Ditto", "Yes", "No", "Maybe, so?", &msg);

    newt::finished();

    println!("Choice: Button {} pressed.", result1);
    println!("Ternary choice: Button {} pressed.", result2);
}

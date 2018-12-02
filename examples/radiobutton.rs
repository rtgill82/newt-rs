extern crate newt;
use newt::components::Form;
use newt::components::CompactButton;
use newt::components::Radiobutton;

pub fn main() {
    newt::init();
    newt::cls();
    newt::centered_window(20, 6, Some("Options"));

    let mut form: Form = Form::new(None, None, 0);
    let mut radio1 = Radiobutton::new(4, 1, "Option 1", true, None);
    let mut radio2 = Radiobutton::new(4, 2, "Option 2", false,
                                      Some(&mut radio1));
    let mut radio3 = Radiobutton::new(4, 3, "Option 3", false,
                                      Some(&mut radio2));
    let mut ok = CompactButton::new(7, 5, "Ok");

    form.add_components(&mut [&mut radio1, &mut radio2, &mut radio3,
                              &mut ok])
        .unwrap();
    form.run().unwrap();
    newt::finished();

    let buttons = [(&radio1, "Option 1"),
                   (&radio2, "Option 2"),
                   (&radio3, "Option 3")];

    let current = radio1.get_current();
    for val in buttons.iter() {
        let &(radio, text) = val;
        if *radio == current {
            println!("Selected Option: {}", text);
        }
    }
}

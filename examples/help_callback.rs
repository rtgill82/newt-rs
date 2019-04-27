extern crate newt;
use newt::components::CompactButton;
use newt::components::Form;
use newt::components::Label;

pub fn main() {
    newt::init();
    newt::cls();
    newt::centered_window(20, 6, Some("Help Test"));

    let f = |_form: &Form, data: Option<&i32>| {
        newt::centered_window(23, 4, Some("Help"));
        let mut form = Form::new(None, 0);

        let text = format!("Help Text Data: {}", data.unwrap_or(&0));
        let mut label = Label::new(1, 1, &text);
        let mut ok = CompactButton::new(9, 3, "Ok");
        form.add_component(&mut label).unwrap();
        form.add_component(&mut ok).unwrap();
        form.run().unwrap();
        newt::pop_window();
    };

    let (mut form, _cb) = Form::new_with_help_callback(None, 0, f, Some(12345));
    let mut label = Label::new(1, 2, "Press F1 for help!");
    let mut ok = CompactButton::new(7, 4, "Ok");

    form.add_components(&mut [&mut label, &mut ok]).unwrap();

    form.run().unwrap();
    newt::finished();
}

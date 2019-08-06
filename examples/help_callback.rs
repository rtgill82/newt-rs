extern crate newt;
use newt::prelude::*;

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(20, 6, Some("Help Test")).unwrap();

    let f = |_form: &Form, data: Option<&&str>| {
        let string = data.unwrap_or(&"None");
        let len = string.len();

        let width = (len + 18) as u32;
        newt::centered_window(width, 5, Some("Help")).unwrap();
        let mut form = Form::new(None, 0);

        let text = format!("Help Text Data: {}", string);
        let label = Label::new(1, 1, &text);

        let pos = (width / 2 - 3) as i32;
        let ok = CompactButton::new(pos, 3, "Ok");
        form.add_component(&label).unwrap();
        form.add_component(&ok).unwrap();
        form.run().unwrap();
        newt::pop_window();
    };

    let (mut form, _cb) =
        Form::new_with_help_callback(None, 0, f, Some("This is help text."));
    let label = Label::new(1, 1, "Press F1 for help!");
    let ok = CompactButton::new(7, 4, "Ok");

    form.add_components(&[&label, &ok]).unwrap();

    form.run().unwrap();
    newt::finished();
}

extern crate newt;
use newt::Callback;
use newt::prelude::*;

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(20, 6, Some("Callback Test")).unwrap();

    let cb1 = Checkbox::new(3, 1, "Check 1", None, None);
    let cb2 = Checkbox::new(3, 2, "Check 2", None, None);
    let ok = CompactButton::new(7, 4, "Ok");

    let mut form = Form::new(None, 0);
    form.add_components(&[&cb1, &cb2, &ok]).unwrap();

    let mut value: i32 = 0;
    // Closure `f` borrows `value` as mutable so create a new subscope here
    // allowing `value` to be borrowed immutably when printing the result later.
    {
        // Create closure to be called by Callback
        let mut f = |_c: &dyn Component, data: Option<&i32>| {
            value = *data.unwrap();
        };

        // Create Callback with first Checkbox using `5` as data.
        let mut callback = Callback::new(&cb1, &mut f, Some(5));
        // Add second Checkbox using `10` as data.
        callback.add_component(&cb2, Some(10));

        form.run().unwrap();
        newt::finished();
    }

    // `value` will be 0, 5, or 10 depending on the last Checkbox "clicked".
    println!("value = {}", value);
}

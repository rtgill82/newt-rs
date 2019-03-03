extern crate newt;
use newt::Callback;
use newt::components::Checkbox;
use newt::components::CompactButton;
use newt::components::Component;
use newt::components::Form;

pub fn main() {
    newt::init();
    newt::cls();
    newt::centered_window(20, 6, Some("Callback Test"));

    let mut value: i32 = 0;
    {
        let mut f = |_c: Option<&Component>, data: Option<&i32>| {
            value = *data.unwrap();
        };

        let mut form = Form::new(None, 0);
        let mut cb1 = Checkbox::new(3, 1, "Check 1", None, None);
        let mut cb2 = Checkbox::new(3, 2, "Check 2", None, None);
        let mut ok = CompactButton::new(7, 4, "Ok");

        form.add_components(&mut [&mut cb1, &mut cb2, &mut ok]).unwrap();

        let mut callback = Callback::new(&cb1, &mut f, Some(5));
                callback.add_component(&cb2, Some(10));

        form.run().unwrap();
        newt::finished();
    }

    println!("value = {}", value);
}

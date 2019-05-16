extern crate newt;
use newt::components::CompactButton;
use newt::components::Form;
use newt::components::Label;
use newt::callbacks::SuspendCallback;

pub fn main() {
    newt::init().unwrap();
    newt::cls();
    newt::centered_window(20, 5, Some("Suspend Callback Test")).unwrap();

    let mut value: i32 = 0;
    {
        let mut f = |data: Option<&i32>| {
            value = *data.unwrap();
        };

        let mut form = Form::new(None, 0);
        let mut label = Label::new(4, 1, "Press Ctrl-Z");
        let mut ok = CompactButton::new(7, 4, "Ok");

        form.add_components(&mut [&mut label, &mut ok]).unwrap();

        let _callback = SuspendCallback::new(&mut f, Some(10));

        form.run().unwrap();
        newt::finished();
    }

    println!("value = {}", value);
}

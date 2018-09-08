extern crate newt;
use newt::components::Button;
use newt::components::Checkbox;
use newt::components::CompactButton;
use newt::components::Entry;
use newt::components::Form;
use newt::components::Label;
use newt::components::Radiobutton;
use newt::components::Textbox;
use newt::components::form::ExitReason;
use newt::constants::FLAG_PASSWORD;

pub fn main() {
    newt::init();
    newt::cls();
    newt::draw_root_text(5, 0, "Root Text");

    let close_times = close_window();
    main_window();

    newt::delay(1000000);
    newt::finished();
    println!("refused to close window {} times", close_times);
}

fn close_window() -> i32 {
   newt::centered_window(20, 5, Some("Close Me"));

   let mut form = Form::new(0);
   let mut text = Textbox::new(6, 1, 12, 1, 0);
   let mut no = CompactButton::new(3, 3, "No");
   let mut ok = CompactButton::new(11, 3, "Ok");

   text.set_text("Close Me!");
   form.add_components(&mut [&mut text, &mut no, &mut ok]).unwrap();
   let mut count = 0;
   while form.run().unwrap() == no { count += 1; }
   newt::pop_window();

   return count;
}

fn main_window() {
    newt::open_window(2, 2, 72, 20, Some("Main"));

    let mut button = Button::new(25, 2, "Exit");
    let mut form = Form::new(0);
    let mut label1 = Label::new(1, 1, "Entry:");
    let mut entry = Entry::new(11, 1, None, 10, 0);
    let mut label2 = Label::new(1, 2, "Password:");
    let mut password = Entry::new(11, 2, None, 10, FLAG_PASSWORD);

    let mut rbutton1 = Radiobutton::new(1, 4, "Selection 1", true, None);
    let mut rbutton2 = Radiobutton::new(1, 5, "Selection 2", false, Some(&mut rbutton1));
    let mut rbutton3 = Radiobutton::new(1, 6, "Selection 3", false, Some(&mut rbutton2));

    let seq = [' ', 'X', 'O'];
    let mut cb1 = Checkbox::new(1, 8, "", Some(' '), Some(&seq));
    let mut cb2 = Checkbox::new(1, 9, "", Some(' '), Some(&seq));
    let mut cb3 = Checkbox::new(1, 10, "", Some(' '), Some(&seq));
    let mut cb4 = Checkbox::new(5, 8, "", Some(' '), Some(&seq));
    let mut cb5 = Checkbox::new(5, 9, "", Some(' '), Some(&seq));
    let mut cb6 = Checkbox::new(5, 10, "", Some(' '), Some(&seq));
    let mut cb7 = Checkbox::new(9, 8, "", Some(' '), Some(&seq));
    let mut cb8 = Checkbox::new(9, 9, "", Some(' '), Some(&seq));
    let mut cb9 = Checkbox::new(9, 10, "", Some(' '), Some(&seq));

    form.add_components(&mut [&mut button, &mut label1,
                              &mut entry, &mut label2, &mut password,
                              &mut rbutton1, &mut rbutton2,
                              &mut rbutton3, &mut cb1, &mut cb2,
                              &mut cb3, &mut cb4, &mut cb5,
                              &mut cb6, &mut cb7, &mut cb8,
                              &mut cb9]).unwrap();
    form.run().unwrap();
}

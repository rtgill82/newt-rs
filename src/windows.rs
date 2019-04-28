use std::ffi::CString;
use newt_sys::*;

pub fn win_message(title: &str, button_text: &str, text: &str) {
    unsafe {
        let c_title = CString::new(title).unwrap();
        let c_button = CString::new(button_text).unwrap();
        let c_text = CString::new(text).unwrap();

        newtWinMessage(c_title.as_ptr() as *mut i8,
                       c_button.as_ptr() as *mut i8,
                       c_text.as_ptr() as *mut i8);
    }
}

pub fn win_choice(title: &str, button1: &str, button2: &str, text: &str)
  -> i32 {
    unsafe {
        let c_title = CString::new(title).unwrap();
        let c_button1 = CString::new(button1).unwrap();
        let c_button2 = CString::new(button2).unwrap();
        let c_text = CString::new(text).unwrap();

        newtWinChoice(c_title.as_ptr() as *mut i8,
                      c_button1.as_ptr() as *mut i8,
                      c_button2.as_ptr() as *mut i8,
                      c_text.as_ptr() as *mut i8) as i32
    }
}

pub fn win_ternary(title: &str, button1: &str, button2: &str, button3: &str,
                   text: &str) -> i32 {
    unsafe {
        let c_title = CString::new(title).unwrap();
        let c_button1 = CString::new(button1).unwrap();
        let c_button2 = CString::new(button2).unwrap();
        let c_button3 = CString::new(button3).unwrap();
        let c_text = CString::new(text).unwrap();

        newtWinTernary(c_title.as_ptr() as *mut i8,
                       c_button1.as_ptr() as *mut i8,
                       c_button2.as_ptr() as *mut i8,
                       c_button3.as_ptr() as *mut i8,
                       c_text.as_ptr() as *mut i8) as i32
    }
}

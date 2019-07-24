//!
//! Convenient windowing functions.
//!
use std::ffi::CString;
use libc::c_char;
use newt_sys::*;

#[cfg(feature = "asm")]
#[doc(inline)]
pub use crate::intern::asm::win_menu;

#[cfg(feature = "asm")]
#[doc(inline)]
pub use crate::intern::asm::win_entries;

///
/// Open a simple message window.
///
pub fn win_message(title: &str, button_text: &str, text: &str) {
    unsafe {
        let c_title = CString::new(title).unwrap();
        let c_button = CString::new(button_text).unwrap();
        let escaped = str::replace(text, "%", "%%");
        let c_text = CString::new(escaped).unwrap();

        newtWinMessage(c_title.as_ptr() as *mut c_char,
                       c_button.as_ptr() as *mut c_char,
                       c_text.as_ptr() as *mut c_char);
    }
}

///
/// Open a window providing a choice of buttons.
///
/// Returns the number of the button pressed.
///
pub fn win_choice(title: &str, button1: &str, button2: &str, text: &str)
  -> i32 {
    unsafe {
        let c_title = CString::new(title).unwrap();
        let c_button1 = CString::new(button1).unwrap();
        let c_button2 = CString::new(button2).unwrap();
        let escaped = str::replace(text, "%", "%%");
        let c_text = CString::new(escaped).unwrap();

        newtWinChoice(c_title.as_ptr() as *mut c_char,
                      c_button1.as_ptr() as *mut c_char,
                      c_button2.as_ptr() as *mut c_char,
                      c_text.as_ptr() as *mut c_char) as i32
    }
}

///
/// Open a window with three buttons.
///
/// Returns the number of the button pressed.
///
pub fn win_ternary(title: &str, button1: &str, button2: &str, button3: &str,
                   text: &str) -> i32 {
    unsafe {
        let c_title = CString::new(title).unwrap();
        let c_button1 = CString::new(button1).unwrap();
        let c_button2 = CString::new(button2).unwrap();
        let c_button3 = CString::new(button3).unwrap();
        let escaped = str::replace(text, "%", "%%");
        let c_text = CString::new(escaped).unwrap();

        newtWinTernary(c_title.as_ptr() as *mut c_char,
                       c_button1.as_ptr() as *mut c_char,
                       c_button2.as_ptr() as *mut c_char,
                       c_button3.as_ptr() as *mut c_char,
                       c_text.as_ptr() as *mut c_char) as i32
    }
}

///
/// A struct used to pass initial [`Entry`][entry] information to the
/// `win_entries()` function.
///
/// [entry]: ../components/struct.Entry.html
///
#[cfg(feature = "asm")]
#[derive(Default)]
pub struct WinEntry {
    pub(crate) text: String,
    pub(crate) value: String,
    pub(crate) flags: i32
}

#[cfg(feature = "asm")]
impl WinEntry {
    ///
    /// Create a new `WinEntry`.
    ///
    /// * `text` - The text to display as the entry field label.
    /// * `value` - The initial value of the `Entry` field.
    /// * `flags` - The settings flags for the `Entry`.
    ///
    pub fn new(text: &str, value: &str, flags: i32) -> WinEntry {
        WinEntry {
            text: String::from(text),
            value: String::from(value),
            flags
        }
    }

    ///
    /// Returns the value of the corresponding `Entry`. This is either
    /// the inital `value` set when the `WinEntry` is created, or the user
    /// entered data provided by the [`win_entries()`][win_entries] function
    /// if that has been run.
    ///
    /// [win_entries]: ../windows/fn.win_entries.html
    ///
    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}

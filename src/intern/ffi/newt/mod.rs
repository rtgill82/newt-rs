use std::os::raw::{c_char, c_int, c_uint};
use intern::structs::NewtColors;

pub mod button;
pub mod checkbox;
pub mod checkbox_tree;
pub mod compact_button;
pub mod component;
pub mod entry;
pub mod form;
pub mod label;
pub mod listbox;
pub mod radiobutton;
pub mod scale;
pub mod textbox;

extern "C" {
    pub fn newtInit() -> c_int;
    pub fn newtFinished() -> c_int;
    pub fn newtCls();
    pub fn newtResizeScreen(redraw: c_int);
    pub fn newtWaitForKey();
    pub fn newtClearKeyBuffer();
    pub fn newtDelay(usecs: c_uint);
    pub fn newtOpenWindow(left: c_int, top: c_int,
                          width: c_uint, height: c_uint,
                          title: *const c_char) -> c_int;
    pub fn newtCenteredWindow(width: c_uint, height: c_uint,
                              title: *const c_char) -> c_int;

    pub fn newtPopWindow();
    pub fn newtPopWindowNoRefresh();
    pub fn newtSetColors(colors: *const NewtColors);
    pub fn newtSetColor(colorset: c_int, fg: *const c_char, bg: *const c_char);
    pub fn newtRefresh();
    pub fn newtSuspend();
    pub fn newtResume();
    pub fn newtPushHelpLine(text: *const c_char);
    pub fn newtRedrawHelpLine();
    pub fn newtPopHelpLine();
    pub fn newtDrawRootText(col: c_int, row: c_int, text: *const c_char);
    pub fn newtBell();
    pub fn newtCursorOff();
    pub fn newtCursorOn();
    pub fn newtGetScreenSize(cols: *mut c_int, rows: *mut c_int);
    pub fn newtReflowText(text: *const c_char, width: c_int, flexDown: c_int,
                          flexUp: c_int, actualWidth: *mut c_int,
                          actualHeight: *mut c_int) -> *const c_char;
}

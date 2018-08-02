use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_uint;
use std::ptr;

#[macro_use]
mod intern;
pub mod components;

use components::NewtComponentPtr;
use components::NewtComponent;

pub enum Result {
    HotKey(i32),
    Component(Box<NewtComponent>),
    FDReady(i32),
    Timer,
    Err
}

#[repr(C)]
pub enum FlagsSense {
    Set,
    Reset,
    Toggle
}

pub const COLORSET_ROOT: i32          = 2;
pub const COLORSET_BORDER: i32        = 3;
pub const COLORSET_WINDOW: i32        = 4;
pub const COLORSET_SHADOW: i32        = 5;
pub const COLORSET_TITLE: i32         = 6;
pub const COLORSET_BUTTON: i32        = 7;
pub const COLORSET_ACTBUTTON: i32     = 8;
pub const COLORSET_CHECKBOX: i32      = 9;
pub const COLORSET_ACTCHECKBOX: i32   = 10;
pub const COLORSET_ENTRY: i32         = 11;
pub const COLORSET_LABEL: i32         = 12;
pub const COLORSET_LISTBOX: i32       = 13;
pub const COLORSET_ACTLISTBOX: i32    = 14;
pub const COLORSET_TEXTBOX: i32       = 15;
pub const COLORSET_ACTTEXTBOX: i32    = 16;
pub const COLORSET_HELPLINE: i32      = 17;
pub const COLORSET_ROOTTEXT: i32      = 18;
pub const COLORSET_EMPTYSCALE: i32    = 19;
pub const COLORSET_FULLSCALE: i32     = 20;
pub const COLORSET_DISENTRY: i32      = 21;
pub const COLORSET_COMPACTBUTTON: i32 = 22;
pub const COLORSET_ACTSELLISTBOX: i32 = 23;
pub const COLORSET_SELLISTBOX: i32    = 24;

#[allow(non_snake_case)]
pub fn COLORSET_CUSTOM(x: i32) -> i32 {
    30 + x
}

pub fn init() -> i32 {
    #[link(name="newt")]
    extern "C" { fn newtInit() -> c_int; }
    unsafe { newtInit() }
}

pub fn finished() -> i32 {
    #[link(name="newt")]
    extern "C" { fn newtFinished() -> c_int; }
    unsafe { newtFinished() }
}

pub fn cls() {
    #[link(name="newt")]
    extern "C" { fn newtCls(); }
    unsafe { newtCls(); }
}

pub fn resize_screen(redraw: i32) {
    #[link(name="newt")]
    extern "C" { fn newtResizeScreen(redraw: c_int); }
    unsafe { newtResizeScreen(redraw); }
}

pub fn wait_for_key() {
    #[link(name="newt")]
    extern "C" { fn newtWaitForKey(); }
    unsafe { newtWaitForKey(); }
}

pub fn clear_key_buffer() {
    #[link(name="newt")]
    extern "C" { fn newtClearKeyBuffer(); }
    unsafe { newtClearKeyBuffer(); }
}

pub fn delay(usecs: u32) {
    #[link(name="newt")]
    extern "C" { fn newtDelay(usecs: c_uint); }
    unsafe { newtDelay(usecs); }
}

pub fn open_window(left: i32, top: i32, width: u32, height: u32,
                   title: &str) -> i32 {
    #[link(name="newt")]
    extern "C" {
        fn newtOpenWindow(left: c_int, top: c_int,
                          width: c_uint, height: c_uint,
                          title: *const c_char) -> c_int;
    }
    let c_str = CString::new(title).unwrap();
    unsafe { newtOpenWindow(left, top, width, height, c_str.as_ptr()) }
}

pub fn centered_window(width: u32, height: u32, title: &str) -> i32 {
    #[link(name="newt")]
    extern "C" {
        fn newtCenteredWindow(width: c_uint, height: c_uint,
                              title: *const c_char) -> c_int;
    }
    let c_str = CString::new(title).unwrap();
    unsafe { newtCenteredWindow(width, height, c_str.as_ptr()) }
}

pub fn pop_window() {
    #[link(name="newt")]
    extern "C" { fn newtPopWindow(); }
    unsafe { newtPopWindow(); }
}

pub fn pop_window_no_refresh() {
    #[link(name="newt")]
    extern "C" { fn newtPopWindowNoRefresh(); }
    unsafe { newtPopWindowNoRefresh(); }
}

pub fn refresh() {
    #[link(name="newt")]
    extern "C" { fn newtRefresh(); }
    unsafe { newtRefresh(); }
}

pub fn suspend() {
    #[link(name="newt")]
    extern "C" { fn newtSuspend(); }
    unsafe { newtSuspend(); }
}

pub fn push_help_line(text: &str) {
    #[link(name="newt")]
    extern "C" { fn newtPushHelpLine(text: *const c_char); }
    let c_str = CString::new(text).unwrap();
    unsafe { newtPushHelpLine(c_str.as_ptr()); }
}

pub fn redraw_help_line() {
    #[link(name="newt")]
    extern "C" { fn newtRedrawHelpLine(); }
    unsafe { newtRedrawHelpLine(); }
}

pub fn pop_help_line() {
    #[link(name="newt")]
    extern "C" { fn newtPopHelpLine(); }
    unsafe { newtPopHelpLine(); }
}

pub fn draw_root_text(col: i32, row: i32, text: &str) {
    #[link(name="newt")]
    extern "C" {
        fn newtDrawRootText(col: c_int, row: c_int, text: *const c_char);
    }
    let c_str = CString::new(text).unwrap();
    unsafe { newtDrawRootText(col, row, c_str.as_ptr()); }
}

pub fn bell() {
    #[link(name="newt")]
    extern "C" { fn newtBell(); }
    unsafe { newtBell(); }
}

pub fn cursor_off() {
    #[link(name="newt")]
    extern "C" { fn newtCursorOff(); }
    unsafe { newtCursorOff(); }
}

pub fn cursor_on() {
    #[link(name="newt")]
    extern "C" { fn newtCursorOn(); }
    unsafe { newtCursorOn(); }
}

pub fn get_screen_size() -> (i32, i32) {
    #[link(name="newt")]
    extern "C" { fn newtGetScreenSize(cols: *mut c_int, rows: *mut c_int); }
    let mut cols: c_int = 0;
    let mut rows: c_int = 0;
    unsafe { newtGetScreenSize(&mut cols, &mut rows); }
    return (cols, rows);
}

pub fn reflow_text(text: &str, width: i32, flex_down: i32, flex_up: i32,
                   actual_width: &mut i32, actual_height: &mut i32) -> String {
    #[link(name="newt")]
    extern "C" {
        fn newtReflowText(text: *const c_char, width: c_int, flexDown: c_int,
                          flexUp: c_int, actualWidth: *mut c_int,
                          actualHeight: *mut c_int) -> *const c_char;
    }
    let c_str = CString::new(text).unwrap();
    unsafe {
        let rstr = newtReflowText(c_str.as_ptr(), width, flex_down, flex_up,
                                  actual_width, actual_height);
        CStr::from_ptr(rstr).to_string_lossy().into_owned()
    }
}

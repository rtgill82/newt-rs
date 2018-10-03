use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr;

#[macro_use]
mod intern;
pub mod components;
pub mod constants;
pub use constants::*;

use intern::structs::NewtColors;

pub struct Colors<'a> {
    pub root_fg: &'a str,            pub root_bg: &'a str,
    pub border_fg: &'a str,          pub border_bg: &'a str,
    pub window_fg: &'a str,          pub window_bg: &'a str,
    pub shadow_fg: &'a str,          pub shadow_bg: &'a str,
    pub title_fg: &'a str,           pub title_bg: &'a str,
    pub button_fg: &'a str,          pub button_bg: &'a str,
    pub act_button_fg: &'a str,      pub act_button_bg: &'a str,
    pub checkbox_fg: &'a str,        pub checkbox_bg: &'a str,
    pub act_checkbox_fg: &'a str,    pub act_checkbox_bg: &'a str,
    pub entry_fg: &'a str,           pub entry_bg: &'a str,
    pub label_fg: &'a str,           pub label_bg: &'a str,
    pub listbox_fg: &'a str,         pub listbox_bg: &'a str,
    pub act_listbox_fg: &'a str,     pub act_listbox_bg: &'a str,
    pub textbox_fg: &'a str,         pub textbox_bg: &'a str,
    pub act_textbox_fg: &'a str,     pub act_textbox_bg: &'a str,
    pub help_line_fg: &'a str,       pub help_line_bg: &'a str,
    pub root_text_fg: &'a str,       pub root_text_bg: &'a str,
    pub empty_scale: &'a str,        pub full_scale: &'a str,
    pub disabled_entry_fg: &'a str,  pub disabled_entry_bg: &'a str,
    pub compact_button_fg: &'a str,  pub compact_button_bg: &'a str,
    pub act_sel_listbox_fg: &'a str, pub act_sel_listbox_bg: &'a str,
    pub sel_listbox_fg: &'a str,     pub sel_listbox_bg: &'a str
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
                   title: Option<&str>) -> i32 {
    #[link(name="newt")]
    extern "C" {
        fn newtOpenWindow(left: c_int, top: c_int,
                          width: c_uint, height: c_uint,
                          title: *const c_char) -> c_int;
    }

    let c_str: CString;
    let c_ptr = match title {
        Some(title) => {
            c_str = CString::new(title).unwrap();
            c_str.as_ptr()
        },
        None => ptr::null()
    };

    unsafe { newtOpenWindow(left, top, width, height, c_ptr) }
}

pub fn centered_window(width: u32, height: u32, title: Option<&str>) -> i32 {
    #[link(name="newt")]
    extern "C" {
        fn newtCenteredWindow(width: c_uint, height: c_uint,
                              title: *const c_char) -> c_int;
    }

    let c_str: CString;
    let c_ptr = match title {
        Some(title) => {
            c_str = CString::new(title).unwrap();
            c_str.as_ptr()
        },
        None => ptr::null()
    };

    unsafe { newtCenteredWindow(width, height, c_ptr) }
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

pub fn set_colors(colors: &Colors) {
    #[link(name="newt")]
    extern "C" {
        fn newtSetColors(colors: *const NewtColors);
    }

    let root_fg = CString::new(colors.root_fg).unwrap();
    let root_bg = CString::new(colors.root_bg).unwrap();
    let border_fg = CString::new(colors.border_fg).unwrap();
    let border_bg = CString::new(colors.border_bg).unwrap();
    let window_fg = CString::new(colors.window_fg).unwrap();
    let window_bg = CString::new(colors.window_bg).unwrap();
    let shadow_fg = CString::new(colors.shadow_fg).unwrap();
    let shadow_bg = CString::new(colors.shadow_bg).unwrap();
    let title_fg = CString::new(colors.title_fg).unwrap();
    let title_bg = CString::new(colors.title_bg).unwrap();
    let button_fg = CString::new(colors.button_fg).unwrap();
    let button_bg = CString::new(colors.button_bg).unwrap();
    let act_button_fg = CString::new(colors.act_button_fg).unwrap();
    let act_button_bg = CString::new(colors.act_button_bg).unwrap();
    let checkbox_fg = CString::new(colors.checkbox_fg).unwrap();
    let checkbox_bg = CString::new(colors.checkbox_bg).unwrap();
    let act_checkbox_fg = CString::new(colors.act_checkbox_fg).unwrap();
    let act_checkbox_bg = CString::new(colors.act_checkbox_bg).unwrap();
    let entry_fg = CString::new(colors.entry_fg).unwrap();
    let entry_bg = CString::new(colors.entry_bg).unwrap();
    let label_fg = CString::new(colors.label_fg).unwrap();
    let label_bg = CString::new(colors.label_bg).unwrap();
    let listbox_fg = CString::new(colors.listbox_fg).unwrap();
    let listbox_bg = CString::new(colors.listbox_bg).unwrap();
    let act_listbox_fg = CString::new(colors.act_listbox_fg).unwrap();
    let act_listbox_bg = CString::new(colors.act_listbox_bg).unwrap();
    let textbox_fg = CString::new(colors.textbox_fg).unwrap();
    let textbox_bg = CString::new(colors.textbox_bg).unwrap();
    let act_textbox_fg = CString::new(colors.act_textbox_fg).unwrap();
    let act_textbox_bg = CString::new(colors.act_textbox_bg).unwrap();
    let help_line_fg = CString::new(colors.help_line_fg).unwrap();
    let help_line_bg = CString::new(colors.help_line_bg).unwrap();
    let root_text_fg = CString::new(colors.root_text_fg).unwrap();
    let root_text_bg = CString::new(colors.root_text_bg).unwrap();
    let empty_scale = CString::new(colors.empty_scale).unwrap();
    let full_scale = CString::new(colors.full_scale).unwrap();
    let disabled_entry_fg = CString::new(colors.disabled_entry_fg).unwrap();
    let disabled_entry_bg = CString::new(colors.disabled_entry_bg).unwrap();
    let compact_button_fg = CString::new(colors.compact_button_fg).unwrap();
    let compact_button_bg = CString::new(colors.compact_button_bg).unwrap();
    let act_sel_listbox_fg = CString::new(colors.act_sel_listbox_fg).unwrap();
    let act_sel_listbox_bg = CString::new(colors.act_sel_listbox_bg).unwrap();
    let sel_listbox_fg = CString::new(colors.sel_listbox_fg).unwrap();
    let sel_listbox_bg = CString::new(colors.sel_listbox_bg).unwrap();

    let c_colors = NewtColors {
        rootFg: root_fg.as_ptr(),                     rootBg: root_bg.as_ptr(),
        borderFg: border_fg.as_ptr(),                 borderBg: border_bg.as_ptr(),
        windowFg: window_fg.as_ptr(),                 windowBg: window_bg.as_ptr(),
        shadowFg: shadow_fg.as_ptr(),                 shadowBg: shadow_bg.as_ptr(),
        titleFg: title_fg.as_ptr(),                   titleBg: title_bg.as_ptr(),
        buttonFg: button_fg.as_ptr(),                 buttonBg: button_bg.as_ptr(),
        actButtonFg: act_button_fg.as_ptr(),          actButtonBg: act_button_bg.as_ptr(),
        checkboxFg: checkbox_fg.as_ptr(),             checkboxBg: checkbox_bg.as_ptr(),
        actCheckboxFg: act_checkbox_fg.as_ptr(),      actCheckboxBg: act_checkbox_bg.as_ptr(),
        entryFg: entry_fg.as_ptr(),                   entryBg: entry_bg.as_ptr(),
        labelFg: label_fg.as_ptr(),                   labelBg: label_bg.as_ptr(),
        listboxFg: listbox_fg.as_ptr(),               listboxBg: listbox_bg.as_ptr(),
        actListboxFg: act_listbox_fg.as_ptr(),        actListboxBg: act_listbox_bg.as_ptr(),
        textboxFg: textbox_fg.as_ptr(),               textboxBg: textbox_bg.as_ptr(),
        actTextboxFg: act_textbox_fg.as_ptr(),        actTextboxBg: act_textbox_bg.as_ptr(),
        helpLineFg: help_line_fg.as_ptr(),            helpLineBg: help_line_bg.as_ptr(),
        rootTextFg: root_text_fg.as_ptr(),            rootTextBg: root_text_bg.as_ptr(),
        emptyScale: empty_scale.as_ptr(),             fullScale: full_scale.as_ptr(),
        disabledEntryFg: disabled_entry_fg.as_ptr(),  disabledEntryBg: disabled_entry_bg.as_ptr(),
        compactButtonFg: compact_button_fg.as_ptr(),  compactButtonBg: compact_button_bg.as_ptr(),
        actSelListboxFg: act_sel_listbox_fg.as_ptr(), actSelListboxBg: act_sel_listbox_bg.as_ptr(),
        selListboxFg: sel_listbox_fg.as_ptr(),        selListboxBg: sel_listbox_bg.as_ptr()
    };

    unsafe { newtSetColors(&c_colors); }
}

pub fn set_color(colorset: i32, fg: &str, bg: &str) {
    #[link(name="newt")]
    extern "C" {
        fn newtSetColor(colorset: c_int, fg: *const c_char, bg: *const c_char);
    }

    let c_fg = CString::new(fg).unwrap();
    let c_bg = CString::new(bg).unwrap();
    unsafe { newtSetColor(colorset, c_fg.as_ptr(), c_bg.as_ptr()); }
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

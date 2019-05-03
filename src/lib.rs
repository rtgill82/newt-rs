#![cfg_attr(feature = "asm", feature(asm))]
#[macro_use]
extern crate newt_component_derive;
extern crate newt_sys;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char,c_int};
use std::ptr;

#[macro_use]
mod intern;
pub mod callbacks;
pub mod components;
pub mod constants;
pub mod windows;

#[doc(inline)]
pub use self::callbacks::Callback;

use newt_sys::*;

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
    unsafe { newtInit() }
}

pub fn finished() -> i32 {
    unsafe { newtFinished() }
}

pub fn cls() {
    unsafe { newtCls(); }
}

pub fn resize_screen(redraw: i32) {
    unsafe { newtResizeScreen(redraw); }
}

pub fn wait_for_key() {
    unsafe { newtWaitForKey(); }
}

pub fn clear_key_buffer() {
    unsafe { newtClearKeyBuffer(); }
}

pub fn delay(usecs: u32) {
    unsafe { newtDelay(usecs); }
}

pub fn open_window(left: i32, top: i32, width: u32, height: u32,
                   title: Option<&str>) -> i32 {
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
    unsafe { newtPopWindow(); }
}

pub fn pop_window_no_refresh() {
    unsafe { newtPopWindowNoRefresh(); }
}

pub fn set_colors(colors: &Colors) {
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

    let c_colors = newtColors {
        rootFg: root_fg.as_ptr() as *mut c_char,
        rootBg: root_bg.as_ptr() as *mut c_char,
        borderFg: border_fg.as_ptr() as *mut c_char,
        borderBg: border_bg.as_ptr() as *mut c_char,
        windowFg: window_fg.as_ptr() as *mut c_char,
        windowBg: window_bg.as_ptr() as *mut c_char,
        shadowFg: shadow_fg.as_ptr() as *mut c_char,
        shadowBg: shadow_bg.as_ptr() as *mut c_char,
        titleFg: title_fg.as_ptr() as *mut c_char,
        titleBg: title_bg.as_ptr() as *mut c_char,
        buttonFg: button_fg.as_ptr() as *mut c_char,
        buttonBg: button_bg.as_ptr() as *mut c_char,
        actButtonFg: act_button_fg.as_ptr() as *mut c_char,
        actButtonBg: act_button_bg.as_ptr() as *mut c_char,
        checkboxFg: checkbox_fg.as_ptr() as *mut c_char,
        checkboxBg: checkbox_bg.as_ptr() as *mut c_char,
        actCheckboxFg: act_checkbox_fg.as_ptr() as *mut c_char,
        actCheckboxBg: act_checkbox_bg.as_ptr() as *mut c_char,
        entryFg: entry_fg.as_ptr() as *mut c_char,
        entryBg: entry_bg.as_ptr() as *mut c_char,
        labelFg: label_fg.as_ptr() as *mut c_char,
        labelBg: label_bg.as_ptr() as *mut c_char,
        listboxFg: listbox_fg.as_ptr() as *mut c_char,
        listboxBg: listbox_bg.as_ptr() as *mut c_char,
        actListboxFg: act_listbox_fg.as_ptr() as *mut c_char,
        actListboxBg: act_listbox_bg.as_ptr() as *mut c_char,
        textboxFg: textbox_fg.as_ptr() as *mut c_char,
        textboxBg: textbox_bg.as_ptr() as *mut c_char,
        actTextboxFg: act_textbox_fg.as_ptr() as *mut c_char,
        actTextboxBg: act_textbox_bg.as_ptr() as *mut c_char,
        helpLineFg: help_line_fg.as_ptr() as *mut c_char,
        helpLineBg: help_line_bg.as_ptr() as *mut c_char,
        rootTextFg: root_text_fg.as_ptr() as *mut c_char,
        rootTextBg: root_text_bg.as_ptr() as *mut c_char,
        emptyScale: empty_scale.as_ptr() as *mut c_char,
        fullScale: full_scale.as_ptr() as *mut c_char,
        disabledEntryFg: disabled_entry_fg.as_ptr() as *mut c_char,
        disabledEntryBg: disabled_entry_bg.as_ptr() as *mut c_char,
        compactButtonFg: compact_button_fg.as_ptr() as *mut c_char,
        compactButtonBg: compact_button_bg.as_ptr() as *mut c_char,
        actSelListboxFg: act_sel_listbox_fg.as_ptr() as *mut c_char,
        actSelListboxBg: act_sel_listbox_bg.as_ptr() as *mut c_char,
        selListboxFg: sel_listbox_fg.as_ptr() as *mut c_char,
        selListboxBg: sel_listbox_bg.as_ptr() as *mut c_char
    };

    unsafe { newtSetColors(c_colors); }
}

pub fn set_color(colorset: i32, fg: &str, bg: &str) {
    let c_fg = CString::new(fg).unwrap();
    let c_bg = CString::new(bg).unwrap();
    unsafe {
        newtSetColor(colorset,
                     c_fg.as_ptr() as *mut c_char,
                     c_bg.as_ptr() as *mut c_char);
    }
}

pub fn refresh() {
    unsafe { newtRefresh(); }
}

pub fn suspend() {
    unsafe { newtSuspend(); }
}

pub fn resume() {
    unsafe { newtResume(); }
}

pub fn push_help_line(text: &str) {
    let c_str = CString::new(text).unwrap();
    unsafe { newtPushHelpLine(c_str.as_ptr()); }
}

pub fn redraw_help_line() {
    unsafe { newtRedrawHelpLine(); }
}

pub fn pop_help_line() {
    unsafe { newtPopHelpLine(); }
}

pub fn draw_root_text(col: i32, row: i32, text: &str) {
    let c_str = CString::new(text).unwrap();
    unsafe { newtDrawRootText(col, row, c_str.as_ptr()); }
}

pub fn bell() {
    unsafe { newtBell(); }
}

pub fn cursor_off() {
    unsafe { newtCursorOff(); }
}

pub fn cursor_on() {
    unsafe { newtCursorOn(); }
}

pub fn get_screen_size() -> (i32, i32) {
    let mut cols: c_int = 0;
    let mut rows: c_int = 0;
    unsafe { newtGetScreenSize(&mut cols, &mut rows); }
    return (cols, rows);
}

pub fn reflow_text(text: &str, width: i32, flex_down: i32, flex_up: i32,
                   actual_width: &mut i32, actual_height: &mut i32) -> String {
    let c_str = CString::new(text).unwrap();
    unsafe {
        let rstr = newtReflowText(c_str.as_ptr() as *mut c_char,
                                  width, flex_down, flex_up, actual_width,
                                  actual_height);
        CStr::from_ptr(rstr).to_string_lossy().into_owned()
    }
}

pub fn win_message(title: &str, button_text: &str, msg: &str) {
    let c_title = CString::new(title).unwrap();
    let c_button_text = CString::new(button_text).unwrap();
    let escaped = str::replace(msg, "%", "%%");
    let c_msg = CString::new(escaped).unwrap();
    unsafe {
        newtWinMessage(c_title.as_ptr() as *mut c_char,
                       c_button_text.as_ptr() as *mut c_char,
                       c_msg.as_ptr() as *mut c_char);
    }
}

pub fn win_choice(title: &str, button1_text: &str, button2_text: &str,
                  msg: &str) -> i32 {
    let c_title = CString::new(title).unwrap();
    let c_button1_text = CString::new(button1_text).unwrap();
    let c_button2_text = CString::new(button2_text).unwrap();
    let escaped = str::replace(msg, "%", "%%");
    let c_msg = CString::new(escaped).unwrap();
    unsafe {
        newtWinChoice(c_title.as_ptr() as *mut c_char,
                      c_button1_text.as_ptr() as *mut c_char,
                      c_button2_text.as_ptr() as *mut c_char,
                      c_msg.as_ptr() as *mut c_char) as i32
    }
}

pub fn win_ternary(title: &str, button1_text: &str, button2_text: &str,
                   button3_text: &str, msg: &str) -> i32 {
    let c_title = CString::new(title).unwrap();
    let c_button1_text = CString::new(button1_text).unwrap();
    let c_button2_text = CString::new(button2_text).unwrap();
    let c_button3_text = CString::new(button3_text).unwrap();
    let escaped = str::replace(msg, "%", "%%");
    let c_msg = CString::new(escaped).unwrap();
    unsafe {
        newtWinTernary(c_title.as_ptr() as *mut c_char,
                       c_button1_text.as_ptr() as *mut c_char,
                       c_button2_text.as_ptr() as *mut c_char,
                       c_button3_text.as_ptr() as *mut c_char,
                       c_msg.as_ptr() as *mut c_char) as i32
    }
}

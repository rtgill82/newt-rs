//
// Copyright (C) 2019 Robert Gill <rtgill82@gmail.com>
//
// This file is a part of newt-rs.
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License version 2.1 as published by the Free Software Foundation.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

//! # newt-rs
//!
//! _Rust bindings for the Newt console UI library._
//!
//! This crate provides bindings to Red Hat, Inc.'s [Newt][newt] console
//! UI library. Newt is a small and simple to use UI library providing
//! widgets and basic stacked window management for console applications.
//!
//! [newt]: https://pagure.io/newt
//! [features]: #features
//!
//! ## Example
//!
//! ```rust no_run
//! extern crate newt;
//! use newt::prelude::*;
//!
//! pub fn main() {
//!     newt::init().unwrap();
//!     newt::cls();
//!     newt::centered_window(20, 5, Some("Greetings")).unwrap();
//!
//!     let text = Textbox::new(4, 1, 12, 1, 0);
//!     let ok = CompactButton::new(7, 3, "Ok");
//!     text.set_text("Hello World!");
//!
//!     let mut form = Form::new(None, 0);
//!     form.add_components(&[&text, &ok]).unwrap();
//!     let reason = form.run().unwrap();
//!     newt::finished();
//!
//!     match reason {
//!         ExitReason::HotKey(key) => // F12 is the default HotKey
//!             println!("Execution stopped due to HotKey: {}", key),
//!         ExitReason::Component(co) =>
//!             println!("Execution stopped due to Component: {:?}", co),
//!         _ =>
//!             println!("Execution stopped due to other reason...")
//!     }
//! }
//! ```
//!
//! ## Features
//!
//! - `asm` - Allows building of the [Grid][grid] module and the
//!           [windows::win_entries][win_entries] and [windows::win_menu][win_menu] functions.
//!           These require the inline assembly feature of Rust which is only available in compiler
//!           versions after 1.59. Supported architectures include _x86_, _x86_64_, _arm_,
//!           _aarch64_, _riscv32_, and _riscv64_.
//!
//! - `static` - Builds and links [`newt-sys`][newt_sys] statically against
//!              its included libraries rather than linking dynamically
//!              against available system libraries. This is done
//!              automatically if the required system libraries are
//!              unavailable.
//!
//! [grid]: grid/index.html
//! [win_entries]: windows/fn.win_entries.html
//! [win_menu]: windows/fn.win_menu.html
//! [newt_sys]: https://crates.io/crates/newt-sys
//!
//! ## License
//!
//! Copyright (C) 2018-2022,2025 Robert Gill <<rtgill82@gmail.com>>
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU Lesser General Public
//! License version 2.1 as published by the Free Software Foundation.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
//! Lesser General Public License for more details.
//!
//! You should have received a copy of the GNU Lesser General Public
//! License along with this library; if not, write to the Free Software
//! Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//!
#[macro_use]
extern crate newt_proc_macros;
extern crate newt_sys;

use std::ffi::{CStr,CString};
use std::os::raw::{c_char,c_int};
use std::ptr;

#[macro_use]
mod intern;
pub mod asm;
pub mod callbacks;
pub mod component;
pub mod constants;
pub mod grid;
pub mod prelude;
pub mod widgets;
pub mod windows;

#[doc(hidden)]
pub use self::component::Component;

#[doc(hidden)]
pub use self::callbacks::Callback;

#[doc(hidden)]
pub use self::windows::win_message;
#[doc(hidden)]
pub use self::windows::win_choice;
#[doc(hidden)]
pub use self::windows::win_ternary;

#[doc(hidden)]
pub use self::asm::*;

#[doc(hidden)]
#[cfg(feature = "asm")]
pub use self::grid::r#trait::Grid;

use newt_sys::*;

///
/// A struct containing the color sets for all components.
///
#[derive(Clone,Debug)]
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

macro_rules! cstr {
    ( $c_str:expr ) => { CStr::from_ptr($c_str).to_str().unwrap() }
}

impl<'a> Default for Colors<'a> {
    fn default() -> Self {
        unsafe {
            let c = &newtDefaultColorPalette;
            Colors {
                root_fg: cstr!(c.rootFg),                     root_bg: cstr!(c.rootBg),
                border_fg: cstr!(c.borderFg),                 border_bg: cstr!(c.borderBg),
                window_fg: cstr!(c.windowFg),                 window_bg: cstr!(c.windowBg),
                shadow_fg: cstr!(c.shadowFg),                 shadow_bg: cstr!(c.shadowBg),
                title_fg: cstr!(c.titleFg),                   title_bg: cstr!(c.titleBg),
                button_fg: cstr!(c.buttonFg),                 button_bg: cstr!(c.buttonBg),
                act_button_fg: cstr!(c.actButtonFg),          act_button_bg: cstr!(c.actButtonBg),
                checkbox_fg: cstr!(c.checkboxFg),             checkbox_bg: cstr!(c.checkboxBg),
                act_checkbox_fg: cstr!(c.actCheckboxFg),      act_checkbox_bg: cstr!(c.actCheckboxBg),
                entry_fg: cstr!(c.entryFg),                   entry_bg: cstr!(c.entryBg),
                label_fg: cstr!(c.labelFg),                   label_bg: cstr!(c.labelBg),
                listbox_fg: cstr!(c.listboxFg),               listbox_bg: cstr!(c.listboxBg),
                act_listbox_fg: cstr!(c.actListboxFg),        act_listbox_bg: cstr!(c.actListboxBg),
                textbox_fg: cstr!(c.textboxFg),               textbox_bg: cstr!(c.textboxBg),
                act_textbox_fg: cstr!(c.actTextboxFg),        act_textbox_bg: cstr!(c.actTextboxBg),
                help_line_fg: cstr!(c.helpLineFg),            help_line_bg: cstr!(c.helpLineBg),
                root_text_fg: cstr!(c.rootTextFg),            root_text_bg: cstr!(c.rootTextBg),
                empty_scale: cstr!(c.emptyScale),             full_scale: cstr!(c.fullScale),
                disabled_entry_fg: cstr!(c.disabledEntryFg),  disabled_entry_bg: cstr!(c.disabledEntryBg),
                compact_button_fg: cstr!(c.compactButtonFg),  compact_button_bg: cstr!(c.compactButtonBg),
                act_sel_listbox_fg: cstr!(c.actSelListboxFg), act_sel_listbox_bg: cstr!(c.actSelListboxBg),
                sel_listbox_fg: cstr!(c.selListboxFg),        sel_listbox_bg: cstr!(c.selListboxBg)
            }
        }
    }
}

///
/// Initialize the newt library.
///
pub fn init() -> Result<(), ()> {
    let rv = unsafe { newtInit() };
    if rv == 0 { Ok(()) } else { Err(()) }
}

///
/// Close down the newt library and reset the terminal.
///
pub fn finished() {
    unsafe { newtFinished(); }
}

///
/// Clear the screen.
///
pub fn cls() {
    unsafe { newtCls(); }
}

///
/// Notify newt of a screen resize.
///
/// * `redraw` - Redraw the screen immediately.
///
pub fn resize_screen(redraw: i32) {
    unsafe { newtResizeScreen(redraw); }
}

///
/// Wait until a key is pressed.
///
pub fn wait_for_key() {
    unsafe { newtWaitForKey(); }
}

///
/// Clear the key buffer.
///
pub fn clear_key_buffer() {
    unsafe { newtClearKeyBuffer(); }
}

///
/// Wait for a specified amount of time.
///
/// * `usecs` - The amount of time to wait in microseconds.
///
pub fn delay(usecs: u32) {
    unsafe { newtDelay(usecs); }
}

///
/// Open a window at the specified location.
///
pub fn open_window(left: i32, top: i32, width: u32, height: u32,
                   title: Option<&str>) -> Result<(), ()> {
    let c_str: CString;
    let c_ptr = match title {
        Some(title) => {
            c_str = CString::new(title).unwrap();
            c_str.as_ptr()
        },
        None => ptr::null()
    };

    let rv = unsafe { newtOpenWindow(left, top, width, height, c_ptr) };
    if rv == 0 { Ok(()) } else { Err(()) }
}

///
/// Open a window in the center of the screen.
///
pub fn centered_window(width: u32, height: u32, title: Option<&str>)
      -> Result<(), ()> {
    let c_str: CString;
    let c_ptr = match title {
        Some(title) => {
            c_str = CString::new(title).unwrap();
            c_str.as_ptr()
        },
        None => ptr::null()
    };

    let rv = unsafe { newtCenteredWindow(width, height, c_ptr) };
    if rv == 0 { Ok(()) } else { Err(()) }
}

///
/// Close the most recently opened window.
///
pub fn pop_window() {
    unsafe { newtPopWindow(); }
}

///
/// Close the most recently opened window without redrawing the screen.
///
pub fn pop_window_no_refresh() {
    unsafe { newtPopWindowNoRefresh(); }
}

///
/// Set the colors used by the newt library.
///
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

///
/// Set a specific color set.
///
/// * `colorset` - The color set number to set.
/// * `fg` - The color set foreground color.
/// * `bg` - The color set background color.
///
pub fn set_color(colorset: i32, fg: &str, bg: &str) {
    let c_fg = CString::new(fg).unwrap();
    let c_bg = CString::new(bg).unwrap();
    unsafe {
        newtSetColor(colorset,
                     c_fg.as_ptr() as *mut c_char,
                     c_bg.as_ptr() as *mut c_char);
    }
}

///
/// Redraw the screen.
///
pub fn refresh() {
    unsafe { newtRefresh(); }
}

///
/// Temporarily suspend the newt library and reset the terminal.
///
pub fn suspend() {
    unsafe { newtSuspend(); }
}

///
/// Resume running the newt library.
///
pub fn resume() {
    unsafe { newtResume(); }
}

///
/// Display a help string on the bottom of the screen.
///
pub fn push_help_line(text: &str) {
    let c_str = CString::new(text).unwrap();
    unsafe { newtPushHelpLine(c_str.as_ptr()); }
}

///
/// Redraw the help line.
///
pub fn redraw_help_line() {
    unsafe { newtRedrawHelpLine(); }
}

///
/// Remove the help line.
///
pub fn pop_help_line() {
    unsafe { newtPopHelpLine(); }
}

///
/// Draw text directly to the root window.
///
pub fn draw_root_text(col: i32, row: i32, text: &str) {
    let c_str = CString::new(text).unwrap();
    unsafe { newtDrawRootText(col, row, c_str.as_ptr()); }
}

///
/// Issue a terminal beep.
///
pub fn bell() {
    unsafe { newtBell(); }
}

///
/// Turn the cursor visibility off.
///
pub fn cursor_off() {
    unsafe { newtCursorOff(); }
}

///
/// Turn the cursor visibility on.
///
pub fn cursor_on() {
    unsafe { newtCursorOn(); }
}

///
/// Get the terminal screen size.
///
/// Returns a tuple pair in the order of (columns, rows).
///
pub fn get_screen_size() -> (i32, i32) {
    let mut cols: c_int = 0;
    let mut rows: c_int = 0;
    unsafe { newtGetScreenSize(&mut cols, &mut rows); }
    (cols, rows)
}

///
/// Reflow text according to the provided specifications.
///
/// * `text` - The text to be reformatted.
/// * `width` - The target width of the text.
/// * `flex_down` - The minimum difference from target width.
/// * `flex_up` - The maximum difference from target width.
///
/// Returns the tuple ``(text, width, height)`` where ``text`` is the newly
/// formatted text, ``width`` is the new width of the text, and ``height``
/// is the number of lines in the text.
///
pub fn reflow_text(text: &str, width: i32, flex_down: i32, flex_up: i32)
      -> (String, i32, i32) {
    let c_str = CString::new(text).unwrap();
    let mut actual_width: c_int = 0;
    let mut actual_height: c_int = 0;
    unsafe {
        let rstr = newtReflowText(c_str.as_ptr() as *mut c_char,
                                  width, flex_down, flex_up, &mut actual_width,
                                  &mut actual_height);
        let c_str = CStr::from_ptr(rstr).to_string_lossy().into_owned();
        (c_str, actual_width, actual_height)
    }
}

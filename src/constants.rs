//
// Copyright (C) 2019,2025 Robert Gill <rtgill82@gmail.com>
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

//!
//! Constants used by the newt library.
//!

use newt_sys::*;

const NEWT_FLAGS_SET:    newtFlagsSense = newtFlagsSense_NEWT_FLAGS_SET;
const NEWT_FLAGS_RESET:  newtFlagsSense = newtFlagsSense_NEWT_FLAGS_RESET;
const NEWT_FLAGS_TOGGLE: newtFlagsSense = newtFlagsSense_NEWT_FLAGS_TOGGLE;

///
/// Specifies the method used to set or unset flags when setting flags
/// for a `Component`.
///
#[repr(C)]
#[derive(Clone,Copy)]
pub enum FlagsSense {
    /// Set the specified flags that are not currently set.
    Set    = NEWT_FLAGS_SET    as isize,

    /// Unset the specified flags that are currently set.
    Reset  = NEWT_FLAGS_RESET  as isize,

    /// Toggle the specified flags.
    Toggle = NEWT_FLAGS_TOGGLE as isize
}

///
/// Creates a new color set which can be assigned a pair of colors.
///
/// See the [`set_color`][set_color] function for defining the colors of a
/// color set.
///
/// * `i` - The custom color set number to be defined.
///
/// `Returns` an _id_ for a new color set (_max predefined color sets_ + `i`).
///
/// [set_color]: crate::set_color
///
pub use newt_sys::NEWT_COLORSET_CUSTOM as COLORSET_CUSTOM;

///
/// The default color sets.
///
/// These can be modified individually using [`set_color`][set_color] or
/// completely with [`set_colors`][set_colors].
///
/// [set_color]: crate::set_color
/// [set_colors]: crate::set_colors
///
pub mod colorset {
    use std::os::raw::c_int;
    use newt_sys::*;

    /// Color set for the root window.
    pub const COLORSET_ROOT: c_int          = NEWT_COLORSET_ROOT;
    /// Color set for window borders.
    pub const COLORSET_BORDER: c_int        = NEWT_COLORSET_BORDER;
    /// Color set for windows.
    pub const COLORSET_WINDOW: c_int        = NEWT_COLORSET_WINDOW;
    /// Color set for window/button shadows.
    pub const COLORSET_SHADOW: c_int        = NEWT_COLORSET_SHADOW;
    /// Color set for window titles.
    pub const COLORSET_TITLE: c_int         = NEWT_COLORSET_TITLE;
    /// Color set for [`Buttons`](crate::widgets::Button).
    pub const COLORSET_BUTTON: c_int        = NEWT_COLORSET_BUTTON;
    /// Color set for activated [`Buttons`](crate::widgets::Button).
    pub const COLORSET_ACTBUTTON: c_int     = NEWT_COLORSET_ACTBUTTON;
    /// Color set for [`Checkboxes`](crate::widgets::Checkbox).
    pub const COLORSET_CHECKBOX: c_int      = NEWT_COLORSET_CHECKBOX;
    /// Color set for activated [`Checkboxes`](crate::widgets::Checkbox).
    pub const COLORSET_ACTCHECKBOX: c_int   = NEWT_COLORSET_ACTCHECKBOX;
    /// Color set for [`Entrys`](crate::widgets::Entry).
    pub const COLORSET_ENTRY: c_int         = NEWT_COLORSET_ENTRY;
    /// Color set for [`Labels`](crate::widgets::Label).
    pub const COLORSET_LABEL: c_int         = NEWT_COLORSET_LABEL;
    /// Color set for [`Listboxes`](crate::widgets::Listbox).
    pub const COLORSET_LISTBOX: c_int       = NEWT_COLORSET_LISTBOX;
    /// Color set for activated [`Listboxes`](crate::widgets::Listbox).
    pub const COLORSET_ACTLISTBOX: c_int    = NEWT_COLORSET_ACTLISTBOX;
    /// Color set for [`Textboxes`](crate::widgets::Textbox).
    pub const COLORSET_TEXTBOX: c_int       = NEWT_COLORSET_TEXTBOX;
    /// Color set for activated [`Textboxes`](crate::widgets::Textbox).
    pub const COLORSET_ACTTEXTBOX: c_int    = NEWT_COLORSET_ACTTEXTBOX;
    /// Color set for the [help line](crate::push_help_line).
    pub const COLORSET_HELPLINE: c_int      = NEWT_COLORSET_HELPLINE;
    /// Color set for the [root window text](crate::draw_root_text).
    pub const COLORSET_ROOTTEXT: c_int      = NEWT_COLORSET_ROOTTEXT;
    /// Color set for the empty [`Scale`](crate::widgets::Scale).
    pub const COLORSET_EMPTYSCALE: c_int    = NEWT_COLORSET_EMPTYSCALE;
    /// Color set for the full [`Scale`](crate::widgets::Scale).
    pub const COLORSET_FULLSCALE: c_int     = NEWT_COLORSET_FULLSCALE;
    /// Color set for disabled [`Entrys`](crate::widgets::Entry).
    pub const COLORSET_DISENTRY: c_int      = NEWT_COLORSET_DISENTRY;
    /// Color set for [`CompactButtons`](crate::widgets::CompactButton).
    pub const COLORSET_COMPACTBUTTON: c_int = NEWT_COLORSET_COMPACTBUTTON;
    /// Color set for selected activated [`Listbox`](crate::widgets::Listbox) items.
    pub const COLORSET_ACTSELLISTBOX: c_int = NEWT_COLORSET_ACTSELLISTBOX;
    /// Color set for selected [`Listbox`](crate::widgets::Listbox) items.
    pub const COLORSET_SELLISTBOX: c_int    = NEWT_COLORSET_SELLISTBOX;
}
pub use crate::constants::colorset::*;

///
/// General flags that effect the behavior of
/// [Components](crate::component::Component).
///
pub mod flags {
    use std::os::raw::c_int;
    use newt_sys::*;

    /// Exit the currently running [`Form`](crate::form::Form)
    /// when the `Enter` key is pressed while the `Component` is focused.
    pub const FLAG_RETURNEXIT: c_int    = NEWT_FLAG_RETURNEXIT;

    /// Do not display entered text in an [`Entry`](crate::widgets::Entry).
    pub const FLAG_HIDDEN: c_int        = NEWT_FLAG_HIDDEN;

    /// Enable scrolling in [`Entry`](crate::widgets::Entry) and
    /// [`Textbox`](crate::widgets::Textbox) widgets.
    pub const FLAG_SCROLL: c_int        = NEWT_FLAG_SCROLL;

    /// Disallow entering of text in an [`Entry`](crate::widgets::Entry).
    pub const FLAG_DISABLED: c_int      = NEWT_FLAG_DISABLED;

    /// Display a border around a [`Listbox`](crate::widgets::Listbox).
    pub const FLAG_BORDER: c_int        = NEWT_FLAG_BORDER;

    /// Enable word wrapping in a [`Textbox`](crate::widgets::Textbox) widget.
    pub const FLAG_WRAP: c_int          = NEWT_FLAG_WRAP;

    /// Disable the default exit on F12 hot key in a
    /// [`Form`](crate::form::Form).
    pub const FLAG_NOF12: c_int         = NEWT_FLAG_NOF12;

    /// Allow selecting multiple items in a
    /// [`Listbox`](crate::widgets::Listbox).
    pub const FLAG_MULTIPLE: c_int      = NEWT_FLAG_MULTIPLE;

    /// Set item as selected when adding a new item to a
    /// [`CheckboxTree`](crate::widgets::CheckboxTree).
    pub const FLAG_SELECTED: c_int      = NEWT_FLAG_SELECTED;

    /// Apparently unused (as of `newt-0.52.25`).
    pub const FLAG_CHECKBOX: c_int      = NEWT_FLAG_CHECKBOX;

    /// Display entered text as `*` in an [`Entry`](crate::widgets::Entry).
    pub const FLAG_PASSWORD: c_int      = NEWT_FLAG_PASSWORD;

    /// Display cursor in a [`Listbox`](crate::widgets::Listbox).
    pub const FLAG_SHOWCURSOR: c_int    = NEWT_FLAG_SHOWCURSOR;
}
pub use crate::constants::flags::*;

///
/// Flags specific to [Forms](crate::form::Form).
///
pub mod form {
    use std::os::raw::c_int;
    use newt_sys::*;

    /// Disable the default exit on F12 hot key.
    pub const FORM_NOF12: c_int         = NEWT_FORM_NOF12;
}
pub use crate::constants::form::*;

///
/// Flags specific to the [CheckboxTree](crate::widgets::CheckboxTree)
/// widget.
///
pub mod checkboxtree {
    use std::os::raw::{c_char,c_int};
    use newt_sys::*;

    /// Hide the check box for items in a [`CheckboxTree`](crate::widgets::CheckboxTree)
    /// making it work work as if a [`Listbox`](crate::widgets::Listbox).
    pub const CHECKBOXTREE_HIDE_BOX: c_int     = NEWT_CHECKBOXTREE_HIDE_BOX;

    /// Make items in a [`CheckboxTree`](crate::widgets::CheckboxTree) unselectable.
    pub const CHECKBOXTREE_UNSELECTABLE: c_int = NEWT_CHECKBOXTREE_UNSELECTABLE;

    /// `Returned` by [`CheckboxTree::get_entry_value()`](crate::widgets::CheckboxTree::get_entry_value)
    /// when a tree node is selected and the tree is collapsed.
    pub const CHECKBOXTREE_COLLAPSED: c_char   = NEWT_CHECKBOXTREE_COLLAPSED;

    /// `Returned` by [`CheckboxTree::get_entry_value()`](crate::widgets::CheckboxTree::get_entry_value)
    /// when a tree node is selected and the tree is expanded.
    pub const CHECKBOXTREE_EXPANDED: c_char    = NEWT_CHECKBOXTREE_EXPANDED;

    /// The default selected value for `CheckboxTree` items. May be used with
    /// [`CheckboxTree::set_entry_value()`](crate::widgets::CheckboxTree::set_entry_value).
    pub const CHECKBOXTREE_SELECTED: char    = NEWT_CHECKBOXTREE_SELECTED as u8 as char;

    /// The default unselected value for `CheckboxTree` items. May be used with
    /// [`CheckboxTree::set_entry_value()`](crate::widgets::CheckboxTree::set_entry_value).
    pub const CHECKBOXTREE_UNSELECTED: char  = NEWT_CHECKBOXTREE_UNSELECTED as u8 as char;

    /// Used at the end of a [`CheckboxTree`](crate::widgets::CheckboxTree)
    /// index array to specify that a node should be appended under a parent node.
    pub const ARG_APPEND: c_int = NEWT_ARG_APPEND;

    /// Used internally to terminate a [`CheckboxTree`](crate::widgets::CheckboxTree)
    /// node index array.
    pub const ARG_LAST: c_int   = NEWT_ARG_LAST;
}
pub use crate::constants::checkboxtree::*;

///
/// Flags specific to the [Entry](crate::widgets::Entry) widget.
///
pub mod entry {
    use std::os::raw::c_int;
    use newt_sys::*;

    /// Enable automatic horizontal scrolling as text is entered.
    pub const ENTRY_SCROLL: c_int       = NEWT_ENTRY_SCROLL;

    /// Do not display entered text.
    pub const ENTRY_HIDDEN: c_int       = NEWT_ENTRY_HIDDEN;

    /// Exit the currently running `Form` when the `Enter` key is pressed
    /// while the `Entry` is focused.
    pub const ENTRY_RETURNEXIT: c_int   = NEWT_ENTRY_RETURNEXIT;

    /// Disable the `Entry`, disallowing entering of text.
    pub const ENTRY_DISABLED: c_int     = NEWT_ENTRY_DISABLED;
}
pub use crate::constants::entry::*;

///
/// Flags specific to the [Listbox](crate::widgets::Listbox) widget.
///
pub mod listbox {
    use std::os::raw::c_int;
    use newt_sys::*;

    /// Exit the currently running `Form` when the `Enter` key is pressed
    /// while the `Listbox`is focused.
    pub const LISTBOX_RETURNEXIT: c_int = NEWT_LISTBOX_RETURNEXIT;
}
pub use crate::constants::listbox::*;

///
/// Flags specific to the [Textbox](crate::widgets::Textbox) widget.
///
pub mod textbox {
    use std::os::raw::c_int;
    use newt_sys::*;

    /// Enable word wrapping.
    pub const TEXTBOX_WRAP: c_int       = NEWT_TEXTBOX_WRAP;

    /// Enable vertical scrolling.
    pub const TEXTBOX_SCROLL: c_int     = NEWT_TEXTBOX_SCROLL;
}
pub use crate::constants::textbox::*;

///
/// Flags and constants specific to [Grids][crate::grid].
///
pub mod grid {
    use std::os::raw::c_int;
    use newt_sys::*;

    /// Anchor a component towards the left of its cell.
    pub const GRID_ANCHOR_LEFT: c_int   = NEWT_ANCHOR_LEFT;
    /// Anchor a component towards the right of its cell.
    pub const GRID_ANCHOR_RIGHT: c_int  = NEWT_ANCHOR_RIGHT;
    /// Anchor a component towards the top of its cell.
    pub const GRID_ANCHOR_TOP: c_int    = NEWT_ANCHOR_TOP;
    /// Anchor a component towards the bottom of its cell.
    pub const GRID_ANCHOR_BOTTOM: c_int = NEWT_ANCHOR_BOTTOM;

    /// Grow a sub-Grid to fill the X-dimension of the parent cell.
    pub const GRID_FLAG_GROWX: c_int = NEWT_GRID_FLAG_GROWX;
    /// Grow a sub-Grid to fill the Y-dimension of the parent cell.
    pub const GRID_FLAG_GROWY: c_int = NEWT_GRID_FLAG_GROWY;

    /// Used to terminate a list of elements when adding them to a
    /// [`Grid`](crate::grid::Grid).
    pub const GRID_EMPTY: newtGridElement     = newtGridElement_NEWT_GRID_EMPTY;
    /// Specify that the element being added to a [`Grid`](crate::grid::Grid)
    /// is a Component.
    pub const GRID_COMPONENT: newtGridElement = newtGridElement_NEWT_GRID_COMPONENT;
    /// Specify that the element being added to a [`Grid`](crate::grid::Grid)
    /// is a sub-Grid.
    pub const GRID_SUBGRID: newtGridElement   = newtGridElement_NEWT_GRID_SUBGRID;

    #[deprecated(since="0.6.11", note="please use `GRID_ANCHOR_LEFT`")]
    /// Alias for [`GRID_ANCHOR_LEFT`].
    pub const ANCHOR_LEFT: c_int   = NEWT_ANCHOR_LEFT;
    #[deprecated(since="0.6.11", note="please use `GRID_ANCHOR_RIGHT`")]
    /// Alias for [`GRID_ANCHOR_RIGHT`].
    pub const ANCHOR_RIGHT: c_int  = NEWT_ANCHOR_RIGHT;
    #[deprecated(since="0.6.11", note="please use `GRID_ANCHOR_TOP`")]
    /// Alias for [`GRID_ANCHOR_TOP`].
    pub const ANCHOR_TOP: c_int    = NEWT_ANCHOR_TOP;
    #[deprecated(since="0.6.11", note="please use `GRID_ANCHOR_BOTTOM`")]
    /// Alias for [`GRID_ANCHOR_BOTTOM`].
    pub const ANCHOR_BOTTOM: c_int = NEWT_ANCHOR_BOTTOM;

    #[deprecated(since="0.6.11", note="please use `GRID_EMPTY`")]
    /// Alias for [`GRID_EMPTY`].
    pub const NEWT_GRID_EMPTY: newtGridElement     = GRID_EMPTY;
    #[deprecated(since="0.6.11", note="please use `GRID_COMPONENT`")]
    /// Alias for [`GRID_COMPONENT`].
    pub const NEWT_GRID_COMPONENT: newtGridElement = GRID_COMPONENT;
    #[deprecated(since="0.6.11", note="please use `GRID_SUBGRID`")]
    /// Alias for [`GRID_SUBGRID`].
    pub const NEWT_GRID_SUBGRID: newtGridElement   = GRID_SUBGRID;
}
pub use crate::constants::grid::*;

///
/// Constants representing keyboard keys.
///
pub mod keys {
    use std::os::raw::c_int;
    use newt_sys::*;

    /// Tab Key
    pub const KEY_TAB: c_int         = NEWT_KEY_TAB;
    /// Enter Key
    pub const KEY_ENTER: c_int       = NEWT_KEY_ENTER;
    /// Suspend (Ctrl-Z)
    pub const KEY_SUSPEND: c_int     = NEWT_KEY_SUSPEND;
    /// Escape Key
    pub const KEY_ESCAPE: c_int      = NEWT_KEY_ESCAPE;
    /// Alias for [`KEY_ENTER`]
    pub const KEY_RETURN: c_int      = NEWT_KEY_ENTER;

    // FIXME: Does this need to be public?
    /// Base Key Value
    pub const KEY_EXTRA_BASE: c_int  = NEWT_KEY_EXTRA_BASE;
    /// Up Arrow Key
    pub const KEY_UP: c_int          = NEWT_KEY_UP;
    /// Down Arrow Key
    pub const KEY_DOWN: c_int        = NEWT_KEY_DOWN;
    /// Left Arrow Key
    pub const KEY_LEFT: c_int        = NEWT_KEY_LEFT;
    /// Right Arrow Key
    pub const KEY_RIGHT: c_int       = NEWT_KEY_RIGHT;
    /// Backspace Key
    pub const KEY_BKSPC: c_int       = NEWT_KEY_BKSPC;
    /// Delete Key
    pub const KEY_DELETE: c_int      = NEWT_KEY_DELETE;
    /// Home Key
    pub const KEY_HOME: c_int        = NEWT_KEY_HOME;
    /// End Key
    pub const KEY_END: c_int         = NEWT_KEY_END;
    /// Untab Key
    pub const KEY_UNTAB: c_int       = NEWT_KEY_UNTAB;
    /// Page Up Key
    pub const KEY_PGUP: c_int        = NEWT_KEY_PGUP;
    /// Page Down Key
    pub const KEY_PGDN: c_int        = NEWT_KEY_PGDN;
    /// Insert Key
    pub const KEY_INSERT: c_int      = NEWT_KEY_INSERT;

    /// F1 Key
    pub const KEY_F1: c_int          = NEWT_KEY_F1;
    /// F2 Key
    pub const KEY_F2: c_int          = NEWT_KEY_F2;
    /// F3 Key
    pub const KEY_F3: c_int          = NEWT_KEY_F3;
    /// F4 Key
    pub const KEY_F4: c_int          = NEWT_KEY_F4;
    /// F5 Key
    pub const KEY_F5: c_int          = NEWT_KEY_F5;
    /// F6 Key
    pub const KEY_F6: c_int          = NEWT_KEY_F6;
    /// F7 Key
    pub const KEY_F7: c_int          = NEWT_KEY_F7;
    /// F8 Key
    pub const KEY_F8: c_int          = NEWT_KEY_F8;
    /// F9 Key
    pub const KEY_F9: c_int          = NEWT_KEY_F9;
    /// F10 Key
    pub const KEY_F10: c_int         = NEWT_KEY_F10;
    /// F11 Key
    pub const KEY_F11: c_int         = NEWT_KEY_F11;
    /// F12 Key
    pub const KEY_F12: c_int         = NEWT_KEY_F12;

    /// Returned on `SIGWINCH`, windows may need resizing.
    pub const KEY_RESIZE: c_int      = NEWT_KEY_RESIZE;

    // FIXME: Does not appear to be used externally to `newt`.
    /// Returned when an error occurs
    pub const KEY_ERROR: c_int       = NEWT_KEY_ERROR;
}
pub use crate::constants::keys::*;

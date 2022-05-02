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

//!
//! Constants used by the newt library.
//!

use std::os::raw::c_char;
use std::os::raw::c_int;

use newt_sys::*;

const NEWT_FLAGS_SET:    newtFlagsSense = newtFlagsSense_NEWT_FLAGS_SET;
const NEWT_FLAGS_RESET:  newtFlagsSense = newtFlagsSense_NEWT_FLAGS_RESET;
const NEWT_FLAGS_TOGGLE: newtFlagsSense = newtFlagsSense_NEWT_FLAGS_TOGGLE;

#[repr(C)]
pub enum FlagsSense {
    Set    = NEWT_FLAGS_SET    as isize,
    Reset  = NEWT_FLAGS_RESET  as isize,
    Toggle = NEWT_FLAGS_TOGGLE as isize
}

pub const COLORSET_ROOT: c_int          = NEWT_COLORSET_ROOT;
pub const COLORSET_BORDER: c_int        = NEWT_COLORSET_BORDER;
pub const COLORSET_WINDOW: c_int        = NEWT_COLORSET_WINDOW;
pub const COLORSET_SHADOW: c_int        = NEWT_COLORSET_SHADOW;
pub const COLORSET_TITLE: c_int         = NEWT_COLORSET_TITLE;
pub const COLORSET_BUTTON: c_int        = NEWT_COLORSET_BUTTON;
pub const COLORSET_ACTBUTTON: c_int     = NEWT_COLORSET_ACTBUTTON;
pub const COLORSET_CHECKBOX: c_int      = NEWT_COLORSET_CHECKBOX;
pub const COLORSET_ACTCHECKBOX: c_int   = NEWT_COLORSET_ACTCHECKBOX;
pub const COLORSET_ENTRY: c_int         = NEWT_COLORSET_ENTRY;
pub const COLORSET_LABEL: c_int         = NEWT_COLORSET_LABEL;
pub const COLORSET_LISTBOX: c_int       = NEWT_COLORSET_LISTBOX;
pub const COLORSET_ACTLISTBOX: c_int    = NEWT_COLORSET_ACTLISTBOX;
pub const COLORSET_TEXTBOX: c_int       = NEWT_COLORSET_TEXTBOX;
pub const COLORSET_ACTTEXTBOX: c_int    = NEWT_COLORSET_ACTTEXTBOX;
pub const COLORSET_HELPLINE: c_int      = NEWT_COLORSET_HELPLINE;
pub const COLORSET_ROOTTEXT: c_int      = NEWT_COLORSET_ROOTTEXT;
pub const COLORSET_EMPTYSCALE: c_int    = NEWT_COLORSET_EMPTYSCALE;
pub const COLORSET_FULLSCALE: c_int     = NEWT_COLORSET_FULLSCALE;
pub const COLORSET_DISENTRY: c_int      = NEWT_COLORSET_DISENTRY;
pub const COLORSET_COMPACTBUTTON: c_int = NEWT_COLORSET_COMPACTBUTTON;
pub const COLORSET_ACTSELLISTBOX: c_int = NEWT_COLORSET_ACTSELLISTBOX;
pub const COLORSET_SELLISTBOX: c_int    = NEWT_COLORSET_SELLISTBOX;

pub use newt_sys::NEWT_COLORSET_CUSTOM as COLORSET_CUSTOM;

pub const ARG_LAST: c_int   = NEWT_ARG_LAST;
pub const ARG_APPEND: c_int = NEWT_ARG_APPEND;

pub const FLAG_RETURNEXIT: c_int    = NEWT_FLAG_RETURNEXIT;
pub const FLAG_HIDDEN: c_int        = NEWT_FLAG_HIDDEN;
pub const FLAG_SCROLL: c_int        = NEWT_FLAG_SCROLL;
pub const FLAG_DISABLED: c_int      = NEWT_FLAG_DISABLED;
pub const FLAG_BORDER: c_int        = NEWT_FLAG_BORDER;
pub const FLAG_WRAP: c_int          = NEWT_FLAG_WRAP;
pub const FLAG_NOF12: c_int         = NEWT_FLAG_NOF12;
pub const FLAG_MULTIPLE: c_int      = NEWT_FLAG_MULTIPLE;
pub const FLAG_SELECTED: c_int      = NEWT_FLAG_SELECTED;
pub const FLAG_CHECKBOX: c_int      = NEWT_FLAG_CHECKBOX;
pub const FLAG_PASSWORD: c_int      = NEWT_FLAG_PASSWORD;
pub const FLAG_SHOWCURSOR: c_int    = NEWT_FLAG_SHOWCURSOR;

pub const CHECKBOXTREE_UNSELECTABLE: c_int = NEWT_CHECKBOXTREE_UNSELECTABLE;
pub const CHECKBOXTREE_HIDE_BOX: c_int     = NEWT_CHECKBOXTREE_HIDE_BOX;
pub const CHECKBOXTREE_COLLAPSED: c_char   = NEWT_CHECKBOXTREE_COLLAPSED;
pub const CHECKBOXTREE_EXPANDED: c_char    = NEWT_CHECKBOXTREE_EXPANDED;
pub const CHECKBOXTREE_UNSELECTED: c_char  = NEWT_CHECKBOXTREE_UNSELECTED;
pub const CHECKBOXTREE_SELECTED: c_char    = NEWT_CHECKBOXTREE_SELECTED;

pub const LISTBOX_RETURNEXIT: c_int = NEWT_LISTBOX_RETURNEXIT;

pub const ENTRY_SCROLL: c_int       = NEWT_ENTRY_SCROLL;
pub const ENTRY_HIDDEN: c_int       = NEWT_ENTRY_HIDDEN;
pub const ENTRY_RETURNEXIT: c_int   = NEWT_ENTRY_RETURNEXIT;
pub const ENTRY_DISABLED: c_int     = NEWT_ENTRY_DISABLED;

pub const NEWT_GRID_EMPTY: newtGridElement     = newtGridElement_NEWT_GRID_EMPTY;
pub const NEWT_GRID_COMPONENT: newtGridElement = newtGridElement_NEWT_GRID_COMPONENT;
pub const NEWT_GRID_SUBGRID: newtGridElement   = newtGridElement_NEWT_GRID_SUBGRID;

pub const TEXTBOX_WRAP: c_int       = NEWT_TEXTBOX_WRAP;
pub const TEXTBOX_SCROLL: c_int     = NEWT_TEXTBOX_SCROLL;

pub const FORM_NOF12: c_int         = NEWT_FORM_NOF12;

pub const KEY_TAB: c_int         = NEWT_KEY_TAB;
pub const KEY_ENTER: c_int       = NEWT_KEY_ENTER;
pub const KEY_SUSPEND: c_int     = NEWT_KEY_SUSPEND;
pub const KEY_ESCAPE: c_int      = NEWT_KEY_ESCAPE;
pub const KEY_RETURN: c_int      = NEWT_KEY_ENTER;

pub const KEY_EXTRA_BASE: c_int  = NEWT_KEY_EXTRA_BASE;
pub const KEY_UP: c_int          = NEWT_KEY_UP;
pub const KEY_DOWN: c_int        = NEWT_KEY_DOWN;
pub const KEY_LEFT: c_int        = NEWT_KEY_LEFT;
pub const KEY_RIGHT: c_int       = NEWT_KEY_RIGHT;
pub const KEY_BKSPC: c_int       = NEWT_KEY_BKSPC;
pub const KEY_DELETE: c_int      = NEWT_KEY_DELETE;
pub const KEY_HOME: c_int        = NEWT_KEY_HOME;
pub const KEY_END: c_int         = NEWT_KEY_END;
pub const KEY_UNTAB: c_int       = NEWT_KEY_UNTAB;
pub const KEY_PGUP: c_int        = NEWT_KEY_PGUP;
pub const KEY_PGDN: c_int        = NEWT_KEY_PGDN;
pub const KEY_INSERT: c_int      = NEWT_KEY_INSERT;

pub const KEY_F1: c_int          = NEWT_KEY_F1;
pub const KEY_F2: c_int          = NEWT_KEY_F2;
pub const KEY_F3: c_int          = NEWT_KEY_F3;
pub const KEY_F4: c_int          = NEWT_KEY_F4;
pub const KEY_F5: c_int          = NEWT_KEY_F5;
pub const KEY_F6: c_int          = NEWT_KEY_F6;
pub const KEY_F7: c_int          = NEWT_KEY_F7;
pub const KEY_F8: c_int          = NEWT_KEY_F8;
pub const KEY_F9: c_int          = NEWT_KEY_F9;
pub const KEY_F10: c_int         = NEWT_KEY_F10;
pub const KEY_F11: c_int         = NEWT_KEY_F11;
pub const KEY_F12: c_int         = NEWT_KEY_F12;

pub const KEY_RESIZE: c_int      = NEWT_KEY_RESIZE;
pub const KEY_ERROR: c_int       = NEWT_KEY_ERROR;

pub const ANCHOR_LEFT: c_int   = NEWT_ANCHOR_LEFT;
pub const ANCHOR_RIGHT: c_int  = NEWT_ANCHOR_RIGHT;
pub const ANCHOR_TOP: c_int    = NEWT_ANCHOR_TOP;
pub const ANCHOR_BOTTOM: c_int = NEWT_ANCHOR_BOTTOM;

pub const GRID_FLAG_GROWX: c_int = NEWT_GRID_FLAG_GROWX;
pub const GRID_FLAG_GROWY: c_int = NEWT_GRID_FLAG_GROWY;

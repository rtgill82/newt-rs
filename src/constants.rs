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

pub const COLORSET_ROOT: i32          = NEWT_COLORSET_ROOT;
pub const COLORSET_BORDER: i32        = NEWT_COLORSET_BORDER;
pub const COLORSET_WINDOW: i32        = NEWT_COLORSET_WINDOW;
pub const COLORSET_SHADOW: i32        = NEWT_COLORSET_SHADOW;
pub const COLORSET_TITLE: i32         = NEWT_COLORSET_TITLE;
pub const COLORSET_BUTTON: i32        = NEWT_COLORSET_BUTTON;
pub const COLORSET_ACTBUTTON: i32     = NEWT_COLORSET_ACTBUTTON;
pub const COLORSET_CHECKBOX: i32      = NEWT_COLORSET_CHECKBOX;
pub const COLORSET_ACTCHECKBOX: i32   = NEWT_COLORSET_ACTCHECKBOX;
pub const COLORSET_ENTRY: i32         = NEWT_COLORSET_ENTRY;
pub const COLORSET_LABEL: i32         = NEWT_COLORSET_LABEL;
pub const COLORSET_LISTBOX: i32       = NEWT_COLORSET_LISTBOX;
pub const COLORSET_ACTLISTBOX: i32    = NEWT_COLORSET_ACTLISTBOX;
pub const COLORSET_TEXTBOX: i32       = NEWT_COLORSET_TEXTBOX;
pub const COLORSET_ACTTEXTBOX: i32    = NEWT_COLORSET_ACTTEXTBOX;
pub const COLORSET_HELPLINE: i32      = NEWT_COLORSET_HELPLINE;
pub const COLORSET_ROOTTEXT: i32      = NEWT_COLORSET_ROOTTEXT;
pub const COLORSET_EMPTYSCALE: i32    = NEWT_COLORSET_EMPTYSCALE;
pub const COLORSET_FULLSCALE: i32     = NEWT_COLORSET_FULLSCALE;
pub const COLORSET_DISENTRY: i32      = NEWT_COLORSET_DISENTRY;
pub const COLORSET_COMPACTBUTTON: i32 = NEWT_COLORSET_COMPACTBUTTON;
pub const COLORSET_ACTSELLISTBOX: i32 = NEWT_COLORSET_ACTSELLISTBOX;
pub const COLORSET_SELLISTBOX: i32    = NEWT_COLORSET_SELLISTBOX;

pub use newt_sys::NEWT_COLORSET_CUSTOM as COLORSET_CUSTOM;

pub const ARG_LAST: i32   = NEWT_ARG_LAST;
pub const ARG_APPEND: i32 = NEWT_ARG_APPEND;

pub const FLAG_RETURNEXIT: i32    = NEWT_FLAG_RETURNEXIT;
pub const FLAG_HIDDEN: i32        = NEWT_FLAG_HIDDEN;
pub const FLAG_SCROLL: i32        = NEWT_FLAG_SCROLL;
pub const FLAG_DISABLED: i32      = NEWT_FLAG_DISABLED;
pub const FLAG_BORDER: i32        = NEWT_FLAG_BORDER;
pub const FLAG_WRAP: i32          = NEWT_FLAG_WRAP;
pub const FLAG_NOF12: i32         = NEWT_FLAG_NOF12;
pub const FLAG_MULTIPLE: i32      = NEWT_FLAG_MULTIPLE;
pub const FLAG_SELECTED: i32      = NEWT_FLAG_SELECTED;
pub const FLAG_CHECKBOX: i32      = NEWT_FLAG_CHECKBOX;
pub const FLAG_PASSWORD: i32      = NEWT_FLAG_PASSWORD;
pub const FLAG_SHOWCURSOR: i32    = NEWT_FLAG_SHOWCURSOR;

pub const CHECKBOXTREE_UNSELECTABLE: i32 = NEWT_CHECKBOXTREE_UNSELECTABLE;
pub const CHECKBOXTREE_HIDE_BOX: i32     = NEWT_CHECKBOXTREE_HIDE_BOX;
pub const CHECKBOXTREE_COLLAPSED: i8     = NEWT_CHECKBOXTREE_COLLAPSED;
pub const CHECKBOXTREE_EXPANDED: i8      = NEWT_CHECKBOXTREE_EXPANDED;
pub const CHECKBOXTREE_UNSELECTED: i8    = NEWT_CHECKBOXTREE_UNSELECTED;
pub const CHECKBOXTREE_SELECTED: i8      = NEWT_CHECKBOXTREE_SELECTED;

pub const LISTBOX_RETURNEXIT: i32 = NEWT_LISTBOX_RETURNEXIT;

pub const ENTRY_SCROLL: i32       = NEWT_ENTRY_SCROLL;
pub const ENTRY_HIDDEN: i32       = NEWT_ENTRY_HIDDEN;
pub const ENTRY_RETURNEXIT: i32   = NEWT_ENTRY_RETURNEXIT;
pub const ENTRY_DISABLED: i32     = NEWT_ENTRY_DISABLED;

pub const NEWT_GRID_EMPTY: newtGridElement     = newtGridElement_NEWT_GRID_EMPTY;
pub const NEWT_GRID_COMPONENT: newtGridElement = newtGridElement_NEWT_GRID_COMPONENT;
pub const NEWT_GRID_SUBGRID: newtGridElement   = newtGridElement_NEWT_GRID_SUBGRID;

pub const TEXTBOX_WRAP: i32       = NEWT_TEXTBOX_WRAP;
pub const TEXTBOX_SCROLL: i32     = NEWT_TEXTBOX_SCROLL;

pub const FORM_NOF12: i32         = NEWT_FORM_NOF12;

pub const KEY_TAB: i32         = NEWT_KEY_TAB;
pub const KEY_ENTER: i32       = NEWT_KEY_ENTER;
pub const KEY_SUSPEND: i32     = NEWT_KEY_SUSPEND;
pub const KEY_ESCAPE: i32      = NEWT_KEY_ESCAPE;
pub const KEY_RETURN: i32      = NEWT_KEY_ENTER;

pub const KEY_EXTRA_BASE: i32  = NEWT_KEY_EXTRA_BASE;
pub const KEY_UP: i32          = NEWT_KEY_UP;
pub const KEY_DOWN: i32        = NEWT_KEY_DOWN;
pub const KEY_LEFT: i32        = NEWT_KEY_LEFT;
pub const KEY_RIGHT: i32       = NEWT_KEY_RIGHT;
pub const KEY_BKSPC: i32       = NEWT_KEY_BKSPC;
pub const KEY_DELETE: i32      = NEWT_KEY_DELETE;
pub const KEY_HOME: i32        = NEWT_KEY_HOME;
pub const KEY_END: i32         = NEWT_KEY_END;
pub const KEY_UNTAB: i32       = NEWT_KEY_UNTAB;
pub const KEY_PGUP: i32        = NEWT_KEY_PGUP;
pub const KEY_PGDN: i32        = NEWT_KEY_PGDN;
pub const KEY_INSERT: i32      = NEWT_KEY_INSERT;

pub const KEY_F1: i32          = NEWT_KEY_F1;
pub const KEY_F2: i32          = NEWT_KEY_F2;
pub const KEY_F3: i32          = NEWT_KEY_F3;
pub const KEY_F4: i32          = NEWT_KEY_F4;
pub const KEY_F5: i32          = NEWT_KEY_F5;
pub const KEY_F6: i32          = NEWT_KEY_F6;
pub const KEY_F7: i32          = NEWT_KEY_F7;
pub const KEY_F8: i32          = NEWT_KEY_F8;
pub const KEY_F9: i32          = NEWT_KEY_F9;
pub const KEY_F10: i32         = NEWT_KEY_F10;
pub const KEY_F11: i32         = NEWT_KEY_F11;
pub const KEY_F12: i32         = NEWT_KEY_F12;

pub const KEY_RESIZE: i32      = NEWT_KEY_RESIZE;
pub const KEY_ERROR: i32       = NEWT_KEY_ERROR;

pub const ANCHOR_LEFT: i32   = NEWT_ANCHOR_LEFT;
pub const ANCHOR_RIGHT: i32  = NEWT_ANCHOR_RIGHT;
pub const ANCHOR_TOP: i32    = NEWT_ANCHOR_TOP;
pub const ANCHOR_BOTTOM: i32 = NEWT_ANCHOR_BOTTOM;

pub const GRID_FLAG_GROWX: i32 = NEWT_GRID_FLAG_GROWX;
pub const GRID_FLAG_GROWY: i32 = NEWT_GRID_FLAG_GROWY;

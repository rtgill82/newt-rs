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

pub const ARG_LAST: i32   = -100000;
pub const ARG_APPEND: i32 = -1;

pub const FLAG_RETURNEXIT: i32    = (1 << 0);
pub const FLAG_HIDDEN: i32        = (1 << 1);
pub const FLAG_SCROLL: i32        = (1 << 2);
pub const FLAG_DISABLED: i32      = (1 << 3);
pub const FLAG_BORDER: i32        = (1 << 5);
pub const FLAG_WRAP: i32          = (1 << 6);
pub const FLAG_NOF12: i32         = (1 << 7);
pub const FLAG_MULTIPLE: i32      = (1 << 8);
pub const FLAG_SELECTED: i32      = (1 << 9);
pub const FLAG_CHECKBOX: i32      = (1 << 10);
pub const FLAG_PASSWORD: i32      = (1 << 11);
pub const FLAG_SHOWCURSOR: i32    = (1 << 12);

pub const FD_READ: i32   = (1 << 0);
pub const FD_WRITE: i32  = (1 << 1);
pub const FD_EXCEPT: i32 = (1 << 2);

pub const KEY_TAB: i32     = '\t' as i32;
pub const KEY_ENTER: i32   = '\r' as i32;
pub const KEY_SUSPEND: i32 = 0x1A;
pub const KEY_ESCAPE: i32  = 0x1B;
pub const KEY_RETURN: i32  = KEY_ENTER;

pub const KEY_EXTRA_BASE: i32  = 0x8000;
pub const KEY_UP: i32     = KEY_EXTRA_BASE + 1;
pub const KEY_DOWN: i32   = KEY_EXTRA_BASE + 2;
pub const KEY_LEFT: i32   = KEY_EXTRA_BASE + 4;
pub const KEY_RIGHT: i32  = KEY_EXTRA_BASE + 5;
pub const KEY_BKSPC: i32  = KEY_EXTRA_BASE + 6;
pub const KEY_DELETE: i32 = KEY_EXTRA_BASE + 7;
pub const KEY_HOME: i32   = KEY_EXTRA_BASE + 8;
pub const KEY_END: i32    = KEY_EXTRA_BASE + 9;
pub const KEY_UNTAB: i32  = KEY_EXTRA_BASE + 10;
pub const KEY_PGUP: i32   = KEY_EXTRA_BASE + 11;
pub const KEY_PGDN: i32   = KEY_EXTRA_BASE + 12;
pub const KEY_INSERT: i32 = KEY_EXTRA_BASE + 13;

pub const KEY_F1: i32  = KEY_EXTRA_BASE + 101;
pub const KEY_F2: i32  = KEY_EXTRA_BASE + 102;
pub const KEY_F3: i32  = KEY_EXTRA_BASE + 103;
pub const KEY_F4: i32  = KEY_EXTRA_BASE + 104;
pub const KEY_F5: i32  = KEY_EXTRA_BASE + 105;
pub const KEY_F6: i32  = KEY_EXTRA_BASE + 106;
pub const KEY_F7: i32  = KEY_EXTRA_BASE + 107;
pub const KEY_F8: i32  = KEY_EXTRA_BASE + 108;
pub const KEY_F9: i32  = KEY_EXTRA_BASE + 109;
pub const KEY_F10: i32 = KEY_EXTRA_BASE + 110;
pub const KEY_F11: i32 = KEY_EXTRA_BASE + 111;
pub const KEY_F12: i32 = KEY_EXTRA_BASE + 112;

pub const KEY_RESIZE: i32 = KEY_EXTRA_BASE + 113;
pub const KEY_ERROR: i32  = KEY_EXTRA_BASE + 114;

pub const ANCHOR_LEFT: i32   = (1 << 0);
pub const ANCHOR_RIGHT: i32  = (1 << 1);
pub const ANCHOR_TOP: i32    = (1 << 2);
pub const ANCHOR_BOTTOM: i32 = (1 << 3);

pub const GRID_FLAG_GROWX: i32 = (1 << 0);
pub const GRID_FLAG_GROWY: i32 = (1 << 1);

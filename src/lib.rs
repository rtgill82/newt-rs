use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_uint;
use std::ptr;

#[macro_use]
mod intern;

pub enum NewtComponentStruct {}
pub type NewtComponentPtr = *const NewtComponentStruct;

pub trait NewtComponent {
    fn co(&self) -> NewtComponentPtr;
}

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

#[repr(C)]
#[allow(dead_code)]
pub enum ExitStructEnum {
    HotKey,
    Component,
    FDReady,
    Timer,
    Error
}

#[repr(C)]
pub union ExitStructUnion {
    watch: c_int,
    key: c_int,
    co: NewtComponentPtr
}

#[repr(C)]
pub struct ExitStruct {
    reason: ExitStructEnum,
    u: ExitStructUnion
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

newt_component!(RawComponent);
struct RawComponent {
    co: NewtComponentPtr
}

newt_component!(CompactButton);
pub struct CompactButton {
    co: NewtComponentPtr
}

impl CompactButton {
    pub fn new(left: i32, top: i32, text: &str) -> CompactButton {
        #[link(name="newt")]
        extern "C" {
            fn newtCompactButton(left: c_int, top: c_int, text: *const c_char)
                -> NewtComponentPtr;
        }

        let c_str = CString::new(text).unwrap();
        CompactButton {
            co: unsafe { newtCompactButton(left, top, c_str.as_ptr()) }
        }
    }
}

newt_component!(Button);
pub struct Button {
    co: NewtComponentPtr
}

impl Button {
    pub fn new(left: i32, top: i32, text: &str) -> Button {
        #[link(name="newt")]
        extern "C" {
            fn newtButton(left: c_int, top: c_int, text: *const c_char)
                -> NewtComponentPtr;
        }

        let c_str = CString::new(text).unwrap();
        Button {
            co: unsafe { newtButton(left, top, c_str.as_ptr()) }
        }
    }
}

newt_component!(Checkbox);
pub struct Checkbox {
    co: NewtComponentPtr
}

impl Checkbox {
    pub fn new(left: i32, top: i32, text: &str, def_value: i8, seq: &[u8])
        -> Checkbox {
        #[link(name="newt")]
        extern "C" {
            fn newtCheckbox(left: c_int, top: c_int, text: *const c_char,
                            defValue: c_char, seq: *const c_char,
                            result: *mut c_char) -> NewtComponentPtr;
        }
        let c_text = CString::new(text).unwrap();
        let s_seq = String::from_utf8_lossy(seq);
        let c_seq = CString::new(s_seq.into_owned()).unwrap();
        Checkbox {
            co: unsafe {
                newtCheckbox(left, top, c_text.as_ptr(), def_value,
                             c_seq.as_ptr(), ptr::null_mut())
            }
        }
    }

    pub fn get_value(&self) -> i8 {
        #[link(name="newt")]
        extern "C" { fn newtCheckboxGetValue(co: NewtComponentPtr) -> c_char; }
        unsafe { newtCheckboxGetValue(self.co) }
    }

    pub fn set_value(&self, value: i8) {
        #[link(name="newt")]
        extern "C" {
            fn newtCheckboxSetValue(co: NewtComponentPtr, value: c_char);
        }
        unsafe { newtCheckboxSetValue(self.co, value); }
    }

    pub fn set_flags(&self, flags: i32, sense: FlagsSense) {
        #[link(name="newt")]
        extern "C" {
            fn newtCheckboxSetFlags(co: NewtComponentPtr, flags: c_int,
                                    sense: FlagsSense);
        }
        unsafe { newtCheckboxSetFlags(self.co, flags, sense); }
    }
}

newt_component!(Radiobutton);
pub struct Radiobutton {
    co: NewtComponentPtr
}

impl Radiobutton {
    pub fn new(left: i32, top: i32, text: &str, is_default: i32,
               prev_button: Option<Radiobutton>) -> Radiobutton {
        #[link(name="newt")]
        extern "C" {
            fn newtRadiobutton(left: c_int, top: c_int, text: *const c_char,
                               isDefault: c_int, prevButton: NewtComponentPtr)
                -> NewtComponentPtr;
        }

        let c_text = CString::new(text).unwrap();
        let ptr = match prev_button {
            Some(radio_button) => radio_button.co,
            None => ptr::null()
        };

        Radiobutton {
            co: unsafe {
                newtRadiobutton(left, top, c_text.as_ptr(), is_default, ptr)
            }
        }
    }

    pub fn get_current(&self) -> Radiobutton {
        #[link(name="newt")]
        extern "C" {
            fn newtRadioGetCurrent(setMember: NewtComponentPtr)
                -> NewtComponentPtr;
        }

        Radiobutton {
            co: unsafe { newtRadioGetCurrent(self.co) }
        }
    }

    pub fn set_current(&self) {
        #[link(name="newt")]
        extern "C" {
            fn newtRadioSetCurrent(setMember: NewtComponentPtr);
        }

        unsafe { newtRadioSetCurrent(self.co) }
    }
}

/*
list_item
*/

newt_component!(Label);
pub struct Label {
    co: NewtComponentPtr
}

impl Label  {
    pub fn new(left: i32, top: i32, text: &str) -> Label {
        #[link(name="newt")]
        extern "C" {
            fn newtLabel(left: c_int, top: c_int, text: *const c_char)
                -> NewtComponentPtr;
        }

        let c_text = CString::new(text).unwrap();
        Label {
            co: unsafe { newtLabel(left, top, c_text.as_ptr()) }
        }
    }

    pub fn set_text(&self, text: &str) {
        #[link(name="newt")]
        extern "C" {
            fn newtLabelSetText(co: NewtComponentPtr, text: *const c_char);
        }

        let c_text = CString::new(text).unwrap();
        unsafe { newtLabelSetText(self.co, c_text.as_ptr()); }
    }

    pub fn set_colors(&self, colorset: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtLabelSetColors(co: NewtComponentPtr, colorset: c_int);
        }

        unsafe { newtLabelSetColors(self.co, colorset); }
    }
}

pub struct Textbox {
    co: NewtComponentPtr
}

impl Textbox {
    pub fn new(left: i32, top: i32, width: i32, height: i32, flags: i32)
          -> Textbox {
        #[link(name="newt")]
        extern "C" {
           fn  newtTextbox(left: c_int, top: c_int, width: c_int,
                           height: c_int, flags: c_int) -> NewtComponentPtr;
        }
        Textbox {
            co: unsafe { newtTextbox(left, top, width, height, flags) }
        }
    }

    pub fn new_reflowed(left: i32, top: i32, text: &str, width: i32,
                        flex_down: i32, flex_up: i32, flags: i32) -> Textbox {
        #[link(name="newt")]
        extern "C" {
           fn  newtTextboxReflowed(left: c_int, top: c_int,
                                   text: *const c_char, width: c_int,
                                   flexDown: c_int, flexUp: c_int,
                                   flags: c_int) -> NewtComponentPtr;
        }

        let c_text = CString::new(text).unwrap();
        Textbox {
            co: unsafe {
                newtTextboxReflowed(left, top, c_text.as_ptr(), width,
                                    flex_down, flex_up, flags)
            }
        }
    }

    pub fn set_text(&self, text: &str) {
        #[link(name="newt")]
        extern "C" {
            fn newtTextboxSetText(co: NewtComponentPtr, text: *const c_char);
        }
        let c_text = CString::new(text).unwrap();
        unsafe { newtTextboxSetText(self.co, c_text.as_ptr()); }
    }

    pub fn set_height(&self, height: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtTextboxSetHeight(co: NewtComponentPtr, height: c_int);
        }
        unsafe { newtTextboxSetHeight(self.co, height); }
    }

    pub fn get_num_lines(&self) -> i32 {
        #[link(name="newt")]
        extern "C" {
            fn newtTextboxGetNumLines(co: NewtComponentPtr) -> c_int;
        }
        unsafe { newtTextboxGetNumLines(self.co) }
    }

    pub fn set_colors(&self, normal: i32, active: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtTextboxSetColors(co: NewtComponentPtr, normal: c_int,
                                    active: c_int);
        }
        unsafe { newtTextboxSetColors(self.co, normal, active); }
    }
}

newt_component!(Form);
pub struct Form {
    co: NewtComponentPtr
}

impl Form {
    pub fn new(flags: i32) -> Form {
        #[link(name="newt")]
        extern "C" {
            fn newtForm(vert_bar: NewtComponentPtr, help: *const c_char,
                        flags: c_int) -> NewtComponentPtr;
        }

        Form {
            co: unsafe { newtForm(ptr::null(), ptr::null(), flags) }
        }
    }

    pub fn set_timer(&self, millisecs: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtFormSetTimer(form: NewtComponentPtr, millisecs: c_int);
        }
        unsafe{ newtFormSetTimer(self.co, millisecs); }
    }

    pub fn add_component(&self, component: &NewtComponentPtr) {
        #[link(name="newt")]
        extern "C" {
            fn newtFormAddComponent(form: NewtComponentPtr,
                                    co: NewtComponentPtr);
        }
        unsafe { newtFormAddComponent(self.co, *component); }
    }

    pub fn add_components(&self, components: &[NewtComponentPtr]) {
        for component in components.iter() {
            self.add_component(&component);
        }
    }

    pub fn run(&self) -> Result {
        #[link(name="newt")]
        extern "C" {
            fn newtFormRun(form: NewtComponentPtr, es: *mut ExitStruct);
        }
        let mut es = ExitStruct {
            reason: ExitStructEnum::HotKey,
            u: ExitStructUnion { watch: 0 }
        };
        unsafe {
            newtFormRun(self.co, &mut es);
            match es.reason {
                ExitStructEnum::HotKey => Result::HotKey(es.u.key),
                ExitStructEnum::Component =>
                    Result::Component(Box::new(RawComponent { co: es.u.co })),
                ExitStructEnum::FDReady => Result::FDReady(es.u.watch),
                ExitStructEnum::Timer => Result::Timer,
                ExitStructEnum::Error => Result::Err
            }
        }
    }
}

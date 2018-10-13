extern crate std;
extern crate newt_sys;
use std::ops::Drop;
use ptr;

use newt_sys::*;
use components::Component;

mod exit_reason;
pub use self::exit_reason::ExitReason;

#[allow(non_camel_case_types)]
type newtExitReason = newtExitStruct__bindgen_ty_1;
const NEWT_EXIT_HOTKEY: newtExitReason    = newtExitStruct_NEWT_EXIT_HOTKEY;
const NEWT_EXIT_COMPONENT: newtExitReason = newtExitStruct_NEWT_EXIT_COMPONENT;
const NEWT_EXIT_FDREADY: newtExitReason   = newtExitStruct_NEWT_EXIT_FDREADY;
const NEWT_EXIT_TIMER: newtExitReason     = newtExitStruct_NEWT_EXIT_TIMER;
const NEWT_EXIT_ERROR: newtExitReason     = newtExitStruct_NEWT_EXIT_ERROR;

#[allow(non_camel_case_types)]
type newtExitStructUnion = newtExitStruct__bindgen_ty_2;

newt_component!(BaseComponent);
struct BaseComponent {
    co: newtComponent,
    attached_to_form: bool
}

newt_component!(Form);
pub struct Form {
    co: newtComponent,
    attached_to_form: bool
}

impl Drop for Form {
    fn drop(&mut self) {
        unsafe { newtFormDestroy(self.co); }
    }
}

impl Form {
    pub fn new(flags: i32) -> Form {
        Form {
            co: unsafe { newtForm(ptr::null_mut(), ptr::null_mut(), flags) },
            attached_to_form: false
        }
    }

    pub fn add_component(&mut self, component: &mut dyn Component)
            -> Result<(), &'static str> {
        if component.attached_to_form() {
            return Err("Component is already attached to a Form");
        }

        component.attach_to_form();
        unsafe { newtFormAddComponent(self.co, component.co()); }
        return Ok(());
    }

    pub fn add_components(&mut self, components: &mut [&mut dyn Component])
            -> Result<(), &'static str> {
        for component in components.iter_mut() {
            let result = self.add_component(*component);
            if result.is_err() { return result; }
        }
        return Ok(());
    }

    pub fn set_height(&mut self, height: i32) {
        unsafe { newtFormSetHeight(self.co, height); }
    }

    pub fn set_width(&mut self, width: i32) {
        unsafe { newtFormSetWidth(self.co, width); }
    }

    pub fn set_size(&mut self) {
        unsafe { newtFormSetSize(self.co); }
    }

    pub fn add_hot_key(&mut self, key: i32) {
        unsafe { newtFormAddHotKey(self.co, key); }
    }

    pub fn set_timer(&mut self, millisecs: i32) {
        unsafe { newtFormSetTimer(self.co, millisecs); }
    }

    pub fn get_current(&self) -> Box<dyn Component> {
        Box::new(BaseComponent {
            co: unsafe { newtFormGetCurrent(self.co) },
            attached_to_form: true
        })
    }

    pub fn set_current(&mut self, subcomponent: &dyn Component) {
        unsafe { newtFormSetCurrent(self.co, subcomponent.co()); }
    }

    pub fn set_background(&mut self, color: i32) {
        unsafe { newtFormSetBackground(self.co, color); }
    }

    pub fn get_scroll_position(&self) -> i32 {
        unsafe { newtFormGetScrollPosition(self.co) }
    }

    pub fn set_scroll_position(&mut self, position: i32) {
        unsafe { newtFormSetScrollPosition(self.co, position); }
    }

    pub fn run(&self) -> Result<ExitReason, ()> {
        use self::ExitReason::{HotKey,Component,FDReady,Timer};

        let mut es = newtExitStruct {
            reason: NEWT_EXIT_HOTKEY,
            u: newtExitStructUnion { watch: 0 }
        };

        unsafe {
            newtFormRun(self.co, &mut es);
            match es.reason {
                NEWT_EXIT_HOTKEY => Ok(HotKey(es.u.key)),
                NEWT_EXIT_COMPONENT => Ok(
                    Component(Box::new(BaseComponent {
                                       co: es.u.co,
                                       attached_to_form: true
                    }))
                ),
                NEWT_EXIT_FDREADY => Ok(FDReady(es.u.watch)),
                NEWT_EXIT_TIMER => Ok(Timer),
                NEWT_EXIT_ERROR => Err(()),
                _ => panic!("Unexpected Newt exit reason.")
            }
        }
    }

    pub fn draw(&self) {
        unsafe { newtDrawForm(self.co); }
    }
}

extern crate std;
use std::os::raw::{c_char, c_int};
use ptr;

use components::Component;
use components::NewtComponentPtr;

use intern::structs::ExitStructEnum;
use intern::structs::ExitStructUnion;
use intern::structs::ExitStruct;

#[derive(Debug)]
pub enum ExitReason {
    HotKey(i32),
    Component(Box<Component>),
    FDReady(i32),
    Timer
}

newt_component!(RawComponent);
struct RawComponent {
    co: NewtComponentPtr
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

    pub fn set_height(&self, height: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtFormSetHeight(co: NewtComponentPtr, height: c_int);
        }

        unsafe { newtFormSetHeight(self.co, height); }
    }

    pub fn set_width(&self, width: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtFormSetWidth(co: NewtComponentPtr, width: c_int);
        }

        unsafe { newtFormSetWidth(self.co, width); }
    }

    pub fn run(&self) -> Result<ExitReason, ()> {
        use self::ExitReason::{HotKey,Component,FDReady,Timer};

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

                ExitStructEnum::HotKey => Ok(HotKey(es.u.key)),
                ExitStructEnum::Component => Ok(
                    Component(Box::new(RawComponent { co: es.u.co }))
                ),
                ExitStructEnum::FDReady => Ok(FDReady(es.u.watch)),
                ExitStructEnum::Timer => Ok(Timer),
                ExitStructEnum::Error => Err(())
            }
        }
    }

    pub fn draw(&self) {
        #[link(name="newt")]
        extern "C" {
            fn newtDrawForm(co: NewtComponentPtr);
        }

        unsafe { newtDrawForm(self.co); }
    }

    pub fn add_hot_key(&self, key: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtFormAddHotKey(co: NewtComponentPtr, key: c_int);
        }

        unsafe { newtFormAddHotKey(self.co, key); }
    }
}

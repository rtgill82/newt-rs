extern crate std;
use std::ops::Drop;
use std::os::raw::{c_char, c_int};
use ptr;

use components::c_component;
use components::Component;

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
    co: c_component,
    attached_to_form: bool
}

newt_component!(Form);
pub struct Form {
    co: c_component,
    attached_to_form: bool
}

impl Drop for Form {
    fn drop(&mut self) {
        #[link(name="newt")]
        extern "C" {
            fn newtFormDestroy(form: c_component);
        }

        unsafe { newtFormDestroy(self.co); }
    }
}

impl Form {
    pub fn new(flags: i32) -> Form {
        #[link(name="newt")]
        extern "C" {
            fn newtForm(vert_bar: c_component, help: *const c_char,
                        flags: c_int) -> c_component;
        }

        Form {
            attached_to_form: false,
            co: unsafe { newtForm(ptr::null(), ptr::null(), flags) }
        }
    }

    pub fn set_timer(&self, millisecs: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtFormSetTimer(form: c_component, millisecs: c_int);
        }

        unsafe{ newtFormSetTimer(self.co, millisecs); }
    }

    pub fn add_component(&self, component: &mut Component)
            -> Result<(), &'static str> {
        #[link(name="newt")]
        extern "C" {
            fn newtFormAddComponent(form: c_component, co: c_component);
        }

        if component.attached_to_form() {
            return Err("Component is already attached to a Form");
        }

        component.attach_to_form();
        unsafe { newtFormAddComponent(self.co, component.co()); }
        return Ok(());
    }

    pub fn add_components(&self, components: &mut [&mut Component])
            -> Result<(), &'static str> {
        for component in components.iter_mut() {
            let result = self.add_component(*component);
            if result.is_err() { return result; }
        }
        return Ok(());
    }

    pub fn set_height(&self, height: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtFormSetHeight(co: c_component, height: c_int);
        }

        unsafe { newtFormSetHeight(self.co, height); }
    }

    pub fn set_width(&self, width: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtFormSetWidth(co: c_component, width: c_int);
        }

        unsafe { newtFormSetWidth(self.co, width); }
    }

    pub fn run(&self) -> Result<ExitReason, ()> {
        use self::ExitReason::{HotKey,Component,FDReady,Timer};

        #[link(name="newt")]
        extern "C" {
            fn newtFormRun(form: c_component, es: *mut ExitStruct);
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
                    Component(Box::new(RawComponent {
                                         co: es.u.co,
                                         attached_to_form: true
                    }))
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
            fn newtDrawForm(co: c_component);
        }

        unsafe { newtDrawForm(self.co); }
    }

    pub fn add_hot_key(&self, key: i32) {
        #[link(name="newt")]
        extern "C" {
            fn newtFormAddHotKey(co: c_component, key: c_int);
        }

        unsafe { newtFormAddHotKey(self.co, key); }
    }
}

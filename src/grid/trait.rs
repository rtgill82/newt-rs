use crate::Component;
use crate::widgets::{Form,WidgetFns};
use crate::intern::{Child,ComponentPtr};
use newt_sys::*;

///
/// Implements functions shared by `Grid`s.
///
pub trait Grid: Child + Component + ComponentPtr {
    fn add_to_form(&mut self, form: &mut Form) -> Result<(), &'static str> {
        unsafe {
            newtGridAddComponentsToForm(self.as_grid(), form.co(), 1);
        }
        self.add_to_parent()?;
        Ok(())
    }

    fn get_size(&self) -> (i32, i32) {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        unsafe {
            newtGridGetSize(self.as_grid(), &mut width, &mut height);
        }
        (width, height)
    }

    fn place(&mut self, left: i32, top: i32) {
        unsafe {
            newtGridPlace(self.as_grid(), left, top);
        }
    }
}

impl <T: Grid> WidgetFns for T {
    fn takes_focus(&mut self, _value: bool) {
        panic!("`Grid` is not a `Widget`");
    }

    fn get_position(&self) -> (i32, i32) {
        panic!("`Grid` is not a `Widget`");
    }

    fn get_size(&self) -> (i32, i32) {
        panic!("`Grid` is not a `Widget`");
    }
}

use crate::components::{Component,Form};
use crate::intern::Child;
use newt_sys::*;

///
/// Implements functions shared by `Grid`s.
///
pub trait Grid: Child + Component {
    fn add_to_form(&mut self, form: &mut Form) -> Result<(), &'static str> {
        unsafe {
            newtGridAddComponentsToForm(self.grid(), form.co(), 1);
        }
        self.add_to_parent(true)?;
        Ok(())
    }

    fn get_size(&self) -> (i32, i32) {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        unsafe {
            newtGridGetSize(self.grid(), &mut width, &mut height);
        }
        (width, height)
    }

    fn place(&mut self, left: i32, top: i32) {
        unsafe {
            newtGridPlace(self.grid(), left, top);
        }
    }
}

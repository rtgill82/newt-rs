use libc::c_void;
use std::cell::Cell;

use newt_sys::*;
use crate::Component;
use crate::intern::asm;

///
/// Arrange components horizontally.
///
#[derive(Grid)]
pub struct HorizontalGrid<'a> {
    grid: newtGrid,
    added_to_parent: Cell<bool>,
    children: Option<&'a [&'a dyn Component]>
}

impl<'a> HorizontalGrid<'a> {
    ///
    /// Create a new Grid in which the added components are stacked in a
    /// single row.
    ///
    pub fn new(components: &'a [&'a dyn Component])
      -> HorizontalGrid<'a> {
        let mut types: Vec<newtGridElement> = Vec::new();
        let mut values: Vec<newtComponent> = Vec::new();

        for component in components.iter() {
            types.push(component.grid_element_type());
            values.push(component.co());
        }

        types.reverse();
        values.reverse();

        let func = newtGridHStacked as *const c_void;
        let len = components.len();
        let grid = asm::grid_new(func, types, values, len);
        HorizontalGrid {
            grid: grid,
            added_to_parent: Cell::new(false),
            children: Some(components)
        }
    }

    ///
    /// Create a new Grid in which the added components are closely
    /// stacked in a single row.
    ///
    pub fn new_close_stacked(components: &'a [&'a dyn Component])
      -> HorizontalGrid<'a> {
        let mut types: Vec<newtGridElement> = Vec::new();
        let mut values: Vec<newtComponent> = Vec::new();

        for component in components.iter() {
            types.push(component.grid_element_type());
            values.push(component.co());
        }

        types.reverse();
        values.reverse();

        let func = newtGridHCloseStacked as *const c_void;
        let len = components.len();
        let grid = asm::grid_new(func, types, values, len);
        HorizontalGrid {
            grid: grid,
            added_to_parent: Cell::new(false),
            children: Some(components)
        }
    }
}

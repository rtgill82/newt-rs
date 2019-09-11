use libc::c_void;
use std::cell::Cell;

use newt_sys::*;
use crate::component::Component;
use crate::intern::asm;

///
/// Arrange components vertically.
///
#[derive(Grid)]
pub struct VerticalGrid<'a> {
    grid: Cell<newtGrid>,
    added_to_parent: Cell<bool>,
    children: Vec<&'a dyn Component>
}

impl<'a> VerticalGrid<'a> {
    ///
    /// Create a new Grid in which the added components are stacked in a
    /// single column.
    ///
    pub fn new<'t>(components: &'t [&'a dyn Component])
      -> VerticalGrid<'a> {
        let mut types: Vec<newtGridElement> = Vec::new();
        let mut values: Vec<newtComponent> = Vec::new();

        let mut children = Vec::new();
        for component in components.iter() {
            types.push(component.grid_element_type());
            values.push(component.co());
            children.push(*component);
        }

        types.reverse();
        values.reverse();

        let func = newtGridVStacked as *const c_void;
        let len = components.len();
        let grid = asm::grid_new(func, types, values, len);
        VerticalGrid {
            grid: Cell::new(grid),
            added_to_parent: Cell::new(false),
            children
        }
    }

    ///
    /// Create a new Grid in which the added components are closely
    /// stacked in a single column.
    ///
    pub fn new_close_stacked<'t>(components: &'t [&'a dyn Component])
      -> VerticalGrid<'a> {
        let mut types: Vec<newtGridElement> = Vec::new();
        let mut values: Vec<newtComponent> = Vec::new();

        let mut children = Vec::new();
        for component in components.iter() {
            types.push(component.grid_element_type());
            values.push(component.co());
            children.push(*component);
        }

        types.reverse();
        values.reverse();

        let func = newtGridVCloseStacked as *const c_void;
        let len = components.len();
        let grid = asm::grid_new(func, types, values, len);
        VerticalGrid {
            grid: Cell::new(grid),
            added_to_parent: Cell::new(false),
            children
        }
    }
}

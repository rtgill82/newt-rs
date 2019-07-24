use libc::c_void;
use crate::components::Component;
use crate::intern::asm;
use newt_sys::*;

///
/// Place components horizontally.
///
#[derive(Grid)]
pub struct HorizontalGrid<'a> {
    grid: newtGrid,
    added_to_parent: bool,
    children: Option<&'a mut [&'a mut dyn Component]>
}

impl<'a> HorizontalGrid<'a> {
    ///
    /// Create a new Grid in which the added components are stacked in a
    /// single row.
    ///
    pub fn new(components: &'a mut [&'a mut dyn Component])
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
            added_to_parent: false,
            children: Some(components)
        }
    }

    ///
    /// Create a new Grid in which the added components are closely
    /// stacked in a single row.
    ///
    pub fn new_close_stacked(components: &'a mut [&'a mut dyn Component])
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
            added_to_parent: false,
            children: Some(components)
        }
    }
}

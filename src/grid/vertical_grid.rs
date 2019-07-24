use libc::c_void;
use crate::components::Component;
use crate::intern::asm;
use newt_sys::*;

///
/// Place components vertically.
///
#[derive(Grid)]
pub struct VerticalGrid<'a> {
    grid: newtGrid,
    added_to_parent: bool,
    children: Option<&'a mut [&'a mut dyn Component]>
}

impl<'a> VerticalGrid<'a> {
    ///
    /// Create a new Grid in which the added components are stacked in a
    /// single column.
    ///
    pub fn new(components: &'a mut [&'a mut dyn Component])
      -> VerticalGrid<'a> {
        let mut types: Vec<newtGridElement> = Vec::new();
        let mut values: Vec<newtComponent> = Vec::new();

        for component in components.iter() {
            types.push(component.grid_element_type());
            values.push(component.co());
        }

        types.reverse();
        values.reverse();

        let func = newtGridVStacked as *const c_void;
        let len = components.len();
        let grid = asm::grid_new(func, types, values, len);
        VerticalGrid {
            grid: grid,
            added_to_parent: false,
            children: Some(components)
        }
    }

    ///
    /// Create a new Grid in which the added components are closely
    /// stacked in a single column.
    ///
    pub fn new_close_stacked(components: &'a mut [&'a mut dyn Component])
      -> VerticalGrid<'a> {
        let mut types: Vec<newtGridElement> = Vec::new();
        let mut values: Vec<newtComponent> = Vec::new();

        for component in components.iter() {
            types.push(component.grid_element_type());
            values.push(component.co());
        }

        types.reverse();
        values.reverse();

        let func = newtGridVCloseStacked as *const c_void;
        let len = components.len();
        let grid = asm::grid_new(func, types, values, len);
        VerticalGrid {
            grid: grid,
            added_to_parent: false,
            children: Some(components)
        }
    }
}

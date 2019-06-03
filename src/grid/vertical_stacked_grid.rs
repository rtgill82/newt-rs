use crate::components::Component;
use crate::constants::NEWT_GRID_EMPTY;
use newt_sys::*;

///
/// Place components vertically.
///
#[derive(Grid)]
pub struct VerticalStackedGrid<'a> {
    grid: newtGrid,
    added_to_parent: bool,
    children: Option<&'a mut [&'a mut Component]>
}

impl<'a> VerticalStackedGrid<'a> {
    #[cfg(target_arch = "x86_64")]
    ///
    /// Create a new Grid in which the added components are stacked in a
    /// single column.
    ///
    pub fn new(components: &'a mut [&'a mut Component])
      -> VerticalStackedGrid<'a> {
        let mut types: Vec<newtGridElement> = Vec::new();
        let mut values: Vec<newtComponent> = Vec::new();
        let mut grid: newtGrid;

        for component in components.iter() {
            types.push(component.grid_element_type());
            values.push(component.co());
        }

        types.reverse();
        values.reverse();

        let len = components.len();
        grid_asm_x86_64!(newtGridVStacked, types, values, len, grid);
        VerticalStackedGrid {
            grid: grid,
            added_to_parent: false,
            children: Some(components)
        }
    }

    #[cfg(target_arch = "x86_64")]
    ///
    /// Create a new Grid in which the added components are closely
    /// stacked in a single column.
    ///
    pub fn new_close_stacked(components: &'a mut [&'a mut Component])
      -> VerticalStackedGrid<'a> {
        let mut types: Vec<newtGridElement> = Vec::new();
        let mut values: Vec<newtComponent> = Vec::new();
        let mut grid: newtGrid;

        for component in components.iter() {
            types.push(component.grid_element_type());
            values.push(component.co());
        }

        types.reverse();
        values.reverse();

        let len = components.len();
        grid_asm_x86_64!(newtGridVCloseStacked, types, values, len, grid);
        VerticalStackedGrid {
            grid: grid,
            added_to_parent: false,
            children: Some(components)
        }
    }
}

use std::cell::Cell;

use newt_sys::*;
use crate::component::Component;
use crate::constants::{NEWT_GRID_COMPONENT,NEWT_GRID_SUBGRID};

///
/// Create a simple window using sub-grids.
///
#[derive(Grid)]
pub struct BasicWindow<'a> {
    grid: Cell<newtGrid>,
    added_to_parent: Cell<bool>,
    children: Vec<&'a dyn Component>
}

impl<'a> BasicWindow<'a> {
    ///
    /// Create a new BasicWindow grid.
    ///
    /// * `text` - A text component to be displayed at the top of the window.
    /// * `middle` - A sub-grid to display in the middle of the window.
    /// * `buttons` - A sub-grid to display at the bottom of the window,
    ///               hopefully containing buttons.
    ///
    pub fn new(text: &'a dyn Component, middle: &'a dyn Component,
               buttons: &'a dyn Component)
      -> BasicWindow<'a> {

        assert_eq!(text.grid_element_type(), NEWT_GRID_COMPONENT);
        assert_eq!(middle.grid_element_type(), NEWT_GRID_SUBGRID);
        assert_eq!(buttons.grid_element_type(), NEWT_GRID_SUBGRID);

        let grid = unsafe {
            newtGridBasicWindow(text.co(), middle.grid_ptr(), buttons.grid_ptr())
        };

        let mut children: Vec<&'a dyn Component> = Vec::new();
        children.push(text);
        children.push(middle);
        children.push(buttons);

        BasicWindow {
            grid: Cell::new(grid),
            added_to_parent: Cell::new(false),
            children
        }
    }
}

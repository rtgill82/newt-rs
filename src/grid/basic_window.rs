use std::cell::Cell;

use newt_sys::*;
use crate::Component;
use crate::constants::{NEWT_GRID_COMPONENT,NEWT_GRID_SUBGRID};

///
/// Create a simple window using sub-grids.
///
#[derive(Grid)]
pub struct BasicWindow<'a> {
    grid: newtGrid,
    added_to_parent: Cell<bool>,
    children: Option<Vec<&'a dyn Component>>
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
            newtGridBasicWindow(text.co(), middle.as_grid(), buttons.as_grid())
        };

        let mut children: Vec<&'a dyn Component> = Vec::new();
        let middle  = middle as &dyn Component;
        let buttons = buttons as &dyn Component;
        children.push(text);
        children.push(middle);
        children.push(buttons);

        BasicWindow {
            grid,
            added_to_parent: Cell::new(false),
            children: Some(children)
        }
    }
}

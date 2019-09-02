use std::cell::Cell;

use newt_sys::*;
use crate::component::Component;
use crate::constants::{NEWT_GRID_COMPONENT,NEWT_GRID_SUBGRID};

///
/// Create a simple window for a single `Component`.
///
#[derive(Grid)]
pub struct SimpleWindow<'a> {
    grid: newtGrid,
    added_to_parent: Cell<bool>,
    children: Option<Vec<&'a dyn Component>>
}

impl<'a> SimpleWindow<'a> {
    ///
    /// Create a new SimpleWindow grid.
    ///
    /// * `text` - A text component to be displayed at the top of the window.
    /// * `middle` - A single component to display in the middle of the window.
    /// * `buttons` - A sub-grid to display at the bottom of the window,
    ///               hopefully containing buttons.
    ///
    pub fn new(text: &'a dyn Component, middle: &'a dyn Component,
               buttons: &'a dyn Component)
      -> SimpleWindow<'a> {

        assert_eq!(text.grid_element_type(), NEWT_GRID_COMPONENT);
        assert_eq!(middle.grid_element_type(), NEWT_GRID_COMPONENT);
        assert_eq!(buttons.grid_element_type(), NEWT_GRID_SUBGRID);

        let grid = unsafe {
            newtGridSimpleWindow(text.co(), middle.co(), buttons.grid_ptr())
        };

        let mut children: Vec<&'a dyn Component> = Vec::new();
        let middle  = middle as &dyn Component;
        let buttons = buttons as &dyn Component;
        children.push(text);
        children.push(middle);
        children.push(buttons);

        SimpleWindow {
            grid,
            added_to_parent: Cell::new(false),
            children: Some(children)
        }
    }
}

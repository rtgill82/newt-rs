use crate::components::Component;
use crate::constants::{NEWT_GRID_COMPONENT,NEWT_GRID_SUBGRID};
use newt_sys::*;

///
/// Create a simple window using sub-grids.
///
#[derive(Grid)]
pub struct BasicWindow<'a> {
    grid: newtGrid,
    added_to_parent: bool,
    children: Option<Vec<&'a mut Component>>
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
    pub fn new(text: &'a mut Component, middle: &'a mut Component,
               buttons: &'a mut Component)
      -> BasicWindow<'a> {

        assert_eq!(text.grid_element_type(), NEWT_GRID_COMPONENT);
        assert_eq!(middle.grid_element_type(), NEWT_GRID_SUBGRID);
        assert_eq!(buttons.grid_element_type(), NEWT_GRID_SUBGRID);

        let grid = unsafe {
            newtGridBasicWindow(text.co(), middle.grid(), buttons.grid())
        };

        let mut children: Vec<&'a mut Component> = Vec::new();
        let middle: &mut Component = middle as &mut Component;
        let buttons: &mut Component = buttons as &mut Component;
        children.push(text);
        children.push(middle);
        children.push(buttons);

        BasicWindow {
            grid,
            added_to_parent: false,
            children: Some(children)
        }
    }
}

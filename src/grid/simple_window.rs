use crate::components::Component;
use crate::constants::{NEWT_GRID_COMPONENT,NEWT_GRID_SUBGRID};
use newt_sys::*;

///
/// Create a simple window for a single `Component`.
///
#[derive(Grid)]
pub struct SimpleWindow<'a> {
    grid: newtGrid,
    added_to_parent: bool,
    children: Option<Vec<&'a mut dyn Component>>
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
    pub fn new(text: &'a mut dyn Component, middle: &'a mut dyn Component,
               buttons: &'a mut dyn Component)
      -> SimpleWindow<'a> {

        assert_eq!(text.grid_element_type(), NEWT_GRID_COMPONENT);
        assert_eq!(middle.grid_element_type(), NEWT_GRID_COMPONENT);
        assert_eq!(buttons.grid_element_type(), NEWT_GRID_SUBGRID);

        let grid = unsafe {
            newtGridSimpleWindow(text.co(), middle.co(), buttons.grid())
        };

        let mut children: Vec<&'a mut dyn Component> = Vec::new();
        let middle  = middle as &mut dyn Component;
        let buttons = buttons as &mut dyn Component;
        children.push(text);
        children.push(middle);
        children.push(buttons);

        SimpleWindow {
            grid,
            added_to_parent: false,
            children: Some(children)
        }
    }
}

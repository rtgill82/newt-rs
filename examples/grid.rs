#![allow(unused_imports)]
extern crate newt;
#[cfg(feature = "asm")]
use crate::newt::grid::*;
use crate::newt::components::{Component,Form,Label};

#[cfg(feature = "asm")]
pub fn main() {
    newt::init().unwrap();
    newt::cls();

    let rv;

    let mut form = Form::new(None, 0);
    let mut l1 = Label::new(0, 0, "Hello");
    let mut l2 = Label::new(0, 0, "World");
    let components: &mut [&mut dyn Component] = &mut [&mut l1, &mut l2];

    let mut stacked = HorizontalGrid::new(components);
    let mut button_bar = ButtonBar::new(&["Yes", "No", "Maybe"]);

    {
        let mut grid = Grid::new(2, 2);
        grid.set_field(1, 0, &mut stacked, 1, 1, 1, 1, 0, 0);
        grid.set_field(0, 1, &mut button_bar, 1, 1, 1, 1, 0, 0);

        wrapped_window(&grid, "Grids");
        grid.add_to_form(&mut form).unwrap();
        rv = form.run().unwrap();
    }
    newt::finished();

    for (i, button) in button_bar.buttons().iter().enumerate() {
        if rv == *button {
            println!("Button {} pressed.", i);
        }
    }
}

#[cfg(not(feature = "asm"))]
pub fn main() {
}

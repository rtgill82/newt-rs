extern crate newt;
#[cfg(feature = "asm")]
use newt::grid::*;
use newt::prelude::*;

#[test]
#[should_panic]
fn test_form_nullify() {
    let button = Button::new(0, 0, "Ok");
    {
        let mut form = Form::new(None, 0);
        form.add_component(&button).unwrap();
    }
    button.get_position();
}

#[cfg(feature = "asm")]
#[test]
#[should_panic]
fn test_grid_nullify() {
    let button = Button::new(0, 0, "Ok");
    let mut grid = Grid::new(1, 2);

    {
        grid.set_field(0, 0, &button, 1, 1, 1, 1, 0, 0);
        let mut form = Form::new(None, 0);
        grid.add_to_form(&mut form).unwrap();
    }
    button.get_position();
}

#[cfg(feature = "asm")]
#[test]
#[should_panic]
fn test_deep_grid_nullify() {
    let button = Button::new(0, 0, "Ok");
    let hgrid = HorizontalGrid::new(&[&button]);
    {
        let mut grid = Grid::new(1, 2);
        let mut form = Form::new(None, 0);

        grid.set_field(0, 0, &hgrid, 1, 1, 1, 1, 0, 0);
        grid.add_to_form(&mut form).unwrap();
    }
    button.get_position();
}

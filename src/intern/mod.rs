#[macro_use]
pub mod macros;
pub mod data;
pub mod funcs;

pub trait Child {
    fn add_to_parent(&mut self, grid: bool);
    fn added_to_parent(&self) -> bool;
}

pub trait GridElementType {
    fn grid_element_type(&self) -> u32;
}

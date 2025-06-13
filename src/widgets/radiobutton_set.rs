//
// Copyright (C) 2025 Robert Gill <rtgill82@gmail.com>
//
// This file is a part of newt-rs.
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License version 2.1 as published by the Free Software Foundation.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use crate::component::Component;
use crate::form::Form;
use crate::widgets::Radiobutton;
use crate::private::traits::ComponentClone;

///
/// A convenience wrapper for managing [`Radiobutton`]s.
///
pub struct RadiobuttonSet<'a> {
    radiobuttons: Vec<Radiobutton<'a>>
}

impl<'a> RadiobuttonSet<'a> {
    ///
    /// Create an empty `RadiobuttonSet`.
    ///
    pub fn new() -> RadiobuttonSet<'a> {
        RadiobuttonSet { radiobuttons: Vec::new() }
    }

    ///
    /// Add a `Radiobutton` to the `RadiobuttonSet`.
    ///
    /// * `left` - The left-most position of the `Radiobutton`.
    /// * `top` - The top-most position of the `Radiobutton`.
    /// * `text` - The text to be displayed as the label of the `Radiobutton`.
    ///
    /// `Returns` the index number of the `Radiobutton` that was added
    /// to the `RadiobuttonSet`.
    ///
    pub fn add_radiobutton(&mut self, left: i32, top: i32, text: &str) -> usize
    {
        if self.radiobuttons.is_empty() {
            let radiobutton = Radiobutton::new(
                left,
                top,
                text,
                true,
                None
            );

            self.radiobuttons.push(radiobutton);
            self.radiobuttons.len()
        } else {
            let len = self.radiobuttons.len();
            let last = &self.radiobuttons[len-1];
            unsafe {
                let last = Radiobutton::clone_co(last.co(), true);
                let radiobutton = Radiobutton::new_take(
                    left,
                    top,
                    text,
                    false,
                    Some(last)
                );
                self.radiobuttons.push(radiobutton);
            }
            self.radiobuttons.len()
        }
    }

    ///
    /// Get the number of `Radiobuttons` in the `RadiobuttonSet`.
    ///
    /// `Returns` the number of `Radiobuttons` in the `RadiobuttonSet`.
    ///
    pub fn len(&self) -> usize {
        self.radiobuttons.len()
    }

    ///
    /// Add all `Radiobutton`s in the `RadiobuttonSet` to a `Form`.
    ///
    /// * `form` - The `Form` to add the `Radiobutton`s to.
    ///
    pub fn add_to_form(&'a self, form: &mut Form<'a>)
        -> Result<(), &'static str>
    {
        for radiobutton in &self.radiobuttons {
            form.add_component(radiobutton)?;
        }
        Ok(())
    }

    ///
    /// Get the currently selected `Radiobutton` from the `RadiobuttonSet`.
    ///
    /// `Returns` the index of the currently selected `Radiobutton`.
    ///
    pub fn get_current(&self) -> usize {
        fn unreachable() -> ! {
            unreachable!("No `Radiobutton` selected.");
        }

        let rb1 = match self.radiobuttons[0].get_current() {
            Some(rb) => rb,
            None     => unreachable()
        };

        for (i, rb2) in self.radiobuttons.iter().enumerate() {
            if rb2.co() == rb1.co() { return i; }
        }
        unreachable()
    }

    ///
    /// Set the currently selected `Radiobutton` in the `RadiobuttonSet`.
    ///
    /// * `index` - The index number of the `Radiobutton` in the
    ///             `RadiobuttonSet` to be selected.
    ///
    pub fn set_current(&self, index: usize) {
        self.radiobuttons[index].set_current();
    }
}

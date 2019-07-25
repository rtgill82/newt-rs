use newt_sys::newtComponent;
use crate::components::Component;
use crate::components::Entry;
use crate::intern::funcs::newt_entry_set_filter;

///
/// A callback used to filter text entered into an `Entry` component.
///
/// The function or closure associated with the callback should be
/// defined as follows:
///
/// `fn(entry: &Entry, data: Option<&T>, ch: char, cursor_pos: i32) -> char`
///
/// * `entry` - The `Entry` being filtered.
/// * `data` - The optional user data provided.
/// * `ch` - The character entered into the `Entry`.
/// * `cursor_pos` - The current cursor position in the `Entry`.
///
/// The function should return the character to be entered into the
/// `Entry` field or '\0' to ignore the entered character.
///
/// See the example under [Entry][entry_example].
///
/// [entry_example]: ../components/struct.Entry.html#example
///
pub struct EntryFilter<'a, FN: 'a, T: 'a>
where FN: FnMut(&Entry, Option<&T>, char, i32) -> char
{
    function: FN,
    entries: Vec<(&'a Entry, Option<T>)>
}

impl<'a, FN: 'a, T: 'a> EntryFilter<'a, FN, T>
where FN: FnMut(&Entry, Option<&T>, char, i32) -> char
{
    ///
    /// Create a new `EntryFilter`.
    ///
    /// Creates a new `EntryFilter` associated with the `Entry` `entry`.
    /// The function `function` will be called for each character entered.
    ///
    /// * `entry` - The `Entry` to associate with the callback.
    /// * `function` - The function or closure to be called when a character
    ///                is entered.
    /// * `data` - The optonal user data to pass to the function.
    ///
    pub fn new(entry: &'a Entry, function: FN, data: Option<T>)
      -> Box<EntryFilter<'a, FN, T>> {
        let co: newtComponent = entry.co();
        let filter = Box::new(EntryFilter {
            function,
            entries: vec![(entry, data)]
        });
        newt_entry_set_filter(co, filter.as_ref());
        filter
    }

    ///
    /// Associate another `Entry` with the `EntryFilter`.
    ///
    /// * `entry` - The `Entry` to associate with the callback.
    /// * `data` - The optonal user data to pass to the function.
    ///
    pub fn add_entry(&mut self, entry: &'a Entry, data: Option<T>) {
        let co: newtComponent = entry.co();
        self.entries.push((entry, data));
        newt_entry_set_filter(co, self);
    }

    pub(crate) fn call(&mut self, co: newtComponent, ch: char, cursor: i32)
      -> char {
        for (entry, data) in self.entries.iter() {
            if entry.co() == co {
                return (self.function)(*entry, data.as_ref(), ch, cursor);
            }
        }
        '\0'
    }
}

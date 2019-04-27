use newt_sys::newtComponent;
use crate::components::Component;
use crate::components::Entry;
use crate::intern::funcs::newt_entry_set_filter;

pub struct EntryFilter<'a, FN: 'a, T: 'a>
where FN: FnMut(&Entry, Option<&T>, char, i32) -> char
{
    func: FN,
    entries: Vec<(&'a Entry, Option<T>)>
}

impl<'a, FN: 'a, T: 'a> EntryFilter<'a, FN, T>
where FN: FnMut(&Entry, Option<&T>, char, i32) -> char
{
    pub fn new(entry: &'a Entry, function: FN, data: Option<T>)
      -> Box<EntryFilter<'a, FN, T>> {
        let co: newtComponent = entry.co();
        let filter = Box::new(EntryFilter {
            func: function,
            entries: vec![(entry, data)]
        });
        newt_entry_set_filter(co, filter.as_ref());
        return filter;
    }

    pub fn add_entry(&mut self, entry: &'a Entry, data: Option<T>) {
        let co: newtComponent = entry.co();
        self.entries.push((entry, data));
        newt_entry_set_filter(co, self);
    }

    pub(crate) fn call(&mut self, co: newtComponent, ch: char, cursor: i32)
      -> char {
        for (entry, data) in self.entries.iter() {
            if entry.co() == co {
                return (self.func)(*entry, data.as_ref(), ch, cursor);
            }
        }
        return '\0';
    }
}

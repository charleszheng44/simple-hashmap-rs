#![allow(unused)]

use crate::hash_funs;
use std::cmp::PartialEq;
use std::fmt::Display;

#[derive(Clone)]
struct Entry<T, U>
where
    T: ToString + PartialEq + Clone,
    U: Clone,
{
    key: T,
    val: U,
}

struct Map<T, U>
where
    T: ToString + PartialEq + Clone,
    U: Clone,
{
    key_fn: fn(&T) -> usize,
    entries: Vec<Vec<Entry<T, U>>>,
    num_entries: usize,
}

impl<T, U> Map<T, U>
where
    T: ToString + PartialEq + Clone + Display,
    U: Clone,
{
    fn new() -> Self {
        Map {
            key_fn: hash_funs::time33_key_fn,
            entries: vec![vec![]],
            num_entries: 0,
        }
    }

    fn len(&self) -> usize {
        self.num_entries
    }

    fn add(&mut self, k: T, v: U) {
        let i = (self.key_fn)(&k);
        if i >= self.entries.len() {
            // TODO (charleszheng44) resize the vec on this
            // way may be a waste of space.
            self.entries.resize(i + 1, vec![]);
        }
        let entry_lst = &mut self.entries[i];
        match entry_lst.iter_mut().find(|e| e.key == k) {
            // update the entry if exist
            Some(entry) => *entry = Entry { key: k, val: v },
            // append a new entry if not exist
            None => {
                entry_lst.push(Entry { key: k, val: v });
                self.num_entries += 1
            }
        }
    }

    fn remove(&mut self, k: T) -> Option<U> {
        let i = (self.key_fn)(&k);
        if i >= self.entries.len() {
            return None;
        }

        let entry_lst = &mut self.entries[i];
        match entry_lst.iter_mut().position(|e| e.key == k) {
            // remove the entry if found
            Some(index) => {
                self.num_entries -= 1;
                // TODO (charleszheng44) the remove operation is expensive,
                // use zero value
                Some(entry_lst.remove(index).val)
            }
            None => {
                println!("entry with key {} does not exist", k);
                None
            }
        }
    }

    fn get(&self, k: T) -> Option<&U> {
        let i = (self.key_fn)(&k);
        if i > self.entries.len() {
            return None;
        }
        (&self.entries[i])
            .iter()
            .find(|e| e.key == k)
            .map(|e| &e.val)
    }
}

#[cfg(test)]
mod test {
    use super::Map;
    #[test]
    fn map_add_test() {
        let mut m = Map::new();
        m.add("k1", "v1");
        m.add("k100000", "v100000");
        assert_eq!(m.get("k1"), Some(&"v1"));
        assert_eq!(m.get("k2"), None);
        assert_eq!(m.get("k100000"), Some(&"v100000"))
    }
    #[test]
    fn map_remove_test() {
        let mut m = Map::new();
        m.add("k1", "v1");
        m.add("k100000", "v100000");
        assert_eq!(m.get("k1"), Some(&"v1"));
        assert_eq!(m.get("k100000"), Some(&"v100000"));
        m.remove("k1");
        assert_eq!(m.get("k1"), None);
        assert_eq!(m.get("k100000"), Some(&"v100000"));
    }
    #[test]
    fn map_len_test() {
        let mut m = Map::new();
        m.add("k1", "v1");
        m.add("k100000", "v100000");
        assert_eq!(m.len(), 2);
        m.add("k2", "v2");
        assert_eq!(m.len(), 3);
        m.remove("k2");
        assert_eq!(m.len(), 2);
        m.remove("k1");
        m.remove("k100000");
        assert_eq!(m.len(), 0)
    }
}

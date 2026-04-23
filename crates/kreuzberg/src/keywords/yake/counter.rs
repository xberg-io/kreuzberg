// Vendored from yake-rust 1.0.3 (MIT) — https://github.com/quesurifn/yake-rust
// Replaced hashbrown with ahash.

use std::hash::Hash;

use ahash::AHashSet;

pub(crate) struct Counter<K> {
    list: Vec<K>,
}

impl<K> Default for Counter<K> {
    fn default() -> Self {
        Self { list: Vec::new() }
    }
}

impl<K: Eq + Hash> Counter<K> {
    #[inline]
    pub(crate) fn inc(&mut self, key: K) {
        self.list.push(key);
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// The number of unique keys.
    pub(crate) fn distinct(&self) -> usize {
        self.list.iter().collect::<AHashSet<&K>>().len()
    }

    pub(crate) fn get(&self, key: &K) -> usize {
        self.list.iter().filter(|&k| k == key).count()
    }

    #[inline]
    pub(crate) fn total(&self) -> usize {
        self.list.len()
    }
}

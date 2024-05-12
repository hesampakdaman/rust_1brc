mod key;
mod station;

use fxhash::FxHashMap;
pub use key::Key;
pub use station::Station;
use std::collections::hash_map::{Entry, IntoIter};

pub struct Report(FxHashMap<Key, Station>);

impl Report {
    pub fn get_mut(&mut self, k: &Key) -> Option<&mut Station> {
        self.0.get_mut(k)
    }

    pub fn insert(&mut self, k: Key, st: Station) {
        self.0.insert(k, st);
    }

    pub fn into_iter(self) -> IntoIter<Key, Station> {
        self.0.into_iter()
    }

    pub fn entry(&mut self, k: Key) -> Entry<Key, Station> {
        self.0.entry(k)
    }

    pub fn to_vec(self) -> Vec<Station> {
        self.0.into_values().collect()
    }
}

impl Default for Report {
    fn default() -> Self {
        Self(FxHashMap::default())
    }
}

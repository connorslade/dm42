use std::{
    collections::BTreeMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub struct OrderedMap<T: Ord + Clone, K> {
    map: BTreeMap<T, K>,
    order: Vec<T>,
}

impl<T: Ord + Clone, K> OrderedMap<T, K> {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            order: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: T, value: K) {
        self.map.insert(key.clone(), value);
        self.order.push(key);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &K)> {
        self.order
            .iter()
            .map(move |key| (key, self.map.get(key).unwrap()))
    }

    pub fn values(&self) -> impl Iterator<Item = &K> {
        self.order.iter().map(move |key| self.map.get(key).unwrap())
    }
}

impl<T: Ord + Clone, K> Deref for OrderedMap<T, K> {
    type Target = BTreeMap<T, K>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<T: Ord + Clone, K> DerefMut for OrderedMap<T, K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl<T: Ord + Clone + Debug, K: Debug> Debug for OrderedMap<T, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

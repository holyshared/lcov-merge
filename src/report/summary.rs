use std::collections::btree_map:: { Iter };

pub trait Summary<K, V> {
    fn iter(&self) -> Iter<K, V>;
    fn contains_key(&self, k: &K) -> bool;
    fn get(&self, key: &K) -> Option<&V>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() <= 0
    }
}

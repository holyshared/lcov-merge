use std::collections::btree_map:: { Iter };

pub mod counter;

use summary::counter::Hit;

pub type TestName = String;
pub type SourceFile = String;
pub type LineNumber = u32;
pub type ExecutionCount = u32;
pub type FunctionName = String;
pub type CheckSum = String;

pub trait Summary<K, V> {
    fn iter(&self) -> Iter<K, V>;
    fn contains_key(&self, k: &K) -> bool;
    fn get(&self, key: &K) -> Option<&V>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() <= 0
    }
}

impl Hit for ExecutionCount {
    fn is_hit(&self) -> bool {
        *self > 0
    }
}

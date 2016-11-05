use std::cmp:: { Eq, PartialEq };
use std::clone::Clone;
use std::convert::AsRef;
use std::collections::btree_map:: { BTreeMap, Entry, Iter };
use std::fmt:: { Debug, Formatter, Result };
use std::ops::AddAssign;

pub mod counter;

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
}

#[derive(Clone)]
pub struct AggregateResult<K, V> {
    results: BTreeMap<K, V>
}

impl<K: Ord, V: Clone> AggregateResult<K, V> {
    pub fn new() -> Self {
        AggregateResult {
            results: BTreeMap::new()
        }
    }
    pub fn get_mut(&mut self , key: &K) -> Option<&mut V> {
        self.results.get_mut(key)
    }
    pub fn entry(&mut self, k: K) -> Entry<K, V> {
        self.results.entry(k)
    }
    pub fn len(&self) -> usize {
        self.results.len()
    }
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.results.insert(k, v)
    }
}

impl<K: Ord, V: Clone> Summary<K, V> for AggregateResult<K, V> {
    fn iter(&self) -> Iter<K, V> {
        self.results.iter()
    }
    fn contains_key(&self, k: &K) -> bool {
        self.results.contains_key(k)
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.results.get(key)
    }
}

impl<K: Ord, V: Clone> AsRef<BTreeMap<K, V>> for AggregateResult<K, V> {
    fn as_ref(&self) -> &BTreeMap<K, V> {
        &self.results
    }
}

impl<K: Ord, V: Eq + Clone> PartialEq for AggregateResult<K, V> {
    fn eq(&self, other: &AggregateResult<K, V>) -> bool {
        &self.results == other.as_ref()
    }
}

impl<K: Ord + Debug, V: Debug> Debug for AggregateResult<K, V> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.results)
    }
}

impl<'a, K: Ord + Clone, V: AddAssign + Clone> AddAssign<&'a AggregateResult<K, V>> for AggregateResult<K, V> {
    fn add_assign(&mut self, other: &'a AggregateResult<K, V>) {
        for (unit, count) in other.iter() {
            if self.results.contains_key(unit) {
                let current_count = self.results.get_mut(unit).unwrap();
                *current_count += count.clone();
            } else {
                self.results.insert(unit.clone(), count.clone());
            }
        }
    }
}

#[derive(Clone)]
pub struct AggregateRegistry<K, V> {
    results: BTreeMap<K, V>
}

impl<K: Ord, V: Clone> AggregateRegistry<K, V> {
    pub fn new() -> Self {
        AggregateRegistry {
            results: BTreeMap::new()
        }
    }
    pub fn entry(&mut self, k: K) -> Entry<K, V> {
        self.results.entry(k)
    }
    pub fn len(&self) -> usize {
        self.results.len()
    }
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.results.insert(k, v)
    }
}

impl<K: Ord, V: Clone> Summary<K, V> for AggregateRegistry<K, V> {
    fn iter(&self) -> Iter<K, V> {
        self.results.iter()
    }
    fn contains_key(&self, k: &K) -> bool {
        self.results.contains_key(k)
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.results.get(key)
    }
}

impl<K: Ord, V: Clone> AsRef<BTreeMap<K, V>> for AggregateRegistry<K, V> {
    fn as_ref(&self) -> &BTreeMap<K, V> {
        &self.results
    }
}

impl<K: Ord, V: Eq + Clone> PartialEq for AggregateRegistry<K, V> {
    fn eq(&self, other: &AggregateRegistry<K, V>) -> bool {
        &self.results == other.as_ref()
    }
}

impl<K: Ord + Debug, V: Debug> Debug for AggregateRegistry<K, V> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.results)
    }
}

impl<'a, K: Ord + Clone, V: Clone> AddAssign<&'a AggregateRegistry<K, V>> for AggregateRegistry<K, V> {
    fn add_assign(&mut self, other: &'a AggregateRegistry<K, V>) {
        for (key, value) in other.iter() {
            if self.results.contains_key(key) {
                return;
            }
            self.results.insert(key.clone(), value.clone());
        }
    }
}

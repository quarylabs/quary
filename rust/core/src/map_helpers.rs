use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::Hash;

// Define a trait with the functionalities you need.
pub trait MapLike<K, V> {
    fn insert(&mut self, k: K, v: V) -> Option<V>;
    fn contains_key(&self, k: &K) -> bool;
}

// Implement the trait for HashMap
impl<K, V> MapLike<K, V> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn insert(&mut self, k: K, v: V) -> Option<V> {
        HashMap::insert(self, k, v)
    }

    fn contains_key(&self, k: &K) -> bool {
        HashMap::contains_key(self, k)
    }
}

// Implement the trait for BTreeMap
impl<K, V> MapLike<K, V> for BTreeMap<K, V>
where
    K: Ord,
{
    fn insert(&mut self, k: K, v: V) -> Option<V> {
        BTreeMap::insert(self, k, v)
    }

    fn contains_key(&self, k: &K) -> bool {
        BTreeMap::contains_key(self, k)
    }
}

// Now update your function to use MapLike
pub(crate) fn safe_adder_map<K, V, M>(m: &mut M, key: K, value: V) -> Result<(), String>
where
    K: std::fmt::Debug,
    M: MapLike<K, V>,
{
    if m.contains_key(&key) {
        Err(format!("{:?} already exists in map", key))
    } else {
        m.insert(key, value);
        Ok(())
    }
}

// Define a new trait for set-like collections.
pub trait SetLike<T> {
    fn insert(&mut self, value: T) -> bool;
    fn contains(&self, value: &T) -> bool;
}

// Implement the SetLike trait for HashSet.
impl<T> SetLike<T> for HashSet<T>
where
    T: Eq + Hash,
{
    fn insert(&mut self, value: T) -> bool {
        HashSet::insert(self, value)
    }

    fn contains(&self, value: &T) -> bool {
        HashSet::contains(self, value)
    }
}

pub(crate) fn safe_adder_set<T, S>(s: &mut S, value: T) -> Result<(), String>
where
    T: std::fmt::Debug,
    S: SetLike<T>,
{
    if s.contains(&value) {
        Err(format!("{:?} already exists in set", value))
    } else {
        s.insert(value);
        Ok(())
    }
}

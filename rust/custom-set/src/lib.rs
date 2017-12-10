use std::collections::HashMap;
use std::cmp::Ordering;
use std::hash::Hash;

#[derive(Debug, Eq)]
pub struct CustomSet<T: Hash + Eq> {
    data: HashMap<T, bool>,
}

impl<T: Hash + Eq + Clone> CustomSet<T> {
    pub fn new(input: Vec<T>) -> CustomSet<T> {
        if input.is_empty() {
            return CustomSet { data: HashMap::new() };
        }

        let mut data = HashMap::with_capacity(input.len());

        for n in input {
            data.insert(n, true);
        }

        CustomSet { data: data }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn add(&mut self, item: T) {
        self.data.insert(item, true);
    }

    pub fn contains(&self, _other: &T) -> bool {
        self.data.contains_key(_other)
    }

    pub fn is_subset(&self, other: &CustomSet<T>) -> bool {
        self.data.keys().all(|k| other.data.contains_key(k))
    }

    pub fn union(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let mut result: CustomSet<T> = CustomSet::new(vec![]);

        for k in self.data.keys().chain(other.data.keys()) {
            result.add(k.clone());
        }

        result
    }

    pub fn is_disjoint(&self, other: &CustomSet<T>) -> bool {
        !self.data.keys().any(|k| other.data.contains_key(k))
    }

    pub fn intersection(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let data: Vec<T> = self.data
            .keys()
            .cloned()
            .filter(|k| other.data.contains_key(k))
            .collect();

        Self::new(data)
    }

    pub fn difference(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let data: Vec<_> = self.data
            .keys()
            .cloned()
            .filter(|k| !other.data.contains_key(k))
            .collect();

        CustomSet::new(data)
    }
}

impl<T: Hash + Eq + Ord> Ord for CustomSet<T> {
    fn cmp(&self, other: &CustomSet<T>) -> Ordering {
        let mut keys: Vec<&T> = self.data.keys().collect();
        let mut other_keys: Vec<&T> = other.data.keys().collect();

        keys.sort();
        other_keys.sort();

        keys.cmp(&other_keys)
    }
}

impl<T: Hash + Eq + Ord> PartialOrd for CustomSet<T> {
    fn partial_cmp(&self, other: &CustomSet<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Hash + Eq> PartialEq for CustomSet<T> {
    fn eq(&self, other: &CustomSet<T>) -> bool {
        self.data == other.data
    }
}

use std::collections::{BTreeMap, BTreeSet};

pub struct School {
    data: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> Self {
        Self { data: BTreeMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let names = self.data.entry(grade).or_insert_with(BTreeSet::new);
        names.insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.data.keys().cloned().collect()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        match self.data.get(&grade) {
            Some(set) => Some(set.iter().cloned().collect()),
            None => None,
        }
    }
}

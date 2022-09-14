use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Default)]
pub struct TimeMap {
    m: HashMap<String, BTreeMap<i32, String>>,
}

/*
    981 - Time based key value store
    Note: Might be inefficient due to BTreeMap lack of O(1) pred operation
*/
impl TimeMap {
    pub fn new() -> Self {
        Self::default()
    }

    // Time: O(logn)
    pub fn set(
        &mut self,
        key: String,
        value: String,
        timestamp: i32,
    ) {
        let tree = self.m.entry(key).or_default();
        tree.insert(timestamp, value);
    }

    // Space: O(logn) (Assuming pred() is O(logn))
    pub fn get(
        &self,
        key: String,
        timestamp: i32,
    ) -> String {
        self.m
            .get(&key)
            .map(|tree| {
                if let Some((_, val)) =
                    tree.range(..=timestamp).rev().next()
                {
                    val.clone()
                } else {
                    "".to_string()
                }
            })
            .unwrap_or_default()
    }
}

use std::collections::HashMap;

use rand::prelude::*;

#[derive(Default)]
pub struct RandomizedSet {
    map: HashMap<i32, usize>,
    list: Vec<i32>,
}

/*
    380 - Insert Remove GetRandom O(1)
*/
impl RandomizedSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }
        self.map.insert(val, self.list.len());
        self.list.push(val);
        true
    }

    pub fn remove(&mut self, val: i32) -> bool {
        if let Some(index) = self.map.remove(&val) {
            let j = self.list.len() - 1;
            if j != index {
                self.list.swap(index, j);
                let val = self.list[index];
                self.map.insert(val, index);
            }
            self.list.pop();
            true
        } else {
            false
        }
    }

    pub fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        let i = rng.gen_range(0..self.list.len());
        self.list[i]
    }
}

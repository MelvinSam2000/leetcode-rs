use std::collections::HashMap;

/*
    NOT WORKING!
*/
pub struct LRUCache {
    k_v_map: HashMap<i32, i32>,
    k_p_map: HashMap<i32, usize>,
    p_k_map: HashMap<usize, i32>,
    top_prio: usize,
    cap: usize,
    pub len: usize
}

impl LRUCache {

    pub fn new(capacity: i32) -> Self {
        Self {
            k_v_map: HashMap::new(),
            k_p_map: HashMap::new(),
            p_k_map: HashMap::new(),
            top_prio: capacity as usize - 1,
            cap: capacity as usize,
            len: 0,
        }
    }
    
    pub fn get(&mut self, key: i32) -> i32 {
        self.k_v_map.get(&key).cloned().map(|value| {
            let &pri = self.k_p_map.get(&key).unwrap();
            if pri != self.top_prio {
                self.top_prio = (self.top_prio + 1) % self.cap;
                self.k_p_map.insert(key, self.top_prio);
                self.p_k_map.insert(self.top_prio, key);
            }
            value
        }).unwrap_or(-1)
    }
    
    pub fn put(&mut self, key: i32, value: i32) {
        if self.k_v_map.contains_key(&key) {
            self.k_v_map.insert(key, value);
            let &pri = self.k_p_map.get(&key).unwrap();
            if pri != self.top_prio {
                self.top_prio = (self.top_prio + 1) % self.cap;
                self.k_p_map.insert(key, self.top_prio);
                self.p_k_map.insert(self.top_prio, key);
            }
        } else {
            if self.len == self.cap {
                // evict lru
                let lru = (self.top_prio + 1) % self.cap;
                let evict_key = self.p_k_map.get(&lru).unwrap();
                self.k_p_map.remove(evict_key);
                self.k_v_map.remove(evict_key);
            } else {
                self.len += 1;
            }
            self.top_prio = (self.top_prio + 1) % self.cap;
            self.k_p_map.insert(key, self.top_prio);
            self.p_k_map.insert(self.top_prio, key);
            self.k_v_map.insert(key, value);
        }
    }

    pub fn iter(&self) -> std::vec::IntoIter<(i32, i32)> {

        let pri_range = if self.len < self.cap {
            (0..self.len).rev().collect::<Vec<_>>()
        } else {
            (0..self.cap).map(|i | {
                if i <= self.top_prio {
                    self.top_prio - i
                } else {
                    self.cap - i
                }
            }).collect::<Vec<_>>()
        };
        dbg!(&pri_range);
        pri_range
            .into_iter()
            .map(|pri| self.p_k_map.get(&pri).unwrap())
            .map(|key| (*key, *self.k_v_map.get(key).unwrap()))
            .collect::<Vec<_>>()
            .into_iter()
    }
}
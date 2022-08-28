use std::collections::HashMap;

/*
    460 - LFU Cache
    Time: O(n^2)
    Note: TLEs...
*/
pub struct LFUCache {
    cap: usize,
    len: usize,
    map: HashMap<i32, usize>,
    entries: Vec<Entry>,
}

struct Entry {
    key: i32,
    value: i32,
    freq: usize,
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            cap: capacity as usize,
            len: 0,
            map: HashMap::new(),
            entries: vec![],
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.map
            .get(&key)
            .cloned()
            .map(|index| {
                let entry = &mut self.entries[index];
                entry.freq += 1;
                let val = entry.value;
                self.reorder(index);
                val
            })
            .unwrap_or(-1)
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&index) = self.map.get(&key) {
            self.entries[index].value = value;
            self.entries[index].freq += 1;
            self.reorder(index);
        } else {
            if self.cap == 0 {
                return;
            }
            if self.len == self.cap {
                let entry = self.entries.pop().unwrap();
                self.map.remove(&entry.key);
            } else {
                self.len += 1;
            }
            let entry = Entry {
                key,
                value,
                freq: 0,
            };
            self.entries.push(entry);
            self.map.insert(key, self.len - 1);
            self.reorder(self.len - 1);
        }
    }

    fn reorder(&mut self, mut i: usize) {
        while i != 0 && self.entries[i].freq >= self.entries[i - 1].freq {
            self.map
                .entry(self.entries[i].key)
                .and_modify(|index| *index -= 1);
            self.map
                .entry(self.entries[i - 1].key)
                .and_modify(|index| *index += 1);
            self.entries.swap(i, i - 1);
            i -= 1;
        }
    }
}

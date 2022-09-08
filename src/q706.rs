const N: usize = 10001;

#[derive(Default)]
pub struct MyHashMap {
    map: Vec<Vec<(i32, i32)>>,
}

/*
    706 - Design Hashmap
    All operations are O(a(n)),
    where a(n) is small assuming # of collisions is small
*/
impl MyHashMap {
    pub fn new() -> Self {
        Self {
            map: vec![vec![]; N],
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let hashed_key = key as usize % N;
        if self.get(key) == -1 {
            self.map[hashed_key].push((key, value));
        } else {
            for (k, v) in self.map[hashed_key].iter_mut() {
                if *k == key {
                    *v = value;
                }
            }
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let hashed_key = key as usize % N;
        for &(k, v) in self.map[hashed_key].iter() {
            if k == key {
                return v;
            }
        }
        -1
    }

    pub fn remove(&mut self, key: i32) {
        let hashed_key = key as usize % N;
        let mut j = None;
        for (i, &(k, _)) in
            self.map[hashed_key].iter().enumerate()
        {
            if k == key {
                j = Some(i);
            }
        }
        if let Some(j) = j {
            self.map[hashed_key].remove(j);
        }
    }
}

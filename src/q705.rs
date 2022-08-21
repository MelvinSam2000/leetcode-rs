const N: usize = 10001;

pub struct MyHashSet {
    set: Vec<Vec<i32>>,
}

/*
    705 - Design HashSet
    All operations are O(a(n)),
    where a(n) is small assuming # of collisions is small
*/
impl MyHashSet {
    pub fn new() -> Self {
        Self {
            set: vec![vec![]; N],
        }
    }

    pub fn add(&mut self, key: i32) {
        let hashed_key = key as usize % N;
        if !self.set[hashed_key].contains(&key) {
            self.set[hashed_key].push(key);
        }
    }

    pub fn remove(&mut self, key: i32) {
        let hashed_key = key as usize % N;
        let mut i = None;
        for (j, &k) in self.set[hashed_key].iter().enumerate() {
            if k == key {
                i = Some(j);
            }
        }
        if let Some(i) = i {
            self.set[hashed_key].remove(i);
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        let hashed_key = key as usize % N;
        self.set[hashed_key].contains(&key)
    }
}
